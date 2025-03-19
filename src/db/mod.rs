use color_eyre::{Result, eyre::Context};
use jiff::{Span, Zoned};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::Path;
use types::DbChore;

mod types;
pub use types::{Chore, ChoreEvent, ChoreId};

#[derive(Clone, Debug)]
pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn new(db_path: &Path) -> Result<Db> {
        let connection_options = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        tracing::info!(db_path = ?db_path, "Connecting to database");
        let pool = SqlitePool::connect_with(connection_options)
            .await
            .wrap_err_with(|| {
                format!(
                    "Failed to connect to SQLite database {path}",
                    path = db_path.display()
                )
            })?;

        tracing::info!("Running migrations...");
        sqlx::migrate!()
            .run(&pool)
            .await
            .wrap_err("Failed to run migrations")?;

        Ok(Db { pool })
    }

    pub async fn create_chore(&self, name: &str, interval: Span) -> Result<ChoreId> {
        let interval = interval.to_string();

        let id: i64 = sqlx::query_scalar!(
            r#"
insert into chores (name, interval)
values (?, ?)
returning id
            "#,
            name,
            interval,
        )
        .fetch_one(&self.pool)
        .await
        .wrap_err("Failed to create chore")?;

        Ok(id.into())
    }

    pub async fn update_chore(&self, chore: Chore) -> Result<()> {
        let db_chore: DbChore = chore.into();

        sqlx::query!(
            r#"
update chores
set name = ?, interval = ?
where id = ?
            "#,
            db_chore.name,
            db_chore.interval,
            db_chore.id,
        )
        .execute(&self.pool)
        .await
        .wrap_err("Failed to update chore")?;

        Ok(())
    }

    pub async fn delete_chore(&self, id: ChoreId) -> Result<()> {
        let dbid: i64 = id.into();

        sqlx::query!(
            r#"
delete from chores
where id = ?
            "#,
            dbid,
        )
        .execute(&self.pool)
        .await
        .wrap_err("Failed to delete chore")?;

        Ok(())
    }

    pub async fn get_all_chores(&self) -> Result<Vec<Chore>> {
        let chores = sqlx::query_as!(
            types::DbChore,
            r#"
select id, name, interval
from chores
order by name asc
            "#
        )
        .fetch_all(&self.pool)
        .await
        .wrap_err("Failed to get all chores")?;

        chores.into_iter().map(|chore| chore.try_into()).collect()
    }

    pub async fn get_all_chore_events(&self) -> Result<Vec<ChoreEvent>> {
        // assert times in the query, see
        // https://docs.rs/sqlx/0.8.3/sqlx/macro.query_as.html#troubleshooting-error-mismatched-types
        // for more information
        let chores = sqlx::query_as!(
            types::DbChoreEvent,
            r#"
select
    chores.id as "id!",
    chores.name as "name!",
    chores.interval as "interval!",
    events.timestamp
from
    chores
left join
    (select
        chore_id,
        max(timestamp) as timestamp
     from
        events
     group by
        chore_id) as events
on chores.id = events.chore_id
"#
        )
        .fetch_all(&self.pool)
        .await
        .wrap_err("Failed to get all chores")?;

        chores.into_iter().map(|chore| chore.try_into()).collect()
    }

    pub async fn record_chore_event(&self, chore_id: ChoreId) -> Result<()> {
        let dbid: i64 = chore_id.into();
        let timestamp = Zoned::now().to_string();

        sqlx::query!(
            r#"
insert into events (chore_id, timestamp)
values (?, ?)
            "#,
            dbid,
            timestamp,
        )
        .execute(&self.pool)
        .await
        .wrap_err("Failed to record chore event")?;

        sqlx::query!(r#"delete from redo_events"#,)
            .execute(&self.pool)
            .await
            .wrap_err("Failed to clear redo events")?;

        Ok(())
    }

    pub async fn record_chore_event_when(&self, chore_id: ChoreId, timestamp: Zoned) -> Result<()> {
        let dbid: i64 = chore_id.into();
        let timestamp = timestamp.to_string();

        sqlx::query!(
            r#"
insert into events (chore_id, timestamp)
values (?, ?)
            "#,
            dbid,
            timestamp,
        )
        .execute(&self.pool)
        .await
        .wrap_err("Failed to record chore event")?;

        sqlx::query!(r#"delete from redo_events"#,)
            .execute(&self.pool)
            .await
            .wrap_err("Failed to clear redo events")?;

        Ok(())
    }

    pub async fn can_undo_chore_event(&self) -> Result<bool> {
        let can_undo = sqlx::query!(
            r#"
select count(*) as count
from events
            "#
        )
        .fetch_one(&self.pool)
        .await
        .wrap_err("Failed to check if can undo chore event")?;

        Ok(can_undo.count > 0)
    }

    pub async fn undo_chore_event(&self) -> Result<bool> {
        let most_recent_chore_event = sqlx::query_as!(
            types::DbEvent,
            r#"
select chore_id, timestamp
from events
order by timestamp desc
limit 1
            "#
        )
        .fetch_optional(&self.pool)
        .await
        .wrap_err("Failed to get most recent chore event")?;

        if most_recent_chore_event.is_none() {
            return Ok(false);
        }
        let most_recent_chore_event = most_recent_chore_event.unwrap();

        let mut transaction = self
            .pool
            .begin()
            .await
            .wrap_err("Failed to start transaction")?;

        sqlx::query!(
            r#"
delete from events
where chore_id = ? and timestamp = ?
"#,
            most_recent_chore_event.chore_id,
            most_recent_chore_event.timestamp,
        )
        .execute(&mut *transaction)
        .await
        .wrap_err("Failed to delete most recent chore event")?;

        sqlx::query!(
            r#"
insert into redo_events (chore_id, timestamp)
values (?, ?)
"#,
            most_recent_chore_event.chore_id,
            most_recent_chore_event.timestamp,
        )
        .execute(&mut *transaction)
        .await
        .wrap_err("Failed to record redo event")?;

        transaction
            .commit()
            .await
            .wrap_err("Failed to commit undo transaction")?;

        Ok(true)
    }

    pub async fn can_redo_chore_event(&self) -> Result<bool> {
        let can_redo = sqlx::query!(
            r#"
select count(*) as count
from redo_events
            "#
        )
        .fetch_one(&self.pool)
        .await
        .wrap_err("Failed to check if can redo chore event")?;

        Ok(can_redo.count > 0)
    }

    pub async fn redo_chore_event(&self) -> Result<bool> {
        let most_recent_redo_chore_event = sqlx::query_as!(
            types::DbEvent,
            r#"
select chore_id, timestamp
from redo_events
order by timestamp desc
limit 1
            "#
        )
        .fetch_optional(&self.pool)
        .await
        .wrap_err("Failed to get most recent redo chore event")?;

        if most_recent_redo_chore_event.is_none() {
            return Ok(false);
        }
        let most_recent_redo_chore_event = most_recent_redo_chore_event.unwrap();

        let mut transaction = self
            .pool
            .begin()
            .await
            .wrap_err("Failed to start transaction")?;

        sqlx::query!(
            r#"
delete from redo_events
where chore_id = ? and timestamp = ?
"#,
            most_recent_redo_chore_event.chore_id,
            most_recent_redo_chore_event.timestamp,
        )
        .execute(&mut *transaction)
        .await
        .wrap_err("Failed to delete most recent redo chore event")?;

        sqlx::query!(
            r#"
insert into events (chore_id, timestamp)
values (?, ?)
"#,
            most_recent_redo_chore_event.chore_id,
            most_recent_redo_chore_event.timestamp,
        )
        .execute(&mut *transaction)
        .await
        .wrap_err("Failed to record redo event")?;

        transaction
            .commit()
            .await
            .wrap_err("Failed to commit redo transaction")?;

        Ok(true)
    }
}
