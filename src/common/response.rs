//! Standard response format for all API endpoints

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<HashMap<String, serde_json::Value>>,
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<()>>;

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: message.into(),
            timestamp: chrono::Utc::now(),
            request_id: None,
        }
    }
    pub fn success_with_id(data: T, message: impl Into<String>, request_id: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: message.into(),
            timestamp: chrono::Utc::now(),
            request_id: Some(request_id),
        }
    }
}

impl ApiResponse<()> {
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        let message_str = message.into();
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code: code.into(),
                message: message_str.clone(),
                details: None,
            }),
            message: message_str,
            timestamp: chrono::Utc::now(),
            request_id: None,
        }
    }

    pub fn error_with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: HashMap<String, serde_json::Value>,
    ) -> Self {
        let message_str = message.into();
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code: code.into(),
                message: message_str.clone(),
                details: Some(details),
            }),
            message: message_str,
            timestamp: chrono::Utc::now(),
            request_id: None,
        }
    }

    pub fn error_with_id(
        code: impl Into<String>,
        message: impl Into<String>,
        request_id: String,
    ) -> Self {
        let message_str = message.into();
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code: code.into(),
                message: message_str.clone(),
                details: None,
            }),
            message: message_str,
            timestamp: chrono::Utc::now(),
            request_id: Some(request_id),
        }
    }
}


impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            match self.error.as_ref().map(|e| e.code.as_str()) {
                Some("VALIDATION_ERROR") => StatusCode::BAD_REQUEST,
                Some("NOT_FOUND") => StatusCode::NOT_FOUND,
                Some("UNAUTHORIZED") => StatusCode::UNAUTHORIZED,
                Some("FORBIDDEN") => StatusCode::FORBIDDEN,
                Some("CONFLICT") => StatusCode::CONFLICT,
                Some("INTERNAL_ERROR") => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        };

        (status, Json(self)).into_response()
    }
}


pub mod error_codes {
    pub const VALIDATION_ERROR: &str = "VALIDATION_ERROR";
    pub const NOT_FOUND: &str = "NOT_FOUND";
    pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
    pub const FORBIDDEN: &str = "FORBIDDEN";
    pub const CONFLICT: &str = "CONFLICT";
    pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
    pub const DATABASE_ERROR: &str = "DATABASE_ERROR";
    pub const CLOUD_SERVICE_ERROR: &str = "CLOUD_SERVICE_ERROR";
    pub const NETWORK_ERROR: &str = "NETWORK_ERROR";
}
#[macro_export]
macro_rules! success_response {
    ($data:expr, $message:expr) => {
        $crate::common::response::ApiResponse::success($data, $message)
    };
    ($data:expr, $message:expr, $request_id:expr) => {
        $crate::common::response::ApiResponse::success_with_id($data, $message, $request_id)
    };
}

#[macro_export]
macro_rules! error_response {
    ($code:expr, $message:expr) => {
        $crate::common::response::ApiResponse::error($code, $message)
    };
    ($code:expr, $message:expr, $details:expr) => {
        $crate::common::response::ApiResponse::error_with_details($code, $message, $details)
    };
    ($code:expr, $message:expr, $request_id:expr) => {
        $crate::common::response::ApiResponse::error_with_id($code, $message, $request_id)
    };
}
