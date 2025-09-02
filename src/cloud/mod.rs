//! Cloud integrations module
//!
//! This module provides conditional cloud service initialization based on configuration.
//! Services are only initialized when explicitly enabled in the configuration.

pub mod aws;
pub mod huawei;

use anyhow::Result;
use tracing::info;

pub async fn initialize_enabled_services(config: &crate::config::CloudConfig) -> Result<(Option<aws::AwsConfig>, Option<huawei::HuaweiConfig>)> {
    let aws_config = if config.enable_aws_services {
        info!("Initializing AWS services...");
        Some(aws::initialize_aws_config().await?)
    } else {
        info!("AWS services disabled");
        None
    };

    let huawei_config = if config.enable_huawei_services {
        info!("Initializing Huawei Cloud services...");
        Some(huawei::initialize_huawei_config().await?)
    } else {
        info!("Huawei Cloud services disabled");
        None
    };

    Ok((aws_config, huawei_config))
}
