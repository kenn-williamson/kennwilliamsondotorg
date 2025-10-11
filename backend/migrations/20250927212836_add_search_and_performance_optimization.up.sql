-- Migration: Search and Performance Optimization
-- Adds comprehensive search support (full-text + trigram) and composite indexes

-- Enable pg_trgm extension for trigram-based ILIKE searches
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- ========================================
-- Composite Indexes for Query Performance
-- ========================================

-- Composite index for phrases table - optimizes random selection and filtering
-- Covers queries: WHERE active = true ORDER BY id, WHERE active = true AND id NOT IN (...)
CREATE INDEX idx_phrases_active_id ON phrases(active, id);

-- Composite index for user_excluded_phrases - optimizes exclusion checks
-- Covers queries: WHERE user_id = ? AND phrase_id NOT IN (...)
CREATE INDEX idx_user_excluded_phrases_user_phrase ON user_excluded_phrases(user_id, phrase_id);

-- Additional index for user_roles to optimize the array_agg() query
-- Covers queries: JOIN user_roles ur ON u.id = ur.user_id JOIN roles r ON ur.role_id = r.id
CREATE INDEX idx_user_roles_user_role ON user_roles(user_id, role_id);

-- ========================================
-- Full-Text Search Support
-- ========================================

-- Add search_vector columns
ALTER TABLE users ADD COLUMN search_vector tsvector;
ALTER TABLE phrases ADD COLUMN search_vector tsvector;

-- Create GIN indexes for full-text search performance
CREATE INDEX idx_users_search_vector ON users USING GIN (search_vector);
CREATE INDEX idx_phrases_search_vector ON phrases USING GIN (search_vector);

-- ========================================
-- Trigram Search Support (ILIKE)
-- ========================================

-- Create trigram indexes for fast ILIKE searches
CREATE INDEX idx_users_display_name_trgm ON users USING GIN (display_name gin_trgm_ops);
CREATE INDEX idx_users_email_trgm ON users USING GIN (email gin_trgm_ops);
CREATE INDEX idx_users_slug_trgm ON users USING GIN (slug gin_trgm_ops);
CREATE INDEX idx_phrases_text_trgm ON phrases USING GIN (phrase_text gin_trgm_ops);

-- ========================================
-- Search Vector Maintenance Functions
-- ========================================

-- Function to update user search vector
CREATE OR REPLACE FUNCTION update_user_search_vector()
RETURNS TRIGGER AS $$
BEGIN
  NEW.search_vector := to_tsvector('english',
    COALESCE(NEW.display_name, '') || ' ' || COALESCE(NEW.email, '') || ' ' || COALESCE(NEW.slug, ''));
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to update phrase search vector
CREATE OR REPLACE FUNCTION update_phrase_search_vector()
RETURNS TRIGGER AS $$
BEGIN
  NEW.search_vector := to_tsvector('english', COALESCE(NEW.phrase_text, ''));
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers for automatic search vector updates
CREATE TRIGGER trigger_update_user_search_vector
  BEFORE INSERT OR UPDATE OF display_name, email, slug ON users
  FOR EACH ROW EXECUTE FUNCTION update_user_search_vector();

CREATE TRIGGER trigger_update_phrase_search_vector
  BEFORE INSERT OR UPDATE OF phrase_text ON phrases
  FOR EACH ROW EXECUTE FUNCTION update_phrase_search_vector();

-- Populate search vectors for existing data
UPDATE users SET search_vector = to_tsvector('english',
  COALESCE(display_name, '') || ' ' || COALESCE(email, '') || ' ' || COALESCE(slug, ''));

UPDATE phrases SET search_vector = to_tsvector('english', COALESCE(phrase_text, ''));

-- ========================================
-- Documentation Comments
-- ========================================

COMMENT ON COLUMN users.search_vector IS 'Full-text search vector for display_name, email, and slug fields';
COMMENT ON COLUMN phrases.search_vector IS 'Full-text search vector for phrase_text field';
COMMENT ON INDEX idx_users_search_vector IS 'GIN index for fast full-text search on users';
COMMENT ON INDEX idx_phrases_search_vector IS 'GIN index for fast full-text search on phrases';
COMMENT ON INDEX idx_users_display_name_trgm IS 'Trigram index for fast ILIKE searches on display_name';
COMMENT ON INDEX idx_users_email_trgm IS 'Trigram index for fast ILIKE searches on email';
COMMENT ON INDEX idx_users_slug_trgm IS 'Trigram index for fast ILIKE searches on slug';
COMMENT ON INDEX idx_phrases_text_trgm IS 'Trigram index for fast ILIKE searches on phrase_text';
