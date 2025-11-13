use crate::{
    application::{
        dto::{BoardDto, CreateBoardDto},
        services::BoardService,
    },
    shared::{
        error::{ApplicationError, ErrorResponse},
        response::{ApiResponse, ApiResponseSchema},
    },
};
use actix_web::{post, web};
use std::sync::Arc;
use uuid::Uuid;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/board").service(create_board));
}

#[utoipa::path(
    post,
    description = "Creates a new board",
    path = "/board/",
    request_body = CreateBoardDto,
    responses(
        (status = 201, description = "Board created successfully", body = ApiResponseSchema<BoardDto>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
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
