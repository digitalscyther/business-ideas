use std::env;
use axum::Json;
use serde::Serialize;

pub fn get_env_var(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("{} must be set", key))
}

#[derive(Serialize)]
pub struct PongResponse {
    message: String,
}

pub async fn ping_pong() -> Json<PongResponse> {
    let response = PongResponse {
        message: "pong".to_string(),
    };
    Json(response)
}
