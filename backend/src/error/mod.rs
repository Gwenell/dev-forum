use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use argon2::password_hash;
use sea_orm::DbErr;
use serde::Serialize;
use std::fmt;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DbErr),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Resource not found: {0}")]
    NotFoundError(String),

    #[error("Password hashing error: {0}")]
    PasswordHashingError(String),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Bad request: {0}")]
    BadRequestError(String),

    #[error("File scanning error: {0}")]
    FileScanningError(String),
}

impl AppError {
    pub fn internal_server_error<T: fmt::Display>(err: T) -> Self {
        AppError::InternalServerError(err.to_string())
    }

    pub fn authentication_error<T: fmt::Display>(err: T) -> Self {
        AppError::AuthenticationError(err.to_string())
    }

    pub fn authorization_error<T: fmt::Display>(err: T) -> Self {
        AppError::AuthorizationError(err.to_string())
    }

    pub fn not_found_error<T: fmt::Display>(err: T) -> Self {
        AppError::NotFoundError(err.to_string())
    }

    pub fn bad_request_error<T: fmt::Display>(err: T) -> Self {
        AppError::BadRequestError(err.to_string())
    }

    pub fn file_scanning_error<T: fmt::Display>(err: T) -> Self {
        AppError::FileScanningError(err.to_string())
    }

    pub fn password_hashing_error<T: fmt::Display>(err: T) -> Self {
        AppError::PasswordHashingError(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        if err.is::<password_hash::Error>() {
            AppError::PasswordHashingError(err.to_string())
        } else {
            AppError::InternalServerError(err.to_string())
        }
    }
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        let mut error_message = String::new();
        for (field, field_errors) in errors.field_errors() {
            error_message.push_str(&format!("{}: ", field));
            let messages: Vec<String> = field_errors.iter().map(|e| e.to_string()).collect();
            error_message.push_str(&messages.join(", "));
            error_message.push_str("; ");
        }
        
        for (field, error_kind) in errors.errors() {
            match error_kind {
                validator::ValidationErrorsKind::Field(field_errors) => {
                    // Already handled above
                },
                validator::ValidationErrorsKind::Struct(struct_errors) => {
                    error_message.push_str(&format!("Struct {}: Invalid; ", field));
                },
                validator::ValidationErrorsKind::List(list_errors) => {
                    for (index, error) in list_errors {
                        error_message.push_str(&format!("{}[{}]: Invalid; ", field, index));
                    }
                }
            }
        }
        
        AppError::ValidationError(error_message.trim_end_matches("; ").to_string())
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            AppError::AuthorizationError(_) => StatusCode::FORBIDDEN,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
            AppError::PasswordHashingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JwtError(_) => StatusCode::UNAUTHORIZED,
            AppError::BadRequestError(_) => StatusCode::BAD_REQUEST,
            AppError::FileScanningError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = match self.status_code() {
            StatusCode::UNAUTHORIZED => "unauthorized",
            StatusCode::FORBIDDEN => "forbidden",
            StatusCode::NOT_FOUND => "not_found",
            StatusCode::BAD_REQUEST => "bad_request",
            _ => "error",
        };

        HttpResponse::build(self.status_code()).json(ErrorResponse {
            status: status.to_string(),
            message: self.to_string(),
        })
    }
} 