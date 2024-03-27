use axum::http::{header, StatusCode};
use axum::response::Response;
use serde_json::json;

pub async fn status() -> Response<String> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(json!({ "success": true }).to_string())
        .unwrap_or_default()
}

pub async fn not_implemented() -> (StatusCode, String) {
    (
        StatusCode::NOT_IMPLEMENTED,
        String::from("This route is not implemented yet"),
    )
}
