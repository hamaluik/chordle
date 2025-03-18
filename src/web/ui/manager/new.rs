use axum::{Form, extract::State};
use jiff::Span;
use maud::Markup;
use serde::Deserialize;

use crate::web::{AppState, ui::error::ErrorResponse};

#[derive(Deserialize)]
pub struct NewChoreForm {
    name: String,
    interval: String,
}

pub async fn new_chore(
    State(app_state): State<AppState>,
    Form(form): Form<NewChoreForm>,
) -> Result<Markup, ErrorResponse> {
    let name_is_valid = !form.name.is_empty() && form.name.len() <= 160;
    let interval: Option<Span> = form.interval.parse().ok();
    let interval_is_valid = interval.is_some();

    if !name_is_valid || !interval_is_valid {
        return Ok(super::render::render(
            &app_state,
            super::render::RenderErrors {
                create_has_name_error: !name_is_valid,
                create_has_interval_error: !interval_is_valid,
                ..Default::default()
            },
        )
        .await?);
    }
    let interval = interval.expect("interval is valid");
    if let Err(e) = app_state.db.create_chore(&form.name, interval).await {
        tracing::warn!("Failed to create chore: {e:#?}");
        return Ok(super::render::render(
            &app_state,
            super::render::RenderErrors {
                create_created_ok: Some(false),
                ..Default::default()
            },
        )
        .await?);
    }

    Ok(super::render::render(
        &app_state,
        super::render::RenderErrors {
            create_created_ok: Some(true),
            ..Default::default()
        },
    )
    .await?)
}
