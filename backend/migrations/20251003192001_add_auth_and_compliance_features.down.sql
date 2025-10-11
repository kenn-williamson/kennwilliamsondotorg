-- Rollback authentication and compliance features

-- Drop tables
DROP TABLE IF EXISTS email_suppressions;
DROP TABLE IF EXISTS password_reset_tokens;
DROP TABLE IF EXISTS verification_tokens;

-- Remove roles
DELETE FROM roles WHERE name IN ('email-verified', 'trusted-contact');

-- Remove OAuth fields and timer privacy fields from users table
DROP INDEX IF EXISTS idx_users_timer_visibility;
DROP INDEX IF EXISTS idx_users_google_id;
ALTER TABLE users DROP COLUMN IF EXISTS timer_show_in_list;
ALTER TABLE users DROP COLUMN IF EXISTS timer_is_public;
ALTER TABLE users DROP COLUMN IF EXISTS google_user_id;
ALTER TABLE users DROP COLUMN IF EXISTS real_name;

-- Restore password_hash as NOT NULL (only safe if all users have passwords)
-- Note: This may fail if OAuth-only users exist
ALTER TABLE users ALTER COLUMN password_hash SET NOT NULL;
