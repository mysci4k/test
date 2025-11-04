use crate::{
    application::{
        dto::{CreateUserDto, LoginDto},
        services::AuthService,
    },
    shared::error::ApplicationError,
};
use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use std::sync::Arc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(register)
            .service(login)
            .service(logout),
    );
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

#[post("/login")]
async fn login(
    auth_service: web::Data<Arc<AuthService>>,
    dto: web::Json<LoginDto>,
    req: HttpRequest,
) -> Result<HttpResponse, ApplicationError> {
    let user = auth_service.login(dto.into_inner()).await?;

    Identity::login(&req.extensions(), user.id.to_string()).map_err(|_| {
        ApplicationError::InternalServerError {
            message: "Failed to create user session".to_string(),
        }
    })?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User logged in successfully",
        "data": user
    })))
}

#[post("/logout")]
async fn logout(identity: Identity) -> Result<HttpResponse, ApplicationError> {
    identity.logout();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User logged out successfully"
    })))
}
