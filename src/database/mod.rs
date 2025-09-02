//! Database connection modules

pub mod postgres;
pub mod mongodb;

use anyhow::Result;

pub async fn initialize_databases(config: &crate::config::DatabaseConfig) -> Result<()> {
    postgres::run_migrations(&config.postgres_url).await?;
    mongodb::test_connection(&config.mongodb_url).await?;

    Ok(())
}
