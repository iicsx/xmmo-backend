use crate::models::entities::{
    inventory::Inventory,
    item::{Item, ItemMods, ItemReqs, ItemStats},
};
use sqlx::postgres::PgPool;

pub async fn get_inventory_by_id(pool: &PgPool, user_id: &String) -> Option<Vec<Inventory>> {
    let rows = sqlx::query!(
        "SELECT 
            \"inventory\".user_id,
            \"inventory\".quantity,
            \"inventory\".level,
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
        JOIN \"inventory\" ON \"item\".id = \"inventory\".item_id
        WHERE \"inventory\".user_id = $1",
        user_id.parse::<i32>().unwrap()
    )
    .fetch_all(pool)
    .await;

    let rows = match rows {
        Ok(rows) => rows,
        Err(_) => return None,
    };

    let mut inv: Vec<Inventory> = Vec::new();
    for row in rows {
        let item = Inventory {
            user_id: row.user_id.clone() as u32,
            quantity: row.quantity.clone() as u32,
            level: row.level.clone() as u32,
            item: Item {
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
            },
        };
        inv.push(item);
    }

    Some(inv)
}
