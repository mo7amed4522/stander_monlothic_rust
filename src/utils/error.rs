//! Error handling utilities

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;


#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    AuthError(String),
    ValidationError(String),
    NotFound(String),
    Forbidden(String),
    BadRequest(String),
    InternalError(String),
    ExternalServiceError(String),
    ConfigError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            AppError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}
impl std::error::Error for AppError {}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ExternalServiceError(msg) => (StatusCode::BAD_GATEWAY, msg),
            AppError::ConfigError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = Json(json!({
            "error": {
                "message": error_message,
                "status": status.as_u16(),
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        }));
        (status, body).into_response()
    }
}
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}
impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => AppError::NotFound("Record not found".to_string()),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}
impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::BadRequest(format!("JSON parsing error: {}", err))
    }
}
impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        AppError::ValidationError(format!("Invalid UUID: {}", err))
    }
}
pub type AppResult<T> = Result<T, AppError>;
impl AppError {
    pub fn database<T: Into<String>>(msg: T) -> Self {
        AppError::DatabaseError(msg.into())
    }
    pub fn auth<T: Into<String>>(msg: T) -> Self {
        AppError::AuthError(msg.into())
    }
    pub fn validation<T: Into<String>>(msg: T) -> Self {
        AppError::ValidationError(msg.into())
    }
    pub fn not_found<T: Into<String>>(msg: T) -> Self {
        AppError::NotFound(msg.into())
    }
    pub fn forbidden<T: Into<String>>(msg: T) -> Self {
        AppError::Forbidden(msg.into())
    }
    pub fn bad_request<T: Into<String>>(msg: T) -> Self {
        AppError::BadRequest(msg.into())
    }
    pub fn internal<T: Into<String>>(msg: T) -> Self {
        AppError::InternalError(msg.into())
    }
    pub fn external_service<T: Into<String>>(msg: T) -> Self {
        AppError::ExternalServiceError(msg.into())
    }
    pub fn config<T: Into<String>>(msg: T) -> Self {
        AppError::ConfigError(msg.into())
    }
}
#[macro_export]
macro_rules! validation_error {
    ($field:expr, $message:expr) => {
        AppError::ValidationError(format!("{}: {}", $field, $message))
    };
}
#[macro_export]
macro_rules! not_found_error {
    ($resource:expr, $id:expr) => {
        AppError::NotFound(format!("{} with id '{}' not found", $resource, $id))
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_error_display() {
        let error = AppError::ValidationError("Invalid email format".to_string());
        assert_eq!(error.to_string(), "Validation error: Invalid email format");
    }
    #[test]
    fn test_error_helpers() {
        let error = AppError::not_found("User not found");
        assert!(matches!(error, AppError::NotFound(_)));
        let error = AppError::validation("Invalid input");
        assert!(matches!(error, AppError::ValidationError(_)));
    }
    #[test]
    fn test_validation_error_macro() {
        let error = validation_error!("email", "must be a valid email address");
        assert_eq!(error.to_string(), "Validation error: email: must be a valid email address");
    }
    #[test]
    fn test_not_found_error_macro() {
        let error = not_found_error!("User", "123");
        assert_eq!(error.to_string(), "Not found: User with id '123' not found");
    }
}
