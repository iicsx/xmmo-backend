pub mod middleware;

use crate::handlers::auth::{decode_jwt, get_jwt};
use crate::handlers::user::get_user_by_email;
use crate::models::entities::user::InsertUser;
use axum::{
    extract::Extension,
    http::{header, StatusCode},
    response::Response,
    Json,
};
use serde_json::{json, Value};

use crate::models::entities::auth::JwtPayload;
use sqlx::postgres::PgPool;

pub async fn jwt_login(
    Extension(pool): Extension<PgPool>,
    Json(data): Json<JwtPayload>,
) -> Response<String> {
    let claims = decode_jwt(&data.token);

    match claims {
        Ok(token) => {
            let found_user = match get_user_by_email(&pool, &token.email).await {
                Some(user) => user,
                None => {
                    return Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(json!(Value::Null).to_string())
                        .unwrap();
                }
            };

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(json!({ "user": found_user}).to_string())
                .unwrap()
        }
        Err(_) => {
            let refresh_token = decode_jwt(&data.refresh_token);

            match refresh_token {
                Ok(token) => {
                    let found_user = match get_user_by_email(&pool, &token.email).await {
                        Some(user) => user,
                        None => {
                            return Response::builder()
                                .status(StatusCode::UNAUTHORIZED)
                                .header(header::CONTENT_TYPE, "application/json")
                                .body(json!(Value::Null).to_string())
                                .unwrap();
                        }
                    };

                    let new_token = get_jwt(
                        &InsertUser {
                            email: found_user.email.clone(),
                            password: String::from(""),
                            name: found_user.name.clone(),
                        },
                        true,
                    );

                    Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(
                            json!({
                                "user": found_user,
                                "token": new_token
                            })
                            .to_string(),
                        )
                        .unwrap()
                }
                Err(_) => Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(json!(Value::Null).to_string())
                    .unwrap(),
            }
        }
    }
}

pub async fn refresh_token(
    Extension(pool): Extension<PgPool>,
    Json(data): Json<JwtPayload>,
) -> Response<String> {
    let claims = decode_jwt(&data.refresh_token);
    let user = match get_user_by_email(&pool, &claims.unwrap().email).await {
        Some(user) => user,
        None => {
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header(header::CONTENT_TYPE, "application/json")
                .body(json!(Value::Null).to_string())
                .unwrap();
        }
    };

    let new_token = get_jwt(
        &InsertUser {
            email: user.email.clone(),
            password: String::from(""),
            name: user.name.clone(),
        },
        true,
    );

    match new_token {
        Ok(token) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(json!({ "token": token }).to_string())
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(header::CONTENT_TYPE, "application/json")
            .body(json!(Value::Null).to_string())
            .unwrap(),
    }
}
