//! MongoDB database connection and operations

use mongodb::{Client, Database, Collection};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

pub type MongoClient = Client;


pub async fn create_client(mongodb_url: &str) -> Result<MongoClient> {
    let client_options = mongodb::options::ClientOptions::parse(mongodb_url)
        .await
        .context("Failed to parse MongoDB connection string")?;
    let client = Client::with_options(client_options)
        .context("Failed to create MongoDB client")?;
    test_connection_with_client(&client).await?;
    Ok(client)
}
pub async fn test_connection(mongodb_url: &str) -> Result<()> {
    let client = create_client(mongodb_url).await?;
    test_connection_with_client(&client).await
}
pub async fn test_connection_with_client(client: &MongoClient) -> Result<()> {
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1}, None)
        .await
        .context("Failed to ping MongoDB server")?;
    println!("MongoDB connection test successful");
    Ok(())
}
pub fn get_database(client: &MongoClient, db_name: &str) -> Database {
    client.database(db_name)
}
pub fn get_collection<T>(database: &Database, collection_name: &str) -> Collection<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Unpin + Send + Sync,
{
    database.collection::<T>(collection_name)
}

