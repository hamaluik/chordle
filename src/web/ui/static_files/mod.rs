use crate::web::AppState;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

fn handle_matching_etag<S>(id: S, headers: &HeaderMap, app_state: &AppState) -> Option<Response>
where
    S: AsRef<str>,
{
    let id = id.as_ref();
    let cache = app_state.cache.read().expect("Can get read lock on cache");
    if headers
        .get("If-None-Match")
        .is_some_and(|val| val.to_str().is_ok_and(|val| cache.etag_matches(id, val)))
    {
        return Some(
            Response::builder()
                .status(304)
                .header("Last-Modified", env!("BUILD_TIME_LAST_MODIFIED"))
                .header(
                    "ETag",
                    format!(
                        "\"{etag}\"",
                        etag = cache.get_etag(id).expect("Can get etag for icon")
                    ),
                )
                .body(Body::empty())
                .expect("Can build not modified response"),
        );
    }
    None
}

pub async fn styles(headers: HeaderMap, State(app_state): State<AppState>) -> impl IntoResponse {
    if let Some(response) = handle_matching_etag("styles.css", &headers, &app_state) {
        return response;
    }

    let css = include_str!("styles.css");
    let etag = format!("{:x}", md5::compute(css));
    {
        let mut cache = app_state
            .cache
            .write()
            .expect("Can get write lock on cache");
        cache.set_etag("styles.css", etag.clone());
    }

    Response::builder()
        .header("Content-Type", "text/css; charset=utf-8")
        .header("Content-Length", css.len())
        .header("Last-Modified", env!("BUILD_TIME_LAST_MODIFIED"))
        .header("ETag", format!("\"{etag}\""))
        .header("Cache-Control", "public, max-age=604800")
        .body(Body::from(css))
        .expect("Can build styles response")
}

pub async fn svg_icon(
    Path(icon): Path<String>,
    headers: HeaderMap,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = handle_matching_etag(&icon, &headers, &app_state) {
        return response;
    }

    let icon_contents = match icon.as_str() {
        "undo.svg" => include_str!("undo.svg"),
        "redo.svg" => include_str!("redo.svg"),
        "save.svg" => include_str!("save.svg"),
        "trash.svg" => include_str!("trash.svg"),
        _ => return Response::builder().status(404).body(Body::empty()).unwrap(),
    };

    let etag = format!("{:x}", md5::compute(&icon));
    {
        let mut cache = app_state
            .cache
            .write()
            .expect("Can get write lock on cache");
        cache.set_etag(icon, etag.clone());
    }

    Response::builder()
        .header("Content-Type", "image/svg+xml; charset=utf-8")
        .header("Content-Length", icon_contents.len())
        .header("Last-Modified", env!("BUILD_TIME_LAST_MODIFIED"))
        .header("ETag", format!("\"{etag}\""))
        .header("Cache-Control", "public, max-age=604800")
        .body(Body::from(icon_contents))
        .expect("Can build icon response")
}

#[derive(Deserialize, Debug)]
pub struct IconQuery {
    pub s: Option<u32>,
    pub ico: Option<bool>,
}

pub async fn favicon(headers: HeaderMap, State(app_state): State<AppState>) -> impl IntoResponse {
    let query = IconQuery {
        s: Some(16),
        ico: Some(true),
    };
    app_icon(Query(query), headers, State(app_state)).await
}

pub async fn app_icon(
    Query(query): Query<IconQuery>,
    headers: HeaderMap,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let etag_id = format!("favicon/{query:?}");
    if let Some(response) = handle_matching_etag(&etag_id, &headers, &app_state) {
        return response;
    }

    let icon = include_bytes!("./icon512.png");
    let (len, etag, body) = match query.s {
        Some(512) | None => {
            let len = icon.len();
            (
                len,
                format!("{:x}", md5::compute(icon)),
                Body::from(&icon[..]),
            )
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
            (len, format!("{:x}", md5::compute(icon)), Body::from(buf))
        }
    };

    {
        let mut cache = app_state
            .cache
            .write()
            .expect("Can get write lock on cache");
        cache.set_etag(etag_id, etag.clone());
    }

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
        .header("ETag", format!("\"{etag}\""))
        .header("Cache-Control", "public, max-age=604800")
        .body(body)
        .expect("Can build icon response")
}

pub async fn manifest() -> impl IntoResponse {
    let manifest = include_str!("manifest.json");
    Response::builder()
        .header("Content-Type", "application/json; charset=utf-8")
        .header("Content-Length", manifest.len())
        .header("Last-Modified", env!("BUILD_TIME_LAST_MODIFIED"))
        .header("Cache-Control", "public, max-age=604800")
        .body(Body::from(manifest))
        .expect("Can build manifest response")
}
