use crate::{
    db::{Chore, ChoreId},
    web::{
        AppState,
        ui::{
            MANAGER_EDIT_URI, MANAGER_LANGUAGE_URI, MANAGER_NEW_URI,
            l10n::{L10N, Lang},
            template,
        },
    },
};
use color_eyre::{Result, eyre::Context};
use fluent::fluent_args;
use maud::{Markup, PreEscaped, html};

fn render_chore(
    chore: &Chore,
    has_name_error: bool,
    has_interval_error: bool,
    lang: Lang,
    l10n: &L10N,
) -> Markup {
    html! {
        div.form-item {
            input type="text" form=(format!("chore-form-{id}", id=chore.id.0)) .name-field .is-invalid[has_name_error] name="name" value=(chore.name) placeholder=(l10n.translate(lang, "name-placeholder")) required minlength="1" maxlength="160";
            span.form-item-error { (l10n.translate(lang, "invalid-chore-name")) }
        }
        div.form-item {
            input type="text" form=(format!("chore-form-{id}", id=chore.id.0)) .interval-field .is-invalid[has_interval_error] name="interval" value=(format!("{interval:#}", interval = chore.interval)) placeholder="2w 4d" required minlength="2" maxlength="160";
            span.form-item-error {
                (PreEscaped(l10n.translate_with(lang, "invalid-interval", fluent_args![
                                                "link" => r#"<a href="https://docs.rs/jiff/latest/jiff/fmt/friendly/index.html" target="_blank">jiff::fmt::friendly ↗</a>"#,
                ])))
            }
        }
        div.form-item.form-item-button {
            button type="submit"
                form=(format!("chore-form-{id}", id=chore.id.0))
                name="save"
                value="Save"
                alt=(l10n.translate(lang, "save"))
                title=(l10n.translate(lang, "save")) {
                img src="/icons/save.svg" alt=(l10n.translate(lang, "save"));
            }
        }
        div.form-item.form-item-button {
            button type="submit"
                form=(format!("chore-form-{id}", id=chore.id.0))
                name="delete"
                value="Delete"
                alt=(l10n.translate(lang, "delete"))
                title=(l10n.translate(lang, "delete")) {
                img src="/icons/trash.svg" alt=(l10n.translate(lang, "delete"));
            }
        }
        hr;
    }
}

fn render_chore_forms<I>(chores: I) -> Markup
where
    I: Iterator,
    I::Item: AsRef<Chore>,
{
    html!(
        @for chore in chores {
            form id=(format!("chore-form-{id}", id=chore.as_ref().id.0)) method="post" action=(MANAGER_EDIT_URI) {
                input type="hidden" name="id" value=(chore.as_ref().id.0);
            }
        }
    )
}

fn render_chores<I>(
    chores: I,
    edit_errors: Option<(ChoreId, bool, bool)>,
    lang: Lang,
    l10n: &L10N,
) -> Markup
where
    I: Iterator,
    I::Item: AsRef<Chore>,
{
    html!(
        div.chore-list {
            @for chore in chores {
                ({
                    let (name_error, interval_error) = match edit_errors {
                        Some((id, name_error, interval_error)) if id == chore.as_ref().id => (name_error, interval_error),
                        _ => (false, false),
                    };
                    render_chore(chore.as_ref(), name_error, interval_error, lang, l10n)
                })
            }
        }
    )
}

fn render_new_chore(
    has_name_error: bool,
    has_interval_error: bool,
    created_ok: Option<bool>,
    lang: Lang,
    l10n: &L10N,
) -> Markup {
    html! {
        form method="post" action=(MANAGER_NEW_URI) {
            div.chore-list {
                div.form-item {
                    label for="name" { (l10n.translate(lang, "name")) }
                    input type="text" .name-field .is-invalid[has_name_error] name="name" placeholder=(l10n.translate(lang, "name-placeholder")) required minlength="1" maxlength="160";
                    span.form-item-error { (l10n.translate(lang, "invalid-chore-name")) }
                }
                div.form-item {
                    label for="interval" { (l10n.translate(lang, "interval")) }
                    input type="text" .interval-field .is-invalid[has_interval_error] name="interval" placeholder="2w 4d" required minlength="2" maxlength="160";
                    span.form-item-error {
                        (PreEscaped(l10n.translate_with(lang, "invalid-interval", fluent_args![
                            "link" => r#"<a href="https://docs.rs/jiff/latest/jiff/fmt/friendly/index.html" target="_blank">jiff::fmt::friendly ↗</a>"#,
                        ])))
                    }
                }
                div.form-item {
                    label for="history" { (l10n.translate(lang, "history")) }
                    input type="date" name="history" id="history";
                }
                div.form-item {
                    label for="submit" { (l10n.translate(lang, "create")) }
                    button type="submit" alt=(l10n.translate(lang, "create")) title=(l10n.translate(lang, "create")) {
                        img src="/icons/new.svg" alt=(l10n.translate(lang, "create"));
                    }
                }
                @if let Some(created_ok) = created_ok {
                    @if created_ok {
                        p { (l10n.translate(lang, "chore-created"))}
                    }
                    @else {
                        p { (l10n.translate(lang, "failed-to-create-chore"))}
                    }
                }
            }
        }
    }
}

fn render_language_select_form(lang: Lang, l10n: &L10N) -> Markup {
    html! {
        form method="post" action=(MANAGER_LANGUAGE_URI) {
            div.language-select {
                div.form-item {
                    label for="lang" { (l10n.translate(lang, "language")) }
                    select name="lang" {
                        option value="en" selected[lang == Lang::En] { "English" }
                        option value="fr" selected[lang == Lang::Fr] { "Français" }
                    }
                }
                div.form-item {
                    label for="submit" { (l10n.translate(lang, "save")) }
                    button type="submit" alt=(l10n.translate(lang, "save")) title=(l10n.translate(lang, "save")) {
                        img src="/icons/save.svg" alt=(l10n.translate(lang, "save"));
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

pub async fn render(
    lang: Lang,
    app_state: &AppState,
    errors: Option<RenderErrors>,
) -> Result<Markup> {
    let chores = app_state
        .db
        .get_all_chores()
        .await
        .wrap_err("Failed to get chores")?;
    let errors = errors.unwrap_or_default();

    Ok(template::page(
        lang,
        &app_state.l10n.translate(lang, "manage-chores"),
        html! {
            main.manager {
                h1 style="view-transition-name: manage-header" {
                    (app_state.l10n.translate(lang, "manage-chores"))
                }
                fieldset {
                    legend { (app_state.l10n.translate(lang, "new-chore")) }
                    (render_new_chore(
                            errors.create_has_name_error,
                            errors.create_has_interval_error,
                            errors.create_created_ok,
                            lang, &app_state.l10n))
                }
                fieldset {
                    legend { (app_state.l10n.translate(lang, "chores")) }
                    (render_chore_forms(chores.iter()))
                    (render_chores(chores.iter(), errors.edit_errors, lang, &app_state.l10n))
                }
                fieldset {
                    legend { (app_state.l10n.translate(lang, "settings")) }
                    (render_language_select_form(lang, &app_state.l10n))
                }
            }
            footer {
                { a href="/" { (app_state.l10n.translate(lang, "back-to-chores")) } }
                { a href="https://github.com/hamaluik/chordle" alt=(app_state.l10n.translate(lang, "chordle-source-code")) target="_blank" { (app_state.l10n.translate(lang, "chordle-source-code")) } }
            }
            (PreEscaped(r#"<script>"#))
            (PreEscaped(include_str!("./input-errors.js")))
            (PreEscaped(r#"</script>"#))
        },
    ))
}
