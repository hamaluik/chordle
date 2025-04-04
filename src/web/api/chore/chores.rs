use crate::{
    db::Chore,
    web::{AppState, api::error::ApiErrorResponse},
};
use axum::{Json, extract::State};
use color_eyre::eyre::WrapErr;

pub async fn get_chores(
    State(state): State<AppState>,
) -> Result<Json<Vec<Chore>>, ApiErrorResponse> {
    let chores = state
        .db
        .get_all_chores()
        .await
        .wrap_err("Failed to get all chores")?;
    Ok(Json(chores))
}
