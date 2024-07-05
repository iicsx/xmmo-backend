use crate::handlers::inventory::{find_item, get_inventory_by_id, insert_inventory_item};
use crate::models::entities::inventory::Inventory;

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

pub async fn add_item_to_inventory(
    Extension(pool): Extension<PgPool>,
    Path(inventory): Path<Inventory>,
) -> Response<String> {
    let _item = match insert_inventory_item(
        &pool,
        &inventory.user_id.to_string(),
        &inventory.item.id.to_string(),
        &inventory.quantity,
        inventory.level.as_ref(),
    )
    .await
    {
        Some(_) => true,
        None => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(
                    json!({
                      "success": false,
                      "data": {
                        "message": "No item found"
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
            })
            .to_string(),
        )
        .unwrap()
}

pub async fn user_has_item(
    Extension(pool): Extension<PgPool>,
    Path(inventory): Path<Inventory>,
) -> Response<String> {
    let item = match find_item(
        &pool,
        &inventory.user_id.to_string(),
        &inventory.item.id.to_string(),
    )
    .await
    {
        Some(_) => true,
        None => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(
                    json!({
                      "success": false,
                      "data": {
                        "message": "No item found"
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
                "has_item": item
              }
            })
            .to_string(),
        )
        .unwrap()
}
