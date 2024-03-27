use crate::utils;
use axum::{
    extract::{Extension, Path},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use serde_json::{json, Value};

use crate::models::entities::{InsertUser, Permission, User};
use sqlx::postgres::PgPool;

pub async fn get_all_users(Extension(pool): Extension<PgPool>) -> Result<Json<Value>, StatusCode> {
    let rows = sqlx::query!(
        "SELECT 
            \"user\".*,
            \"permission\".id AS \"permission_id\",
            \"permission\".name AS \"permission_name\",
            \"permission\".description AS \"permission_description\"
        FROM \"user\" 
        JOIN \"user_permission\" ON \"user\".id = \"user_permission\".user_id 
        JOIN \"permission\" ON \"user_permission\".permission_id = \"permission\".id"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)
    .unwrap();

    if rows.is_empty() {
        return Err(StatusCode::NOT_FOUND);
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
            permission: Permission {
                id: row.permission_id.clone() as u32,
                name: row.permission_name.clone(),
                description: row.permission_description.clone(),
            },
            banned: row.banned,
        })
        .collect::<Vec<User>>();

    Ok(Json(json!(users)))
}

pub async fn get_user_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> Json<Value> {
    println!("ID: {}", id);
    let row = sqlx::query!(
        "SELECT 
            \"user\".*,
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
        password: row.password.clone(),
        created_at: row.created_at.to_string(),
        last_login: row.last_login.to_string(),
        permission: Permission {
            id: row.permission_id.clone() as u32,
            name: row.permission_name.clone(),
            description: row.permission_description.clone(),
        },
        banned: row.banned,
    };

    Json(json!(user))
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
) -> Response<String> {
    let user_query = "INSERT INTO \"user\"(name, email, password) VALUES($1, $2, $3) RETURNING id";

    let res: (i32,) = sqlx::query_as(&user_query)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(&pool)
        .await
        .unwrap_or_else(|_| (0,));

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

    let token = utils::get_jwt(&user);
    match token {
        Ok(token) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(
                json!({
                  "success": true,
                  "data": {
                    "token": token
                  }
                })
                .to_string(),
            )
            .unwrap_or_default(),

        Err(error) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, "application/json")
            .body(
                json!({
                  "success": false,
                  "data": {
                    "message": error
                  }
                })
                .to_string(),
            )
            .unwrap_or_default(),
    }
}
