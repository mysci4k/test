use crate::{
    application::dto::{BoardDto, CreateBoardDto},
    domain::repositories::{Board, BoardMember, BoardMemberRepository, BoardRepository},
    shared::error::ApplicationError,
};
use entity::{BoardMemberRoleEnum, BoardModel};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct BoardService {
    board_repository: Arc<dyn BoardRepository>,
    board_member_repository: Arc<dyn BoardMemberRepository>,
}

impl BoardService {
    pub fn new(
        board_repository: Arc<dyn BoardRepository>,
        board_member_repository: Arc<dyn BoardMemberRepository>,
    ) -> Self {
        Self {
            board_repository,
            board_member_repository,
        }
    }

    pub async fn create_board(
        &self,
        dto: CreateBoardDto,
        owner_id: Uuid,
    ) -> Result<BoardDto, ApplicationError> {
        dto.validate()?;

        let board_id = Uuid::now_v7();
        let board = Board::new(board_id, dto.name, dto.description, owner_id);

        let saved_board = self.board_repository.create(board).await?;

        let board_member = BoardMember::new(
            Uuid::now_v7(),
            board_id,
            owner_id,
            BoardMemberRoleEnum::Owner,
        );

        self.board_member_repository.create(board_member).await?;

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
