//! Health check handlers

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
    pub services: ServiceStatus,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceStatus {
    pub database: String,
    pub mongodb: String,
    pub aws: String,
    pub huawei: String,
}


pub async fn health_check(
    State(app_state): State<crate::AppState>,
) -> Result<Json<HealthResponse>, StatusCode> {
    info!("Health check requested");

    let postgres_status = match crate::database::postgres::get_connection(&app_state.postgres_pool) {
        Ok(_) => "healthy".to_string(),
        Err(_) => "unhealthy".to_string(),
    };

    let mongodb_status = match crate::database::mongodb::test_connection_with_client(&app_state.mongodb_client).await {
        Ok(_) => "healthy".to_string(),
        Err(_) => "unhealthy".to_string(),
    };
    let aws_status = match &app_state.aws_config {
        Some(_) => "enabled".to_string(),
        None => "disabled".to_string(),
    };

    let huawei_status = match &app_state.huawei_config {
        Some(_) => "enabled".to_string(),
        None => "disabled".to_string(),
    };

    let response = HealthResponse {
        status: "ok".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        services: ServiceStatus {
            database: postgres_status,
            mongodb: mongodb_status,
            aws: aws_status,
            huawei: huawei_status,
        },
    };
    Ok(Json(response))
}
