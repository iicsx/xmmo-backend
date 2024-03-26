mod handlers;
mod models;

use handlers::{database, default};

use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Extension, Router,
};
use axum_client_ip::SecureClientIpSource;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let connection_url = dotenv::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_url)
        .await
        .unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    let origins: Vec<HeaderValue> = vec![HeaderValue::from_str("http://localhost:8080").unwrap()];

    let cors_layer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
        ])
        .allow_origin(origins)
        .allow_headers(vec![header::CONTENT_TYPE]);

    let app = Router::new()
        /* get */
        .route("/", get(default::not_implemented))
        .route("/status", get(default::status))
        .route("/users", get(database::get_all_users))
        /* post */
        .route("/user", post(database::single_insert_user))
        .route("/users", post(database::multiple_insert_user))
        /* extensions */
        .layer(cors_layer)
        .layer(Extension(pool))
        .layer(SecureClientIpSource::ConnectInfo.into_extension());

    let address = dotenv::var("ADDRESS").unwrap();
    let port = dotenv::var("PORT").unwrap();

    let binding = format!("{}:{}", address, port);

    let listener = tokio::net::TcpListener::bind(&binding).await.unwrap();

    println!("Server running on: {}", binding);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
