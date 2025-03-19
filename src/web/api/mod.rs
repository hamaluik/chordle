use super::AppState;
use axum::{Router, routing::get};

mod health_check;
mod parse_span;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check::health_check))
        .route("/parse_span", get(parse_span::parse_span))
}
