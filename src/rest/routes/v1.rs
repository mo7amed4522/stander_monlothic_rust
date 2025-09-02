//! API v1 routes

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::{
    rest::handlers::user::{
        register, login, validate_token, get_user, update_user, delete_user, list_users, upload_photo
    },
};


pub fn create_v1_routes() -> Router<crate::AppState> {
    Router::new()

        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/validate", get(validate_token))


        .route("/users", get(list_users))
        .route("/users/:user_id", get(get_user))
        .route("/users/:user_id", put(update_user))
        .route("/users/:user_id", delete(delete_user))


        .route("/users/:user_id/photo", post(upload_photo))
}
