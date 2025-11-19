use chrono::{DateTime, FixedOffset};
use entity::{BoardMemberModel, BoardMemberRoleEnum};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct AddBoardMemberDto {
    pub board_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateBoardMemberRoleDto {
    pub board_id: Uuid,
    pub user_id: Uuid,
    pub role: BoardMemberRoleEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct DeleteBoardMemberDto {
    pub board_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BoardMemberDto {
    pub id: Uuid,
    pub board_id: Uuid,
    pub user_id: Uuid,
    pub role: BoardMemberRoleEnum,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl BoardMemberDto {
    pub fn from_entity(board_member: BoardMemberModel) -> Self {
        Self {
            id: board_member.id,
            board_id: board_member.board_id,
            user_id: board_member.user_id,
            role: board_member.role,
            created_at: board_member.created_at,
            updated_at: board_member.updated_at,
        }
    }
}
