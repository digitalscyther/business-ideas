use std::env;
use axum::{response::Html, routing::get, Router};
use redis::{Client, RedisResult};
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), String> {
    tracing_subscriber::fmt().json()
        .with_max_level(Level::ERROR)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app_state = setup_app_state().await.expect("Failed to build AppState");
    check_redis_connection(&app_state.redis).await.expect("Failed PING redis conn");

    let router = Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let host = get_env_var("HOST")?;
    let port = get_env_var("PORT")?;
    let bind_address = format!("{}:{}", host, port);
    info!("Listening on {}", bind_address);
    let listener = tokio::net::TcpListener::bind(bind_address)
        .await
        .expect("Failed init listener");

    axum::serve(listener, router.into_make_service()).await.expect("Failed start serving");

    Ok(())
}

fn get_env_var(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("{} must be set", key))
}

async fn setup_app_state() -> Result<AppState, String> {
    let db_url = get_env_var("POSTGRES_URL")?;
    let redis_url = get_env_var("REDIS_URL")?;

    let db = get_db_connection(&db_url).await.map_err(|_| "Failed to connect to database".to_string())?;
    let redis = get_redis_client(&redis_url).await.map_err(|_| "Failed to open redis client".to_string())?;

    Ok(AppState { db, redis })
}

async fn check_redis_connection(redis_client: &Client) -> Result<(), String> {
    let mut conn = get_redis_connection(redis_client)
        .await
        .map_err(|_| "Failed to create redis connection".to_string())?;

    let pong: String = redis::cmd("PING")
        .query_async(&mut conn)
        .await
        .map_err(|_| "Failed to send PING via redis connection".to_string())?;

    assert_eq!(pong, "PONG");

    Ok(())
}

async fn get_db_connection(db_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(db_url).await
}

async fn get_redis_client(redis_url: &str) -> RedisResult<Client>{
    Client::open(redis_url)
}

async fn get_redis_connection(redis_client: &Client) -> RedisResult<MultiplexedConnection> {
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