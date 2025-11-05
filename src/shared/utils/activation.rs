use redis::{AsyncTypedCommands, Client as RedisClient, RedisError};

const ACTIVATION_TOKEN_TTL: u64 = 86400;

pub async fn store_activation_token(
    redis_client: &RedisClient,
    user_id: &str,
    activation_token: &str,
) -> Result<(), RedisError> {
    let mut conn = redis_client.get_multiplexed_async_connection().await?;

    let key = format!("activation_token:{}", activation_token);
    conn.set_ex(&key, user_id, ACTIVATION_TOKEN_TTL).await?;

    Ok(())
}

pub async fn get_user_id_from_activation_token(
    redis_client: &RedisClient,
    activation_token: &str,
) -> Result<String, RedisError> {
    let mut conn = redis_client.get_multiplexed_async_connection().await?;

    let key = format!("activation_token:{}", activation_token);
    let user_id: Option<String> = conn.get(&key).await?;

    user_id.ok_or_else(|| {
        RedisError::from((redis::ErrorKind::TypeError, "Token not found or expired"))
    })
}

pub async fn delete_activation_token(
    redis_client: &RedisClient,
    activation_token: &str,
) -> Result<(), RedisError> {
    let mut conn = redis_client.get_multiplexed_async_connection().await?;

    let key = format!("activation_token:{}", activation_token);
    conn.del(&key).await?;

    Ok(())
}
