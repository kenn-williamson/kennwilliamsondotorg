-- Create email suppressions table for AWS SES compliance
-- Handles bounces, complaints, unsubscribes, and manual suppressions

CREATE TABLE email_suppressions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    email VARCHAR(255) NOT NULL UNIQUE,
    suppression_type VARCHAR(50) NOT NULL, -- 'bounce', 'complaint', 'unsubscribe', 'manual'
    reason TEXT,

    -- Scope of suppression
    suppress_transactional BOOLEAN NOT NULL DEFAULT FALSE, -- verification, password reset, etc.
    suppress_marketing BOOLEAN NOT NULL DEFAULT TRUE,      -- newsletters, announcements, etc.

    -- Metadata for bounce tracking
    bounce_count INT NOT NULL DEFAULT 0,
    last_bounce_at TIMESTAMPTZ,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CHECK (suppression_type IN ('bounce', 'complaint', 'unsubscribe', 'manual')),
    CHECK (bounce_count >= 0)
);

-- Indexes for efficient lookups
CREATE INDEX idx_email_suppressions_email ON email_suppressions(email);
CREATE INDEX idx_email_suppressions_type ON email_suppressions(suppression_type);
CREATE INDEX idx_email_suppressions_created_at ON email_suppressions(created_at);

-- Comment for documentation
COMMENT ON TABLE email_suppressions IS 'Email suppression list for AWS SES compliance (bounces, complaints, unsubscribes)';
COMMENT ON COLUMN email_suppressions.suppression_type IS 'Type: bounce (hard bounces), complaint (spam reports), unsubscribe (user opt-out), manual (admin action)';
COMMENT ON COLUMN email_suppressions.suppress_transactional IS 'If true, blocks ALL emails including verification and password reset';
COMMENT ON COLUMN email_suppressions.suppress_marketing IS 'If true, blocks marketing emails (newsletters, announcements)';

-- Create password reset tokens table
CREATE TABLE password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CHECK (expires_at > created_at)
);

-- Indexes for efficient lookups
CREATE INDEX idx_password_reset_tokens_user_id ON password_reset_tokens(user_id);
CREATE INDEX idx_password_reset_tokens_token_hash ON password_reset_tokens(token_hash);
CREATE INDEX idx_password_reset_tokens_expires_at ON password_reset_tokens(expires_at);

-- Comment for documentation
COMMENT ON TABLE password_reset_tokens IS 'Secure tokens for password reset flow with expiration';
COMMENT ON COLUMN password_reset_tokens.token_hash IS 'SHA-256 hash of the reset token sent via email';
COMMENT ON COLUMN password_reset_tokens.used_at IS 'Timestamp when token was used (prevents reuse)';
