-- Rollback migration: Restore old columns and copy data back
-- Phase 9: Cutover & Cleanup (ROLLBACK)
--
-- This migration:
-- 1. Re-adds old columns to users table
-- 2. Copies data back from new tables to old columns
--
-- Note: This restores the old schema but new tables remain intact

-- ============================================================================
-- STEP 1: Re-add Old Columns to users table
-- ============================================================================

ALTER TABLE users
    ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255),
    ADD COLUMN IF NOT EXISTS google_user_id VARCHAR(255),
    ADD COLUMN IF NOT EXISTS real_name VARCHAR(255),
    ADD COLUMN IF NOT EXISTS timer_is_public BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN IF NOT EXISTS timer_show_in_list BOOLEAN NOT NULL DEFAULT false;

-- ============================================================================
-- STEP 2: Copy Data Back (from new tables to old columns)
-- ============================================================================

-- Restore password_hash from user_credentials
UPDATE users u
SET password_hash = uc.password_hash
FROM user_credentials uc
WHERE u.id = uc.user_id;

-- Restore google_user_id from user_external_logins
UPDATE users u
SET google_user_id = uel.provider_user_id
FROM user_external_logins uel
WHERE u.id = uel.user_id
  AND uel.provider = 'google';

-- Restore real_name from user_profiles
UPDATE users u
SET real_name = up.real_name
FROM user_profiles up
WHERE u.id = up.user_id;

-- Restore timer preferences from user_preferences
UPDATE users u
SET timer_is_public = uprefs.timer_is_public,
    timer_show_in_list = uprefs.timer_show_in_list
FROM user_preferences uprefs
WHERE u.id = uprefs.user_id;
