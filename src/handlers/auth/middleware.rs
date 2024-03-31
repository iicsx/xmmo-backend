use crate::models::entities::claims::Claims;
use axum::body::Body;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use dotenv::dotenv;
use jsonwebtoken::{DecodingKey, Validation};
use std::sync::Arc;

const AUTHORIZATION: &str = "authorization";
const BEARER: &str = "Bearer ";

pub async fn jwt_authentification(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    dotenv().ok();

    let secret = match dotenv::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let authorization_header = match request.headers().get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let authorization = match authorization_header.to_str() {
        Ok(v) => v,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    if !authorization.starts_with(BEARER) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let jwt_token = authorization.trim_start_matches(BEARER);

    let token_header = match jsonwebtoken::decode_header(&jwt_token) {
        Ok(header) => header,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let user_claims = match jsonwebtoken::decode::<Claims>(
        jwt_token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(token_header.alg),
    ) {
        Ok(claims) => Arc::new(claims),
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    request.extensions_mut().insert(user_claims.clone());
    Ok(next.run(request).await)
}
