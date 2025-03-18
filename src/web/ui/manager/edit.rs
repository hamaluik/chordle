use axum::extract::State;
use maud::Markup;

use crate::web::{AppState, ui::error::ErrorResponse};

pub async fn edit_chore(State(_app_state): State<AppState>) -> Result<Markup, ErrorResponse> {
    todo!()
}
