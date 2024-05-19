use crate::handlers::{
    item::{get_item_by_id},
};

use axum::{
    extract::{Extension, Path},
    Json,
};
use serde_json::{json, Value};

use sqlx::postgres::PgPool;

pub async fn fetch_item_by_id(
    Extension(pool): Extension<PgPool>,
    Path(item_id): Path<String>,
) -> Json<Value> {
    let item = get_item_by_id(&pool, &item_id).await;

    Json(json!({
      "success": true,
      "data": {
        "item": item
      }
    }))
}