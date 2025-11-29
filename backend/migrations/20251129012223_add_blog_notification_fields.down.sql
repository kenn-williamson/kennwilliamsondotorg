-- Drop unsubscribe_tokens table
DROP TABLE IF EXISTS unsubscribe_tokens;

-- Remove notification preference from user_preferences
DROP INDEX IF EXISTS idx_user_preferences_blog_notifications;
ALTER TABLE user_preferences DROP COLUMN IF EXISTS notify_blog_posts;
