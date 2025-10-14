-- Drop old auth columns migration (with data backfill)
-- Phase 9: Cutover & Cleanup
--
-- This migration:
-- 1. Copies existing data from users table to new normalized tables
-- 2. Drops old columns from users table
--
-- These columns are migrated to dedicated tables:
-- - password_hash -> user_credentials.password_hash
-- - google_user_id -> user_external_logins.provider_user_id
-- - real_name -> user_profiles.real_name
-- - timer_is_public, timer_show_in_list -> user_preferences.*

-- ============================================================================
-- STEP 1: Backfill Data (copy from old columns to new tables)
-- ============================================================================

-- Backfill user_credentials (for users with passwords)
INSERT INTO user_credentials (user_id, password_hash, password_updated_at, created_at)
SELECT
    id,
    password_hash,
    updated_at,  -- Use updated_at as password_updated_at
    created_at
FROM users
WHERE password_hash IS NOT NULL
ON CONFLICT (user_id) DO NOTHING;

-- Backfill user_external_logins (for users with Google OAuth)
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

-- Backfill user_profiles (for users with profile data)
INSERT INTO user_profiles (user_id, real_name, created_at, updated_at)
SELECT
    id,
    real_name,
    created_at,
    updated_at
FROM users
WHERE real_name IS NOT NULL
ON CONFLICT (user_id) DO NOTHING;

-- Backfill user_preferences (ALL users)
INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list, created_at, updated_at)
SELECT
    id,
    timer_is_public,
    timer_show_in_list,
    created_at,
    updated_at
FROM users
ON CONFLICT (user_id) DO NOTHING;

-- ============================================================================
-- STEP 2: Drop Old Columns (data is now safely in new tables)
-- ============================================================================

ALTER TABLE users
    DROP COLUMN IF EXISTS password_hash,
    DROP COLUMN IF EXISTS google_user_id,
    DROP COLUMN IF EXISTS real_name,
    DROP COLUMN IF EXISTS timer_is_public,
    DROP COLUMN IF EXISTS timer_show_in_list;
