use crate::{
    application::dto::{BoardDto, CreateBoardDto},
    domain::repositories::{Board, BoardRepository},
    shared::error::ApplicationError,
};
use entity::BoardModel;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct BoardService {
    board_repository: Arc<dyn BoardRepository>,
}

impl BoardService {
    pub fn new(board_repository: Arc<dyn BoardRepository>) -> Self {
        Self { board_repository }
    }

    pub async fn create_board(
        &self,
        dto: CreateBoardDto,
        owner_id: Uuid,
    ) -> Result<BoardDto, ApplicationError> {
        dto.validate()?;

        let board = Board::new(Uuid::now_v7(), dto.name, dto.description, owner_id);

        let saved_board = self.board_repository.create(board).await?;

        Ok(BoardDto::from_entity(BoardModel {
            id: saved_board.id,
            name: saved_board.name,
            description: saved_board.description,
            owner_id: saved_board.owner_id,
            created_at: saved_board.created_at,
            updated_at: saved_board.updated_at,
        }))
    }
}
