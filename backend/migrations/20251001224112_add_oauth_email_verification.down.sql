-- Rollback OAuth and email verification support

-- Drop trigger
DROP TRIGGER IF EXISTS update_verification_tokens_updated_at ON verification_tokens;

-- Remove email-verified role
DELETE FROM roles WHERE name = 'email-verified';

-- Drop indexes
DROP INDEX IF EXISTS idx_verification_tokens_expires_at;
DROP INDEX IF EXISTS idx_verification_tokens_token_hash;
DROP INDEX IF EXISTS idx_verification_tokens_user_id;
DROP INDEX IF EXISTS idx_users_google_id;

-- Drop verification_tokens table
DROP TABLE IF EXISTS verification_tokens;

-- Make password_hash non-nullable again (requires all users to have passwords)
-- WARNING: This will fail if any OAuth-only users exist
ALTER TABLE users ALTER COLUMN password_hash SET NOT NULL;

-- Remove OAuth and identity fields
ALTER TABLE users DROP COLUMN IF EXISTS google_user_id;
ALTER TABLE users DROP COLUMN IF EXISTS real_name;
