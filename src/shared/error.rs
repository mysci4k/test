use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::Error;
use sea_orm::DbErr;
use serde::Serialize;
use std::fmt;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ApplicationError {
    BadRequest { message: String },
    Forbidden,
    Conflict { message: String },
    InternalServerError { message: String },
    ValidationError { message: String },
    DatabaseError(DbErr),
}

impl From<DbErr> for ApplicationError {
    fn from(err: DbErr) -> Self {
        ApplicationError::DatabaseError(err)
    }
}

impl From<ValidationErrors> for ApplicationError {
    fn from(err: ValidationErrors) -> Self {
        ApplicationError::ValidationError {
            message: format!("{}", err),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type, message, details) = match self {
            ApplicationError::BadRequest { message } => (
                StatusCode::BAD_REQUEST,
                "Bad Request",
                message.clone(),
                None,
            ),
            ApplicationError::Forbidden => (
                StatusCode::FORBIDDEN,
                "Forbidden",
                "Access denied".to_string(),
                None,
            ),
            ApplicationError::Conflict { message } => {
                (StatusCode::CONFLICT, "Conflict", message.clone(), None)
            }
            ApplicationError::InternalServerError { message } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                message.clone(),
                None,
            ),
            ApplicationError::ValidationError { message } => (
                StatusCode::BAD_REQUEST,
                "Validation Error",
                message.clone(),
                None,
            ),
            ApplicationError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database Error",
                "A database error occurred".to_string(),
                Some(err.to_string()),
            ),
        };

        let error_response = ErrorResponse {
            error: error_type.to_string(),
            message,
            details,
        };

        HttpResponse::build(status).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApplicationError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ApplicationError::Forbidden => StatusCode::FORBIDDEN,
            ApplicationError::Conflict { .. } => StatusCode::CONFLICT,
            ApplicationError::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApplicationError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            ApplicationError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplicationError::BadRequest { message } => write!(f, "Bad request: {}", message),
            ApplicationError::Forbidden => write!(f, "Forbidden"),
            ApplicationError::Conflict { message } => write!(f, "Conflict: {}", message),
            ApplicationError::InternalServerError { message } => {
                write!(f, "Internal server error: {}", message)
            }

            ApplicationError::ValidationError { message } => {
                write!(f, "Validation error: {}", message)
            }
            ApplicationError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}
