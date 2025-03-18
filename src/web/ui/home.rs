use crate::web::{AppState, ui::MANAGER_URI};
use axum::extract::State;
use maud::{Markup, html};

/// GET handler for the home page
pub async fn home(State(_state): State<AppState>) -> Markup {
    super::template::page(
        "Home",
        html! {
            main {
            }
            footer {
                { a href=(MANAGER_URI) { "Manage Chores →" } }
            }
        },
    )
}
