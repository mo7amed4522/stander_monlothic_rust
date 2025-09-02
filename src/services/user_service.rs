//! User business logic service

use anyhow::{Result, Context};
use uuid::Uuid;
use diesel::prelude::*;
use chrono::Utc;
use crate::models::user::{User, CreateUser, UpdateUser, UserPhoto};
use crate::models::db_models::{DbUser, NewDbUser, UpdateDbUser, DbUserPhoto};
use crate::database::postgres::get_connection;
use crate::schema::{users, user_photos};
use crate::utils::encryption::hash_password;
use crate::AppState;

#[derive(Clone)]
pub struct UserService {
    app_state: AppState,
}

impl UserService {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }

    pub async fn list_users(&self, limit: u32, offset: u32) -> Result<Vec<User>> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        let db_users = users::table
            .limit(limit as i64)
            .offset(offset as i64)
            .load::<DbUser>(&mut conn)
            .context("Failed to load users from database")?;
        let mut result_users = Vec::new();
        for db_user in db_users {
            let photos = self.get_user_photos(db_user.id).await?;
            result_users.push(self.db_user_to_user(db_user, photos));
        }
        Ok(result_users)
    }

    pub async fn get_user(&self, id: Uuid) -> Result<Option<User>> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        let db_user = users::table
            .find(id)
            .first::<DbUser>(&mut conn)
            .optional()
            .context("Failed to query user from database")?;
        match db_user {
            Some(user) => {
                let photos = self.get_user_photos(user.id).await?;
                Ok(Some(self.db_user_to_user(user, photos)))
            }
            None => Ok(None),
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        let db_user = users::table
            .filter(users::email.eq(email))
            .first::<DbUser>(&mut conn)
            .optional()
            .context("Failed to query user by email from database")?;
        match db_user {
            Some(user) => {
                let photos = self.get_user_photos(user.id).await?;
                Ok(Some(self.db_user_to_user(user, photos)))
            }
            None => Ok(None),
        }
    }

    pub async fn create_user(&self, create_data: CreateUser) -> Result<User> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        let now = Utc::now();
        let password_hash = hash_password(&create_data.password);
        let new_user = NewDbUser {
            email: create_data.email,
            password_hash,
            country_code: Some(create_data.country_code),
            phone: create_data.phone,
            first_name: create_data.first_name,
            last_name: create_data.last_name,
            role: create_data.role,
            is_active: true,
            email_verified: false,
            phone_verified: false,
            created_at: now,
            updated_at: now,
        };
        let db_user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<DbUser>(&mut conn)
            .context("Failed to insert user into database")?;
        Ok(self.db_user_to_user(db_user, vec![]))
    }

    pub async fn update_user(&self, id: Uuid, update_data: UpdateUser) -> Result<Option<User>> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        let update_changeset = UpdateDbUser {
            email: update_data.email,
            country_code: update_data.country_code,
            phone: update_data.phone,
            first_name: update_data.first_name,
            last_name: update_data.last_name,
            is_active: update_data.is_active,
            email_verified: None,
            phone_verified: None,
            updated_at: Utc::now(),
        };

        let updated_user = diesel::update(users::table.find(id))
            .set(&update_changeset)
            .get_result::<DbUser>(&mut conn)
            .optional()
            .context("Failed to update user in database")?;
        match updated_user {
            Some(user) => {
                let photos = self.get_user_photos(user.id).await?;
                Ok(Some(self.db_user_to_user(user, photos)))
            }
            None => Ok(None),
        }
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<bool> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        let deleted_count = diesel::delete(users::table.find(id))
            .execute(&mut conn)
            .context("Failed to delete user from database")?;
        Ok(deleted_count > 0)
    }

    pub async fn activate_user(&self, id: Uuid) -> Result<bool> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;
        let update_changeset = UpdateDbUser {
            email: None,
            country_code: None,
            phone: None,
            first_name: None,
            last_name: None,
            is_active: Some(true),
            email_verified: None,
            phone_verified: None,
            updated_at: Utc::now(),
        };
        let updated_count = diesel::update(users::table.find(id))
            .set(&update_changeset)
            .execute(&mut conn)
            .context("Failed to activate user in database")?;

        Ok(updated_count > 0)
    }

    pub async fn deactivate_user(&self, id: Uuid) -> Result<bool> {
        let mut conn = get_connection(&self.app_state.postgres_pool)
            .context("Failed to get database connection")?;

        let update_changeset = UpdateDbUser {
            email: None,
            country_code: None,
            phone: None,
            first_name: None,
            last_name: None,
            is_active: Some(false),
            email_verified: None,
            phone_verified: None,
            updated_at: Utc::now(),
        };

        let updated_count = diesel::update(users::table.find(id))
            .set(&update_changeset)
            .execute(&mut conn)
            .context("Failed to deactivate user in database")?;

        Ok(updated_count > 0)
    }

    async fn get_user_photos(&self, user_id: Uuid) -> Result<Vec<UserPhoto>> {
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

    fn db_user_to_user(&self, db_user: DbUser, photos: Vec<UserPhoto>) -> User {
        User {
            id: db_user.id,
            email: db_user.email,
            phone: db_user.phone,
            country_code: db_user.country_code.unwrap_or_default(),
            first_name: db_user.first_name,
            last_name: db_user.last_name,
            role: db_user.role,
            is_active: db_user.is_active,
            created_at: db_user.created_at,
            updated_at: db_user.updated_at,
            photos,
        }
    }
}
