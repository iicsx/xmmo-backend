use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct ItemStats {
    pub itype: String,
    pub value: u32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ItemMods {
    pub itype: String,
    pub value: u32,
    pub duration: u32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ItemReqs {
    pub itype: String,
    pub value: u32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub itype: String,
    pub rarity: String,
    pub stats: ItemStats,
    pub mods: ItemMods,
    pub reqs: ItemReqs,
    pub weight: f64,
    pub img: String,
    pub desc: String,
}

