use axum::{
    body::Body,
    extract::{Path, Query},
    response::{IntoResponse, Response},
};
use serde::Deserialize;

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

pub async fn svg_icon(Path(icon): Path<String>) -> impl IntoResponse {
    let icon = match icon.as_str() {
        "undo.svg" => include_str!("undo.svg"),
        "redo.svg" => include_str!("redo.svg"),
        "save.svg" => include_str!("save.svg"),
        "trash.svg" => include_str!("trash.svg"),
        _ => return Response::builder().status(404).body(Body::empty()).unwrap(),
    };

    Response::builder()
        .header("Content-Type", "image/svg+xml")
        .header("Content-Length", icon.len())
        .header("Last-Modified", env!("BUILD_TIME_LAST_MODIFIED"))
        .header("Cache-Control", "public, max-age=604800")
        .body(Body::from(icon))
        .expect("Can build icon response")
}

#[derive(Deserialize)]
pub struct IconQuery {
    pub s: Option<u32>,
    pub ico: Option<bool>,
}

pub async fn favicon(Query(query): Query<IconQuery>) -> impl IntoResponse {
    let icon = include_bytes!("./icon512.png");
    let (len, body) = match query.s {
        Some(512) | None => {
            let len = icon.len();
            (len, Body::from(&icon[..]))
        }
        Some(s) => {
            let img = image::load_from_memory(icon).expect("Can load icon image");
            let img = img.resize(s, s, image::imageops::FilterType::Lanczos3);
            let mut buf = std::io::Cursor::new(Vec::new());

            let format = if query.ico.unwrap_or(false) {
                image::ImageFormat::Ico
            } else {
                image::ImageFormat::Png
            };

            img.write_to(&mut buf, format)
                .expect("Can write resized icon to buffer");
            let buf = buf.into_inner();
            let len = buf.len();
            (len, Body::from(buf))
        }
    };

    Response::builder()
        .header(
            "Content-Type",
            if query.ico.unwrap_or(false) {
                "image/x-icon"
            } else {
                "image/png"
            },
        )
        .header("Content-Length", len)
        .header("Last-Modified", env!("BUILD_TIME_LAST_MODIFIED"))
        .header("Cache-Control", "public, max-age=604800")
        .body(body)
        .expect("Can build icon response")
}

pub async fn manifest() -> impl IntoResponse {
    let manifest = include_str!("manifest.json");
    Response::builder()
        .header("Content-Type", "application/json")
        .header("Content-Length", manifest.len())
        .header("Last-Modified", env!("BUILD_TIME_LAST_MODIFIED"))
        .header("Cache-Control", "public, max-age=604800")
        .body(Body::from(manifest))
        .expect("Can build manifest response")
}
