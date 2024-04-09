mod handlers;
mod models;
mod utils;

use handlers::{auth, database, default};

use axum::{
    http::{header, HeaderValue, Method},
    middleware::from_fn,
    routing::{get, post},
    Extension, Router,
};
use axum_client_ip::SecureClientIpSource;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    dotenv().ok();

    let connection_url = dotenv::var("DATABASE_URL").unwrap();

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Successfully established Database connection.");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to establish Dtabase connection: {:?}", err);
            std::process::exit(1);
        }
    };

    let origins: Vec<HeaderValue> = vec![
        HeaderValue::from_str("http://localhost:3000").unwrap(),
        HeaderValue::from_str("http://localhost:5173").unwrap(),
        HeaderValue::from_str("http://localhost:8080").unwrap(),
    ];

    let cors_layer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
        ])
        .allow_origin(origins)
        .allow_headers(vec![
            header::CONTENT_TYPE,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            header::AUTHORIZATION,
        ]);

    let app = Router::new()
        /* get */
        .route("/", get(default::not_implemented))
        .route("/status", get(default::status))
        .route("/user/:id", get(database::user::fetch_user_by_id))
        /* post */
        .layer(from_fn(auth::middleware::jwt_authentification))
        /* routes without middlware */
        .route("/jwtlogin", post(auth::jwt_login))
        .route("/refresh", post(auth::refresh_token))
        .route("/register", post(database::user::single_insert_user))
        .route("/login", post(database::user::login_user))
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
    .unwrap()
}
