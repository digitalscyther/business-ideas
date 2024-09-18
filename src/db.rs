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
        "SELECT EXISTS(SELECT 1 FROM short_link WHERE short_key = $1)",
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
            "INSERT INTO short_link (short_key, url, token) VALUES ($1, $2, $3) RETURNING *",
            short_key,
            url,
            token,
        )
        .fetch_one(db)
        .await
}

pub async fn get_short_link(db: &PgPool, short_key: &str) -> Result<ShortLink, sqlx::Error> {
    sqlx::query_as!(
            ShortLink, "SELECT * FROM short_link WHERE short_key = $1", short_key
        )
        .fetch_one(db)
        .await
}

pub async fn increment_short_link_clicks(db: &PgPool, short_key: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE short_link SET clicks = clicks + 1 WHERE short_key = $1",
        short_key
    )
        .execute(db)
        .await?;

    Ok(())
}


pub struct LandingPage {
    pub id: i32,
    pub path: String,
    pub html: Vec<u8>,
}

pub async fn create_landing_page(db: &PgPool, path: &str, html: Vec<u8>) -> Result<LandingPage, sqlx::Error> {
    sqlx::query_as!(
        LandingPage,
        "INSERT INTO landing_page (path, html) VALUES ($1, $2) RETURNING *",
        path,
        html
    )
        .fetch_one(db)
        .await
}

pub async fn get_landing_page(db: &PgPool, path: &str) -> Result<LandingPage, sqlx::Error> {
    sqlx::query_as!(
            LandingPage, "SELECT * FROM landing_page WHERE path = $1", path
        )
        .fetch_one(db)
        .await
}
