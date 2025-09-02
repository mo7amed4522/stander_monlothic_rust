//! Conversion utilities between gRPC proto types and Rust model types

use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::user::{User as ModelUser, UserPhoto as ModelUserPhoto};
use crate::grpc::user_services::{User as ProtoUser, UserPhoto as ProtoUserPhoto};


impl From<ModelUser> for ProtoUser {
    fn from(user: ModelUser) -> Self {
        ProtoUser {
            id: user.id.to_string(),
            email: user.email,
            phone: user.phone,
            country_code: user.country_code,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at.timestamp(),
            updated_at: user.updated_at.timestamp(),
            photos: user.photos.into_iter().map(|p| p.into()).collect(),
        }
    }
}


impl TryFrom<ProtoUser> for ModelUser {
    type Error = anyhow::Error;
    fn try_from(proto_user: ProtoUser) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&proto_user.id)
            .map_err(|e| anyhow::anyhow!("Invalid UUID: {}", e))?;
        let created_at = DateTime::from_timestamp(proto_user.created_at, 0)
            .ok_or_else(|| anyhow::anyhow!("Invalid created_at timestamp: {}", proto_user.created_at))?
            .with_timezone(&Utc);
        let updated_at = DateTime::from_timestamp(proto_user.updated_at, 0)
            .ok_or_else(|| anyhow::anyhow!("Invalid updated_at timestamp: {}", proto_user.updated_at))?
            .with_timezone(&Utc);
        let photos: Result<Vec<ModelUserPhoto>, anyhow::Error> = proto_user.photos
            .into_iter()
            .map(|p| p.try_into())
            .collect();
        Ok(ModelUser {
            id,
            email: proto_user.email,
            phone: proto_user.phone,
            country_code: proto_user.country_code,
            first_name: proto_user.first_name,
            last_name: proto_user.last_name,
            role: proto_user.role,
            is_active: proto_user.is_active,
            created_at,
            updated_at,
            photos: photos?,
        })
    }
}
impl From<ModelUserPhoto> for ProtoUserPhoto {
    fn from(photo: ModelUserPhoto) -> Self {
        ProtoUserPhoto {
            id: photo.id.to_string(),
            user_id: photo.user_id.to_string(),
            photo_type: photo.photo_type,
            photo_url: photo.photo_url,
            is_verified: photo.is_verified,
            created_at: photo.created_at.timestamp(),
            updated_at: photo.updated_at.timestamp(),
        }
    }
}
impl TryFrom<ProtoUserPhoto> for ModelUserPhoto {
    type Error = anyhow::Error;
    fn try_from(proto_photo: ProtoUserPhoto) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&proto_photo.id)
            .map_err(|e| anyhow::anyhow!("Invalid photo UUID: {}", e))?;
        let user_id = Uuid::parse_str(&proto_photo.user_id)
            .map_err(|e| anyhow::anyhow!("Invalid user_id UUID: {}", e))?;
        let created_at = DateTime::from_timestamp(proto_photo.created_at, 0)
            .ok_or_else(|| anyhow::anyhow!("Invalid created_at timestamp: {}", proto_photo.created_at))?
            .with_timezone(&Utc);
        let updated_at = DateTime::from_timestamp(proto_photo.updated_at, 0)
            .ok_or_else(|| anyhow::anyhow!("Invalid updated_at timestamp: {}", proto_photo.updated_at))?
            .with_timezone(&Utc);
        Ok(ModelUserPhoto {
            id,
            user_id,
            photo_type: proto_photo.photo_type,
            photo_url: proto_photo.photo_url,
            is_verified: proto_photo.is_verified,
            created_at,
            updated_at,
        })
    }
}
