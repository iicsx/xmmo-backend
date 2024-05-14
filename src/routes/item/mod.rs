use crate::handlers::{
    auth::{get_jwt, get_refresh_token},
    crypt::{hash_password, verify_password},
    user::{get_user_by_id, patch_user},
    item::{get_item_by_id},
};

use axum::{
    extract::{Extension, Path},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use serde_json::{json, Value};

use crate::models::entities::item::{ItemStats, ItemMods, ItemReqs, Item};
use sqlx::postgres::PgPool;

pub async fn fetch_item_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> Json<Value> {
    let item = get_item_by_id(&pool, &id).await;

    Json(json!({
      "success": true,
      "data": {
        "item": item
      }
    }))
}