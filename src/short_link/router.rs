use std::sync::Arc;
use axum::extract::{Host, Path, Query, State};
use axum::http::{StatusCode};
use axum::{Json, Router};
use axum::response::Redirect;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use crate::db::{check_key_exists, create_short_link, get_short_link, increment_short_link_clicks, ShortLink};
use crate::short_link::link::{generate_key, rand_string};
use crate::state::AppState;

pub async fn get_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/gen", post(create_link))
        .route("/:short_key", get(proxy))
        .route("/:short_key/info", get(get_link))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

#[derive(Deserialize)]
struct CreateLinkRequest {
    url: String,
}

#[derive(Serialize)]
struct CreateShortLinkResponse {
    short_url: String,
    stats_url: String,
}

#[derive(Deserialize)]
struct StatsQuery {
    token: Option<String>,
}

async fn create_link(
    State(state): State<Arc<AppState>>,
    Host(hostname): Host,
    Json(payload): Json<CreateLinkRequest>,
) -> Result<Json<CreateShortLinkResponse>, StatusCode> {
    let scheme = std::env::var("LINK_SCHEME").unwrap_or_else(|_| "http".to_string());

    let short_key = generate_key(
        | key| {
            let db_ref = &state.db;
            async move { check_key_exists(db_ref, &key).await }
        },
        3,
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;


    let token = rand_string(24);

    create_short_link(&state.db, &short_key, &payload.url, &token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let short_url = format!("{scheme}://{hostname}/{short_key}");
    let stats_url = format!("{short_url}/info?token={token}");

    Ok(Json(CreateShortLinkResponse {
        short_url,
        stats_url,
    }))
}

async fn proxy(
    Path(short_key): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, StatusCode> {
    let short_url = get_short_link(&state.db, &short_key)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    increment_short_link_clicks(&state.db, &short_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::temporary(&short_url.url))
}

async fn get_link(
    Path(short_key): Path<String>,
    Query(params): Query<StatsQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ShortLink>, StatusCode> {
    let short_link = get_short_link(&state.db, &short_key)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    match params.token {
        None => return Err(StatusCode::NOT_FOUND),
        Some(token) if token != short_link.token => return Err(StatusCode::UNAUTHORIZED),
        _ => {},
    }

    Ok(Json(short_link))
}
