-- ============================================================================
-- ROLLBACK: CONSOLIDATED AUTH REFACTOR MIGRATION
-- Restores users table to original state and drops all new tables
-- ============================================================================

-- ============================================================================
-- STEP 1: Restore password_hash column to users table
-- ============================================================================
ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255);

-- Copy password_hash back from user_credentials to users
UPDATE users u
SET password_hash = uc.password_hash
FROM user_credentials uc
WHERE u.id = uc.user_id;

-- Make password_hash NOT NULL again (as it was originally)
-- NOTE: This assumes all users have passwords. If OAuth-only users exist, this will fail.
ALTER TABLE users ALTER COLUMN password_hash SET NOT NULL;

-- ============================================================================
-- STEP 2: Drop all new tables (in reverse dependency order)
-- ============================================================================
DROP TABLE IF EXISTS access_requests CASCADE;
DROP TABLE IF EXISTS user_preferences CASCADE;
DROP TABLE IF EXISTS user_profiles CASCADE;
DROP TABLE IF EXISTS user_external_logins CASCADE;
DROP TABLE IF EXISTS user_credentials CASCADE;
DROP TABLE IF EXISTS email_suppressions CASCADE;
DROP TABLE IF EXISTS password_reset_tokens CASCADE;
DROP TABLE IF EXISTS verification_tokens CASCADE;

-- ============================================================================
-- STEP 3: Remove new RBAC roles
-- ============================================================================
DELETE FROM roles WHERE name IN ('email-verified', 'trusted-contact');
