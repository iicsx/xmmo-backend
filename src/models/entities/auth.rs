use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct JwtPayload {
    pub token: String,
    pub refresh_token: String,
}
