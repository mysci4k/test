use crate::{
    application::{dto::CreateBoardDto, services::BoardService},
    shared::error::ApplicationError,
};
use actix_web::{HttpResponse, post, web};
use std::sync::Arc;
use uuid::Uuid;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/board").service(create_board));
}

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
