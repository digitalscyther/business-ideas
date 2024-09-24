use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::{Json, Router};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tower_http::trace::TraceLayer;
use crate::db::{create_landing_page, get_landing_page};
use crate::state::AppState;
use crate::utils;

pub async fn get_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ping", get(utils::ping_pong))
        .route("/:path", get(get_page))
        .route("/:path", post(create_page))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

#[derive(Serialize)]
struct CreatePageResponse {
    success: bool,
}

async fn create_page(
    Path(path): Path<String>,
    State(state): State<Arc<AppState>>,
    body: axum::body::Bytes,
) -> Result<Json<CreatePageResponse>, StatusCode> {
    create_landing_page(&state.db, &path, body.to_vec())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreatePageResponse { success: true }))
}

#[derive(Error, Debug)]
pub enum AppError {
    // #[error("Invalid input: {0}")]
    // InvalidInput(String),

    // #[error("Resource not found")]
    // NotFound,

    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            // AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            // AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

async fn get_page(
    Path(path): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    match get_landing_page(&state.db, &path).await {
        Ok(page) => {
            Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html")],
                String::from_utf8(page.html).map_err(|_| AppError::InternalServerError)?,
            ))
        }
        Err(sqlx::Error::RowNotFound) => {
            Ok((
                StatusCode::NOT_FOUND,
                [(header::CONTENT_TYPE, "text/plain")],
                "Page not found".into(),
            ))
        }
        Err(_) => Err(AppError::InternalServerError),
    }
}

