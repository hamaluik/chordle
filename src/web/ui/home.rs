use crate::{
    db::ChoreEvent,
    web::{
        AppState,
        ui::{MANAGER_URI, REDO_URI, UNDO_URI},
    },
};
use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
use color_eyre::eyre::Context;
use fluent::fluent_args;
use jiff::{Span, SpanTotal, Unit, Zoned};
use maud::{Markup, PreEscaped, html};

use super::{
    HOME_URI,
    error::ErrorResponse,
    l10n::{L10N, Lang},
};

pub async fn home(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<impl IntoResponse, ErrorResponse> {
    let chore_events = app_state
        .db
        .get_all_chore_events()
        .await
        .wrap_err("Failed to get all chores")?;
    let chore_events = sort_chores(chore_events);

    let can_undo = app_state
        .db
        .can_undo_chore_event()
        .await
        .wrap_err("Can check if undo is possible")?;
    let can_redo = app_state
        .db
        .can_redo_chore_event()
        .await
        .wrap_err("Can check if redo is possible")?;

    let accept_language = headers
        .get("accept-language")
        .and_then(|value| value.to_str().ok());
    let lang = Lang::from_accept_language_header_and_cookie(accept_language, &jar);

    let page = super::template::page(
        lang,
        "Chordle",
        html! {
            main.home {
                div.chores {
                    @for chore_event in chore_events {
                        (render_chore(&chore_event, lang, &app_state.l10n))
                    }
                }
            }
            footer {
                div.undo-redo {
                    @if can_undo {
                        form action=(UNDO_URI) method="POST" {
                            button type="submit" class="undo" {
                                img src="/icons/undo.svg" alt=(app_state.l10n.translate(lang, "undo"));
                            }
                        }
                    }
                    @if can_redo {
                        form action=(REDO_URI) method="POST" {
                            button type="submit" class="redo" {
                                img src="/icons/redo.svg" alt=(app_state.l10n.translate(lang, "redo"));
                            }
                        }
                    }
                }
                div {
                    a href=(MANAGER_URI) {
                        (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-cog-icon lucide-cog"><path d="M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z"/><path d="M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"/><path d="M12 2v2"/><path d="M12 22v-2"/><path d="m17 20.66-1-1.73"/><path d="M11 10.27 7 3.34"/><path d="m20.66 17-1.73-1"/><path d="m3.34 7 1.73 1"/><path d="M14 12h8"/><path d="M2 12h2"/><path d="m20.66 7-1.73 1"/><path d="m3.34 17 1.73-1"/><path d="m17 3.34-1 1.73"/><path d="m11 13.73-4 6.93"/></svg>"#))
                        (app_state.l10n.translate(lang, "manage-chores"))
                    }
                }
            }
            (PreEscaped(r#"<script>"#));
            (PreEscaped(include_str!("./static_files/loading-spinner.js")));
            (PreEscaped(include_str!("./static_files/reload.js")));
            (PreEscaped(r#"</script>"#));
        },
    );
    let body = page.into_string();

    Ok(Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .header("Content-Length", body.len()) // String.len() returns bytes not chars
        .header("Cache-Control", "private, max-age=0, no-cache")
        .body(Body::from(body))
        .expect("Can build home response"))
}

pub async fn record_event(
    State(app_state): State<AppState>,
    Path(chore_id): Path<i64>,
) -> Result<Redirect, ErrorResponse> {
    app_state
        .db
        .record_chore_event(chore_id.into())
        .await
        .wrap_err_with(|| format!("Failed to record event for chore with ID: {}", chore_id))?;
    Ok(Redirect::to(HOME_URI))
}

pub async fn undo_event(State(app_state): State<AppState>) -> Result<Redirect, ErrorResponse> {
    app_state
        .db
        .undo_chore_event()
        .await
        .wrap_err("Failed to undo event")?;
    Ok(Redirect::to(HOME_URI))
}

pub async fn redo_event(State(app_state): State<AppState>) -> Result<Redirect, ErrorResponse> {
    app_state
        .db
        .redo_chore_event()
        .await
        .wrap_err("Failed to redo event")?;
    Ok(Redirect::to(HOME_URI))
}

#[tracing::instrument]
fn time_until_next_chore(now: &Zoned, chore_event: &ChoreEvent) -> Span {
    if chore_event.timestamp.is_none() {
        return Span::new().microseconds(0);
    }
    let last_chore = chore_event.timestamp.as_ref().unwrap();
    let interval = chore_event.interval;
    let next_chore = last_chore.saturating_add(interval);
    next_chore.since(now).expect("can calculate time since")
}

#[tracing::instrument]
fn sort_chores(mut chores: Vec<ChoreEvent>) -> Vec<ChoreEvent> {
    let now = Zoned::now();

    chores.sort_by(|a, b| {
        let dt_a = time_until_next_chore(&now, a)
            .total(Unit::Second)
            .expect("can calculate total seconds");
        let dt_b = time_until_next_chore(&now, b)
            .total(Unit::Second)
            .expect("can calculate total seconds");
        dt_a.total_cmp(&dt_b)
    });
    chores
}

fn classify(
    now: &Zoned,
    next_due: &Zoned,
    interval: &Span,
    last_completed: &Option<Zoned>,
) -> &'static str {
    let is_daily = interval
        .total((Unit::Day, Zoned::now().date()))
        .expect("can calculate total days")
        < 1.0;

    if let Some(last_completed) = last_completed {
        if last_completed.date() == now.date() {
            return "chore-done";
        }
    }

    let due_days = next_due
        .since(now)
        .ok()
        .map_or(0.0, |d| {
            d.total(SpanTotal::from(Unit::Day).days_are_24_hours())
                .expect("can calculate total days")
        })
        .ceil() as i64;

    if next_due < now || (due_days <= 1 && !is_daily) {
        "chore-due"
    } else if due_days <= 3 && !is_daily {
        "chore-due-soon"
    } else {
        "chore-due-later"
    }
}

#[tracing::instrument]
fn render_chore(chore_event: &ChoreEvent, lang: Lang, l10n: &L10N) -> Markup {
    let now = Zoned::now();
    let days_since_last = chore_event
        .timestamp
        .as_ref()
        .map(|t| {
            let days = now
                .since(t)
                .expect("can calculate time since")
                .total(SpanTotal::from(Unit::Day).days_are_24_hours())
                .expect("can calculate total days");
            days.floor() as i64
        })
        .unwrap_or_else(|| -1);

    let next = time_until_next_chore(&now, chore_event);
    let class = classify(
        &now,
        &now.saturating_add(next),
        &chore_event.interval,
        &chore_event.timestamp,
    );

    let next_days = next
        .total(SpanTotal::from(Unit::Day).days_are_24_hours())
        .expect("can calculate total days")
        .ceil() as i64;

    let next = match next_days.cmp(&0) {
        std::cmp::Ordering::Equal => html! { (l10n.translate(lang, "due-today")) },
        std::cmp::Ordering::Less => {
            html! { (l10n.translate_with(lang, "due-ago", fluent_args![
                "days" => next_days.abs(),
            ])) }
        }
        std::cmp::Ordering::Greater => {
            html! { (l10n.translate_with(lang, "due-in", fluent_args![
                "days" => next_days,
            ])) }
        }
    };

    let days_since_last_prefix = l10n.translate_with(
        lang,
        "days-ago-prefix",
        fluent_args![
            "days" => days_since_last,
        ],
    );

    html! {
        div.chore style=(format!("view-transition-name: chore-event-{id}", id=chore_event.id)) {
            form action=(format!("/events/{id}", id=chore_event.id)) id=(format!("chore-form-{id}", id=chore_event.id)) class="chore-form" method="POST" {
                p.name {
                    (chore_event.name)
                }
                @if days_since_last_prefix != "﻿" {
                    p.info {
                        (l10n.translate_with(lang, "days-ago-prefix", fluent_args![
                            "days" => days_since_last,
                        ]))
                    }
                }
                button type="submit" class=(class) {
                    (l10n.translate_with(lang, "days-ago-number", fluent_args![
                        "days" => days_since_last,
                    ]))
                }
                p.info {
                    (l10n.translate_with(lang, "days-ago-suffix", fluent_args![
                        "days" => days_since_last,
                    ]))
                    br;
                    (next)
                }
                div.spinner.hidden role="status" {
                    (PreEscaped(include_str!("./static_files/spinner.svg")));
                }
            }
        }
    }
}
