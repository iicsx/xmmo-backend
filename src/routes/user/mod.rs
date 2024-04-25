use crate::handlers::{
    auth::{get_jwt, get_refresh_token},
    crypt::{hash_password, verify_password},
    user::get_user_by_id,
};

use axum::{
    extract::{Extension, Path},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use serde_json::{json, Value};

use crate::models::entities::user::{InsertUser, LoginCheckUser, LoginUser, User};
use sqlx::postgres::PgPool;

pub async fn fetch_user_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> Json<Value> {
    let user = get_user_by_id(&pool, &id).await;

    Json(json!({
      "success": true,
      "data": {
        "user": user
      }
    }))
}

pub async fn patch_user_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(user): Json<User>,
) -> Response<String> {
    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return {
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(
                        json!({
                          "success": false,
                          "data": {
                            "message": "Invalid user id"
                          }
                        })
                        .to_string(),
                    )
                    .unwrap()
            }
        }
    };

    // patch user details
    let query = "UPDATE \"user_details\" SET strength = $1, defence = $2, dexterity = $3, current_energy = $4, max_energy = $5, current_hp = $6, max_hp = $7, exp = $8, gold = $9, profession_exp = $10 WHERE user_id = $11";
    sqlx::query(&query)
        .bind(user.details.strength as i32)
        .bind(user.details.defence as i32)
        .bind(user.details.dexterity as i32)
        .bind(user.details.current_energy as i32)
        .bind(user.details.max_energy as i32)
        .bind(user.details.current_hp as i32)
        .bind(user.details.max_hp as i32)
        .bind(user.details.exp as i32)
        .bind(user.details.gold as i32)
        .bind(user.details.profession_exp as i32)
        .bind(&id)
        .execute(&pool)
        .await
        .unwrap();

    // patch user stats
    let query = "UPDATE \"user_stats\" SET ledges_grabbed = $1, npc_kills = $2, items_dropped = $3, height = $4 WHERE user_id = $5";
    sqlx::query(&query)
        .bind(user.stats.ledges_grabbed as i32)
        .bind(user.stats.npc_kills as i32)
        .bind(user.stats.items_dropped as i32)
        .bind(user.stats.height as i32)
        .bind(&id)
        .execute(&pool)
        .await
        .unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .body(
            json!({
              "success": true,
              "data": {
                "message": "Successfully updated user"
                }
            })
            .to_string(),
        )
        .unwrap()
}

async fn insert_user_permission(pool: &PgPool, user_id: i32) {
    let permission_query = "INSERT INTO \"user_permission\"(user_id, permission_id) VALUES($1, $2)";
    sqlx::query(&permission_query)
        .bind(user_id)
        .bind(1)
        .execute(pool)
        .await
        .unwrap();
}

async fn insert_user_stats(pool: &PgPool, user_id: i32) {
    let user_stats_query = "INSERT INTO \"user_stats\"(user_id) VALUES($1)";
    sqlx::query(&user_stats_query)
        .bind(user_id)
        .execute(pool)
        .await
        .unwrap();
}

async fn insert_user_details(pool: &PgPool, user_id: i32) {
    let user_details_query = "INSERT INTO \"user_details\"(user_id) VALUES($1)";
    sqlx::query(&user_details_query)
        .bind(user_id)
        .execute(pool)
        .await
        .unwrap();
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

    insert_user_details(&pool, res.0).await;
    insert_user_stats(&pool, res.0).await;
    insert_user_permission(&pool, res.0).await;

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
