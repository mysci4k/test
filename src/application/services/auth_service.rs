use crate::{
    application::dto::{CreateUserDto, LoginDto, UserDto},
    domain::repositories::{User, UserRepository},
    shared::{
        error::ApplicationError,
        utils::{self, email::EmailService},
    },
};
use actix_web::rt::task;
use entity::UserModel;
use redis::Client as RedisClient;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    redis_client: RedisClient,
    email_service: Arc<EmailService>,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        redis_client: RedisClient,
        email_service: Arc<EmailService>,
    ) -> Self {
        Self {
            user_repository,
            redis_client,
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

        let hashed_password = task::spawn_blocking(move || utils::password::hash(dto.password))
            .await
            .map_err(|_| ApplicationError::InternalServerError {
                message: "Failed to hash password".to_string(),
            })?
            .map_err(|_| ApplicationError::InternalServerError {
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

        let activation_token = utils::password::generate_activation_token();
        utils::activation::store_activation_token(
            &self.redis_client,
            &saved_user.id.to_string(),
            &activation_token,
        )
        .map_err(|_| ApplicationError::InternalServerError {
            message: "Failed to store activation token".to_string(),
        })?;

        let username = format!("{} {}", saved_user.first_name, saved_user.last_name);
        self.email_service
            .send_activation_email(&saved_user.email, &username, &activation_token)
            .map_err(|_| ApplicationError::InternalServerError {
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
            move || utils::password::verify_hash(dto.password, user_password)
        })
        .await
        .map_err(|_| ApplicationError::InternalServerError {
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
            return Err(ApplicationError::Forbidden);
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
}
