mod handlers;
mod models;
mod routes;

use axum::{
    http::{header, HeaderValue, Method},
    middleware::from_fn,
    routing::{get, patch, post},
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
            println!("🔥 Failed to establish Database connection: {:?}", err);
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
        .route("/", get(routes::default::not_implemented))
        .route("/status", get(routes::default::status))
        .route("/user/:id", get(routes::user::fetch_user_by_id))
        .route("/item/:id", get(routes::item::fetch_item_by_id))
        .route(
            "/inventory/:id",
            get(routes::inventory::fetch_inventory_by_id),
        )
        /* patch */
        .route("/user/:id", patch(routes::user::patch_user_by_id))
        /* post */
        .route(
            "/inventory/:id",
            post(routes::inventory::add_item_to_inventory),
        )
        .route("/has_item", post(routes::inventory::user_has_item))
        /* middleware */
        .layer(from_fn(routes::auth::middleware::jwt_authentification))
        /* routes without middlware */
        .route("/jwtlogin", post(routes::auth::jwt_login))
        .route("/refresh", post(routes::auth::refresh_token))
        .route("/register", post(routes::user::single_insert_user))
        .route("/login", post(routes::user::login_user))
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
