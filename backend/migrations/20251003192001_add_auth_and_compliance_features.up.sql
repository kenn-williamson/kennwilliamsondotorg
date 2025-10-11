-- Migration: Authentication and Compliance Features
-- Adds OAuth, email verification, password reset, and email suppression

-- ========================================
-- OAuth Support
-- ========================================

-- Add OAuth and user identity fields to users table
ALTER TABLE users
  ADD COLUMN real_name VARCHAR(255),
  ADD COLUMN google_user_id VARCHAR(255) UNIQUE;

-- Make password_hash nullable for OAuth-only users (users who only sign in via Google)
ALTER TABLE users ALTER COLUMN password_hash DROP NOT NULL;

-- Create index for OAuth lookups
CREATE INDEX idx_users_google_id ON users(google_user_id);

-- ========================================
-- Email Verification
-- ========================================

-- Create verification tokens table for email verification flow
CREATE TABLE verification_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_verification_tokens_user_id ON verification_tokens(user_id);
CREATE INDEX idx_verification_tokens_token_hash ON verification_tokens(token_hash);
CREATE INDEX idx_verification_tokens_expires_at ON verification_tokens(expires_at);

-- Add trigger to automatically update updated_at for verification_tokens
CREATE TRIGGER update_verification_tokens_updated_at
    BEFORE UPDATE ON verification_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ========================================
-- Password Reset
-- ========================================

-- Create password reset tokens table for secure token-based password reset flow
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

-- ========================================
-- Email Suppression (GDPR/CAN-SPAM/AWS SES)
-- ========================================

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

-- ========================================
-- RBAC Roles
-- ========================================

-- Add email-verified and trusted-contact roles for RBAC-based verification and access control
INSERT INTO roles (name, description) VALUES
    ('email-verified', 'User has verified their email address'),
    ('trusted-contact', 'Trusted contact with access to personal/family content');

-- ========================================
-- Documentation Comments
-- ========================================

COMMENT ON TABLE verification_tokens IS 'Tokens for email verification flow with expiration';
COMMENT ON TABLE password_reset_tokens IS 'Secure tokens for password reset flow with expiration';
COMMENT ON COLUMN password_reset_tokens.token_hash IS 'SHA-256 hash of the reset token sent via email';
COMMENT ON COLUMN password_reset_tokens.used_at IS 'Timestamp when token was used (prevents reuse)';
COMMENT ON TABLE email_suppressions IS 'Email suppression list for AWS SES compliance (bounces, complaints, unsubscribes)';
COMMENT ON COLUMN email_suppressions.suppression_type IS 'Type: bounce (hard bounces), complaint (spam reports), unsubscribe (user opt-out), manual (admin action)';
COMMENT ON COLUMN email_suppressions.suppress_transactional IS 'If true, blocks ALL emails including verification and password reset';
COMMENT ON COLUMN email_suppressions.suppress_marketing IS 'If true, blocks marketing emails (newsletters, announcements)';
