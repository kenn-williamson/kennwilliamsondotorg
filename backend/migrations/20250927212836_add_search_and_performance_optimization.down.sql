-- Rollback search and performance optimization

-- Drop triggers
DROP TRIGGER IF EXISTS trigger_update_phrase_search_vector ON phrases;
DROP TRIGGER IF EXISTS trigger_update_user_search_vector ON users;

-- Drop functions
DROP FUNCTION IF EXISTS update_phrase_search_vector();
DROP FUNCTION IF EXISTS update_user_search_vector();

-- Drop trigram indexes
DROP INDEX IF EXISTS idx_phrases_text_trgm;
DROP INDEX IF EXISTS idx_users_slug_trgm;
DROP INDEX IF EXISTS idx_users_email_trgm;
DROP INDEX IF EXISTS idx_users_display_name_trgm;

-- Drop full-text search indexes
DROP INDEX IF EXISTS idx_phrases_search_vector;
DROP INDEX IF EXISTS idx_users_search_vector;

-- Drop search_vector columns
ALTER TABLE phrases DROP COLUMN IF EXISTS search_vector;
ALTER TABLE users DROP COLUMN IF EXISTS search_vector;

-- Drop composite indexes
DROP INDEX IF EXISTS idx_user_roles_user_role;
DROP INDEX IF EXISTS idx_user_excluded_phrases_user_phrase;
DROP INDEX IF EXISTS idx_phrases_active_id;

-- Drop pg_trgm extension (only if no other extensions depend on it)
DROP EXTENSION IF EXISTS pg_trgm;
