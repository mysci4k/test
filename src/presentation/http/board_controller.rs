use crate::{
    application::{
        dto::{BoardDto, CreateBoardDto},
        services::BoardService,
    },
    shared::error::{ApplicationError, ErrorResponse},
};
use actix_web::{HttpResponse, post, web};
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
        (status = 201, description = "Board created successfully", body = BoardDto),
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
) -> Result<HttpResponse, ApplicationError> {
    let user_id = user_id.into_inner();
    let board = board_service
        .create_board(dto.into_inner(), user_id)
        .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Board created successfully",
        "data": board
    })))
}
