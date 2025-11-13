use crate::{
    application::{
        dto::{
            ActivationQueryDto, CreateUserDto, ForgotPasswordQueryDto, LoginDto,
            ResendActivationQueryDto, ResetPasswordDto, UserDto,
        },
        services::AuthService,
    },
    shared::{
        error::{ApplicationError, ErrorResponse},
        response::{ApiResponse, ApiResponseSchema},
    },
};
use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, post, web};
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

#[utoipa::path(
    post,
    description = "Registers a new user",
    path = "/auth/register",
    request_body=CreateUserDto,
    responses(
        (status = 201, description = "User registered successfully", body = ApiResponseSchema<UserDto>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
#[post("/register")]
async fn register(
    auth_service: web::Data<Arc<AuthService>>,
    dto: web::Json<CreateUserDto>,
) -> Result<ApiResponse<UserDto>, ApplicationError> {
    let user = auth_service.register(dto.into_inner()).await?;

    Ok(ApiResponse::Created {
        message: "User registered successfully. Please check your email to activate your account"
            .to_string(),
        data: user,
    })
}

#[utoipa::path(
    post,
    description = "Logs in a user and creates a session",
    path = "/auth/login",
    request_body=LoginDto,
    responses(
        (status = 200, description = "User logged in successfully", body = ApiResponseSchema<UserDto>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
#[post("/login")]
async fn login(
    auth_service: web::Data<Arc<AuthService>>,
    dto: web::Json<LoginDto>,
    req: HttpRequest,
) -> Result<ApiResponse<UserDto>, ApplicationError> {
    let user = auth_service.login(dto.into_inner()).await?;

    Identity::login(&req.extensions(), user.id.to_string()).map_err(|_| {
        ApplicationError::InternalServerError {
            message: "Failed to create user session".to_string(),
        }
    })?;

    Ok(ApiResponse::Ok {
        message: "User logged in successfully".to_string(),
        data: Some(user),
    })
}

#[utoipa::path(
    post,
    description = "Logs out the currently authenticated user by invalidating their session",
    path = "/auth/logout",
    responses(
        (status = 200, description = "User logged out successfully", body = ApiResponseSchema<String>),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    ),
    tag = "Authentication",
    security(
        ("session_cookie" = [])
    )
)]
#[post("/logout")]
async fn logout(identity: Identity) -> Result<ApiResponse<String>, ApplicationError> {
    identity.logout();

    Ok(ApiResponse::Ok {
        message: "User logged out successfully".to_string(),
        data: None,
    })
}

#[utoipa::path(
    post,
    description = "Activates a user account using the provided activation token",
    path = "/auth/activate",
    params(
        ("userId" = String, Query, description = "User ID to activate"),
        ("activationToken" = String, Query, description = "Activation token received via email")
    ),
    responses(
        (status = 200, description = "Account activated successfully", body = ApiResponseSchema<UserDto>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 409, description = "Account already activated", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
#[post("/activate")]
async fn activate(
    auth_service: web::Data<Arc<AuthService>>,
    query: web::Query<ActivationQueryDto>,
) -> Result<ApiResponse<UserDto>, ApplicationError> {
    let user = auth_service
        .activate_user(query.user_id.clone(), query.activation_token.clone())
        .await?;

    Ok(ApiResponse::Ok {
        message: "Account activated successfully".to_string(),
        data: Some(user),
    })
}

#[utoipa::path(
    post,
    description = "Resends the account activation email to the specified email address",
    path = "/auth/resend-activation",
    params(
        ("email" = String, Query, description = "Email address to resend activation link")
    ),
    responses(
        (status = 200, description = "Activation email resent successfully", body = ApiResponseSchema<String>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 409, description = "Account already activated", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
#[post("/resend-activation")]
async fn resend_activation(
    auth_service: web::Data<Arc<AuthService>>,
    query: web::Query<ResendActivationQueryDto>,
) -> Result<ApiResponse<String>, ApplicationError> {
    auth_service
        .resend_activation_email(query.email.clone())
        .await?;

    Ok(ApiResponse::Ok {
        message: "Activation email resent successfully".to_string(),
        data: None,
    })
}

#[utoipa::path(
    post,
    description = "Sends a password reset email to the specified email address",
    path = "/auth/forgot-password",
    params(
        ("email" = String, Query, description = "Email address to send password reset link")
    ),
    responses(
        (status = 200, description = "Password reset email sent successfully", body = ApiResponseSchema<String>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
#[post("/forgot-password")]
async fn forgot_password(
    auth_service: web::Data<Arc<AuthService>>,
    query: web::Query<ForgotPasswordQueryDto>,
) -> Result<ApiResponse<String>, ApplicationError> {
    auth_service.forgot_password(query.email.clone()).await?;

    Ok(ApiResponse::Ok {
        message: "Password reset email sent successfully".to_string(),
        data: None,
    })
}

#[utoipa::path(
    post,
    description = "Resets the user's password using the provided reset token",
    path = "/auth/reset-password",
    request_body=ResetPasswordDto,
    responses(
        (status = 200, description = "Password reset successfully", body = ApiResponseSchema<String>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 409, description = "Invalid or expired reset token", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
#[post("/reset-password")]
async fn reset_password(
    auth_service: web::Data<Arc<AuthService>>,
    dto: web::Json<ResetPasswordDto>,
) -> Result<ApiResponse<String>, ApplicationError> {
    auth_service.reset_password(dto.into_inner()).await?;

    Ok(ApiResponse::Ok {
        message: "Password reset successfully".to_string(),
        data: None,
    })
}
