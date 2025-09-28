-- Rollback Database Query Optimization Migration
-- Removes composite indexes added for query performance

-- Drop indexes in reverse order
DROP INDEX IF EXISTS idx_user_roles_user_role;
DROP INDEX IF EXISTS idx_users_search;
DROP INDEX IF EXISTS idx_user_excluded_phrases_user_phrase;
DROP INDEX IF EXISTS idx_phrases_active_id;