use crate::models::entities::item::Item;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Inventory {
    pub user_id: u32,
    pub quantity: u32,
    pub level: u32,
    pub item: Item,
}
