use crate::{
    application::{
        dto::{
            ActivationQueryDto, CreateUserDto, ForgotPasswordQueryDto, LoginDto,
            ResendActivationQueryDto, ResetPasswordDto,
        },
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
            .service(logout)
            .service(activate)
            .service(resend_activation)
            .service(forgot_password)
            .service(reset_password),
    );
}

#[post("/register")]
async fn register(
    auth_service: web::Data<Arc<AuthService>>,
    dto: web::Json<CreateUserDto>,
) -> Result<HttpResponse, ApplicationError> {
    let user = auth_service.register(dto.into_inner()).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "User registered successfully. Please check your email to activate your account",
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

#[post("/activate")]
async fn activate(
    auth_service: web::Data<Arc<AuthService>>,
    query: web::Query<ActivationQueryDto>,
) -> Result<HttpResponse, ApplicationError> {
    let user = auth_service
        .activate_user(query.user_id.clone(), query.activation_token.clone())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Account activated successfully",
        "data": user
    })))
}

#[post("/resend-activation")]
async fn resend_activation(
    auth_service: web::Data<Arc<AuthService>>,
    query: web::Query<ResendActivationQueryDto>,
) -> Result<HttpResponse, ApplicationError> {
    auth_service
        .resend_activation_email(query.email.clone())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Activation email resent successfully"
    })))
}

#[post("/forgot-password")]
async fn forgot_password(
    auth_service: web::Data<Arc<AuthService>>,
    query: web::Query<ForgotPasswordQueryDto>,
) -> Result<HttpResponse, ApplicationError> {
    auth_service.forgot_password(query.email.clone()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Password reset email sent successfully"
    })))
}

#[post("/reset-password")]
async fn reset_password(
    auth_service: web::Data<Arc<AuthService>>,
    dto: web::Json<ResetPasswordDto>,
) -> Result<HttpResponse, ApplicationError> {
    auth_service.reset_password(dto.into_inner()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Password reset successfully"
    })))
}
