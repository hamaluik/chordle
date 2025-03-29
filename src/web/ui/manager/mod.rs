use super::{error::ErrorResponse, l10n::Lang};
use crate::web::AppState;
use axum::{extract::State, http::HeaderMap};
use axum_extra::extract::CookieJar;
use color_eyre::Result;
use maud::Markup;

mod edit;
mod new;
mod render;
mod settings;

pub use edit::edit_chore;
pub use new::new_chore;
pub use settings::change_language;

/// GET handler for the manager page
pub async fn manager_home(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<Markup, ErrorResponse> {
    let accept_language = headers
        .get("accept-language")
        .and_then(|value| value.to_str().ok());
    let lang = Lang::from_accept_language_header_and_cookie(accept_language, &jar);

    render::render(lang, &app_state, Default::default())
        .await
        .map_err(ErrorResponse::from)
}
