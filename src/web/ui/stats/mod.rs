use axum::{extract::State, http::HeaderMap};
use axum_extra::extract::CookieJar;
use color_eyre::eyre::Context;
use maud::{Markup, html};

use crate::{db::Chore, stats::ChoreStats, web::AppState};

use super::{error::ErrorResponse, l10n::Lang};

pub async fn stats_page(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<Markup, ErrorResponse> {
    let accept_language = headers
        .get("accept-language")
        .and_then(|value| value.to_str().ok());
    let lang = Lang::from_accept_language_header_and_cookie(accept_language, &jar);

    let chores = app_state
        .db
        .get_all_chores()
        .await
        .wrap_err("Failed to get all chores for stats page")?;
    let mut stats: Vec<(Chore, ChoreStats)> = Vec::with_capacity(chores.len());
    for chore in chores.into_iter() {
        let chore_stats = crate::stats::get_stats(&app_state.db, chore.id)
            .await
            .wrap_err_with(|| format!("Failed to get stats for chore {id}", id = chore.id))?;
        if let Some(chore_stats) = chore_stats {
            stats.push((chore, chore_stats));
        } else {
            tracing::warn!("Chore {id} has no stats", id = chore.id);
        }
    }
    stats.sort_by(|a, b| {
        b.1.num_completed
            .cmp(&a.1.num_completed)
            .then_with(|| b.1.num_overdue.cmp(&a.1.num_overdue))
            .then_with(|| a.0.name.cmp(&b.0.name))
    });

    Ok(super::template::page(
        lang,
        "Stats",
        html! {
            main.stats {
                h1 { (app_state.l10n.translate(lang, "stats")) }
                table {
                    thead {
                        tr {
                            th { (app_state.l10n.translate(lang, "chore")) }
                            th { (app_state.l10n.translate(lang, "times-completed")) }
                            th { (app_state.l10n.translate(lang, "times-overdue")) }
                            th { (app_state.l10n.translate(lang, "mean-days-overdue")) }
                        }
                    }
                    tbody {
                        @for stat in stats.iter() {
                            tr {
                                td { (stat.0.name) }
                                td { (stat.1.num_completed) }
                                td { (stat.1.num_overdue) }
                                td { (format!("{mean:.1} ± {var:.2}", mean=stat.1.mean_overdue_days, var=stat.1.variance_overdue_days.sqrt())) }
                            }
                        }
                    }
                }
            }
            footer {
                { a href="/" { (app_state.l10n.translate(lang, "back-to-chores")) } }
            }
        },
    ))
}
