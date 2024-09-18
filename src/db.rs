use serde::Serialize;
use sqlx::PgPool;

pub async fn get_db_connection(db_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(db_url).await
}


#[derive(Serialize)]
pub struct ShortLink {
    pub id: i32,
    pub short_key: String,
    pub url: String,
    pub token: String,
    pub clicks: i32,
}

pub async fn check_key_exists(db: &PgPool, short_key: &str) -> Result<bool, String> {
    match sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM short_links WHERE short_key = $1)",
        short_key
    )
        .fetch_one(db)
        .await
    {
        Ok(exists) => Ok(exists.unwrap_or(false)),
        Err(err) => Err(format!("Database query failed: {}", err))
    }
}

pub async fn create_short_link(db: &PgPool, short_key: &str, url: &str, token: &str) -> Result<ShortLink, sqlx::Error> {
    sqlx::query_as!(
            ShortLink,
            "INSERT INTO short_links (short_key, url, token) VALUES ($1, $2, $3) RETURNING *",
            short_key,
            url,
            token,
        )
        .fetch_one(db)
        .await
}

pub async fn get_short_link(db: &PgPool, short_key: &str) -> Result<ShortLink, sqlx::Error> {
    sqlx::query_as!(
            ShortLink, "SELECT * FROM short_links WHERE short_key = $1", short_key
        )
        .fetch_one(db)
        .await
}

pub async fn increment_short_link_clicks(db: &PgPool, short_key: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE short_links SET clicks = clicks + 1 WHERE short_key = $1",
        short_key
    )
    .execute(db)
    .await?;

    Ok(())
}
