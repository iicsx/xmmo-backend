use axum::http::StatusCode;

use crate::models::entities::user::{Permission, User, UserDetails, UserStats};
use sqlx::postgres::PgPool;

pub async fn get_user_by_email(pool: &PgPool, email: &String) -> User {
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
            \"permission\".description AS \"permission_description\",

            \"user_stats\".ledges_grabbed,
            \"user_stats\".npc_kills,
            \"user_stats\".items_dropped,
            \"user_stats\".height,

            \"user_details\".strength,
            \"user_details\".defence,
            \"user_details\".dexterity,
            \"user_details\".current_energy,
            \"user_details\".max_energy,
            \"user_details\".current_hp,
            \"user_details\".max_hp,
            \"user_details\".exp,
            \"user_details\".gold,
            \"user_details\".profession_exp

        FROM \"user\" 
        LEFT JOIN \"user_permission\" ON \"user\".id = \"user_permission\".user_id 
        LEFT JOIN \"user_stats\" ON \"user\".id = \"user_stats\".user_id 
        LEFT JOIN \"user_details\" ON \"user\".id = \"user_details\".user_id 
        LEFT JOIN \"permission\" ON \"user_permission\".permission_id = \"permission\".id 
        WHERE \"user\".email = $1",
        email
    )
    .fetch_one(pool)
    .await;

    let row = row.unwrap();

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
        stats: UserStats {
            ledges_grabbed: row.ledges_grabbed as u32,
            npc_kills: row.npc_kills as u32,
            items_dropped: row.items_dropped as u32,
            height: row.height.to_string().parse::<f64>().unwrap(),
        },
        details: UserDetails {
            strength: row.strength as u32,
            defence: row.defence as u32,
            dexterity: row.dexterity as u32,
            current_energy: row.current_energy as u32,
            max_energy: row.max_energy as u32,
            current_hp: row.current_hp as u32,
            max_hp: row.max_hp as u32,
            exp: row.exp as u32,
            gold: row.gold as u32,
            profession_exp: row.profession_exp.unwrap_or(0) as u32,
        },
        muted: row.muted,
        locked: row.locked,
        banned: row.banned,
    };

    user
}

pub async fn get_user_by_id(pool: &PgPool, id: &String) -> User {
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
            \"permission\".description AS \"permission_description\",

            \"user_stats\".ledges_grabbed,
            \"user_stats\".npc_kills,
            \"user_stats\".items_dropped,
            \"user_stats\".height,

            \"user_details\".strength,
            \"user_details\".defence,
            \"user_details\".dexterity,
            \"user_details\".current_energy,
            \"user_details\".max_energy,
            \"user_details\".current_hp,
            \"user_details\".max_hp,
            \"user_details\".exp,
            \"user_details\".gold,
            \"user_details\".profession_exp

        FROM \"user\" 
        JOIN \"user_permission\" ON \"user\".id = \"user_permission\".user_id 
        JOIN \"user_stats\" ON \"user\".id = \"user_stats\".user_id 
        JOIN \"user_details\" ON \"user\".id = \"user_details\".user_id 
        JOIN \"permission\" ON \"user_permission\".permission_id = \"permission\".id 
        WHERE \"user\".id = $1",
        id.parse::<i32>().unwrap()
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
        stats: UserStats {
            ledges_grabbed: row.ledges_grabbed as u32,
            npc_kills: row.npc_kills as u32,
            items_dropped: row.items_dropped as u32,
            height: row.height.to_string().parse::<f64>().unwrap(),
        },
        details: UserDetails {
            strength: row.strength as u32,
            defence: row.defence as u32,
            dexterity: row.dexterity as u32,
            current_energy: row.current_energy as u32,
            max_energy: row.max_energy as u32,
            current_hp: row.current_hp as u32,
            max_hp: row.max_hp as u32,
            exp: row.exp as u32,
            gold: row.gold as u32,
            profession_exp: row.profession_exp.unwrap_or(0) as u32,
        },
        muted: row.muted,
        locked: row.locked,
        banned: row.banned,
    };

    user
}
