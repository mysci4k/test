use crate::{
    application::services::WebSocketService,
    shared::error::{ApplicationError, ApplicationErrorSchema},
};
use actix_web::{
    HttpRequest, Responder, get,
    web::{self, Payload},
};
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/ws").service(websocket_handler));
}

#[utoipa::path(
    get,
    description = "Establishes WebSocket connection",
    path = "/ws/board/{board_id}",
    responses(
        (status = 101, description = "WebSocket connection established"),
        (status = 401, description = "Unauthorized", body = ApplicationErrorSchema),
        (status = 403, description = "Forbidden - no access to this board", body = ApplicationErrorSchema),
        (status = 404, description = "Board not found", body = ApplicationErrorSchema)
    ),
    tag = "WebSocket",
    security(
        ("session_cookie" = [])
    )
)]
#[get("/board/{board_id}")]
async fn websocket_handler(
    websocket_service: web::Data<Arc<WebSocketService>>,
    req: HttpRequest,
    stream: Payload,
    board_id: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<impl Responder, ApplicationError> {
    let board_id = board_id.into_inner();
    let user_id = user_id.into_inner();

    websocket_service
        .verify_board_access(board_id, user_id)
        .await?;

    let (response, session, msg_stream) = actix_ws::handle(&req, stream).map_err(|err| {
        error!("WebSocket handshake error: {}", err);
        ApplicationError::InternalError {
            message: "Failed to establish WebSocket connection".to_string(),
        }
    })?;

    actix_web::rt::spawn(async move {
        websocket_service
            .handle_connection(board_id, user_id, session, msg_stream)
            .await;
    });

    Ok(response)
}
