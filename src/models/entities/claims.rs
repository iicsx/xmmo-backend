use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub from_refresh: bool,
    pub email: String,
    pub exp: i64,
    pub refresh: bool,
}
