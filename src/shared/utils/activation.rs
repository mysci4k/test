use redis::{Client as RedisClient, RedisError, TypedCommands};

const ACTIVATION_TOKEN_TTL: u64 = 86400;

pub fn store_activation_token(
    redis_client: &RedisClient,
    user_id: &str,
    activation_token: &str,
) -> Result<(), RedisError> {
    let mut conn = redis_client.get_connection()?;

    let key = format!("activation_token:{}", activation_token);
    conn.set_ex(&key, user_id, ACTIVATION_TOKEN_TTL)?;

    Ok(())
}

pub fn get_user_id_from_activation_token(
    redis_client: &RedisClient,
    activation_token: &str,
) -> Result<String, RedisError> {
    let mut conn = redis_client.get_connection()?;

    let key = format!("activation_token:{}", activation_token);
    let user_id: Option<String> = conn.get(&key)?;

    user_id.ok_or_else(|| {
        RedisError::from((redis::ErrorKind::TypeError, "Token not found or expired"))
    })
}

pub fn delete_activation_token(
    redis_client: &RedisClient,
    activation_token: &str,
) -> Result<(), RedisError> {
    let mut conn = redis_client.get_connection()?;

    let key = format!("activation_token:{}", activation_token);
    conn.del(&key)?;

    Ok(())
}
