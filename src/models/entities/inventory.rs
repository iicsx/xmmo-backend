use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct InventoryItem {
    pub user_id: u32,
    pub quantity: u32,
    pub level: u32,
    pub item_id: u32,
    pub name: String,
    pub itype: String,
    pub rarity: String,
    pub weight: f64,
    pub img: String,
}
