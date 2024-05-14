use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct ItemStats {
    pub stat_type: String,
    pub stat_value: u32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ItemMods {
    pub mod_type: String,
    pub mod_value: u32,
    pub mod_duration: u32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ItemReqs {
    pub req_type: String,
    pub req_value: u32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Item {
    pub item_id: u32,
    pub item_name: String,
    pub item_type: String,
    pub item_rarity: String,
    pub item_stats: ItemStats,
    pub item_mods: ItemMods,
    pub item_reqs: ItemReqs,
    pub item_weight: f64,
    pub item_img: String,
    pub item_desc: String,
}

