use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

pub async fn styles() -> impl IntoResponse {
    let css = include_str!("styles.css");
    Response::builder()
        .header("Content-Type", "text/css")
        .header("Content-Length", css.len())
        .header("Last-Modified", env!("BUILD_TIME_LAST_MODIFIED"))
        // .header("Cache-Control", "public, max-age=604800")
        .body(Body::from(css))
        .expect("Can build styles response")
}
