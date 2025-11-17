use crate::{
    application::dto::{
        AddBoardMemberDto, BoardDto, BoardMemberDto, CreateBoardDto, UpdateBoardDto,
    },
    domain::repositories::{
        Board, BoardMember, BoardMemberRepository, BoardRepository, UserRepository,
    },
    shared::error::ApplicationError,
};
use chrono::Utc;
use entity::{BoardMemberModel, BoardMemberRoleEnum, BoardModel};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct BoardService {
    user_repository: Arc<dyn UserRepository>,
    board_repository: Arc<dyn BoardRepository>,
    board_member_repository: Arc<dyn BoardMemberRepository>,
}

impl BoardService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        board_repository: Arc<dyn BoardRepository>,
        board_member_repository: Arc<dyn BoardMemberRepository>,
    ) -> Self {
        Self {
            user_repository,
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

    pub async fn get_board_by_id(
        &self,
        board_id: Uuid,
        user_id: Uuid,
    ) -> Result<BoardDto, ApplicationError> {
        let board = self
            .board_repository
            .find_by_id(board_id, user_id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound {
                message: "Board with the given ID not found".to_string(),
            })?;

        Ok(BoardDto::from_entity(BoardModel {
            id: board.id,
            name: board.name,
            description: board.description,
            owner_id: board.owner_id,
            created_at: board.created_at,
            updated_at: board.updated_at,
        }))
    }

    pub async fn get_boards_by_membership(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<BoardDto>, ApplicationError> {
        let boards = self.board_repository.find_by_membership(user_id).await?;

        Ok(boards
            .into_iter()
            .map(|board| {
                BoardDto::from_entity(BoardModel {
                    id: board.id,
                    name: board.name,
                    description: board.description,
                    owner_id: board.owner_id,
                    created_at: board.created_at,
                    updated_at: board.updated_at,
                })
            })
            .collect())
    }

    pub async fn update_board(
        &self,
        dto: UpdateBoardDto,
        board_id: Uuid,
        user_id: Uuid,
    ) -> Result<BoardDto, ApplicationError> {
        dto.validate()?;

        let mut board = self
            .board_repository
            .find_by_id(board_id, user_id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound {
                message: "Board with the given ID not found".to_string(),
            })?;

        if !self
            .board_member_repository
            .check_permissions(
                board_id,
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
            board.name = name;
        }
        board.description = dto.description;
        board.updated_at = Utc::now().fixed_offset();

        let updated_board = self.board_repository.update(board).await?;

        Ok(BoardDto::from_entity(BoardModel {
            id: updated_board.id,
            name: updated_board.name,
            description: updated_board.description,
            owner_id: updated_board.owner_id,
            created_at: updated_board.created_at,
            updated_at: updated_board.updated_at,
        }))
    }

    pub async fn add_board_member(
        &self,
        dto: AddBoardMemberDto,
        user_id: Uuid,
    ) -> Result<BoardMemberDto, ApplicationError> {
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

        if !self.user_repository.exists_by_id(dto.user_id).await? {
            return Err(ApplicationError::NotFound {
                message: "User with the given ID not found".to_string(),
            });
        }

        let board_member = BoardMember::new(
            Uuid::now_v7(),
            dto.board_id,
            dto.user_id,
            BoardMemberRoleEnum::Member,
        );

        let saved_board_member = self.board_member_repository.create(board_member).await?;

        Ok(BoardMemberDto::from_entity(BoardMemberModel {
            id: saved_board_member.id,
            board_id: saved_board_member.board_id,
            user_id: saved_board_member.user_id,
            role: saved_board_member.role,
            created_at: saved_board_member.created_at,
            updated_at: saved_board_member.updated_at,
        }))
    }
}
