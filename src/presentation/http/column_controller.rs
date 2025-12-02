use crate::{
    application::{
        dto::{ColumnDto, CreateColumnDto, UpdateColumnDto},
        services::ColumnService,
    },
    shared::{
        error::{ApplicationError, ApplicationErrorSchema},
        response::{ApiResponse, ApiResponseSchema},
    },
};
use actix_web::{delete, get, post, put, web};
use std::sync::Arc;
use uuid::Uuid;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/column")
            .service(create_column)
            .service(get_column)
            .service(get_board_columns)
            .service(update_column)
            .service(move_column)
            .service(delete_column),
    );
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

#[utoipa::path(
    get,
    description = "Retrieves a column by its ID",
    path = "/column/{column_id}",
    responses(
        (status = 200, description = "Column data retrieved successfully", body = ApiResponseSchema<ColumnDto>),
        (status = 400, description = "Bad Request", body = ApplicationErrorSchema),
        (status = 401, description = "Unauthorized", body = ApplicationErrorSchema),
        (status = 403, description = "You don't have access to this board", body = ApplicationErrorSchema),
        (status = 404, description = "Column with the given ID not found", body = ApplicationErrorSchema)
    ),
    tag = "Column",
    security(
        ("session_cookie" = [])
    )
 )]
#[get("/{column_id}")]
async fn get_column(
    column_service: web::Data<Arc<ColumnService>>,
    column_id: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<ColumnDto>, ApplicationError> {
    let user_id = user_id.into_inner();
    let column = column_service
        .get_column_by_id(column_id.into_inner(), user_id)
        .await?;

    Ok(ApiResponse::Found {
        message: "Column data retrieved successfully".to_string(),
        data: column,
        page: None,
        total_pages: None,
    })
}

#[utoipa::path(
    get,
    description = "Retrieves all columns for a given board",
    path = "/column/board/{board_id}",
    responses(
        (status = 200, description = "Columns retrieved successfully", body = ApiResponseSchema<Vec<ColumnDto>>),
        (status = 400, description = "Bad Request", body = ApplicationErrorSchema),
        (status = 401, description = "Unauthorized", body = ApplicationErrorSchema),
        (status = 403, description = "You don't have access to this board", body = ApplicationErrorSchema),
    ),
    tag = "Column",
    security(
        ("session_cookie" = [])
    )
 )]
#[get("/board/{board_id}")]
async fn get_board_columns(
    column_service: web::Data<Arc<ColumnService>>,
    board_id: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<Vec<ColumnDto>>, ApplicationError> {
    let user_id = user_id.into_inner();
    let columns = column_service
        .get_board_columns(board_id.into_inner(), user_id)
        .await?;

    Ok(ApiResponse::Found {
        message: "Columns retrieved successfully".to_string(),
        data: columns,
        page: None,
        total_pages: None,
    })
}

#[utoipa::path(
    put,
    description = "Updates an existing column",
    path = "/column/{column_id}",
    request_body = UpdateColumnDto,
    responses(
        (status = 200, description = "Column updated successfully", body = ApiResponseSchema<ColumnDto>),
        (status = 400, description = "Bad Request", body = ApplicationErrorSchema),
        (status = 401, description = "Unauthorized", body = ApplicationErrorSchema),
        (status = 403, description = "You don't have access to this board", body = ApplicationErrorSchema),
        (status = 404, description = "Column with the given ID not found", body = ApplicationErrorSchema)
    ),
    tag = "Column",
    security(
        ("session_cookie" = [])
    )
 )]
#[put("/{column_id}")]
async fn update_column(
    column_service: web::Data<Arc<ColumnService>>,
    dto: web::Json<UpdateColumnDto>,
    column_id: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<ColumnDto>, ApplicationError> {
    let column_id = column_id.into_inner();
    let user_id = user_id.into_inner();
    let column = column_service
        .update_column(dto.into_inner(), column_id, user_id)
        .await?;

    Ok(ApiResponse::Updated {
        message: "Column updated successfully".to_string(),
        data: column,
    })
}

#[utoipa::path(
    put,
    description = "Moves a column to a new position",
    path = "/column/{column_id}/move/{position}",
    responses(
        (status = 200, description = "Column moved successfully", body = ApiResponseSchema<ColumnDto>),
        (status = 400, description = "Bad Request", body = ApplicationErrorSchema),
        (status = 401, description = "Unauthorized", body = ApplicationErrorSchema),
        (status = 403, description = "You don't have permission to perform this action", body = ApplicationErrorSchema),
        (status = 404, description = "Column with the given ID not found", body = ApplicationErrorSchema)
    ),
    tag = "Column",
    security(
        ("session_cookie" = [])
    )
 )]
#[put("/{column_id}/move/{position}")]
async fn move_column(
    column_service: web::Data<Arc<ColumnService>>,
    path: web::Path<(Uuid, usize)>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<ColumnDto>, ApplicationError> {
    let (column_id, position) = path.into_inner();
    let user_id = user_id.into_inner();
    let column = column_service
        .move_column(position, column_id, user_id)
        .await?;

    Ok(ApiResponse::Updated {
        message: "Column moved successfully".to_string(),
        data: column,
    })
}

#[utoipa::path(
    delete,
    description = "Deletes a column by its ID",
    path = "/column/{column_id}",
    responses(
        (status = 200, description = "Column deleted successfully", body = ApiResponseSchema<u64>),
        (status = 400, description = "Bad Request", body = ApplicationErrorSchema),
        (status = 401, description = "Unauthorized", body = ApplicationErrorSchema),
        (status = 403, description = "You don't have permission to perform this action", body = ApplicationErrorSchema),
        (status = 404, description = "Column with the given ID not found", body = ApplicationErrorSchema)
    ),
    tag = "Column",
    security(
        ("session_cookie" = [])
    )
 )]
#[delete("/{column_id}")]
async fn delete_column(
    column_service: web::Data<Arc<ColumnService>>,
    column_id: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<()>, ApplicationError> {
    let column_id = column_id.into_inner();
    let user_id = user_id.into_inner();
    let rows_affected = column_service.delete_column(column_id, user_id).await?;

    Ok(ApiResponse::Deleted {
        message: "Column deleted successfully".to_string(),
        rows_affected,
    })
}
