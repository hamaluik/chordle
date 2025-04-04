use std::any::Any;

use super::AppState;
use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode},
    routing::get,
};
use tower_http::catch_panic::CatchPanicLayer;

mod chore;
mod error;
mod health_check;
mod parse_span;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check::health_check))
        .route("/parse_span", get(parse_span::parse_span))
        .route("/chore/:id", get(chore::get_chore))
        .route("/chore/:id/stats", get(chore::get_chore_stats))
        .route("/chores", get(chore::get_chores))
        .layer(CatchPanicLayer::custom(handle_panic))
        .fallback(handler_404)
}

fn handle_panic(_err: Box<dyn Any + Send + 'static>) -> Response<Body> {
    // err can be ignored because color_eyre will log it
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .expect("Internal Server Error response should be valid")
}

async fn handler_404() -> StatusCode {
    StatusCode::NOT_FOUND
}
