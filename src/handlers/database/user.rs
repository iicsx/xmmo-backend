use crate::utils::auth::{get_jwt, get_refresh_token};
use crate::utils::crypt::{hash_password, verify_password};

use axum::{
    extract::{Extension, Path},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use serde_json::{json, Value};

use crate::models::entities::user::{InsertUser, LoginCheckUser, LoginUser, Permission, User};
use sqlx::postgres::PgPool;

pub async fn get_user_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> Json<Value> {
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
        WHERE \"user\".id = $1",
        id.parse::<i32>().unwrap()
    )
    .fetch_one(&pool)
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

pub async fn single_insert_user(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<InsertUser>,
) -> Response<String> {
    let user_query = "INSERT INTO \"user\"(name, email, password) VALUES($1, $2, $3) RETURNING id";
    let password = hash_password(&user.password);

    let res: (i32,) = sqlx::query_as(&user_query)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&password)
        .fetch_one(&pool)
        .await
        .unwrap();

    if res.0 == 0 {
        return Response::builder()
            .status(StatusCode::CONFLICT)
            .header(header::CONTENT_TYPE, "application/json")
            .body(
                json!({
                  "success": false,
                  "data": {
                    "message": "User already exists"
                  }
                })
                .to_string(),
            )
            .unwrap_or_default();
    }

    // Insert default user permission
    let permission_query = "INSERT INTO \"user_permission\"(user_id, permission_id) VALUES($1, $2)";
    sqlx::query(&permission_query)
        .bind(res.0)
        .bind(1)
        .execute(&pool)
        .await
        .unwrap();

    let token = match get_jwt(&user, false) {
        Ok(token) => token,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(
                    json!({
                      "success": false,
                      "data": {
                        "message": "Failed to generate token"
                      }
                    })
                    .to_string(),
                )
                .unwrap_or_default();
        }
    };

    let refresh_token = match get_refresh_token(&user) {
        Ok(token) => token,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(
                    json!({
                      "success": false,
                      "data": {
                        "message": "Failed to generate refresh token"
                      }
                    })
                    .to_string(),
                )
                .unwrap_or_default();
        }
    };

    // insert refresh token into database
    let refresh_token_query = "INSERT INTO \"refresh_token\"(user_id, token) VALUES($1, $2)";
    sqlx::query(&refresh_token_query)
        .bind(res.0)
        .bind(&refresh_token)
        .execute(&pool)
        .await
        .unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(
            json!({
              "success": true,
              "data": {
                "token": token,
                "refresh_token": refresh_token
              }
            })
            .to_string(),
        )
        .unwrap_or_default()
}

pub async fn login_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<LoginUser>,
) -> Response<String> {
    let row = sqlx::query!(
        "SELECT
            id,
            name,
            email,
            password,
            banned
        FROM \"user\"
        WHERE email = $1",
        payload.email,
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND);

    let row = match row {
        Ok(row) => row,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header(header::CONTENT_TYPE, "application/json")
                .body(
                    json!({
                      "success": false,
                      "data": {
                        "message": "email_not_found"
                      }
                    })
                    .to_string(),
                )
                .unwrap_or_default()
        }
    };

    let user = LoginCheckUser {
        id: row.id.clone() as u32,
        name: row.name.clone(),
        email: row.email.clone(),
        password: row.password.clone(),
        banned: row.banned,
    };

    if user.banned {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(header::CONTENT_TYPE, "application/json")
            .body(
                json!({
                  "success": false,
                  "data": {
                    "message": "user_banned"
                  }
                })
                .to_string(),
            )
            .unwrap_or_default();
    }

    if !verify_password(&payload.password, &user.password) {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(header::CONTENT_TYPE, "application/json")
            .body(
                json!({
                  "success": false,
                  "data": {
                    "message": "password_incorrect"
                  }
                })
                .to_string(),
            )
            .unwrap_or_default();
    }

    let token = match get_jwt(
        &InsertUser {
            name: user.name.clone(),
            email: user.email.clone(),
            password: user.password.clone(),
        },
        false,
    ) {
        Ok(token) => token,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(
                    json!({
                      "success": false,
                      "data": {
                        "message": "Failed to generate token"
                      }
                    })
                    .to_string(),
                )
                .unwrap_or_default();
        }
    };

    let refresh_token = match get_refresh_token(&InsertUser {
        name: user.name.clone(),
        email: user.email.clone(),
        password: user.password.clone(),
    }) {
        Ok(token) => token,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(
                    json!({
                      "success": false,
                      "data": {
                        "message": "Failed to generate refresh token"
                      }
                    })
                    .to_string(),
                )
                .unwrap_or_default();
        }
    };

    let refresh_token_query = "
        INSERT INTO \"refresh_token\"(user_id, token)
        VALUES($1, $2) ON CONFLICT (user_id) DO UPDATE SET token = $2";

    sqlx::query(&refresh_token_query)
        .bind(row.id)
        .bind(&refresh_token)
        .execute(&pool)
        .await
        .unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(
            json!({
              "success": true,
              "data": {
                "token": token,
                "refresh_token": refresh_token
              }
            })
            .to_string(),
        )
        .unwrap_or_default()
}
