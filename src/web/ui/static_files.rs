use axum::{
    body::Body,
    extract::Path,
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

pub async fn icon(Path(icon): Path<String>) -> impl IntoResponse {
    let iphone_icon = include_bytes!("./icons/apple-touch-icon-iphone-60x60.png");
    let ipad_icon = include_bytes!("./icons/apple-touch-icon-ipad-76x76.png");
    let iphone_retina_icon = include_bytes!("./icons/apple-touch-icon-iphone-retina-120x120.png");
    let ipad_retina_icon = include_bytes!("./icons/apple-touch-icon-ipad-retina-152x152.png");
    let icon_192 = include_bytes!("./icons/192.png");
    let icon_512 = include_bytes!("./icons/512.png");

    let (len, body) = match icon.as_str() {
        "iphone.png" => (iphone_icon.len(), Body::from(&iphone_icon[..])),
        "ipad.png" => (ipad_icon.len(), Body::from(&ipad_icon[..])),
        "iphone-retina.png" => (
            iphone_retina_icon.len(),
            Body::from(&iphone_retina_icon[..]),
        ),
        "ipad-retina.png" => (ipad_retina_icon.len(), Body::from(&ipad_retina_icon[..])),
        "192.png" => (icon_192.len(), Body::from(&icon_192[..])),
        _ => (icon_512.len(), Body::from(&icon_512[..])),
    };
    Response::builder()
        .header("Content-Type", "image/png")
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
