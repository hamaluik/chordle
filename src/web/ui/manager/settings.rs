use crate::web::ui::{MANAGER_URI, error::ErrorResponse, l10n::Lang};
use axum::{
    Form,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LanguageForm {
    lang: String,
}

pub async fn change_language(
    jar: CookieJar,
    Form(form): Form<LanguageForm>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let lang = Lang::from_str(&form.lang);
    let jar = jar.add(
        Cookie::build(("lang", lang.to_string()))
            .path("/")
            .http_only(true)
            .build(),
    );

    Ok((jar, Redirect::to(MANAGER_URI)))
}
