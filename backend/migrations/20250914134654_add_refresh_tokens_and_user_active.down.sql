-- Revert refresh token system and user active field
-- Remove in reverse order of creation

-- Drop refresh_tokens table (CASCADE will handle triggers)
DROP TABLE IF EXISTS refresh_tokens CASCADE;

-- Remove active column from users table
ALTER TABLE users DROP COLUMN IF EXISTS active;
