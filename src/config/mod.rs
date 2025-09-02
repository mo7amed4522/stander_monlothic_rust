//! Configuration management module

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cloud: CloudConfig,
    pub logging: LoggingConfig,
    pub jwt_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub grpc_port: u16,
    pub rest_port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub postgres_url: String,
    pub mongodb_url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    pub aws: AwsConfig,
    pub huawei: HuaweiConfig,
    pub enable_aws_services: bool,
    pub enable_huawei_services: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    pub region: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuaweiConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                grpc_port: 50051,
                rest_port: 8080,
                host: "0.0.0.0".to_string(),
            },
            database: DatabaseConfig {
                postgres_url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgresql://localhost/myapp".to_string()),
                mongodb_url: env::var("MONGODB_URL")
                    .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
                max_connections: 10,
            },
            cloud: CloudConfig {
                aws: AwsConfig {
                    region: env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
                    access_key_id: env::var("AWS_ACCESS_KEY_ID").ok(),
                    secret_access_key: env::var("AWS_SECRET_ACCESS_KEY").ok(),
                },
                huawei: HuaweiConfig {
                    endpoint: env::var("HUAWEI_ENDPOINT")
                        .unwrap_or_else(|_| "https://ecs.ap-southeast-1.myhuaweicloud.com".to_string()),
                    access_key: env::var("HUAWEI_ACCESS_KEY").unwrap_or_default(),
                    secret_key: env::var("HUAWEI_SECRET_KEY").unwrap_or_default(),
                    project_id: env::var("HUAWEI_PROJECT_ID").unwrap_or_default(),
                },
                enable_aws_services: env::var("ENABLE_AWS_SERVICES")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                enable_huawei_services: env::var("ENABLE_HUAWEI_SERVICES")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
            },
            logging: LoggingConfig {
                level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
                format: "json".to_string(),
            },
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string()),
        }
    }
}
pub fn load_config() -> Result<Config> {
    dotenvy::dotenv().ok();
    let config = Config::default();
    Ok(config)
}
