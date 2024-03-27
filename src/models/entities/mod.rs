use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub last_login: String,
    pub permission: Permission,
    pub banned: bool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
