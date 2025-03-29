use axum::{Form, extract::State, http::HeaderMap};
use axum_extra::extract::CookieJar;
use color_eyre::eyre::Context;
use jiff::{Span, civil::Date, tz::TimeZone};
use maud::Markup;
use serde::Deserialize;

use crate::web::{
    AppState,
    ui::{error::ErrorResponse, l10n::Lang},
};

#[derive(Deserialize)]
pub struct NewChoreForm {
    name: String,
    interval: String,
    history: Option<String>,
}

pub async fn new_chore(
    headers: HeaderMap,
    jar: CookieJar,
    State(app_state): State<AppState>,
    Form(form): Form<NewChoreForm>,
) -> Result<Markup, ErrorResponse> {
    let name_is_valid = !form.name.is_empty() && form.name.len() <= 160;
    let interval: Option<Span> = form.interval.parse().ok();
    let interval_is_valid = interval.is_some();

    let accept_language = headers
        .get("accept-language")
        .and_then(|value| value.to_str().ok());
    let lang = Lang::from_accept_language_header_and_cookie(accept_language, &jar);

    if !name_is_valid || !interval_is_valid {
        return Ok(super::render::render(
            lang,
            &app_state,
            Some(super::render::RenderErrors {
                create_has_name_error: !name_is_valid,
                create_has_interval_error: !interval_is_valid,
                ..Default::default()
            }),
        )
        .await?);
    }
    let interval = interval.expect("interval is valid");
    let chore_id = match app_state.db.create_chore(&form.name, interval).await {
        Ok(id) => id,
        Err(e) => {
            tracing::warn!("Failed to create chore: {e:#?}");
            return Ok(super::render::render(
                lang,
                &app_state,
                Some(super::render::RenderErrors {
                    create_created_ok: Some(false),
                    ..Default::default()
                }),
            )
            .await?);
        }
    };

    if let Some(history) = form.history {
        if let Ok(history) = history.parse::<Date>() {
            let history = history.to_zoned(TimeZone::system()).wrap_err_with(|| {
                format!(
                    "Failed to create history timestamp for date: {history}",
                    history = history
                )
            })?;

            if let Err(e) = app_state
                .db
                .record_chore_event_when(chore_id, history)
                .await
            {
                tracing::warn!("Failed to record chore event when creating a new chore: {e:#?}");
            }
        } else {
            tracing::warn!("Failed to parse history date, not recording event history: {history}");
        }
    }

    Ok(super::render::render(
        lang,
        &app_state,
        Some(super::render::RenderErrors {
            create_created_ok: Some(true),
            ..Default::default()
        }),
    )
    .await?)
}
