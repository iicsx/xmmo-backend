use crate::models::entities::inventory::{InventoryItem};
use sqlx::postgres::PgPool;

pub async fn get_inventory_by_id(pool: &PgPool, user_id: &String) -> Option<Vec<InventoryItem>>{
    let rows = sqlx::query!(
        "SELECT 
            \"inventory\".user_id,
            \"inventory\".quantity,
            \"inventory\".level,
            \"inventory\".item_id,
            \"inventory\".name,
            \"inventory\".itype,
            \"inventory\".rarity,
            \"inventory\".weight,
            \"inventory\".img
        FROM \"inventory\" 
        WHERE \"inventory\".user_id = $1",
        user_id.parse::<i32>().unwrap()
    )
    .fetch_all(pool)
    .await;

    let mut inv = Vec::new();

    for row in rows{
        
        let row = match row {
            Ok(row) => row,
            Err(_) => return None,
        };

        let item = InventoryItem {
            user_id: row.user_id.clone() as u32,
            quantity: row.quantity.clone() as u32,
            level: row.level.clone() as u32,
            item_id: row.item_id.clone() as u32,
            name: row.name.clone(),
            itype: row.itype.clone(),
            rarity: row.rarity.clone(),
            weight: row.weight.to_string().parse::<f64>().unwrap(),
            img: row.img.clone(),
        };

        inv.push(item)
    }

    Some(inv)
}
