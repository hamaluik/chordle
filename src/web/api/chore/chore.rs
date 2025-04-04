use crate::{
    db::ChoreId,
    web::{AppState, api::error::ApiErrorResponse},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use color_eyre::eyre::WrapErr;

pub async fn get_chore(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Response, ApiErrorResponse> {
    let chore = state
        .db
        .get_chore(ChoreId(id))
        .await
        .wrap_err_with(|| format!("Failed to get chore {id}",))?;
    Ok(match chore {
        Some(chore) => (StatusCode::OK, Json(chore)).into_response(),
        None => (StatusCode::NOT_FOUND, Json(())).into_response(),
    })
}
