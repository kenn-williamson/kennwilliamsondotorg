-- ============================================================================
-- ROLLBACK: USER AUTHENTICATION SCHEMA REFACTOR
-- Drop all new tables (data will be lost - use with caution)
-- ============================================================================

-- Drop tables in reverse order (respects foreign keys)
DROP TABLE IF EXISTS access_requests CASCADE;
DROP TABLE IF EXISTS user_preferences CASCADE;
DROP TABLE IF EXISTS user_profiles CASCADE;
DROP TABLE IF EXISTS user_external_logins CASCADE;
DROP TABLE IF EXISTS user_credentials CASCADE;
