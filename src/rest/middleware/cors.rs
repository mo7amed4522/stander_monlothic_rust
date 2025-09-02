//! CORS middleware configuration

use axum::http::{HeaderValue, HeaderName, Method};
use tower_http::cors::{Any, CorsLayer};
use std::time::Duration;


pub fn setup_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            "authorization".parse::<HeaderName>().unwrap(),
            "content-type".parse::<HeaderName>().unwrap(),
            "x-requested-with".parse::<HeaderName>().unwrap(),
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600))
}


pub fn setup_cors_dev() -> CorsLayer {
    CorsLayer::permissive()
}
pub fn setup_cors_prod(allowed_origins: Vec<&str>) -> CorsLayer {
    let origins: Vec<HeaderValue> = allowed_origins
        .into_iter()
        .map(|origin| origin.parse().unwrap())
        .collect();
    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([
            "authorization".parse::<HeaderName>().unwrap(),
            "content-type".parse::<HeaderName>().unwrap(),
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(86400))
}
