//! MongoDB document models

use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, DateTime as BsonDateTime};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoPhoto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub photo_type: String,
    pub file_name: String,
    pub file_size: i64,
    pub content_type: String,
    pub photo_data: Vec<u8>,
    pub is_verified: bool,
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
}

impl MongoPhoto {
    pub fn new(
        user_id: Uuid,
        photo_type: String,
        file_name: String,
        file_size: i64,
        content_type: String,
        photo_data: Vec<u8>,
    ) -> Self {
        let now = BsonDateTime::now();
        Self {
            id: None,
            user_id: user_id.to_string(),
            photo_type,
            file_name,
            file_size,
            content_type,
            photo_data,
            is_verified: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn get_photo_url(&self) -> String {
        if let Some(id) = &self.id {
            format!("/api/v1/photos/{}", id.to_hex())
        } else {
            String::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoMetadata {
    pub photo_id: String,
    pub user_id: String,
    pub photo_type: String,
    pub file_name: String,
    pub file_size: i64,
    pub content_type: String,
    pub photo_url: String,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&MongoPhoto> for PhotoMetadata {
    fn from(photo: &MongoPhoto) -> Self {
        Self {
            photo_id: photo.id.as_ref().map(|id| id.to_hex()).unwrap_or_default(),
            user_id: photo.user_id.clone(),
            photo_type: photo.photo_type.clone(),
            file_name: photo.file_name.clone(),
            file_size: photo.file_size,
            content_type: photo.content_type.clone(),
            photo_url: photo.get_photo_url(),
            is_verified: photo.is_verified,
            created_at: DateTime::<Utc>::from_timestamp_millis(photo.created_at.timestamp_millis()).unwrap_or_else(|| Utc::now()),
            updated_at: DateTime::<Utc>::from_timestamp_millis(photo.updated_at.timestamp_millis()).unwrap_or_else(|| Utc::now()),
        }
    }
}
