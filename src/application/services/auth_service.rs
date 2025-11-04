use crate::{
    application::dto::{CreateUserDto, UserDto},
    domain::repositories::{User, UserRepository},
    shared::error::ApplicationError,
};
use entity::UserModel;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
}

impl AuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn register(&self, dto: CreateUserDto) -> Result<UserDto, ApplicationError> {
        dto.validate()?;

        if self.user_repository.exists_by_email(&dto.email).await? {
            return Err(ApplicationError::Conflict {
                message: "User with this email already exists".to_string(),
            });
        }

        let user = User::new(
            Uuid::now_v7(),
            dto.email,
            dto.password,
            dto.first_name,
            dto.last_name,
        );

        let saved_user = self.user_repository.create(user).await?;

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
}
