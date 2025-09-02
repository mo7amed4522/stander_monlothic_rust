//! Database model structs for Diesel ORM

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::schema::{users, user_photos, verification_codes, refresh_tokens};


#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub country_code: Option<String>,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub is_active: bool,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewDbUser {
    pub email: String,
    pub password_hash: String,
    pub country_code: Option<String>,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub is_active: bool,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateDbUser {
    pub email: Option<String>,
    pub country_code: Option<String>,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: Option<bool>,
    pub email_verified: Option<bool>,
    pub phone_verified: Option<bool>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = user_photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbUserPhoto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub photo_type: String,
    pub photo_url: String,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Insertable)]
#[diesel(table_name = user_photos)]
pub struct NewDbUserPhoto {
    pub user_id: Uuid,
    pub photo_type: String,
    pub photo_url: String,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = verification_codes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbVerificationCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub verification_type: String,
    pub expires_at: DateTime<Utc>,
    pub is_used: bool,
    pub created_at: DateTime<Utc>,
}


#[derive(Debug, Insertable)]
#[diesel(table_name = verification_codes)]
pub struct NewDbVerificationCode {
    pub user_id: Uuid,
    pub code: String,
    pub verification_type: String,
    pub expires_at: DateTime<Utc>,
    pub is_used: bool,
    pub created_at: DateTime<Utc>,
}


#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbRefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
    pub created_at: DateTime<Utc>,
}


#[derive(Debug, Insertable)]
#[diesel(table_name = refresh_tokens)]
pub struct NewDbRefreshToken {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
    pub created_at: DateTime<Utc>,
}
