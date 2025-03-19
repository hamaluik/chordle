use crate::{
    db::{Chore, ChoreId},
    web::{
        AppState,
        ui::{MANAGER_EDIT_URI, MANAGER_NEW_URI, template},
    },
};
use color_eyre::{Result, eyre::Context};
use maud::{Markup, PreEscaped, html};

fn render_chore(chore: &Chore, has_name_error: bool, has_interval_error: bool) -> Markup {
    html! {
        form method="post" action=(MANAGER_EDIT_URI) {
            input type="hidden" name="id" value=(chore.id.0);
            div.chore style=(format!("view-transition-name: chore-{id}", id=chore.id.0)) {
                div.form-item {
                    label for="name" { "Name" }
                    input type="text" .name-field .is-invalid[has_name_error] name="name" value=(chore.name) placeholder="Take out the trash" required minlength="1" maxlength="160";
                    span.form-item-error { "Invalid chore name, must not be empty and ≤ 160 characters." }
                }
                div.form-item {
                    label for="interval" { "Interval" }
                    input type="text" .interval-field .is-invalid[has_interval_error] name="interval" value=(format!("{interval:#}", interval = chore.interval)) placeholder="2w 4d" required minlength="2" maxlength="160";
                    span.form-item-error {
                        "Invalid interval, see "
                        a href="https://docs.rs/jiff/latest/jiff/fmt/friendly/index.html" target="_blank" {
                            "jiff::fmt::friendly ↗"
                        }
                        " for formatting help"
                    }
                }
                div.form-item {
                    input type="submit" name="save" value="Save";
                    input type="submit" name="delete" value="Delete";
                }
            }
        }
    }
}

fn render_chores<I>(chores: I, edit_errors: Option<(ChoreId, bool, bool)>) -> Markup
where
    I: Iterator,
    I::Item: AsRef<Chore>,
{
    html!(
        div.chores {
            @for chore in chores {
                ({
                    let (name_error, interval_error) = match edit_errors {
                        Some((id, name_error, interval_error)) if id == chore.as_ref().id => (name_error, interval_error),
                        _ => (false, false),
                    };
                    render_chore(chore.as_ref(), name_error, interval_error)
                })
            }
        }
    )
}

fn render_new_chore(
    has_name_error: bool,
    has_interval_error: bool,
    created_ok: Option<bool>,
) -> Markup {
    html! {
        form method="post" action=(MANAGER_NEW_URI) {
            div.chore {
                div.form-item {
                    label for="name" { "Name" }
                    input type="text" .name-field .is-invalid[has_name_error] name="name" placeholder="Take out the trash" required minlength="1" maxlength="160";
                    span.form-item-error { "Invalid chore name, must not be empty and ≤ 160 characters." }
                }
                div.form-item {
                    label for="interval" { "Interval" }
                    input type="text" .interval-field .is-invalid[has_interval_error] name="interval" placeholder="2w 4d" required minlength="2" maxlength="160";
                    span.form-item-error {
                        "Invalid interval, see "
                        a href="https://docs.rs/jiff/latest/jiff/fmt/friendly/index.html" target="_blank" {
                            "jiff::fmt::friendly ↗"
                        }
                        " for formatting help"
                    }
                }
                div.form-item {
                    label for="history" { "History" }
                    input type="date" name="history" id="history";
                }
                div.form-item {
                    input type="submit" value="Create";
                }
                @if let Some(created_ok) = created_ok {
                    @if created_ok {
                        p { "Chore created successfully!" }
                    }
                    @else {
                        p { "Failed to create chore" }
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct RenderErrors {
    pub edit_errors: Option<(ChoreId, bool, bool)>,
    pub create_has_name_error: bool,
    pub create_has_interval_error: bool,
    pub create_created_ok: Option<bool>,
}

pub async fn render(app_state: &AppState, errors: Option<RenderErrors>) -> Result<Markup> {
    let chores = app_state
        .db
        .get_all_chores()
        .await
        .wrap_err("Failed to get chores")?;
    let errors = errors.unwrap_or_default();

    Ok(template::page(
        "Manage Chores",
        html! {
            main.manager {
                h1 style="view-transition-name: manage-header" { "Manage" }
                fieldset {
                    legend { "Chores" }
                    (render_chores(chores.iter(), errors.edit_errors))
                }
                fieldset {
                    legend { "New Chore" }
                    (render_new_chore(
                            errors.create_has_name_error,
                            errors.create_has_interval_error,
                            errors.create_created_ok))
                }
            }
            footer {
                { a href="/" { "← Back to Chores" } }
                { a href="https://github.com/hamaluik/chordle" alt="chordle on GitHub" target="_blank" { "chordle Source Code ↗" } }
            }
            (PreEscaped(r#"<script>"#))
            (PreEscaped(include_str!("./input-errors.js")))
            (PreEscaped(r#"</script>"#))
        },
    ))
}
