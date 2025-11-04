use crate::{
    domain::repositories::{User, UserRepository},
    shared::error::ApplicationError,
};
use async_trait::async_trait;
use entity::{UserActiveModel, UserColumn, UserEntity, UserModel};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};

pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn to_domain(model: UserModel) -> User {
        User {
            id: model.id,
            email: model.email,
            password: model.password,
            first_name: model.first_name,
            last_name: model.last_name,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }

    fn to_active_model(user: User) -> UserActiveModel {
        UserActiveModel {
            id: Set(user.id),
            email: Set(user.email),
            password: Set(user.password),
            first_name: Set(user.first_name),
            last_name: Set(user.last_name),
            is_active: Set(user.is_active),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn create(&self, user: User) -> Result<User, ApplicationError> {
        let active_model = Self::to_active_model(user);

        let result = UserEntity::insert(active_model)
            .exec_with_returning(&self.db)
            .await
            .map_err(ApplicationError::DatabaseError)?;

        Ok(Self::to_domain(result))
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, ApplicationError> {
        let count = UserEntity::find()
            .filter(UserColumn::Email.eq(email.to_string()))
            .count(&self.db)
            .await
            .map_err(ApplicationError::DatabaseError)?;

        Ok(count > 0)
    }
}
