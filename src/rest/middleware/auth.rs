//! Authentication middleware

use axum::{
    extract::{Request},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use tower::{Layer, Service};
use std::task::{Context, Poll};
use tracing::{info, warn};


#[derive(Clone)]
pub struct AuthLayer;

impl AuthLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;
    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for AuthMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let future = self.inner.call(request);
        Box::pin(async move {
            future.await
        })
    }
}

pub fn extract_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(header[7..].to_string())
            } else {
                None
            }
        })
}

pub fn validate_token(token: &str) -> Result<Claims, AuthError> {
    if token == "valid_token" {
        Ok(Claims {
            sub: "user123".to_string(),
            exp: chrono::Utc::now().timestamp() + 3600,
            iat: chrono::Utc::now().timestamp(),
        })
    } else {
        Err(AuthError::InvalidToken)
    }
}

#[derive(Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Missing authorization header")]
    MissingHeader,
}
pub async fn require_auth(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token(&headers)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    match validate_token(&token) {
        Ok(claims) => {
            info!("Authenticated user: {}", claims.sub);
            Ok(next.run(request).await)
        }
        Err(e) => {
            warn!("Authentication failed: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
