-- Revert initial schema - drop all tables, triggers, functions, and extensions
-- Must be done in reverse dependency order

-- Drop tables (in reverse dependency order)
DROP TABLE IF EXISTS incident_timers CASCADE;
DROP TABLE IF EXISTS user_roles CASCADE;
DROP TABLE IF EXISTS roles CASCADE;
DROP TABLE IF EXISTS users CASCADE;

-- Drop function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Note: We don't drop the pg_uuidv7 extension as it may be used by other applications
-- DROP EXTENSION IF EXISTS pg_uuidv7;
