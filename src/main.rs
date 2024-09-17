use std::env;
use axum::{response::Html, routing::get, Router};
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
    let db = get_connection(&db_url).await.expect("Failed to connect to database");
    let app_state = AppState { db };

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

async fn get_connection(db_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(db_url).await
}

#[derive(Clone)]
struct AppState {
    pub db: PgPool,
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}