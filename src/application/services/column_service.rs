use crate::{
    application::dto::{ColumnDto, CreateColumnDto, UpdateColumnDto},
    domain::{
        events::{BoardEvent, ColumnCreatedEvent, ColumnUpdatedEvent, SharedEventBus},
        repositories::{BoardMemberRepository, Column, ColumnRepository},
    },
    shared::{error::ApplicationError, utils::FractionalIndexGenerator},
};
use chrono::Utc;
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

        let mut existing_columns = self
            .column_repository
            .find_by_board_id(dto.board_id)
            .await?;

        existing_columns.sort_by(|a, b| a.position.cmp(&b.position));
        let existing_positions: Vec<String> = existing_columns
            .iter()
            .map(|c| c.position.clone())
            .collect();

        let position = if existing_positions.is_empty() {
            FractionalIndexGenerator::first()
        } else {
            FractionalIndexGenerator::after(&existing_positions[existing_positions.len() - 1])
                .map_err(|err| ApplicationError::BadRequest {
                    message: format!("Failed to generate position: {}", err),
                })?
        };

        let column = Column::new(Uuid::now_v7(), dto.name, position, dto.board_id);

        let saved_column = self.column_repository.create(column).await?;

        self.event_bus
            .publish(
                dto.board_id,
                BoardEvent::ColumnCreated(ColumnCreatedEvent {
                    column_id: saved_column.id,
                    name: saved_column.name.clone(),
                    position: saved_column.position.clone(),
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

    pub async fn get_column_by_id(
        &self,
        column_id: Uuid,
        user_id: Uuid,
    ) -> Result<ColumnDto, ApplicationError> {
        let column = self
            .column_repository
            .find_by_id(column_id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound {
                message: "Column with the given ID not found".to_string(),
            })?;

        if !self
            .board_member_repository
            .find_by_board_and_user_id(column.board_id, user_id)
            .await?
            .is_some()
        {
            return Err(ApplicationError::Forbidden {
                message: "You don't have access to this board".to_string(),
            });
        }

        Ok(ColumnDto::from_entity(ColumnModel {
            id: column.id,
            name: column.name,
            position: column.position,
            board_id: column.board_id,
            created_at: column.created_at,
            updated_at: column.updated_at,
        }))
    }

    pub async fn get_board_columns(
        &self,
        board_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<ColumnDto>, ApplicationError> {
        if !self
            .board_member_repository
            .find_by_board_and_user_id(board_id, user_id)
            .await?
            .is_some()
        {
            return Err(ApplicationError::Forbidden {
                message: "You don't have access to this board".to_string(),
            });
        }

        let columns = self.column_repository.find_by_board_id(board_id).await?;

        Ok(columns
            .into_iter()
            .map(|column| {
                ColumnDto::from_entity(ColumnModel {
                    id: column.id,
                    name: column.name,
                    position: column.position,
                    board_id: column.board_id,
                    created_at: column.created_at,
                    updated_at: column.updated_at,
                })
            })
            .collect())
    }

    pub async fn update_column(
        &self,
        dto: UpdateColumnDto,
        column_id: Uuid,
        user_id: Uuid,
    ) -> Result<ColumnDto, ApplicationError> {
        dto.validate()?;

        let mut column = self
            .column_repository
            .find_by_id(column_id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound {
                message: "Column with the given ID not found".to_string(),
            })?;

        if !self
            .board_member_repository
            .check_permissions(
                column.board_id,
                user_id,
                vec![BoardMemberRoleEnum::Owner, BoardMemberRoleEnum::Moderator],
            )
            .await?
        {
            return Err(ApplicationError::Forbidden {
                message: "You don't have permission to perform this action".to_string(),
            });
        }

        if let Some(name) = dto.name {
            column.name = name;
        }
        column.updated_at = Utc::now().fixed_offset();

        let updated_column = self.column_repository.update(column).await?;

        self.event_bus
            .publish(
                updated_column.board_id,
                BoardEvent::ColumnUpdated(ColumnUpdatedEvent {
                    column_id,
                    name: Some(updated_column.name.clone()),
                    updated_by: user_id,
                    timestamp: updated_column.updated_at,
                }),
            )
            .await;

        Ok(ColumnDto::from_entity(ColumnModel {
            id: updated_column.id,
            name: updated_column.name,
            position: updated_column.position,
            board_id: updated_column.board_id,
            created_at: updated_column.created_at,
            updated_at: updated_column.updated_at,
        }))
    }
}
