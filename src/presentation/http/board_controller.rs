use crate::{
    application::{
        dto::{BoardDto, CreateBoardDto},
        services::BoardService,
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
    cfg.service(
        web::scope("/board")
            .service(create_board)
            .service(get_user_boards),
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
