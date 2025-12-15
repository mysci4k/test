use crate::{
    application::dto::{CreateTaskDto, TaskDto},
    domain::{
        events::{BoardEvent, SharedEventBus, TaskCreatedEvent},
        repositories::{BoardMemberRepository, ColumnRepository, Task, TaskRepository},
    },
    shared::{error::ApplicationError, utils::FractionalIndexGenerator},
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct TaskService {
    task_repository: Arc<dyn TaskRepository>,
    column_repository: Arc<dyn ColumnRepository>,
    board_member_repository: Arc<dyn BoardMemberRepository>,
    event_bus: SharedEventBus,
}

impl TaskService {
    pub fn new(
        task_repository: Arc<dyn TaskRepository>,
        column_repository: Arc<dyn ColumnRepository>,
        board_member_repository: Arc<dyn BoardMemberRepository>,
        event_bus: SharedEventBus,
    ) -> Self {
        Self {
            task_repository,
            column_repository,
            board_member_repository,
            event_bus,
        }
    }

    pub async fn create_task(
        &self,
        dto: CreateTaskDto,
        user_id: Uuid,
    ) -> Result<TaskDto, ApplicationError> {
        dto.validate()?;

        let column = self
            .column_repository
            .find_by_id(dto.column_id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound {
                message: "Column with the given ID not found".to_string(),
            })?;

        if self
            .board_member_repository
            .find_by_board_and_user_id(column.board_id, user_id)
            .await?
            .is_none()
        {
            return Err(ApplicationError::Forbidden {
                message: "You don't have access to this board".to_string(),
            });
        }

        let mut existing_tasks = self
            .task_repository
            .find_by_column_id(dto.column_id)
            .await?;

        existing_tasks.sort_by(|a, b| a.position.cmp(&b.position));
        let existing_positions: Vec<String> =
            existing_tasks.iter().map(|t| t.position.clone()).collect();

        let position = if existing_positions.is_empty() {
            FractionalIndexGenerator::first()
        } else {
            FractionalIndexGenerator::after(&existing_positions[existing_positions.len() - 1])
                .map_err(|err| ApplicationError::BadRequest {
                    message: format!("Failed to generate position: {}", err),
                })?
        };

        let task = Task::new(
            Uuid::now_v7(),
            dto.title,
            dto.description,
            dto.tags,
            position,
            dto.column_id,
        );

        let saved_task = self.task_repository.create(task).await?;

        self.event_bus
            .publish(
                column.board_id,
                BoardEvent::TaskCreated(TaskCreatedEvent {
                    task_id: saved_task.id,
                    title: saved_task.title.clone(),
                    description: saved_task.description.clone(),
                    tags: saved_task.tags.clone(),
                    position: saved_task.position.clone(),
                    column_id: saved_task.column_id,
                    created_by: user_id,
                    timestamp: saved_task.created_at,
                }),
            )
            .await;

        Ok(TaskDto::from_domain(saved_task))
    }
}
