//! Photo upload and management service

use anyhow::{Result, Context};
use uuid::Uuid;
use diesel::prelude::*;
use mongodb::bson::oid::ObjectId;
use chrono::Utc;
use tracing::{info};

use crate::models::{
    MongoPhoto,
    DbUserPhoto, NewDbUserPhoto,
    UserPhoto
};
use crate::database::{
    postgres::get_connection,
    mongodb::{get_database, get_collection}
};
use crate::schema::user_photos;
use crate::AppState;

pub struct PhotoService {
    app_state: AppState,
}

impl PhotoService {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }

    pub async fn upload_photo(
        &self,
        user_id: Uuid,
        photo_type: String,
        photo_data: Vec<u8>,
        file_extension: String,
    ) -> Result<UserPhoto> {
        if !matches!(photo_type.as_str(), "profile" | "emirates_id" | "verification") {
            return Err(anyhow::anyhow!("Invalid photo type: {}", photo_type));
        }
        if photo_data.len() > 10 * 1024 * 1024 {
            return Err(anyhow::anyhow!("File size too large. Maximum 10MB allowed"));
        }
        let content_type = match file_extension.to_lowercase().as_str() {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "webp" => "image/webp",
            _ => return Err(anyhow::anyhow!("Unsupported file format: {}", file_extension)),
        };

        let file_name = format!("{}_{}.{}", user_id, photo_type, file_extension);
        let file_size = photo_data.len() as i64;

        let mongo_photo = MongoPhoto::new(
            user_id,
            photo_type.clone(),
            file_name,
            file_size,
            content_type.to_string(),
            photo_data,
        );

        let photo_id = self.store_photo_in_mongodb(mongo_photo).await
            .context("Failed to store photo in MongoDB")?;

        let photo_url = format!("/api/v1/photos/{}", photo_id.to_hex());

        let user_photo = self.store_photo_metadata_in_postgres(
            user_id,
            photo_type,
            photo_url,
        ).await.context("Failed to store photo metadata in PostgreSQL")?;

        info!("Photo uploaded successfully for user {}: {}", user_id, user_photo.id);
        Ok(user_photo)
    }

    pub async fn get_photo_data(&self, photo_id: &str) -> Result<MongoPhoto> {
        let object_id = ObjectId::parse_str(photo_id)
            .context("Invalid photo ID format")?;
        let db = get_database(&self.app_state.mongodb_client, "stander_db");
        let collection = get_collection::<MongoPhoto>(&db, "photos");
        let filter = mongodb::bson::doc! { "_id": object_id };
        let photo = collection.find_one(filter, None).await
            .context("Failed to query MongoDB")?;
        photo.ok_or_else(|| anyhow::anyhow!("Photo not found"))
    }
    pub async fn get_user_photos(&self, user_id: Uuid) -> Result<Vec<UserPhoto>> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        let db_photos = user_photos::table
            .filter(user_photos::user_id.eq(user_id))
            .load::<DbUserPhoto>(&mut conn)
            .context("Failed to load user photos from database")?;
        let photos = db_photos.into_iter().map(|db_photo| UserPhoto {
            id: db_photo.id,
            user_id: db_photo.user_id,
            photo_type: db_photo.photo_type,
            photo_url: db_photo.photo_url,
            is_verified: db_photo.is_verified,
            created_at: db_photo.created_at,
            updated_at: db_photo.updated_at,
        }).collect();

        Ok(photos)
    }

    pub async fn delete_photo(&self, user_id: Uuid, photo_id: Uuid) -> Result<()> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        let db_photo = user_photos::table
            .filter(user_photos::id.eq(photo_id))
            .filter(user_photos::user_id.eq(user_id))
            .first::<DbUserPhoto>(&mut conn)
            .context("Photo not found or access denied")?;
        let mongo_id = self.extract_mongo_id_from_url(&db_photo.photo_url)
            .context("Invalid photo URL format")?;
        self.delete_photo_from_mongodb(&mongo_id).await
            .context("Failed to delete photo from MongoDB")?;
        diesel::delete(user_photos::table.filter(user_photos::id.eq(photo_id)))
            .execute(&mut conn)
            .context("Failed to delete photo metadata from PostgreSQL")?;
        info!("Photo deleted successfully: {}", photo_id);
        Ok(())
    }

    pub async fn verify_photo(&self, user_id: Uuid, photo_id: Uuid) -> Result<UserPhoto> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        let updated_photo = diesel::update(
            user_photos::table
                .filter(user_photos::id.eq(photo_id))
                .filter(user_photos::user_id.eq(user_id))
        )
        .set((
            user_photos::is_verified.eq(true),
            user_photos::updated_at.eq(Utc::now()),
        ))
        .get_result::<DbUserPhoto>(&mut conn)
        .context("Failed to verify photo or photo not found")?;
        Ok(UserPhoto {
            id: updated_photo.id,
            user_id: updated_photo.user_id,
            photo_type: updated_photo.photo_type,
            photo_url: updated_photo.photo_url,
            is_verified: updated_photo.is_verified,
            created_at: updated_photo.created_at,
            updated_at: updated_photo.updated_at,
        })
    }
    async fn store_photo_in_mongodb(&self, photo: MongoPhoto) -> Result<ObjectId> {
        let db = get_database(&self.app_state.mongodb_client, "stander_db");
        let collection = get_collection::<MongoPhoto>(&db, "photos");
        let result = collection.insert_one(photo, None).await
            .context("Failed to insert photo into MongoDB")?;
        result.inserted_id.as_object_id()
            .ok_or_else(|| anyhow::anyhow!("Failed to get inserted photo ID"))
    }

    async fn store_photo_metadata_in_postgres(
        &self,
        user_id: Uuid,
        photo_type: String,
        photo_url: String,
    ) -> Result<UserPhoto> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        let now = Utc::now();
        let new_photo = NewDbUserPhoto {
            user_id,
            photo_type: photo_type.clone(),
            photo_url: photo_url.clone(),
            is_verified: false,
            created_at: now,
            updated_at: now,
        };
        let db_photo = diesel::insert_into(user_photos::table)
            .values(&new_photo)
            .get_result::<DbUserPhoto>(&mut conn)
            .context("Failed to insert photo metadata into PostgreSQL")?;
        Ok(UserPhoto {
            id: db_photo.id,
            user_id: db_photo.user_id,
            photo_type: db_photo.photo_type,
            photo_url: db_photo.photo_url,
            is_verified: db_photo.is_verified,
            created_at: db_photo.created_at,
            updated_at: db_photo.updated_at,
        })
    }

    async fn delete_photo_from_mongodb(&self, photo_id: &str) -> Result<()> {
        let object_id = ObjectId::parse_str(photo_id)
            .context("Invalid photo ID format")?;
        let db = get_database(&self.app_state.mongodb_client, "stander_db");
        let collection = get_collection::<MongoPhoto>(&db, "photos");
        let filter = mongodb::bson::doc! { "_id": object_id };
        let result = collection.delete_one(filter, None).await
            .context("Failed to delete photo from MongoDB")?;
        if result.deleted_count == 0 {
            return Err(anyhow::anyhow!("Photo not found in MongoDB"));
        }
        Ok(())
    }

    fn extract_mongo_id_from_url(&self, photo_url: &str) -> Result<String> {
        let parts: Vec<&str> = photo_url.split('/').collect();
        if parts.len() >= 4 && parts[parts.len() - 2] == "photos" {
            Ok(parts[parts.len() - 1].to_string())
        } else {
            Err(anyhow::anyhow!("Invalid photo URL format: {}", photo_url))
        }
    }
}
