use crate::{application::services::UserService, shared::error::ApplicationError};
use actix_web::{HttpResponse, get, web};
use std::sync::Arc;
use uuid::Uuid;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(get_user_profile));
}

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
