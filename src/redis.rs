use redis::{Client, RedisResult};
use redis::aio::MultiplexedConnection;

pub async fn get_redis_client(redis_url: &str) -> RedisResult<Client>{
    Client::open(redis_url)
}

pub async fn get_redis_connection(redis_client: &Client) -> RedisResult<MultiplexedConnection> {
    redis_client.get_multiplexed_async_connection().await
}

pub async fn check_redis_connection(redis_client: &Client) -> Result<(), String> {
    let mut conn = get_redis_connection(redis_client)
        .await
        .map_err(|_| "Failed to create redis connection".to_string())?;

    let pong: String = redis::cmd("PING")
        .query_async(&mut conn)
        .await
        .map_err(|_| "Failed to send PING via redis connection".to_string())?;

    assert_eq!(pong, "PONG");

    Ok(())
}