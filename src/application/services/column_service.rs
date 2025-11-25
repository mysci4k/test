use crate::{
    application::dto::{ColumnDto, CreateColumnDto},
    domain::{
        events::{BoardEvent, ColumnCreatedEvent, SharedEventBus},
        repositories::{BoardMemberRepository, Column, ColumnRepository},
    },
    shared::error::ApplicationError,
};
use entity::{BoardMemberRoleEnum, ColumnModel};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct ColumnService {
    column_repository: Arc<dyn ColumnRepository>,
    board_member_repository: Arc<dyn BoardMemberRepository>,
    event_bus: SharedEventBus,
}

impl ColumnService {
    pub fn new(
        column_repository: Arc<dyn ColumnRepository>,
        board_member_repository: Arc<dyn BoardMemberRepository>,
        event_bus: SharedEventBus,
    ) -> Self {
        Self {
            column_repository,
            board_member_repository,
            event_bus,
        }
    }

    pub async fn create_column(
        &self,
        dto: CreateColumnDto,
        user_id: Uuid,
    ) -> Result<ColumnDto, ApplicationError> {
        dto.validate()?;

        if !self
            .board_member_repository
            .check_permissions(
                dto.board_id,
                user_id,
                vec![BoardMemberRoleEnum::Owner, BoardMemberRoleEnum::Moderator],
            )
            .await?
        {
            return Err(ApplicationError::Forbidden {
                message: "You don't have permission to perform this action".to_string(),
            });
        }

        let column = Column::new(Uuid::now_v7(), dto.name, dto.position, dto.board_id);

        let saved_column = self.column_repository.create(column).await?;

        self.event_bus
            .publish(
                dto.board_id,
                BoardEvent::ColumnCreated(ColumnCreatedEvent {
                    column_id: saved_column.id,
                    name: saved_column.name.clone(),
                    position: saved_column.position,
                    board_id: saved_column.board_id,
                    created_by: user_id,
                    timestamp: saved_column.created_at,
                }),
            )
            .await;

        Ok(ColumnDto::from_entity(ColumnModel {
            id: saved_column.id,
            name: saved_column.name,
            position: saved_column.position,
            board_id: saved_column.board_id,
            created_at: saved_column.created_at,
            updated_at: saved_column.updated_at,
        }))
    }
}
