-- Backfill Auth Schema Data Migration Script
-- Phase 7: Migrate existing data from users table to normalized auth tables
--
-- This script safely migrates existing data while maintaining integrity.
-- Uses ON CONFLICT to support idempotency (can be run multiple times safely).

BEGIN;

-- Step 1: Backfill user_credentials (for users with passwords)
-- Only copy users who have a password_hash set
INSERT INTO user_credentials (user_id, password_hash, password_updated_at, created_at)
SELECT
    id,
    password_hash,
    updated_at,  -- Use updated_at as password_updated_at
    created_at
FROM users
WHERE password_hash IS NOT NULL
ON CONFLICT (user_id) DO NOTHING;

-- Step 2: Backfill user_external_logins (for users with Google OAuth)
-- Only copy users who have a google_user_id set
INSERT INTO user_external_logins (user_id, provider, provider_user_id, linked_at, created_at)
SELECT
    id,
    'google',  -- Provider is always 'google' for existing data
    google_user_id,
    created_at,  -- Use created_at as linked_at
    created_at
FROM users
WHERE google_user_id IS NOT NULL
ON CONFLICT (provider, provider_user_id) DO NOTHING;

-- Step 3: Backfill user_profiles (for users with profile data)
-- Only copy users who have a real_name set
INSERT INTO user_profiles (user_id, real_name, created_at, updated_at)
SELECT
    id,
    real_name,
    created_at,
    updated_at
FROM users
WHERE real_name IS NOT NULL
ON CONFLICT (user_id) DO NOTHING;

-- Step 4: Backfill user_preferences (ALL users)
-- Every user should have preferences, so we copy all users
INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list, created_at, updated_at)
SELECT
    id,
    timer_is_public,
    timer_show_in_list,
    created_at,
    updated_at
FROM users
ON CONFLICT (user_id) DO NOTHING;

COMMIT;

-- Verification Query: Show counts from all tables
SELECT
    (SELECT COUNT(*) FROM users) as total_users,
    (SELECT COUNT(*) FROM user_credentials) as users_with_password,
    (SELECT COUNT(*) FROM user_external_logins) as users_with_oauth,
    (SELECT COUNT(*) FROM user_profiles) as users_with_profile,
    (SELECT COUNT(*) FROM user_preferences) as users_with_prefs;

-- Show summary of migration results
\echo ''
\echo '=== Migration Summary ==='
\echo 'Total users:', (SELECT COUNT(*) FROM users);
\echo 'Users with passwords:', (SELECT COUNT(*) FROM user_credentials);
\echo 'Users with OAuth:', (SELECT COUNT(*) FROM user_external_logins);
\echo 'Users with profiles:', (SELECT COUNT(*) FROM user_profiles);
\echo 'Users with preferences:', (SELECT COUNT(*) FROM user_preferences);
\echo ''
\echo 'Migration completed successfully!'
