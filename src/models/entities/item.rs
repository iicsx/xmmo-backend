use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct ItemStats {
    pub stat_type: Option<String>,
    pub stat_value: Option<u32>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ItemMods {
    pub mod_type: Option<String>,
    pub mod_value: Option<u32>,
    pub mod_duration: Option<u32>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ItemReqs {
    pub req_type: Option<String>,
    pub req_value: Option<u32>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub itype: String,
    pub rarity: String,
    pub weight: f64,
    pub img: String,
    pub group_name: Option<String>,
    pub description: Option<String>,

    pub stats: ItemStats,
    pub mods: ItemMods,
    pub reqs: ItemReqs,
}
