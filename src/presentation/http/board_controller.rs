use crate::{
    application::{
        dto::{BoardDto, CreateBoardDto, UpdateBoardDto},
        services::BoardService,
    },
    shared::{
        error::{ApplicationError, ApplicationErrorSchema},
        response::{ApiResponse, ApiResponseSchema},
    },
};
use actix_web::{get, post, put, web};
use std::sync::Arc;
use uuid::Uuid;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/board")
            .service(create_board)
            .service(get_board)
            .service(get_user_boards)
            .service(update_board),
    );
}

#[utoipa::path(
    post,
    description = "Creates a new board",
    path = "/board/",
    request_body = CreateBoardDto,
    responses(
        (status = 201, description = "Board created successfully", body = ApiResponseSchema<BoardDto>),
        (status = 400, description = "Bad Request", body = ApplicationErrorSchema),
        (status = 401, description = "Unauthorized", body = ApplicationErrorSchema)
    ),
    tag = "Board",
    security(
        ("session_cookie" = [])
    )
)]
#[post("/")]
async fn create_board(
    board_service: web::Data<Arc<BoardService>>,
    dto: web::Json<CreateBoardDto>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<BoardDto>, ApplicationError> {
    let user_id = user_id.into_inner();
    let board = board_service
        .create_board(dto.into_inner(), user_id)
        .await?;

    Ok(ApiResponse::Created {
        message: "Board created successfully".to_string(),
        data: board,
    })
}

#[utoipa::path(
    get,
    description = "Retrieves a board by its ID",
    path = "/board/{board_id}",
    responses(
        (status = 200, description = "Board data retrieved successfully", body = ApiResponseSchema<BoardDto>),
        (status = 401, description = "Unauthorized", body = ApplicationErrorSchema),
        (status = 404, description = "Board not found", body = ApplicationErrorSchema)
    ),
    tag = "Board",
    security(
        ("session_cookie" = [])
    )
)]
#[get("/{board_id}")]
async fn get_board(
    board_service: web::Data<Arc<BoardService>>,
    board_id: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<BoardDto>, ApplicationError> {
    let board_id = board_id.into_inner();
    let user_id = user_id.into_inner();
    let board = board_service.get_board_by_id(board_id, user_id).await?;

    Ok(ApiResponse::Found {
        message: "Board data retrieved successfully".to_string(),
        data: board,
        page: None,
        total_pages: None,
    })
}

#[utoipa::path(
    get,
    description = "Retrieves all boards the user is a member of",
    path = "/board/",
    responses(
        (status = 200, description = "Boards retrieved successfully", body = ApiResponseSchema<Vec<BoardDto>>),
        (status = 401, description = "Unauthorized", body = ApplicationErrorSchema)
    ),
    tag = "Board",
    security(
        ("session_cookie" = [])
    )
)]
#[get("/")]
async fn get_user_boards(
    board_service: web::Data<Arc<BoardService>>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<Vec<BoardDto>>, ApplicationError> {
    let user_id = user_id.into_inner();
    let boards = board_service.get_boards_by_membership(user_id).await?;

    Ok(ApiResponse::Found {
        message: "Boards retrieved successfully".to_string(),
        data: boards,
        page: None,
        total_pages: None,
    })
}

#[utoipa::path(
    put,
    description = "Updates an existing board",
    path = "/board/{board_id}",
    request_body = UpdateBoardDto,
    responses(
        (status = 200, description = "Board updated successfully", body = ApiResponseSchema<BoardDto>),
        (status = 403, description = "You don't have permission to perform this action", body = ApplicationErrorSchema),
        (status = 404, description = "Board with the given ID not found", body = ApplicationErrorSchema)
    ),
    tag = "Board",
    security(
        ("session_cookie" = [])
    )
)]
#[put("/{board_id}")]
async fn update_board(
    board_service: web::Data<Arc<BoardService>>,
    dto: web::Json<UpdateBoardDto>,
    board_id: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<ApiResponse<BoardDto>, ApplicationError> {
    let board_id = board_id.into_inner();
    let user_id = user_id.into_inner();
    let board = board_service
        .update_board(dto.into_inner(), board_id, user_id)
        .await?;

    Ok(ApiResponse::Updated {
        message: "Board updated successfully".to_string(),
        data: board,
    })
}
