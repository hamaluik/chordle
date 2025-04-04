use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use color_eyre::eyre::Error as EyreError;

#[derive(Debug)]
pub struct ApiErrorResponse;

impl std::error::Error for ApiErrorResponse {}
impl std::fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred")
    }
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .expect("Can build error response")
    }
}

impl From<EyreError> for ApiErrorResponse {
    fn from(err: EyreError) -> Self {
        tracing::error!("API Error: {:?}", err);
        Self
    }
}
