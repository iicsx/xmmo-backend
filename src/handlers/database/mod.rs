use axum::{http::StatusCode, response::Json, Extension};
use serde_json::{json, Value};

use crate::models::entities::{InsertUser, User};
use sqlx::postgres::PgPool;

pub async fn get_all_users(Extension(pool): Extension<PgPool>) -> Result<Json<Value>, StatusCode> {
    let rows = sqlx::query!("SELECT * FROM \"user\"")
        .fetch_all(&pool)
        .await
        .unwrap();

    if rows.is_empty() {
        return Err(StatusCode::NO_CONTENT);
    }

    let users = rows
        .iter()
        .map(|row| User {
            id: row.id.clone() as u32,
            name: row.name.clone(),
            email: row.email.clone(),
            password: row.password.clone(),
            created_at: row.created_at.to_string(),
            last_login: row.last_login.to_string(),
            banned: row.banned,
        })
        .collect::<Vec<User>>();

    Ok(Json(json!(users)))
}

pub async fn multiple_insert_user(
    Extension(pool): Extension<PgPool>,
    Json(users): Json<Vec<User>>,
) -> StatusCode {
    let mut insert_statements: Vec<String> = vec![];

    for user in users.into_iter() {
        let insert_statement = format!("('{}', '{}')", user.name, user.email);

        insert_statements.push(insert_statement);
    }

    let base_query = String::from("INSERT INTO \"user\"(name, email) VALUES");

    let query = format!("{} {}", base_query, insert_statements.join(","));

    let res = sqlx::query(&query).execute(&pool).await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}

pub async fn single_insert_user(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<InsertUser>,
) -> StatusCode {
    let query = "INSERT INTO \"user\"(name, email, password) VALUES($1, $2, $3)";

    let res = sqlx::query(&query)
        .bind(user.name)
        .bind(user.email)
        .bind(user.password)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}
