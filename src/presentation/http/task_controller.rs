use crate::{
    application::{
        dto::{CreateTaskDto, TaskDto},
        services::TaskService,
    },
    shared::{
        error::{ApplicationError, ApplicationErrorSchema},
        response::{ApiResponse, ApiResponseSchema},
    },
};
use actix_web::{get, post, web};
use std::sync::Arc;
use uuid::Uuid;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/task").service(create_task).service(get_task));
}

#[utoipa::path(
    post,
    description = "***PROTECTED ENDPOINT***\n\nCreates a new task within a column. The task will be positioned at the end of the column. All board members can create tasks.",
    path = "/task/",
    request_body = CreateTaskDto,
    responses(
        (status = 201, description = "Created - Task created successfully", body = ApiResponseSchema<TaskDto>),
        (status = 400, description = "Bad Request - Invalid input data", body = ApplicationErrorSchema),
        (status = 401, description = "Unauthorized - No active session or session has expired", body = ApplicationErrorSchema),
        (status = 403, description = "Forbidden - User doesn't have access to this board", body = ApplicationErrorSchema),
        (status = 404, description = "Not Found - Column with the given ID not found", body = ApplicationErrorSchema),
        (status = 500, description = "Internal Server Error - Failed to create task", body = ApplicationErrorSchema)
    ),
    tag = "Task",
    security(
        ("session_cookie" = [])
    )
)]
#[post("/")]
async fn create_task(
    task_service: web::Data<Arc<TaskService>>,
    dto: web::Json<CreateTaskDto>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<TaskDto>, ApplicationError> {
    let user_id = user_id.into_inner();
    let task = task_service.create_task(dto.into_inner(), user_id).await?;

    Ok(ApiResponse::Created {
        message: "Task created successfully".to_string(),
        data: task,
    })
}

#[utoipa::path(
    get,
    description = "***PROTECTED ENDPOINT***\n\nRetrieves detailed information about a specific task by its ID. User must be a member of the board to access this endpoint.",
    path = "/task/{taskId}",
    params(
        ("taskId" = Uuid, Path, description = "Unique identifier of the task")
    ),
    responses(
        (status = 200, description = "OK - Task data retrieved successfully", body = ApiResponseSchema<TaskDto>),
        (status = 401, description = "Unauthorized - No active session or session has expired", body = ApplicationErrorSchema),
        (status = 403, description = "Forbidden - User doesn't have access to this board", body = ApplicationErrorSchema),
        (status = 404, description = "Not Found - Task with the given ID not found", body = ApplicationErrorSchema),
        (status = 500, description = "Internal Server Error - Failed to retrieve task", body = ApplicationErrorSchema)
    ),
    tag = "Task",
    security(
        ("session_cookie" = [])
    )
)]
#[get("/{taskId}")]
async fn get_task(
    task_service: web::Data<Arc<TaskService>>,
    task_id: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<TaskDto>, ApplicationError> {
    let task_id = task_id.into_inner();
    let user_id = user_id.into_inner();
    let task = task_service.get_task_by_id(task_id, user_id).await?;

    Ok(ApiResponse::Found {
        message: "Task data retrieved successfully".to_string(),
        data: task,
        page: None,
        total_pages: None,
    })
}
