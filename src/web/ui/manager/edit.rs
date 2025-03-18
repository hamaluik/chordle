use crate::{
    db::Chore,
    web::{AppState, ui::error::ErrorResponse},
};
use axum::{Form, extract::State};
use color_eyre::eyre::WrapErr;
use jiff::Span;
use maud::Markup;
use serde::Deserialize;

use super::render::RenderErrors;

#[derive(Deserialize)]
pub struct EditChoreForm {
    id: i64,
    name: String,
    interval: String,
    save: Option<String>,
    delete: Option<String>,
}

pub async fn edit_chore(
    State(app_state): State<AppState>,
    Form(form): Form<EditChoreForm>,
) -> Result<Markup, ErrorResponse> {
    let render_errors = if form.save.is_some() {
        handle_save(&app_state, &form)
            .await
            .wrap_err("Failed to handle chore save")?
    } else if form.delete.is_some() {
        handle_delete(&app_state, &form)
            .await
            .wrap_err("Failed to handle chore delete")?;
        None
    } else {
        None
    };

    Ok(super::render::render(&app_state, render_errors)
        .await
        .wrap_err("Failed to render edit chore page")?)
}

async fn handle_save(
    app_state: &AppState,
    form: &EditChoreForm,
) -> Result<Option<RenderErrors>, ErrorResponse> {
    let name_is_valid = !form.name.is_empty() && form.name.len() <= 160;
    let interval: Option<Span> = form.interval.parse().ok();
    let interval_is_valid = interval.is_some();
    if !name_is_valid || !interval_is_valid {
        return Ok(Some(RenderErrors {
            edit_errors: Some((form.id.into(), !name_is_valid, !interval_is_valid)),
            ..Default::default()
        }));
    }
    let interval = interval.expect("interval is valid");

    let chore = Chore {
        id: form.id.into(),
        name: form.name.clone(),
        interval,
    };
    app_state
        .db
        .update_chore(chore)
        .await
        .wrap_err("Failed to update chore")?;

    Ok(None)
}

async fn handle_delete(app_state: &AppState, form: &EditChoreForm) -> Result<(), ErrorResponse> {
    app_state.db.delete_chore(form.id.into()).await?;
    Ok(())
}
