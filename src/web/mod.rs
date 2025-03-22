use std::sync::{Arc, RwLock};

use axum::Router;
use color_eyre::Result;
use jiff::Timestamp;
use tokio::net::TcpListener;
use ui::cache::Cache;

use crate::{cli::Cli, db::Db};

mod api;
mod ui;

#[derive(Clone, Debug)]
pub struct AppState {
    pub launch_time: Arc<Timestamp>,
    pub db: Arc<Db>,
    pub cache: Arc<RwLock<Cache>>,
}

pub async fn run(cli: Cli, db: Db) -> Result<()> {
    let state = AppState {
        launch_time: Arc::new(Timestamp::now()),
        db: Arc::new(db),
        cache: Arc::new(RwLock::new(Cache::new())),
    };

    let app = Router::new()
        .merge(ui::routes())
        .nest("/api", api::routes())
        .with_state(state);

    tracing::info!("Starting chordle web server on {}", cli.bind);
    let listener = TcpListener::bind(cli.bind).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
