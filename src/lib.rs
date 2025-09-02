//! Monolithic Rust Service with gRPC and REST API
//!
//! This service provides both gRPC and REST API endpoints with support for:
//! - PostgreSQL database using Diesel ORM
//! - MongoDB integration
//! - AWS SDK integration
//! - Huawei Cloud SDK integration

pub mod config;
pub mod database;
pub mod grpc;
pub mod rest;
pub mod cloud;
pub mod common;
pub mod models;
pub mod services;
pub mod utils;
pub mod schema;

use anyhow::Result;
use tracing::{info, instrument};


#[derive(Clone, Debug)]
pub struct AppState {
    pub postgres_pool: database::postgres::PgPool,
    pub mongodb_client: database::mongodb::MongoClient,
    pub aws_config: Option<cloud::aws::AwsConfig>,
    pub huawei_config: Option<cloud::huawei::HuaweiConfig>,
    pub config: config::Config,
}


#[instrument]
pub async fn initialize_app() -> Result<AppState> {
    info!("Initializing monolithic service...");
    let config = config::load_config()?;
    let postgres_pool = database::postgres::create_pool(&config.database.postgres_url).await?;
    let mongodb_client = database::mongodb::create_client(&config.database.mongodb_url).await?;
    let aws_config = if config.cloud.enable_aws_services {
        info!("Initializing AWS services...");
        Some(cloud::aws::initialize_aws_config().await?)
    } else {
        info!("AWS services disabled in configuration");
        None
    };
    let huawei_config = if config.cloud.enable_huawei_services {
        info!("Initializing Huawei Cloud services...");
        Some(cloud::huawei::initialize_huawei_config().await?)
    } else {
        info!("Huawei Cloud services disabled in configuration");
        None
    };
    info!("Application initialized successfully");
    Ok(AppState {
        postgres_pool,
        mongodb_client,
        aws_config,
        huawei_config,
        config,
    })
}
