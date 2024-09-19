mod landing_page;
mod message;
mod short_link;
mod state;
mod db;
mod redis;
mod utils;

use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), String> {
    tracing_subscriber::fmt().json()
        .with_max_level(Level::ERROR)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app_state = state::setup_app_state().await.expect("Failed to build AppState");
    sqlx::migrate!("./migrations")
        .run(&app_state.db)
        .await.expect("Failed run migrations");

    let router = Router::new()
        .route("/ping", get(utils::ping_pong))
        .nest("/short-link", short_link::router::get_router(app_state.clone()).await)
        .nest("/landing-page", landing_page::router::get_router(app_state.clone()).await)
        .nest("/contact", message::router::get_router(app_state.clone()).await)
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