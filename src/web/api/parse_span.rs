use axum::{extract::Query, http::StatusCode};
use jiff::Span;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SpanReq {
    pub span: String,
}

pub async fn parse_span(Query(query): Query<SpanReq>) -> StatusCode {
    if query.span.parse::<Span>().is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}
