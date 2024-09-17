use std::env;
use axum::{response::Html, routing::get, Router};
use redis::{Client, RedisResult};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json()
        .with_max_level(Level::ERROR)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db_url = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let db = get_db_connection(&db_url).await.expect("Failed to connect to database");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis = get_redis_client(&redis_url).await.expect("Failed to open redis client");
    let app_state = AppState { db, redis };
    let mut conn = get_redis_connection(&app_state.redis).await.expect("Failed to create redis connection");
    let result: String = redis::cmd("PING").query_async(&mut conn).await.expect("Failed send PING via reddis connection");
    assert_eq!(result, "PONG");

    let router = Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let bind_address = format!("{}:{}", host, port);
    info!("Listening on {}", bind_address);
    let listener = tokio::net::TcpListener::bind(bind_address)
        .await
        .unwrap();

    axum::serve(listener, router.into_make_service()).await.unwrap();
}

async fn get_db_connection(db_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(db_url).await
}

async fn get_redis_client(redis_url: &str) -> RedisResult<Client>{
    Client::open(redis_url)
}

async fn get_redis_connection(redis_client: &Client) -> RedisResult<redis::aio::MultiplexedConnection> {
    redis_client.get_multiplexed_async_connection().await
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
    redis: Client,
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}