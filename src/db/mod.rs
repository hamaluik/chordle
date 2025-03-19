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

    // pub async fn get_chore(&self, id: ChoreId) -> Result<Chore> {
    //     let dbid: i64 = id.into();
    //     let chore = sqlx::query_as!(types::DbChore, r#"SELECT * FROM chores WHERE id = ?"#, dbid)
    //         .fetch_one(&self.pool)
    //         .await
    //         .wrap_err_with(|| format!("Failed to get chore with ID {dbid}"))?;
    //
    //     Ok(chore.try_into()?)
    // }
    //
    // pub async fn get_most_recent_event_for_chore(
    //     &self,
    //     chore_id: ChoreId,
    // ) -> Result<Option<Event>> {
    //     let dbid: i64 = chore_id.into();
    //     let event = sqlx::query_as!(
    //         types::DbEvent,
    //         r#"
    //         SELECT * FROM events
    //         WHERE chore_id = ?
    //         ORDER BY timestamp DESC
    //         LIMIT 1
    //     "#,
    //         dbid
    //     )
    //     .fetch_optional(&self.pool)
    //     .await
    //     .wrap_err_with(|| format!("Failed to get most recent event for chore {dbid}"))?;
    //
    //     event.map(|event| event.try_into()).transpose()
    // }

    pub async fn create_chore(&self, name: &str, interval: Span) -> Result<()> {
        let interval = interval.to_string();

        sqlx::query!(
            r#"
insert into chores (name, interval)
values (?, ?)
            "#,
            name,
            interval,
        )
        .execute(&self.pool)
        .await
        .wrap_err("Failed to create chore")?;

        Ok(())
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

        Ok(())
    }
}
