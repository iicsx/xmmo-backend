use crate::models::entities::item::{ItemStats, ItemMods, ItemReqs, Item};
use sqlx::postgres::PgPool;

pub async fn get_item_by_id(pool: &PgPool, item_id: &String) -> Item {
    let row = sqlx::query!(
        "SELECT 
            \"item\".item_id,
            \"item\".item_name,
            \"item\".item_type,
            \"item\".item_rarity,
            \"item\".stat_type,
            \"item\".stat_value,
            \"item\".mod_type,
            \"item\".mod_value,
            \"item\".mod_duration,
            \"item\".req_type,
            \"item\".req_value,
            \"item\".item_weight,
            \"item\".item_img,
            \"item\".item_desc

        FROM \"item\" 
        WHERE \"item\".item_id = $1",
        item_id.parse::<i32>().unwrap()
    )
    .fetch_one(pool)
    .await;

    let row = row.unwrap();

    let item = Item {
        item_id: row.item_id.clone() as u32,
        item_name: row.item_name.clone(),
        item_type: row.item_type.clone(),
        item_rarity: row.item_rarity.clone(),
        item_stats: ItemStats {
            stat_type: row.stat_type.clone(),
            stat_value: row.stat_value.clone() as u32,
        },
        item_mods: ItemMods {
            mod_type: row.mod_type.clone(),
            mod_value: row.mod_value.clone() as u32,
            mod_duration: row.mod_duration.clone() as u32,
        },
        item_reqs: ItemReqs {
            req_type: row.req_type.clone(),
            req_value: row.req_value.clone() as u32,
        },
        item_weight: row.item_weight.to_string().parse::<f64>().unwrap(),
        item_img: row.item_img.clone(),
        item_desc: row.item_desc.clone(),
    };

    item
}

