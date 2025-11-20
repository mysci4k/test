use crate::{
    application::dto::{CreateUserDto, LoginDto, ResetPasswordDto, UserDto},
    domain::{
        repositories::{User, UserRepository},
        services::{EmailService, TokenService},
    },
    shared::{error::ApplicationError, utils::argon},
};
use actix_web::rt::task;
use entity::UserModel;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    token_service: Arc<dyn TokenService>,
    email_service: Arc<dyn EmailService>,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        token_service: Arc<dyn TokenService>,
        email_service: Arc<dyn EmailService>,
    ) -> Self {
        Self {
            user_repository,
            token_service,
            email_service,
        }
    }

    pub async fn register(&self, dto: CreateUserDto) -> Result<UserDto, ApplicationError> {
        dto.validate()?;

        if self.user_repository.exists_by_email(&dto.email).await? {
            return Err(ApplicationError::Conflict {
                message: "User with this email already exists".to_string(),
            });
        }

        let hashed_password = task::spawn_blocking(move || argon::hash_password(dto.password))
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to hash password".to_string(),
            })?
            .map_err(|_| ApplicationError::InternalError {
                message: "Password hashing failed".to_string(),
            })?;

        let user = User::new(
            Uuid::now_v7(),
            dto.email,
            hashed_password,
            dto.first_name,
            dto.last_name,
        );

        let saved_user = self.user_repository.create(user).await?;

        let activation_token = argon::generate_token();
        self.token_service
            .store_activation_token(&saved_user.id.to_string(), &activation_token)
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to store activation token".to_string(),
            })?;

        let username = format!("{} {}", saved_user.first_name, saved_user.last_name);
        self.email_service
            .send_activation_email(
                &saved_user.email,
                &username,
                &saved_user.id.to_string(),
                &activation_token,
            )
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to send activation email".to_string(),
            })?;

        Ok(UserDto::from_entity(UserModel {
            id: saved_user.id,
            email: saved_user.email,
            password: saved_user.password,
            first_name: saved_user.first_name,
            last_name: saved_user.last_name,
            is_active: saved_user.is_active,
            created_at: saved_user.created_at,
            updated_at: saved_user.updated_at,
        }))
    }

    pub async fn login(&self, dto: LoginDto) -> Result<UserDto, ApplicationError> {
        dto.validate()?;

        let user = self
            .user_repository
            .find_by_email(&dto.email)
            .await?
            .ok_or_else(|| ApplicationError::BadRequest {
                message: "Invalid credentials".to_string(),
            })?;

        let password_valid = task::spawn_blocking({
            let user_password = user.password.clone();
            move || argon::verify_password_hash(dto.password, user_password)
        })
        .await
        .map_err(|_| ApplicationError::InternalError {
            message: "Failed to verify password".to_string(),
        })?
        .map_err(|_| ApplicationError::BadRequest {
            message: "Password verification error".to_string(),
        })?;

        if !password_valid {
            return Err(ApplicationError::BadRequest {
                message: "Invalid credentials".to_string(),
            });
        }

        if !user.is_active {
            return Err(ApplicationError::Forbidden {
                message: "Access denied".to_string(),
            });
        }

        Ok(UserDto::from_entity(UserModel {
            id: user.id,
            email: user.email,
            password: user.password,
            first_name: user.first_name,
            last_name: user.last_name,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }))
    }

    pub async fn activate_user(
        &self,
        user_id: String,
        activation_token: String,
    ) -> Result<UserDto, ApplicationError> {
        let is_valid = self
            .token_service
            .validate_activation_token(&user_id, &activation_token)
            .await
            .map_err(|_| ApplicationError::BadRequest {
                message: "Invalid or expired activation token".to_string(),
            })?;

        if !is_valid {
            return Err(ApplicationError::BadRequest {
                message: "Invalid or expired activation token".to_string(),
            });
        }

        let user_id = Uuid::parse_str(&user_id).map_err(|_| ApplicationError::BadRequest {
            message: "Invalid user ID in token".to_string(),
        })?;

        let activated_user = self.user_repository.activate(user_id).await?;

        self.token_service
            .delete_activation_token(&user_id.to_string())
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to delete activation token".to_string(),
            })?;

        Ok(UserDto::from_entity(UserModel {
            id: activated_user.id,
            email: activated_user.email,
            password: activated_user.password,
            first_name: activated_user.first_name,
            last_name: activated_user.last_name,
            is_active: activated_user.is_active,
            created_at: activated_user.created_at,
            updated_at: activated_user.updated_at,
        }))
    }

    pub async fn resend_activation_email(&self, email: String) -> Result<(), ApplicationError> {
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| ApplicationError::NotFound {
                message: "User with the given email not found".to_string(),
            })?;

        if user.is_active {
            return Err(ApplicationError::BadRequest {
                message: "Account is already activated".to_string(),
            });
        }

        let has_token = self
            .token_service
            .has_active_token(&user.id.to_string())
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to check existing activation token".to_string(),
            })?;

        if has_token {
            return Err(ApplicationError::BadRequest {
                message: "An activation email was already sent. Please check your inbox or wait for the token to expire".to_string(),
            });
        }

        let activation_token = argon::generate_token();
        self.token_service
            .store_activation_token(&user.id.to_string(), &activation_token)
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to store activation token".to_string(),
            })?;

        let username = format!("{} {}", user.first_name, user.last_name);
        self.email_service
            .send_activation_email(
                &user.email,
                &username,
                &user.id.to_string(),
                &activation_token,
            )
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to send activation email".to_string(),
            })?;

        Ok(())
    }

    pub async fn forgot_password(&self, email: String) -> Result<(), ApplicationError> {
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| ApplicationError::NotFound {
                message: "User with the given email not found".to_string(),
            })?;

        if !user.is_active {
            return Err(ApplicationError::BadRequest {
                message: "Account is not activated. Please activate your account first".to_string(),
            });
        }

        let has_token = self
            .token_service
            .has_active_password_reset_token(&user.id.to_string())
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to check existing reset token".to_string(),
            })?;

        if has_token {
            return Err(ApplicationError::BadRequest {
                message: "A password reset email was already sent. Please check your inbox or wait for the token to expire".to_string(),
            });
        }

        let reset_token = argon::generate_token();
        self.token_service
            .store_password_reset_token(&user.id.to_string(), &reset_token)
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to store password reset token".to_string(),
            })?;

        let username = format!("{} {}", user.first_name, user.last_name);
        self.email_service
            .send_password_reset_email(&user.email, &username, &user.id.to_string(), &reset_token)
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to send password reset email".to_string(),
            })?;

        Ok(())
    }

    pub async fn reset_password(&self, dto: ResetPasswordDto) -> Result<(), ApplicationError> {
        dto.validate()?;

        let is_valid = self
            .token_service
            .validate_password_reset_token(&dto.user_id, &dto.reset_token)
            .await
            .map_err(|_| ApplicationError::BadRequest {
                message: "Invalid or expired reset token".to_string(),
            })?;

        if !is_valid {
            return Err(ApplicationError::BadRequest {
                message: "Invalid or expired reset token".to_string(),
            });
        }

        let user_id = Uuid::parse_str(&dto.user_id).map_err(|_| ApplicationError::BadRequest {
            message: "Invalid user ID in token".to_string(),
        })?;

        let hashed_password = task::spawn_blocking(move || argon::hash_password(dto.new_password))
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to hash new password".to_string(),
            })?
            .map_err(|_| ApplicationError::InternalError {
                message: "Password hashing failed".to_string(),
            })?;

        self.user_repository
            .update_password(user_id, &hashed_password)
            .await?;

        self.token_service
            .delete_password_reset_token(&dto.user_id)
            .await
            .map_err(|_| ApplicationError::InternalError {
                message: "Failed to delete password reset token".to_string(),
            })?;

        Ok(())
    }
}
