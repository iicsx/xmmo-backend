use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

use crate::models::entities::{Permission, User};
use sqlx::postgres::PgPool;

pub async fn get_user_by_email(pool: &PgPool, email: &String) -> Json<Value> {
    let row = sqlx::query!(
        "SELECT 
            \"user\".id,
            \"user\".name,
            \"user\".email,
            \"user\".created_at,
            \"user\".last_login,
            \"user\".muted,
            \"user\".locked,
            \"user\".banned,    
            \"permission\".id AS \"permission_id\",
            \"permission\".name AS \"permission_name\",
            \"permission\".description AS \"permission_description\"
        FROM \"user\" 
        JOIN \"user_permission\" ON \"user\".id = \"user_permission\".user_id 
        JOIN \"permission\" ON \"user_permission\".permission_id = \"permission\".id 
        WHERE \"user\".email = $1",
        email
    )
    .fetch_one(pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)
    .unwrap();

    let user = User {
        id: row.id.clone() as u32,
        name: row.name.clone(),
        email: row.email.clone(),
        password: None,
        created_at: row.created_at.to_string(),
        last_login: row.last_login.to_string(),
        permission: Permission {
            id: row.permission_id.clone() as u32,
            name: row.permission_name.clone(),
            description: row.permission_description.clone(),
        },
        muted: row.muted,
        locked: row.locked,
        banned: row.banned,
    };

    Json(json!(user))
}
