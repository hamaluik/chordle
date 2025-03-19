use crate::{
    db::ChoreEvent,
    web::{AppState, ui::MANAGER_URI},
};
use axum::{
    extract::{Path, State},
    response::Redirect,
};
use color_eyre::eyre::Context;
use jiff::{Span, SpanTotal, Unit, Zoned};
use maud::{Markup, html};

use super::{HOME_URI, error::ErrorResponse};

pub async fn home(State(app_state): State<AppState>) -> Result<Markup, ErrorResponse> {
    let chore_events = app_state
        .db
        .get_all_chore_events()
        .await
        .wrap_err("Failed to get all chores")?;
    let chore_events = sort_chores(chore_events);

    Ok(super::template::page(
        "Home",
        html! {
            main.home {
                div.chores {
                    @for chore_event in chore_events {
                        (render_chore(&chore_event))
                    }
                }
            }
            footer {
                { a href=(MANAGER_URI) { "Manage Chores →" } }
            }
        },
    ))
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

fn time_until_next_chore(now: &Zoned, chore_event: &ChoreEvent) -> Span {
    if chore_event.timestamp.is_none() {
        return Span::new().microseconds(0);
    }
    let last_chore = chore_event.timestamp.as_ref().unwrap();
    let interval = chore_event.interval;
    let next_chore = last_chore.saturating_add(interval);
    next_chore.since(now).expect("can calculate time since")
}

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

fn render_chore(chore_event: &ChoreEvent) -> Markup {
    let now = Zoned::now();
    tracing::debug!(chore_event = ?chore_event, "Rendering chore");
    let days_since_last = chore_event
        .timestamp
        .as_ref()
        .map(|t| {
            let days = now
                .since(t)
                .expect("can calculate time since")
                .total(SpanTotal::from(Unit::Day).days_are_24_hours())
                .expect("can calculate total days");
            days.floor().to_string()
        })
        .unwrap_or_else(|| "∞".to_string());

    let next = time_until_next_chore(&now, chore_event)
        .total(SpanTotal::from(Unit::Day).days_are_24_hours())
        .expect("can calculate total days")
        .ceil() as i64;

    let next = if next == 0 {
        html! { "(due today)" }
    } else if next < 0 {
        html! { "(due " (next.abs()) " day" @if next.abs() != 1 { "s" } " ago)" }
    } else {
        html! { "(due in " (next) " day" @if next != 1 { "s" } ")" }
    };

    html! {
        div.chore {
            form action=(format!("/events/{id}", id=chore_event.id)) method="POST" {
                p.name {
                    (chore_event.name)
                }
                button type="submit" {
                    (days_since_last)
                }
                p.info {
                    "days ago"
                    br;
                    (next)
                }
            }
        }
    }
}
