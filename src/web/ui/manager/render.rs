use crate::{
    db::Chore,
    web::{
        AppState,
        ui::{MANAGER_EDIT_URI, MANAGER_NEW_URI},
    },
};
use color_eyre::{Result, eyre::Context};
use maud::{Markup, html};

fn render_chore(chore: &Chore, has_name_error: bool, has_interval_error: bool) -> Markup {
    html! {
        form method="post" action=(MANAGER_EDIT_URI) {
            input type="hidden" name="id" value=(chore.id.0);
            div.chore {
                div.form-item {
                    label for="name" { "Name" }
                    input type="text" name="name" value=(chore.name) placeholder="Take out the trash";
                    @if has_name_error {
                        span.form-item-error { "Invalid chore name, must not be empty and ≤ 160 characters." }
                    }
                }
                div.form-item {
                    label for="interval" { "Interval" }
                    input type="text" name="interval" value=(format!("{interval:#}", interval = chore.interval)) placeholder="2w 4d";
                    @if has_interval_error {
                        span.form-item-error {
                            "Invalid interval, see "
                            a href="https://docs.rs/jiff/latest/jiff/fmt/friendly/index.html" target="_blank" {
                                "jiff::fmt::friendly ↗"
                            }
                            " for formatting help"
                        }
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

fn render_chores<I>(chores: I) -> Markup
where
    I: Iterator,
    I::Item: AsRef<Chore>,
{
    html!(
        div.chores {
            @for chore in chores {
                (render_chore(chore.as_ref(), false, false))
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
                    input type="text" name="name" placeholder="Take out the trash";
                    @if has_name_error {
                        span.form-item-error { "Invalid chore name, must not be empty and ≤ 160 characters." }
                    }
                }
                div.form-item {
                    label for="interval" { "Interval" }
                    input type="text" name="interval" placeholder="2w 4d";
                    @if has_interval_error {
                        span.form-item-error {
                            "Invalid interval, see "
                            a href="https://docs.rs/jiff/latest/jiff/fmt/friendly/index.html" target="_blank" {
                                "jiff::fmt::friendly ↗"
                            }
                            " for formatting help"
                        }
                    }
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
    pub create_has_name_error: bool,
    pub create_has_interval_error: bool,
    pub create_created_ok: Option<bool>,
}

pub async fn render(app_state: &AppState, errors: RenderErrors) -> Result<Markup> {
    let chores = app_state
        .db
        .get_all_chores()
        .await
        .wrap_err("Failed to get chores")?;

    Ok(html! {
        main.manager {
            h1 { "Manage" }
            fieldset {
                legend { "Chores" }
                (render_chores(chores.iter()))
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
        }
    })
}
