//! Authentication and authorization service

use anyhow::{Result, Context};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use crate::models::user::{User, LoginRequest, LoginResponse};
use crate::services::UserService;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

#[derive(Clone)]
pub struct AuthService {
    app_state: AppState,
    user_service: UserService,
}

impl AuthService {
    pub fn new(app_state: AppState) -> Self {
        let user_service = UserService::new(app_state.clone());
        Self { app_state, user_service }
    }

    pub async fn login(&self, login_request: LoginRequest) -> Result<Option<LoginResponse>> {
        let user = self.user_service.get_user_by_email(&login_request.email).await?;
        if let Some(user) = user {
            if self.verify_password(&login_request.password, &user).await? {
                let token = self.generate_jwt_token(&user)?;
                let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);
                return Ok(Some(LoginResponse {
                    token,
                    user,
                    expires_at,
                }));
            }
        }
        Ok(None)
    }

    pub async fn verify_token(&self, token: &str) -> Result<Option<User>> {
        let jwt_secret = self.app_state.config.jwt_secret.as_bytes();
        let decoding_key = DecodingKey::from_secret(jwt_secret);
        let validation = Validation::new(Algorithm::HS256);
        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(token_data) => {
                let user_id = Uuid::parse_str(&token_data.claims.sub)
                    .context("Invalid user ID in token")?;
                self.user_service.get_user(user_id).await
            }
            Err(_) => Ok(None),
        }
    }

    pub async fn refresh_token(&self, token: &str) -> Result<Option<String>> {
        if let Some(user) = self.verify_token(token).await? {
            let new_token = self.generate_jwt_token(&user)?;
            Ok(Some(new_token))
        } else {
            Ok(None)
        }
    }

    pub async fn logout(&self, token: &str) -> Result<bool> {
        Ok(self.verify_token(token).await?.is_some())
    }

    async fn verify_password(&self, password: &str, user: &User) -> Result<bool> {
        let mut conn = crate::database::postgres::get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        use crate::schema::users;
        use crate::models::db_models::DbUser;
        use diesel::prelude::*;
        let db_user = users::table
            .find(user.id)
            .first::<DbUser>(&mut conn)
            .optional()
            .context("Failed to query user from database")?;

        match db_user {
            Some(db_user) => {
                verify(password, &db_user.password_hash)
                    .context("Failed to verify password")
            }
            None => Ok(false),
        }
    }

    pub fn generate_jwt_token(&self, user: &User) -> Result<String> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24);
        let claims = Claims {
            sub: user.id.to_string(),
            exp: expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
        };
        let jwt_secret = self.app_state.config.jwt_secret.as_bytes();
        let encoding_key = EncodingKey::from_secret(jwt_secret);
        encode(&Header::default(), &claims, &encoding_key)
            .context("Failed to generate JWT token")
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        hash(password, DEFAULT_COST)
            .context("Failed to hash password")
    }
    pub async fn change_password(&self, user_id: Uuid, old_password: &str, new_password: &str) -> Result<bool> {
        if let Some(user) = self.user_service.get_user(user_id).await? {
            if self.verify_password(old_password, &user).await? {
                let new_hash = self.hash_password(new_password)?;
                use crate::models::user::UpdateUser;
                let update_data = UpdateUser {
                    email: None,
                    country_code: None,
                    phone: None,
                    first_name: None,
                    last_name: None,
                    is_active: None,
                };
                return Ok(true);
            }
        }

        Ok(false)
    }
}
