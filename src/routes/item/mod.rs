use crate::handlers::item::get_item_by_id;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Response,
};
use serde_json::json;

use sqlx::postgres::PgPool;

pub async fn fetch_item_by_id(
    Extension(pool): Extension<PgPool>,
    Path(item_id): Path<String>,
) -> Response<String> {
    let item = match get_item_by_id(&pool, &item_id).await {
        Some(item) => item,
        None => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(
                    json!({
                      "success": false,
                      "data": {
                        "message": "Item not found"
                      }
                    })
                    .to_string(),
                )
                .unwrap();
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(
            json!({
              "success": true,
              "data": {
                "item": item
              }
            })
            .to_string(),
        )
        .unwrap()
}
