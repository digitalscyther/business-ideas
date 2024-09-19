use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{get, post};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use tower_http::trace::TraceLayer;
use crate::db::{create_topic, get_topic, create_message, get_messages, Message};
use crate::state::AppState;
use crate::utils;

pub async fn get_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ping", get(utils::ping_pong))
        .route("/topics", post(create_topic_handler))
        .route("/messages", post(create_message_handler))
        .route("/topics/:topic_id/messages", get(get_messages_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

#[derive(Deserialize)]
struct CreateTopicRequest {
    name: String,
}

#[derive(Serialize)]
struct CreateTopicResponse {
    id: Uuid,
}

#[derive(Deserialize)]
struct CreateMessageRequest {
    email: String,
    text: String,
    topic_id: Uuid,
}

async fn create_topic_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTopicRequest>,
) -> Result<Json<CreateTopicResponse>, StatusCode> {
    let topic = create_topic(&state.db, &payload.name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateTopicResponse { id: topic.id }))
}

async fn create_message_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<StatusCode, StatusCode> {
    let topic_exists = get_topic(&state.db, &payload.topic_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if !topic_exists.is_some() {
        return Err(StatusCode::NOT_FOUND);
    }

    create_message(&state.db, &payload.email, &payload.text, &payload.topic_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

async fn get_messages_handler(
    Path(topic_id): Path<Uuid>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    let token = std::env::var("CONTACT_TOKEN").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if bearer.token() != token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let messages = get_messages(&state.db, &topic_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(messages))
}
