-- Revert phrases system - remove all phrase-related tables and system user
-- Remove in reverse dependency order

-- Drop phrase-related tables (CASCADE will handle triggers and foreign keys)
DROP TABLE IF EXISTS phrase_suggestions CASCADE;
DROP TABLE IF EXISTS user_excluded_phrases CASCADE;
DROP TABLE IF EXISTS phrases CASCADE;

-- Remove system user (only if no other data depends on it)
-- Note: This will fail if the system user has created other data
-- In that case, you may want to deactivate instead of delete
DELETE FROM users WHERE email = 'system@kennwilliamson.org';
