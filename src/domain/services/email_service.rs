use async_trait::async_trait;

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_activation_email(
        &self,
        to_email: &str,
        username: &str,
        activation_token: &str,
    ) -> Result<(), String>;
}
