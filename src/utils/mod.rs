use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::models::entities::{InsertUser, User};

#[derive(Serialize, Deserialize)]
struct Claims {
    email: String,
    exp: i64,
}

pub fn get_jwt(user: &InsertUser) -> Result<String, String> {
    dotenv().ok();
    let secret = dotenv::var("JWT_SECRET").unwrap();

    let token = encode(
        &Header::default(),
        &Claims {
            email: user.email.clone(),
            exp: (Utc::now() + Duration::minutes(1)).timestamp(),
        },
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| e.to_string());

    token
}

pub fn decode_jwt(token: &str) -> Result<User, String> {
    dotenv().ok();
    let secret = dotenv::var("JWT_SECRET").unwrap();

    let token_data = decode::<User>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),

        Err(e) => Err(e.to_string()),
    }
}
