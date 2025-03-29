use std::any::Any;

use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use maud::html;
use tower_http::catch_panic::CatchPanicLayer;

use super::AppState;

pub mod cache;
mod error;
mod home;
mod manager;
mod static_files;
mod template;

static HOME_URI: &str = "/";
static EVENT_URI: &str = "/events/{chore_id}";
static UNDO_URI: &str = "/events/undo";
static REDO_URI: &str = "/events/redo";
static MANAGER_URI: &str = "/manager";
static MANAGER_EDIT_URI: &str = "/manager/edit";
static MANAGER_NEW_URI: &str = "/manager/new";
static STYLES_URI: &str = "/styles.css";

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(HOME_URI, get(home::home))
        .route(UNDO_URI, post(home::undo_event))
        .route(REDO_URI, post(home::redo_event))
        .route(EVENT_URI, post(home::record_event))
        .route(MANAGER_URI, get(manager::manager_home))
        .route(MANAGER_EDIT_URI, post(manager::edit_chore))
        .route(MANAGER_NEW_URI, post(manager::new_chore))
        .route(STYLES_URI, get(static_files::styles))
        .route("/icons/{icon}", get(static_files::svg_icon))
        .route("/manifest.json", get(static_files::manifest))
        .route("/icon.png", get(static_files::app_icon))
        .route("/favicon.ico", get(static_files::favicon))
        .layer(CatchPanicLayer::custom(handle_panic))
        .fallback(handler_404)
}

fn handle_panic(_err: Box<dyn Any + Send + 'static>) -> Response<Body> {
    // err can be ignored because color_eyre will log it
    let page = template::page(
        "Internal Server Error",
        html! {
            main {
                h1 { "Internal Server Error" }
                p { "Sorry bud." }
            }
        },
    );
    let page = page.into_string();
    let page_len_bytes = page.as_bytes().len();

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header("Content-Type", "text/html; charset=utf-8")
        .header("Content-Length", page_len_bytes)
        .body(Body::from(page))
        .unwrap()
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        template::page(
            "404 Not Found",
            html! {
                main {
                    h1 { "404 Not Found" }
                    p { "The page you are looking for does not exist." }
                }
            },
        ),
    )
}
