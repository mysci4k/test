use async_trait::async_trait;

#[async_trait]
pub trait TokenService: Send + Sync {
    async fn store_activation_token(
        &self,
        user_id: &str,
        activation_token: &str,
    ) -> Result<(), String>;
    async fn get_user_id_from_activation_token(
        &self,
        activation_token: &str,
    ) -> Result<String, String>;
    async fn delete_activation_token(&self, activation_token: &str) -> Result<(), String>;
}
