use crate::{
    application::{dto::UserDto, services::UserService},
    shared::error::{ApplicationError, ErrorResponse},
};
use actix_web::{HttpResponse, get, web};
use std::sync::Arc;
use uuid::Uuid;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(get_user_profile));
}

#[utoipa::path(
    get,
    description = "Retrieves the profile of the authenticated user",
    path = "/user/profile",
    responses(
        (status = 200, description = "User profile retrieved successfully", body = UserDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse)
    ),
    tag = "User",
    security(
        ("session_cookie" = [])
    )
)]
#[get("/profile")]
async fn get_user_profile(
    user_service: web::Data<Arc<UserService>>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ApplicationError> {
    let user_id = user_id.into_inner();
    let user = user_service.get_user_by_id(user_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User profile retrieved successfully",
        "data": user
    })))
}
