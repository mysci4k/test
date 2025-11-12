use crate::{
    domain::repositories::{BoardMember, BoardMemberRepository},
    shared::error::ApplicationError,
};
use async_trait::async_trait;
use entity::{BoardMemberActiveModel, BoardMemberEntity, BoardMemberModel};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

pub struct SeaOrmBoardMemberRepository {
    db: DatabaseConnection,
}

impl SeaOrmBoardMemberRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn to_domain(model: BoardMemberModel) -> BoardMember {
        BoardMember {
            id: model.id,
            board_id: model.board_id,
            user_id: model.user_id,
            role: model.role,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }

    fn to_active_model(board_member: BoardMember) -> BoardMemberActiveModel {
        BoardMemberActiveModel {
            id: Set(board_member.id),
            board_id: Set(board_member.board_id),
            user_id: Set(board_member.user_id),
            role: Set(board_member.role),
            created_at: Set(board_member.created_at),
            updated_at: Set(board_member.updated_at),
        }
    }
}

#[async_trait]
impl BoardMemberRepository for SeaOrmBoardMemberRepository {
    async fn create(&self, board_member: BoardMember) -> Result<BoardMember, ApplicationError> {
        let active_model = Self::to_active_model(board_member);

        let result = BoardMemberEntity::insert(active_model)
            .exec_with_returning(&self.db)
            .await
            .map_err(ApplicationError::DatabaseError)?;

        Ok(Self::to_domain(result))
    }
}
