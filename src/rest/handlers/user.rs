//! User REST API handlers

use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    response::Json,
};
use axum_extra::{
    extract::Multipart,
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, error};
use crate::{
    AppState,
    services::{UserService, AuthService, PhotoService},
    models::user::{CreateUser, UpdateUser, LoginRequest, LoginResponse, User},
    common::response::ApiResponse,
};


#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub country_code: Option<String>,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
}

// Using LoginResponse from models::user

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub country_code: Option<String>,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListUsersResponse {
    pub users: Vec<User>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

// Authentication handlers
pub async fn register(
    State(app_state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    let user_service = UserService::new(app_state);

    let create_user = CreateUser {
        email: req.email,
        password: req.password,
        country_code: req.country_code.unwrap_or_default(),
        phone: req.phone,
        first_name: req.first_name,
        last_name: req.last_name,
        role: "user".to_string(),
    };

    match user_service.create_user(create_user).await {
        Ok(user) => {
            info!("User registered successfully: {}", user.email);
            Ok(Json(ApiResponse {
                success: true,
                data: Some(user),
                error: None,
                message: "User registered successfully".to_string(),
                timestamp: chrono::Utc::now(),
                request_id: None,
            }))
        }
        Err(e) => {
            error!("Failed to register user: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, StatusCode> {
    let auth_service = AuthService::new(app_state);

    match auth_service.login(req).await {
        Ok(Some(login_response)) => {
            info!("User logged in successfully: {}", login_response.user.email);
            Ok(Json(ApiResponse {
                success: true,
                data: Some(login_response),
                error: None,
                message: "Login successful".to_string(),
                timestamp: chrono::Utc::now(),
                request_id: None,
            }))
        }
        Ok(None) => {
            error!("Invalid credentials provided");
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("Login failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn validate_token(
    State(app_state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    let auth_service = AuthService::new(app_state);

    match auth_service.verify_token(auth.token()).await {
        Ok(Some(user)) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(user),
                error: None,
                message: "Token is valid".to_string(),
                timestamp: chrono::Utc::now(),
                request_id: None,
            }))
        }
        Ok(None) => {
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("Token validation failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


pub async fn get_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<Uuid>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    let auth_service = AuthService::new(app_state.clone());
    let user_service = UserService::new(app_state);
    let current_user = match auth_service.verify_token(auth.token()).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    if current_user.id != user_id && current_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    match user_service.get_user(user_id).await {
        Ok(Some(user)) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(user),
                error: None,
                message: "User retrieved successfully".to_string(),
                timestamp: chrono::Utc::now(),
                request_id: None,
            }))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to get user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<Uuid>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    let auth_service = AuthService::new(app_state.clone());
    let user_service = UserService::new(app_state);
    let current_user = match auth_service.verify_token(auth.token()).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    if current_user.id != user_id && current_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    let update_user = UpdateUser {
        email: req.email,
        country_code: req.country_code,
        phone: req.phone,
        first_name: req.first_name,
        last_name: req.last_name,
        is_active: None,
    };
    match user_service.update_user(user_id, update_user).await {
        Ok(Some(user)) => {
            info!("User updated successfully: {}", user.id);
            Ok(Json(ApiResponse {
                success: true,
                data: Some(user),
                error: None,
                message: "User updated successfully".to_string(),
                timestamp: chrono::Utc::now(),
                request_id: None,
            }))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to update user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<Uuid>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let auth_service = AuthService::new(app_state.clone());
    let user_service = UserService::new(app_state);
    let current_user = match auth_service.verify_token(auth.token()).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    if current_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    match user_service.delete_user(user_id).await {
        Ok(true) => {
            info!("User deleted successfully: {}", user_id);
            Ok(Json(ApiResponse {
                success: true,
                data: Some(()),
                error: None,
                message: "User deleted successfully".to_string(),
                timestamp: chrono::Utc::now(),
                request_id: None,
            }))
        }
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to delete user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_users(
    State(app_state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<ApiResponse<ListUsersResponse>>, StatusCode> {
    let auth_service = AuthService::new(app_state.clone());
    let user_service = UserService::new(app_state);
    let current_user = match auth_service.verify_token(auth.token()).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    if current_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    match user_service.list_users(limit, (page - 1) * limit).await {
        Ok(users) => {
            let total = users.len() as u64;
            Ok(Json(ApiResponse {
                success: true,
                data: Some(ListUsersResponse {
                    users,
                    total,
                    page,
                    limit,
                }),
                error: None,
                message: "Users retrieved successfully".to_string(),
                timestamp: chrono::Utc::now(),
                request_id: None,
            }))
        }
        Err(e) => {
            error!("Failed to list users: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
pub async fn upload_photo(
    State(app_state): State<AppState>,
    Path(user_id): Path<Uuid>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let auth_service = AuthService::new(app_state.clone());
    let photo_service = PhotoService::new(app_state);
    let current_user = match auth_service.verify_token(auth.token()).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    if current_user.id != user_id && current_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut photo_type = String::new();
    let mut photo_data = Vec::new();
    let mut file_extension = String::new();

    // Process multipart form data
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "photo_type" => {
                photo_type = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            }
            "file" => {
                let filename = field.file_name().unwrap_or("unknown").to_string();
                if let Some(ext) = filename.split('.').last() {
                    file_extension = ext.to_string();
                }
                photo_data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec();
            }
            _ => {}
        }
    }

    if photo_type.is_empty() || photo_data.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    match photo_service.upload_photo(user_id, photo_type, photo_data, file_extension).await {
        Ok(user_photo) => {
            info!("Photo uploaded successfully for user: {}", user_id);
            Ok(Json(ApiResponse {
                success: true,
                data: Some(user_photo.photo_url),
                error: None,
                message: "Photo uploaded successfully".to_string(),
                timestamp: chrono::Utc::now(),
                request_id: None,
            }))
        }
        Err(e) => {
            error!("Failed to upload photo: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
