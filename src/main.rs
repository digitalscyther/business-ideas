mod short_link;
mod state;
mod db;
mod redis;
mod utils;

use axum::{response::Html, Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), String> {
    tracing_subscriber::fmt().json()
        .with_max_level(Level::ERROR)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app_state = state::setup_app_state().await.expect("Failed to build AppState");

    let router = Router::new()
        .route("/", get(handler))
        .nest("/short-link", short_link::router::get_router(app_state.clone()).await)
        .layer(TraceLayer::new_for_http());

    let host = utils::get_env_var("HOST")?;
    let port = utils::get_env_var("PORT")?;
    let bind_address = format!("{}:{}", host, port);
    info!("Listening on {}", bind_address);
    let listener = tokio::net::TcpListener::bind(bind_address)
        .await
        .expect("Failed init listener");

    axum::serve(listener, router.into_make_service()).await.expect("Failed start serving");

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}