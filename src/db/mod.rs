use color_eyre::{Result, eyre::Context};
use jiff::Span;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::Path;

mod types;
pub use types::Chore;

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

    //     pub async fn get_all_chore_events(&self) -> Result<Vec<ChoreEvent>> {
    //         let chores = sqlx::query_as!(
    //             types::DbChoreEvent,
    //             r#"
    // select
    //     chores.id, chores.name, chores.interval, events.timestamp
    // from chores
    // left join events on chores.id=events.chore_id
    // where events.timestamp=(
    //     select max(timestamp)
    //     from events
    //     where events.chore_id=chores.id
    // )"#
    //         )
    //         .fetch_all(&self.pool)
    //         .await
    //         .wrap_err("Failed to get all chores")?;
    //
    //         chores.into_iter().map(|chore| chore.try_into()).collect()
    //     }
}
