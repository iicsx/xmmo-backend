use crate::models::entities::item::{ItemStats, ItemMods, ItemReqs, Item};
use sqlx::postgres::PgPool;

use axum::http::StatusCode;

pub async fn get_item_by_id(pool: &PgPool, id: &String) -> Item {
    let row = sqlx::query!(
        "SELECT 
            \"item\".id,
            \"item\".name,
            \"item\".itype,
            \"item\".rarity,
            \"item\".stat_type,
            \"item\".stat_value,
            \"item\".mod_type,
            \"item\".mod_value,
            \"item\".mod_duration,
            \"item\".req_type,
            \"item\".req_value,
            \"item\".iweight,
            \"item\".img,
            \"item\".idesc

        FROM \"item\" 
        WHERE \"item\".id = $1",
        id.parse::<i32>().unwrap()
    )
    .fetch_one(pool)
    .await;

    let row = row.unwrap();

    let item = Item {
        id: row.id.clone() as u32,
        name: row.name.clone(),
        itype: row.itype.clone(),
        rarity: row.rarity.clone(),
        stats: ItemStats {
            itype: row.stat_type.clone(),
            value: row.stat_value.clone() as u32,
        },
        mods: ItemMods {
            itype: row.mod_type.clone(),
            value: row.mod_value.clone() as u32,
            duration: row.mod_duration.clone() as u32,
        },
        reqs: ItemReqs {
            itype: row.req_type.clone(),
            value: row.req_value.clone() as u32,
        },
        weight: row.iweight.to_string().parse::<f64>().unwrap(),
        img: row.img.clone(),
        desc: row.idesc.clone(),
    };

    item
}

