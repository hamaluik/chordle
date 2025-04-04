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

pub async fn get_chore_stats(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Response, ApiErrorResponse> {
    let stats = crate::stats::get_stats(&state.db, ChoreId(id))
        .await
        .wrap_err_with(|| format!("Failed to get stats for chore {id}",))?;
    Ok(match stats {
        Some(stats) => Json(stats).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    })
}
