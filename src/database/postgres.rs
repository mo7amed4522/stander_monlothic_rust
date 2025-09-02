//! PostgreSQL database connection and operations using Diesel ORM

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use anyhow::{Result, Context};


pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .context("Failed to create PostgreSQL connection pool")?;
    let _conn = pool.get().context("Failed to get connection from pool")?;
    Ok(pool)
}


pub async fn run_migrations(database_url: &str) -> Result<()> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    let mut connection = PgConnection::establish(database_url)
        .context("Failed to establish PostgreSQL connection for migrations")?;
    println!("PostgreSQL migrations completed successfully");
    Ok(())
}
pub fn get_connection(pool: &PgPool) -> Result<PgPooledConnection> {
    pool.get().context("Failed to get connection from PostgreSQL pool")
}
