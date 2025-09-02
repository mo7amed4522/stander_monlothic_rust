-- Rollback migration for initial schema
-- This drops all tables and types created in the up migration

-- Drop triggers first
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
DROP TRIGGER IF EXISTS update_user_photos_updated_at ON user_photos;

-- Drop the trigger function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop indexes
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_users_role;
DROP INDEX IF EXISTS idx_users_is_active;
DROP INDEX IF EXISTS idx_user_photos_user_id;
DROP INDEX IF EXISTS idx_user_photos_type;
DROP INDEX IF EXISTS idx_verification_codes_user_id;
DROP INDEX IF EXISTS idx_verification_codes_expires_at;
DROP INDEX IF EXISTS idx_refresh_tokens_user_id;
DROP INDEX IF EXISTS idx_refresh_tokens_expires_at;

-- Drop tables (in reverse order due to foreign key constraints)
DROP TABLE IF EXISTS refresh_tokens;
DROP TABLE IF EXISTS verification_codes;
DROP TABLE IF EXISTS user_photos;
DROP TABLE IF EXISTS users;

-- Drop enum types
DROP TYPE IF EXISTS verification_type;
DROP TYPE IF EXISTS photo_type;
DROP TYPE IF EXISTS user_role;

-- Drop extensions (optional, as they might be used by other applications)
-- DROP EXTENSION IF EXISTS "pgcrypto";
-- DROP EXTENSION IF EXISTS "uuid-ossp";