-- Add notification preference to user_preferences
ALTER TABLE user_preferences
ADD COLUMN notify_blog_posts BOOLEAN NOT NULL DEFAULT true;

CREATE INDEX idx_user_preferences_blog_notifications
  ON user_preferences(notify_blog_posts)
  WHERE notify_blog_posts = true;

-- Create separate unsubscribe_tokens table (extensible for future email types)
CREATE TABLE unsubscribe_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    email_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_user_email_type UNIQUE (user_id, email_type)
);

CREATE INDEX idx_unsubscribe_tokens_token_hash ON unsubscribe_tokens(token_hash);
CREATE INDEX idx_unsubscribe_tokens_user_id ON unsubscribe_tokens(user_id);

COMMENT ON TABLE unsubscribe_tokens IS 'Stores hashed unsubscribe tokens for one-click email unsubscribe';
COMMENT ON COLUMN unsubscribe_tokens.email_type IS 'Type of email notifications this token unsubscribes from (e.g., blog_notifications, announcements)';
