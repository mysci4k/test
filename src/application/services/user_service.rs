use crate::{
    application::dto::UserDto, domain::repositories::UserRepository,
    shared::error::ApplicationError,
};
use entity::UserModel;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<UserDto, ApplicationError> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound {
                message: "User with the given ID not found".to_string(),
            })?;

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
