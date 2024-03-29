pub mod user;

use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::models::entities::InsertUser;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    exp: i64,
}

pub fn get_jwt(user: &InsertUser) -> Result<String, String> {
    dotenv().ok();
    let secret = dotenv::var("JWT_SECRET").unwrap();
    let ttl = dotenv::var("JWT_TTL").unwrap();

    let token = encode(
        &Header::default(),
        &Claims {
            email: user.email.clone(),
            exp: (Utc::now()
                + Duration::minutes(ttl.parse::<i64>().expect("Failed to parse TTL for JWT")))
            .timestamp(),
        },
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| e.to_string());

    token
}

pub fn decode_jwt(token: &str) -> Result<Claims, String> {
    dotenv().ok();
    let secret = dotenv::var("JWT_SECRET").unwrap();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    match token_data {
        Ok(token_data) => {
            let now = Utc::now().timestamp();
            if token_data.claims.exp < now {
                Err("Token is expired".to_string())
            } else {
                Ok(token_data.claims)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
