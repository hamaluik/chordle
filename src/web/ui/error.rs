use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;

use super::l10n::Lang;

#[derive(Debug)]
pub struct ErrorResponse;

impl std::error::Error for ErrorResponse {}
impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred")
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response<Body> {
        let page = super::template::page(
            Lang::En,
            "Error",
            maud::html! {
                main {
                    h1 { "An error occurred" }
                    p { "An error occurred while processing your request." }
                }
            },
        );

        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "text/html")
            .body(Body::from(page.into_string()))
            .expect("Can build error response")
    }
}

impl From<color_eyre::eyre::Error> for ErrorResponse {
    fn from(err: color_eyre::eyre::Error) -> Self {
        tracing::error!("Error: {:?}", err);
        Self
    }
}
