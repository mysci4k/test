use chrono::{DateTime, FixedOffset};
use entity::BoardMemberRoleEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum BoardEvent {
    BoardCreated(BoardCreatedEvent),
    BoardUpdated(BoardUpdatedEvent),
    BoardDeleted(BoardDeletedEvent),
    MemberAdded(MemberAddedEvent),
    MemberRoleChanged(MemberRoleChangedEvent),
    MemberRemoved(MemberRemovedEvent),
    ColumnCreated(ColumnCreatedEvent),
    ColumnUpdated(ColumnUpdatedEvent),
    ColumnMoved(ColumnMovedEvent),
    ColumnDeleted(ColumnDeletedEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardCreatedEvent {
    pub board_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardUpdatedEvent {
    pub board_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub updated_by: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardDeletedEvent {
    pub board_id: Uuid,
    pub deleted_by: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberAddedEvent {
    pub board_id: Uuid,
    pub user_id: Uuid,
    pub role: BoardMemberRoleEnum,
    pub added_by: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberRoleChangedEvent {
    pub board_id: Uuid,
    pub user_id: Uuid,
    pub role: BoardMemberRoleEnum,
    pub changed_by: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberRemovedEvent {
    pub board_id: Uuid,
    pub user_id: Uuid,
    pub removed_by: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnCreatedEvent {
    pub column_id: Uuid,
    pub name: String,
    pub position: String,
    pub board_id: Uuid,
    pub created_by: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnUpdatedEvent {
    pub column_id: Uuid,
    pub name: Option<String>,
    pub updated_by: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnMovedEvent {
    pub column_id: Uuid,
    pub old_position: usize,
    pub new_position: usize,
    pub moved_by: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDeletedEvent {
    pub column_id: Uuid,
    pub deleted_by: Uuid,
    pub timestamp: DateTime<FixedOffset>,
}
