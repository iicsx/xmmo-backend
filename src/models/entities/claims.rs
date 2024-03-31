use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub from_refresh: bool,
    pub email: String,
    pub exp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshClaims {
    pub email: String,
    pub exp: i64,
}
