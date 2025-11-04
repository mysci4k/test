use crate::{
    application::{dto::CreateUserDto, services::AuthService},
    shared::error::ApplicationError,
};
use actix_web::{HttpResponse, post, web};
use std::sync::Arc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(register));
}

#[post("/register")]
async fn register(
    auth_service: web::Data<Arc<AuthService>>,
    dto: web::Json<CreateUserDto>,
) -> Result<HttpResponse, ApplicationError> {
    let user = auth_service.register(dto.into_inner()).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "User registered successfully",
        "data": user
    })))
}
