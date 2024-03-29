pub mod middleware;

use crate::utils;
use axum::{
    extract::Extension,
    http::{header, StatusCode},
    response::Response,
    Json,
};
use serde_json::{json, Value};

use crate::models::entities::JwtPayload;
use sqlx::postgres::PgPool;

pub async fn jwt_login(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<JwtPayload>,
) -> Response<String> {
    let token = utils::decode_jwt(&user.token);

    match token {
        Ok(token) => {
            let user = utils::user::get_user_by_email(&pool, &token.email).await;

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(json!(*user).to_string())
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(header::CONTENT_TYPE, "application/json")
            .body(json!(Value::Null).to_string())
            .unwrap(),
    }
}
