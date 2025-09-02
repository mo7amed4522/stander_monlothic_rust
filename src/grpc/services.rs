//! gRPC service implementations

use tonic::{Request, Response, Status};
use crate::AppState;
use crate::grpc::user_services::*;
use crate::grpc::user_services::user_service_server::UserService;
use crate::services::{UserService as BusinessUserService, AuthService, PhotoService};
use crate::models::user::{CreateUser, LoginRequest as ModelLoginRequest};
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct UserServiceImpl {
    pub app_state: AppState,
}

impl UserServiceImpl {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn register_new_user(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();

        let user_service = BusinessUserService::new(self.app_state.clone());
        let auth_service = AuthService::new(self.app_state.clone());
        let create_user = CreateUser {
            email: req.email,
            password: req.password,
            phone: req.phone,
            country_code: req.country_code,
            first_name: req.first_name,
            last_name: req.last_name,
            role: req.role,
        };

        match user_service.create_user(create_user).await {
            Ok(user) => {
                match auth_service.generate_jwt_token(&user) {
                    Ok(token) => {
                        let response = AuthResponse {
                            response: Some(StandardResponse {
                                status_code: 201,
                                message: "User registered successfully".to_string(),
                                data: None,
                            }),
                            user: Some(user.into()),
                            token: Some(JwtToken {
                                 access_token: token,
                                 refresh_token: String::new(),
                                 expires_at: chrono::Utc::now().timestamp() + 86400,
                             }),
                            photos: vec![],
                        };
                        Ok(Response::new(response))
                    }
                    Err(e) => {
                        let response = AuthResponse {
                            response: Some(StandardResponse {
                                status_code: 500,
                                message: format!("Failed to generate token: {}", e),
                                data: None,
                            }),
                            user: None,
                            token: None,
                            photos: vec![],
                        };
                        Ok(Response::new(response))
                    }
                }
            }
            Err(e) => {
                let response = AuthResponse {
                    response: Some(StandardResponse {
                        status_code: 400,
                        message: format!("Failed to create user: {}", e),
                        data: None,
                    }),
                    user: None,
                    token: None,
                    photos: vec![],
                };
                Ok(Response::new(response))
            }
        }
    }
    async fn login_user(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();

        let auth_service = AuthService::new(self.app_state.clone());
        let login_request = ModelLoginRequest {
            email: req.email,
            password: req.password,
        };
        match auth_service.login(login_request).await {
            Ok(Some(login_response)) => {
                let response = AuthResponse {
                    response: Some(StandardResponse {
                        status_code: 200,
                        message: "Login successful".to_string(),
                        data: None,
                    }),
                    user: Some(login_response.user.into()),
                    token: Some(JwtToken {
                         access_token: login_response.token,
                         refresh_token: String::new(), // TODO: Implement refresh token
                         expires_at: login_response.expires_at.timestamp(),
                     }),
                    photos: vec![],
                };
                Ok(Response::new(response))
            }
            Ok(None) => {
                let response = AuthResponse {
                    response: Some(StandardResponse {
                        status_code: 401,
                        message: "Invalid credentials".to_string(),
                        data: None,
                    }),
                    user: None,
                    token: None,
                    photos: vec![],
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                let response = AuthResponse {
                    response: Some(StandardResponse {
                        status_code: 500,
                        message: format!("Login failed: {}", e),
                        data: None,
                    }),
                    user: None,
                    token: None,
                    photos: vec![],
                };
                Ok(Response::new(response))
            }
        }
    }
    async fn validate_user_token(
        &self,
        request: Request<ValidateTokenRequest>,
    ) -> Result<Response<ValidateTokenResponse>, Status> {
        let req = request.into_inner();
        let auth_service = AuthService::new(self.app_state.clone());
        match auth_service.verify_token(&req.token).await {
            Ok(Some(user)) => {
                let response = ValidateTokenResponse {
                    response: Some(StandardResponse {
                        status_code: 200,
                        message: "Token is valid".to_string(),
                        data: None,
                    }),
                    user: Some(user.into()),
                    is_valid: true,
                };
                Ok(Response::new(response))
            }
            Ok(None) => {
                let response = ValidateTokenResponse {
                    response: Some(StandardResponse {
                        status_code: 401,
                        message: "Invalid token".to_string(),
                        data: None,
                    }),
                    user: None,
                    is_valid: false,
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                let response = ValidateTokenResponse {
                    response: Some(StandardResponse {
                        status_code: 500,
                        message: format!("Token validation failed: {}", e),
                        data: None,
                    }),
                    user: None,
                    is_valid: false,
                };
                Ok(Response::new(response))
            }
        }
    }

    async fn refresh_user_token(
        &self,
        request: Request<RefreshTokenRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        // TODO: Implement token refresh logic
        let response = AuthResponse {
            response: Some(StandardResponse {
                status_code: 501,
                message: "Not implemented yet".to_string(),
                data: None,
            }),
            user: None,
            token: None,
            photos: vec![],
        };
        Ok(Response::new(response))
    }

    async fn get_user_data(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        let user_service = BusinessUserService::new(self.app_state.clone());
        match Uuid::parse_str(&req.id) {
            Ok(user_id) => {
                match user_service.get_user(user_id).await {
                    Ok(Some(user)) => {
                        let response = UserResponse {
                            response: Some(StandardResponse {
                                status_code: 200,
                                message: "User found".to_string(),
                                data: None,
                            }),
                            user: Some(user.into()),
                        };
                        Ok(Response::new(response))
                    }
                    Ok(None) => {
                        let response = UserResponse {
                            response: Some(StandardResponse {
                                status_code: 404,
                                message: "User not found".to_string(),
                                data: None,
                            }),
                            user: None,
                        };
                        Ok(Response::new(response))
                    }
                    Err(e) => {
                        let response = UserResponse {
                            response: Some(StandardResponse {
                                status_code: 500,
                                message: format!("Failed to get user: {}", e),
                                data: None,
                            }),
                            user: None,
                        };
                        Ok(Response::new(response))
                    }
                }
            }
            Err(_) => {
                let response = UserResponse {
                    response: Some(StandardResponse {
                        status_code: 400,
                        message: "Invalid user ID format".to_string(),
                        data: None,
                    }),
                    user: None,
                };
                Ok(Response::new(response))
            }
        }
    }

    async fn update_user_data(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        // TODO: Implement update user data logic
        let response = UserResponse {
            response: Some(StandardResponse {
                status_code: 501,
                message: "Not implemented yet".to_string(),
                data: None,
            }),
            user: None,
        };
        Ok(Response::new(response))
    }

    async fn delete_user_data(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<StandardResponse>, Status> {
        // TODO: Implement delete user data logic
        let response = StandardResponse {
            status_code: 501,
            message: "Not implemented yet".to_string(),
            data: None,
        };
        Ok(Response::new(response))
    }

    async fn list_users_data(
        &self,
        request: Request<ListUsersRequest>,
    ) -> Result<Response<UsersListResponse>, Status> {
        // TODO: Implement list users data logic
        let response = UsersListResponse {
            response: Some(StandardResponse {
                status_code: 501,
                message: "Not implemented yet".to_string(),
                data: None,
            }),
            users: vec![],
            total: 0,
            page: 0,
            limit: 0,
            role: String::new(),
        };
        Ok(Response::new(response))
    }

    async fn upload_user_data(
        &self,
        request: Request<UploadPhotoRequest>,
    ) -> Result<Response<PhotoResponse>, Status> {
        let req = request.into_inner();
        let auth_service = AuthService::new(self.app_state.clone());
        let user = match auth_service.verify_token(&req.token).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                let response = PhotoResponse {
                    response: Some(StandardResponse {
                        status_code: 401,
                        message: "Invalid or expired token".to_string(),
                        data: None,
                    }),
                    photo: None,
                };
                return Ok(Response::new(response));
            }
            Err(_) => {
                let response = PhotoResponse {
                    response: Some(StandardResponse {
                        status_code: 500,
                        message: "Token verification failed".to_string(),
                        data: None,
                    }),
                    photo: None,
                };
                return Ok(Response::new(response));
            }
        };
        let user_id = match Uuid::parse_str(&req.user_id) {
            Ok(id) => id,
            Err(_) => {
                let response = PhotoResponse {
                    response: Some(StandardResponse {
                        status_code: 400,
                        message: "Invalid user ID format".to_string(),
                        data: None,
                    }),
                    photo: None,
                };
                return Ok(Response::new(response));
            }
        };
        if user.id != user_id && user.role != "admin" {
            let response = PhotoResponse {
                response: Some(StandardResponse {
                    status_code: 403,
                    message: "Access denied".to_string(),
                    data: None,
                }),
                photo: None,
            };
            return Ok(Response::new(response));
        }
        let photo_service = PhotoService::new(self.app_state.clone());
        match photo_service.upload_photo(
            user_id,
            req.photo_type,
            req.photo_data,
            req.file_extension,
        ).await {
            Ok(user_photo) => {
                let response = PhotoResponse {
                    response: Some(StandardResponse {
                        status_code: 200,
                        message: "Photo uploaded successfully".to_string(),
                        data: None,
                    }),
                    photo: Some(user_photo.into()),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                let response = PhotoResponse {
                    response: Some(StandardResponse {
                        status_code: 500,
                        message: format!("Failed to upload photo: {}", e),
                        data: None,
                    }),
                    photo: None,
                };
                Ok(Response::new(response))
            }
        }
    }

    async fn send_verification_code(
        &self,
        request: Request<SendVerificationRequest>,
    ) -> Result<Response<StandardResponse>, Status> {
        // TODO: Implement send verification code logic
        let response = StandardResponse {
            status_code: 501,
            message: "Not implemented yet".to_string(),
            data: None,
        };
        Ok(Response::new(response))
    }
    async fn verify_code(
        &self,
        request: Request<VerifyCodeRequest>,
    ) -> Result<Response<StandardResponse>, Status> {
        // TODO: Implement verify code logic
        let response = StandardResponse {
            status_code: 501,
            message: "Not implemented yet".to_string(),
            data: None,
        };
        Ok(Response::new(response))
    }
}
