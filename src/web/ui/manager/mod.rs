use super::error::ErrorResponse;
use crate::web::AppState;
use axum::extract::State;
use color_eyre::Result;
use maud::Markup;

mod edit;
mod new;
mod render;

pub use edit::edit_chore;
pub use new::new_chore;

/// GET handler for the manager page
pub async fn manager_home(State(app_state): State<AppState>) -> Result<Markup, ErrorResponse> {
    Ok(render::render(&app_state, Default::default())
        .await
        .map_err(ErrorResponse::from)?)
}
