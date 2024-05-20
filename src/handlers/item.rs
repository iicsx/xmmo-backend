use crate::models::entities::item::{Item, ItemMods, ItemReqs, ItemStats};
use sqlx::postgres::PgPool;

pub async fn get_item_by_id(pool: &PgPool, item_id: &String) -> Option<Item> {
    let row = sqlx::query!(
        "SELECT 
            \"item\".id,
            \"item\".name,
            \"item\".itype,
            \"item\".rarity,
            \"item\".weight,
            \"item\".img,
            \"item\".description,
            \"item\".stat_type,
            \"item\".stat_value,
            \"item\".mod_type,
            \"item\".mod_value,
            \"item\".mod_duration,
            \"item\".req_type,
            \"item\".req_value,
            \"item\".group_name
        FROM \"item\" 
        WHERE \"item\".id = $1",
        item_id.parse::<i32>().unwrap()
    )
    .fetch_one(pool)
    .await;

    let row = match row {
        Ok(row) => row,
        Err(_) => return None,
    };

    let item = Item {
        id: row.id.clone() as u32,
        name: row.name.clone(),
        itype: row.itype.clone(),
        rarity: row.rarity.clone(),
        weight: row.weight.to_string().parse::<f64>().unwrap(),
        img: row.img.clone(),
        description: row.description.clone(),
        group_name: row.group_name.clone(),
        stats: ItemStats {
            stat_type: row.stat_type.clone(),
            stat_value: row.stat_value.map(|val| val as u32),
        },
        mods: ItemMods {
            mod_type: row.mod_type.clone(),
            mod_value: row.mod_value.map(|val| val as u32),
            mod_duration: row.mod_duration.map(|val| val as u32),
        },
        reqs: ItemReqs {
            req_type: row.req_type.clone(),
            req_value: row.req_value.map(|val| val as u32),
        },
    };

    Some(item)
}
