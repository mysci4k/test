use crate::application::dto::{
    ActivationQueryDto, BoardDto, CreateBoardDto, CreateUserDto, ForgotPasswordQueryDto, LoginDto,
    ResendActivationQueryDto, ResetPasswordDto, UserDto,
};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth endpoints
        crate::presentation::http::auth_controller::register,
        crate::presentation::http::auth_controller::login,
        crate::presentation::http::auth_controller::logout,
        crate::presentation::http::auth_controller::activate,
        crate::presentation::http::auth_controller::resend_activation,
        crate::presentation::http::auth_controller::forgot_password,
        crate::presentation::http::auth_controller::reset_password,

        // User endpoints
        crate::presentation::http::user_controller::get_user_profile,

        // Board endpoints
        crate::presentation::http::board_controller::create_board
    ),
    components(
        schemas(
            // Auth DTOs
            LoginDto,
            ActivationQueryDto,
            ResendActivationQueryDto,
            ForgotPasswordQueryDto,
            ResetPasswordDto,

            // User DTOs
            UserDto,
            CreateUserDto,

            // Board DTOs
            BoardDto,
            CreateBoardDto
        )
    ),
    tags(
        (name = "Authentication", description = "Authentication management endpoints"),
        (name = "User", description = "User management endpoints"),
        (name = "Board", description = "Board management endpoints")
    ),
    modifiers(&SecurityAddon),
    info(
        title = "Kanban API",
        version = "0.1.0",
        description = "REST API backend for a Kanban application",
        license(
            name = "MIT",
            url = "https://opensource.org/license/mit/"
        ),
        contact(
            name = "API Support",
            url = "https://github.com/mysci4k/kanban_be"
        )
    ),
    servers(
        (url = "http://localhost:8080/api", description = "Local development server")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session_cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("user-session"))),
            );
        }
    }
}
