use crate::domain::services::TokenService;
use crate::shared::utils::constants::ACTIVATION_TOKEN_TTL;
use async_trait::async_trait;
use redis::Client as RedisClient;
use redis::{AsyncTypedCommands, RedisError};

pub struct RedisTokenService {
    redis_client: RedisClient,
}

impl RedisTokenService {
    pub fn new(redis_client: RedisClient) -> Self {
        Self { redis_client }
    }

    fn activation_key(&self, activation_token: &str) -> String {
        format!("activation_token:{}", activation_token)
    }
}

#[async_trait]
impl TokenService for RedisTokenService {
    async fn store_activation_token(
        &self,
        user_id: &str,
        activation_token: &str,
    ) -> Result<(), String> {
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|err| format!("Redis connection error: {}", err))?;

        let key = self.activation_key(activation_token);
        conn.set_ex(&key, user_id, *ACTIVATION_TOKEN_TTL)
            .await
            .map_err(|err: RedisError| format!("Failed to store token: {}", err))?;

        Ok(())
    }

    async fn get_user_id_from_activation_token(
        &self,
        activation_token: &str,
    ) -> Result<String, String> {
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|err| format!("Redis connection error: {}", err))?;

        let key = self.activation_key(activation_token);
        let user_id: Option<String> = conn
            .get(&key)
            .await
            .map_err(|err: RedisError| format!("Failed to get token: {}", err))?;

        user_id.ok_or_else(|| "Token not found or expired".to_string())
    }

    async fn delete_activation_token(&self, activation_token: &str) -> Result<(), String> {
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|err| format!("Redis connection error: {}", err))?;

        let key = self.activation_key(activation_token);
        conn.del(&key)
            .await
            .map_err(|err: RedisError| format!("Failed to delete token: {}", err))?;

        Ok(())
    }
}
