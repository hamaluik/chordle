use std::sync::Arc;

use axum::Router;
use color_eyre::Result;
use jiff::Timestamp;
use tokio::net::TcpListener;

use crate::{cli::Cli, db::Db};

mod api;
mod ui;

#[derive(Clone, Debug)]
pub struct AppState {
    pub launch_time: Arc<Timestamp>,
    pub db: Arc<Db>,
}

pub async fn run(cli: Cli, db: Db) -> Result<()> {
    let state = AppState {
        launch_time: Arc::new(Timestamp::now()),
        db: Arc::new(db),
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
