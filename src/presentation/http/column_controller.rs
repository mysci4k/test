use crate::{
    application::{
        dto::{ColumnDto, CreateColumnDto},
        services::ColumnService,
    },
    shared::{
        error::{ApplicationError, ApplicationErrorSchema},
        response::{ApiResponse, ApiResponseSchema},
    },
};
use actix_web::{post, web};
use std::sync::Arc;
use uuid::Uuid;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/column").service(create_column));
}

#[utoipa::path(
    post,
    description = "Creates a new column",
    path = "/column/",
    request_body = CreateColumnDto,
    responses(
        (status = 201, description = "Column created successfully", body = ApiResponseSchema<ColumnDto>),
        (status = 400, description = "Bad Request", body = ApplicationErrorSchema),
        (status = 401, description = "Unauthorized", body = ApplicationErrorSchema),
        (status = 403, description = "You don't have permission to perform this action", body = ApplicationErrorSchema)
    ),
    tag = "Column",
    security(
        ("session_cookie" = [])
    )
 )]
#[post("/")]
async fn create_column(
    column_service: web::Data<Arc<ColumnService>>,
    dto: web::Json<CreateColumnDto>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<ColumnDto>, ApplicationError> {
    let user_id = user_id.into_inner();
    let column = column_service
        .create_column(dto.into_inner(), user_id)
        .await?;

    Ok(ApiResponse::Created {
        message: "Column created successfully".to_string(),
        data: column,
    })
}
