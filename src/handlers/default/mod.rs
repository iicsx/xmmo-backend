use axum::http::StatusCode;

pub async fn status() -> StatusCode {
    StatusCode::OK
}

pub async fn not_implemented() -> (StatusCode, String) {
    (
        StatusCode::NOT_IMPLEMENTED,
        String::from("This route is not implemented yet"),
    )
}
