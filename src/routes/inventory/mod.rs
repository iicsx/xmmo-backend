use crate::handlers::inventory::get_inventory_by_id;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Response,
};
use serde_json::json;

use sqlx::postgres::PgPool;

pub async fn fetch_inventory_by_id(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<String>,
) -> Response<String> {
    let inventory = match get_inventory_by_id(&pool, &user_id).await {
        Some(inventory) => inventory,
        None => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(
                    json!({
                      "success": false,
                      "data": {
                        "message": "No inventory found"
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
                "inventory": inventory
              }
            })
            .to_string(),
        )
        .unwrap()
}
