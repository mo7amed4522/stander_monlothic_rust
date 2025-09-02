//! Database schema definitions for Diesel ORM

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        country_code -> Nullable<Varchar>,
        phone -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        role -> Varchar,
        is_active -> Bool,
        email_verified -> Bool,
        phone_verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_photos (id) {
        id -> Uuid,
        user_id -> Uuid,
        photo_type -> Varchar,
        photo_url -> Varchar,
        is_verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    verification_codes (id) {
        id -> Uuid,
        user_id -> Uuid,
        code -> Varchar,
        verification_type -> Varchar,
        expires_at -> Timestamptz,
        is_used -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    refresh_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token_hash -> Varchar,
        expires_at -> Timestamptz,
        is_revoked -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(user_photos -> users (user_id));
diesel::joinable!(verification_codes -> users (user_id));
diesel::joinable!(refresh_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    user_photos,
    verification_codes,
    refresh_tokens,
);
