use rand::distributions::Alphanumeric;
use rand::Rng;
use tracing::{error, warn};

pub async fn generate_key<F, Fut>(
    check_key_exists: F,
    num_attempts: usize
) -> Result<String, String>
where
    F: Fn(String) -> Fut,
    Fut: std::future::Future<Output = Result<bool, String>>,
{
    for attempt in 0..num_attempts {
        let short_key: String = rand_string(6);

        let key_exists = check_key_exists(short_key.clone())
            .await
            .map_err(|e| {
                error!("Error during key check: {:?}", e);
                "Key check error".to_string()
            })?;

        if !key_exists {
            return Ok(short_key);
        }

        warn!(
            "Generated key already exists. Attempt {}/{}. Retrying...",
            attempt + 1,
            num_attempts
        );
    }

    Err("Failed to generate a unique key after maximum attempts".to_string())
}

pub fn rand_string(n: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}