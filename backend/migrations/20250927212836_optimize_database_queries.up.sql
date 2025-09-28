-- Database Query Optimization Migration
-- Adds composite indexes for better query performance

-- Composite index for phrases table - optimizes random selection and filtering
-- Covers queries: WHERE active = true ORDER BY id, WHERE active = true AND id NOT IN (...)
CREATE INDEX idx_phrases_active_id ON phrases(active, id);

-- Composite index for user_excluded_phrases - optimizes exclusion checks
-- Covers queries: WHERE user_id = ? AND phrase_id NOT IN (...)
CREATE INDEX idx_user_excluded_phrases_user_phrase ON user_excluded_phrases(user_id, phrase_id);

-- Full-text search index for users - optimizes search queries
-- Covers queries: WHERE email ILIKE ? OR display_name ILIKE ? OR slug ILIKE ?
CREATE INDEX idx_users_search ON users USING gin(to_tsvector('english', email || ' ' || display_name || ' ' || slug));

-- Additional index for user_roles to optimize the array_agg() query
-- Covers queries: JOIN user_roles ur ON u.id = ur.user_id JOIN roles r ON ur.role_id = r.id
CREATE INDEX idx_user_roles_user_role ON user_roles(user_id, role_id);