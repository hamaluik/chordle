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

pub async fn undo_svg() -> impl IntoResponse {
    let svg = include_str!("undo.svg");
    Response::builder()
        .header("Content-Type", "image/svg+xml")
        .header("Content-Length", svg.len())
        .header("Last-Modified", env!("BUILD_TIME_LAST_MODIFIED"))
        .header("Cache-Control", "public, max-age=604800")
        .body(Body::from(svg))
        .expect("Can build undo svg response")
}

pub async fn redo_svg() -> impl IntoResponse {
    let svg = include_str!("redo.svg");
    Response::builder()
        .header("Content-Type", "image/svg+xml")
        .header("Content-Length", svg.len())
        .header("Last-Modified", env!("BUILD_TIME_LAST_MODIFIED"))
        .header("Cache-Control", "public, max-age=604800")
        .body(Body::from(svg))
        .expect("Can build undo svg response")
}
