use super::AppState;
use axum::{Router, routing::get};

mod health_check;

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check::health_check))
}
