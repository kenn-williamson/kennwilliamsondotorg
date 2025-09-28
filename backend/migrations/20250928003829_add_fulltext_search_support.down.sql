-- Rollback migration: Remove comprehensive search support
-- This migration removes tsvector columns, trigram indexes, triggers, and functions

-- Drop triggers first
DROP TRIGGER IF EXISTS trigger_update_user_search_vector ON users;
DROP TRIGGER IF EXISTS trigger_update_phrase_search_vector ON phrases;

-- Drop functions
DROP FUNCTION IF EXISTS update_user_search_vector();
DROP FUNCTION IF EXISTS update_phrase_search_vector();

-- Drop full-text search indexes
DROP INDEX IF EXISTS idx_users_search_vector;
DROP INDEX IF EXISTS idx_phrases_search_vector;

-- Drop trigram indexes
DROP INDEX IF EXISTS idx_users_display_name_trgm;
DROP INDEX IF EXISTS idx_users_email_trgm;
DROP INDEX IF EXISTS idx_users_slug_trgm;
DROP INDEX IF EXISTS idx_phrases_text_trgm;

-- Drop columns
ALTER TABLE users DROP COLUMN IF EXISTS search_vector;
ALTER TABLE phrases DROP COLUMN IF EXISTS search_vector;

-- Note: We don't drop the pg_trgm extension as it might be used by other parts of the system