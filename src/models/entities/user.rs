use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserDetails {
    pub strength: u32,
    pub defence: u32,
    pub dexterity: u32,
    pub current_energy: u32,
    pub max_energy: u32,
    pub current_hp: u32,
    pub max_hp: u32,
    pub exp: u32,
    pub gold: u32,
    pub bank: u32,
    pub profession_exp: u32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserStats {
    pub ledges_grabbed: u32,
    pub npc_kills: u32,
    pub items_dropped: u32,
    pub height: f64,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub created_at: String,
    pub last_login: String,
    pub permission: Permission,
    pub stats: UserStats,
    pub details: UserDetails,
    pub muted: bool,
    pub locked: bool,
    pub banned: bool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct EmailUser {
    pub email: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct LoginCheckUser {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub banned: bool,
}
