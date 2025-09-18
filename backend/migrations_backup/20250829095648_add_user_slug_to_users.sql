-- Add slug field to users table for public URL access
ALTER TABLE users 
ADD COLUMN slug VARCHAR(255) UNIQUE NOT NULL DEFAULT '';

-- Add index for slug lookups
CREATE INDEX idx_users_slug ON users(slug);

-- Update existing users with default slugs (if any exist)
-- In production, this would need to be handled more carefully
UPDATE users SET slug = LOWER(REPLACE(email, '@', '-')) || '-' || LEFT(id::text, 8) WHERE slug = '';
