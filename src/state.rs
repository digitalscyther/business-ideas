use std::sync::Arc;
use sqlx::PgPool;
use redis::Client;
use crate::utils::get_env_var;

pub async fn setup_app_state() -> Result<Arc<AppState>, String> {
    let db_url = get_env_var("POSTGRES_URL")?;
    let redis_url = get_env_var("REDIS_URL")?;

    let db = crate::db::get_db_connection(&db_url).await.map_err(|_| "Failed to connect to database".to_string())?;
    let redis = crate::redis::get_redis_client(&redis_url).await.map_err(|_| "Failed to open redis client".to_string())?;
    crate::redis::check_redis_connection(&redis).await.expect("Failed PING redis conn");

    Ok(Arc::new(AppState { db, redis }))
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: Client,
}
