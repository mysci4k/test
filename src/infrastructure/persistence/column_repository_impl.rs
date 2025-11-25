use crate::{
    domain::repositories::{Column, ColumnRepository},
    shared::error::ApplicationError,
};
use async_trait::async_trait;
use entity::{ColumnActiveModel, ColumnEntity, ColumnModel};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub struct SeaOrmColumnRepository {
    db: DatabaseConnection,
}

impl SeaOrmColumnRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn to_domain(model: ColumnModel) -> Column {
        Column {
            id: model.id,
            name: model.name,
            position: model.position,
            board_id: model.board_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }

    fn to_active_model(column: Column) -> ColumnActiveModel {
        ColumnActiveModel {
            id: Set(column.id),
            name: Set(column.name),
            position: Set(column.position),
            board_id: Set(column.board_id),
            created_at: Set(column.created_at),
            updated_at: Set(column.updated_at),
        }
    }
}

#[async_trait]
impl ColumnRepository for SeaOrmColumnRepository {
    async fn create(&self, column: Column) -> Result<Column, ApplicationError> {
        let active_model = Self::to_active_model(column);

        let result = ColumnEntity::insert(active_model)
            .exec_with_returning(&self.db)
            .await
            .map_err(ApplicationError::DatabaseError)?;

        Ok(Self::to_domain(result))
    }

    async fn find_by_id(&self, column_id: Uuid) -> Result<Option<Column>, ApplicationError> {
        let result = ColumnEntity::find_by_id(column_id)
            .one(&self.db)
            .await
            .map_err(ApplicationError::DatabaseError)?;

        Ok(result.map(Self::to_domain))
    }
}
