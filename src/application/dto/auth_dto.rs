use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ActivationQueryDto {
    #[validate(length(min = 1, message = "User ID is required"))]
    pub user_id: String,
    #[validate(length(min = 1, message = "Activation token is required"))]
    pub activation_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResendActivationQueryDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}
