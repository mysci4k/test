use chrono::{DateTime, FixedOffset};
use entity::ColumnModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateColumnDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Column name must be between 1 and 100 characters long"
    ))]
    pub name: String,
    #[validate(range(min = 0, message = "Column position must be a non-negative integer"))]
    pub position: i32,
    pub board_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateColumnDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Column name must be between 1 and 100 characters long"
    ))]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDto {
    pub id: Uuid,
    pub name: String,
    pub position: i32,
    pub board_id: Uuid,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl ColumnDto {
    pub fn from_entity(column: ColumnModel) -> Self {
        Self {
            id: column.id,
            name: column.name,
            position: column.position,
            board_id: column.board_id,
            created_at: column.created_at,
            updated_at: column.updated_at,
        }
    }
}
