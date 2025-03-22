use axum::{
    Router,
    routing::{get, post},
};

use super::AppState;

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
        .route("/undo.svg", get(static_files::undo_svg))
        .route("/redo.svg", get(static_files::redo_svg))
        .route("/manifest.json", get(static_files::manifest))
        .route("/icon.png", get(static_files::icon))
}
