//! AWS SDK integration

use anyhow::{Result, Context};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::Client as S3Client;
use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_sdk_lambda::Client as LambdaClient;
use std::env;

#[derive(Clone, Debug)]
pub struct AwsConfig {
    pub s3_client: S3Client,
    pub dynamodb_client: DynamoDbClient,
    pub lambda_client: LambdaClient,
    pub region: Region,
}


pub async fn initialize_aws_config() -> Result<AwsConfig> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(
            env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string())
        ))
        .load()
        .await;
    let s3_client = S3Client::new(&config);
    let dynamodb_client = DynamoDbClient::new(&config);
    let lambda_client = LambdaClient::new(&config);
    let region = config.region().unwrap_or(&Region::new("us-east-1")).clone();
    println!("AWS SDK initialized successfully for region: {}", region);
    Ok(AwsConfig {
        s3_client,
        dynamodb_client,
        lambda_client,
        region,
    })
}

pub mod s3 {
    use super::*;
    use aws_sdk_s3::primitives::ByteStream;
    pub async fn upload_file(
        client: &S3Client,
        bucket: &str,
        key: &str,
        body: ByteStream,
    ) -> Result<()> {
        client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .send()
            .await
            .context("Failed to upload file to S3")?;

        println!("File uploaded to S3: s3://{}/{}", bucket, key);
        Ok(())
    }
    pub async fn download_file(
        client: &S3Client,
        bucket: &str,
        key: &str,
    ) -> Result<ByteStream> {
        let response = client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .context("Failed to download file from S3")?;

        Ok(response.body)
    }
}
pub mod dynamodb {
    use super::*;
    use aws_sdk_dynamodb::types::AttributeValue;
    use std::collections::HashMap;
    pub async fn put_item(
        client: &DynamoDbClient,
        table_name: &str,
        item: HashMap<String, AttributeValue>,
    ) -> Result<()> {
        client
            .put_item()
            .table_name(table_name)
            .set_item(Some(item))
            .send()
            .await
            .context("Failed to put item in DynamoDB")?;

        println!("Item inserted into DynamoDB table: {}", table_name);
        Ok(())
    }
    pub async fn get_item(
        client: &DynamoDbClient,
        table_name: &str,
        key: HashMap<String, AttributeValue>,
    ) -> Result<Option<HashMap<String, AttributeValue>>> {
        let response = client
            .get_item()
            .table_name(table_name)
            .set_key(Some(key))
            .send()
            .await
            .context("Failed to get item from DynamoDB")?;

        Ok(response.item)
    }
}

pub mod lambda {
    use super::*;
    use aws_sdk_lambda::primitives::Blob;
    pub async fn invoke_function(
        client: &LambdaClient,
        function_name: &str,
        payload: Option<Blob>,
    ) -> Result<Option<Blob>> {
        let response = client
            .invoke()
            .function_name(function_name)
            .set_payload(payload)
            .send()
            .await
            .context("Failed to invoke Lambda function")?;
        println!("Lambda function invoked: {}", function_name);
        Ok(response.payload)
    }
}
