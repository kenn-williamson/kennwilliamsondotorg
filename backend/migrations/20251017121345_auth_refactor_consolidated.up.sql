-- ============================================================================
-- CONSOLIDATED AUTH REFACTOR MIGRATION
-- Combines: add_auth_and_compliance + add_user_auth_schema + drop_old_auth
-- ============================================================================
-- Creates normalized multi-table auth schema with:
-- - Email verification and password reset
-- - Email suppression list (AWS SES compliance)
-- - Multi-provider OAuth support
-- - Separated user credentials, profiles, and preferences
-- - Access request workflow for trusted-contact role
-- ============================================================================

-- ============================================================================
-- VERIFICATION TOKENS: Email verification flow
-- ============================================================================
CREATE TABLE verification_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_verification_tokens_user_id ON verification_tokens(user_id);
CREATE INDEX idx_verification_tokens_token_hash ON verification_tokens(token_hash);
CREATE INDEX idx_verification_tokens_expires_at ON verification_tokens(expires_at);

CREATE TRIGGER update_verification_tokens_updated_at
    BEFORE UPDATE ON verification_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

COMMENT ON TABLE verification_tokens IS 'Tokens for email verification flow with expiration';

-- ============================================================================
-- PASSWORD RESET TOKENS: Secure password reset flow
-- ============================================================================
CREATE TABLE password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (expires_at > created_at)
);

CREATE INDEX idx_password_reset_tokens_user_id ON password_reset_tokens(user_id);
CREATE INDEX idx_password_reset_tokens_token_hash ON password_reset_tokens(token_hash);
CREATE INDEX idx_password_reset_tokens_expires_at ON password_reset_tokens(expires_at);

COMMENT ON TABLE password_reset_tokens IS 'Secure tokens for password reset flow with expiration';
COMMENT ON COLUMN password_reset_tokens.token_hash IS 'SHA-256 hash of the reset token sent via email';
COMMENT ON COLUMN password_reset_tokens.used_at IS 'Timestamp when token was used (prevents reuse)';

-- ============================================================================
-- EMAIL SUPPRESSIONS: AWS SES compliance (bounces, complaints, unsubscribes)
-- ============================================================================
CREATE TABLE email_suppressions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    email VARCHAR(255) NOT NULL UNIQUE,
    suppression_type VARCHAR(50) NOT NULL,
    reason TEXT,
    suppress_transactional BOOLEAN NOT NULL DEFAULT FALSE,
    suppress_marketing BOOLEAN NOT NULL DEFAULT TRUE,
    bounce_count INT NOT NULL DEFAULT 0,
    last_bounce_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (suppression_type IN ('bounce', 'complaint', 'unsubscribe', 'manual')),
    CHECK (bounce_count >= 0)
);

CREATE INDEX idx_email_suppressions_email ON email_suppressions(email);
CREATE INDEX idx_email_suppressions_type ON email_suppressions(suppression_type);
CREATE INDEX idx_email_suppressions_created_at ON email_suppressions(created_at);

COMMENT ON TABLE email_suppressions IS 'Email suppression list for AWS SES compliance (bounces, complaints, unsubscribes)';
COMMENT ON COLUMN email_suppressions.suppression_type IS 'Type: bounce (hard bounces), complaint (spam reports), unsubscribe (user opt-out), manual (admin action)';
COMMENT ON COLUMN email_suppressions.suppress_transactional IS 'If true, blocks ALL emails including verification and password reset';
COMMENT ON COLUMN email_suppressions.suppress_marketing IS 'If true, blocks marketing emails (newsletters, announcements)';

-- ============================================================================
-- USER_CREDENTIALS: Local password authentication
-- ============================================================================
CREATE TABLE user_credentials (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    password_hash VARCHAR(255) NOT NULL,
    password_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE user_credentials IS 'Local password authentication credentials (optional for OAuth-only users)';
COMMENT ON COLUMN user_credentials.user_id IS 'Foreign key to users table (primary key)';
COMMENT ON COLUMN user_credentials.password_hash IS 'bcrypt hashed password';
COMMENT ON COLUMN user_credentials.password_updated_at IS 'Last password change timestamp';

-- ============================================================================
-- USER_EXTERNAL_LOGINS: Multi-provider OAuth support
-- ============================================================================
CREATE TABLE user_external_logins (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    linked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_user_id)
);

CREATE INDEX idx_user_external_logins_user_id ON user_external_logins(user_id);
CREATE INDEX idx_user_external_logins_provider ON user_external_logins(provider, provider_user_id);

COMMENT ON TABLE user_external_logins IS 'OAuth provider links (supports multiple providers per user)';
COMMENT ON COLUMN user_external_logins.provider IS 'OAuth provider name (google, github, microsoft, etc.)';
COMMENT ON COLUMN user_external_logins.provider_user_id IS 'User ID from OAuth provider';
COMMENT ON COLUMN user_external_logins.linked_at IS 'When account was linked';

-- ============================================================================
-- USER_PROFILES: Optional profile data
-- ============================================================================
CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    real_name VARCHAR(255),
    bio TEXT,
    avatar_url VARCHAR(500),
    location VARCHAR(255),
    website VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE user_profiles IS 'Optional user profile data (bio, avatar, etc.)';
COMMENT ON COLUMN user_profiles.real_name IS 'User real name (from OAuth or user input)';
COMMENT ON COLUMN user_profiles.bio IS 'User bio/description';
COMMENT ON COLUMN user_profiles.avatar_url IS 'URL to user avatar image';

-- ============================================================================
-- USER_PREFERENCES: User settings
-- ============================================================================
CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    timer_is_public BOOLEAN NOT NULL DEFAULT true,
    timer_show_in_list BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE user_preferences IS 'User application preferences (can grow without breaking auth)';
COMMENT ON COLUMN user_preferences.timer_is_public IS 'Whether timer is publicly viewable (default true - opt-out model)';
COMMENT ON COLUMN user_preferences.timer_show_in_list IS 'Whether timer appears in public list (default true - opt-out model)';

-- Composite index for efficient public timer list queries
CREATE INDEX idx_user_preferences_timer_visibility
  ON user_preferences(timer_is_public, timer_show_in_list)
  WHERE timer_is_public = true AND timer_show_in_list = true;

-- ============================================================================
-- ACCESS_REQUESTS: Request trusted-contact role access
-- ============================================================================
CREATE TABLE access_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    message TEXT NOT NULL,
    requested_role VARCHAR(50) NOT NULL DEFAULT 'trusted-contact',
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected')),
    admin_id UUID REFERENCES users(id) ON DELETE SET NULL,
    admin_reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_access_requests_user_id ON access_requests(user_id);
CREATE INDEX idx_access_requests_status ON access_requests(status);
CREATE INDEX idx_access_requests_admin_id ON access_requests(admin_id);
CREATE INDEX idx_access_requests_requested_role ON access_requests(requested_role);

CREATE TRIGGER update_access_requests_updated_at
    BEFORE UPDATE ON access_requests
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

COMMENT ON TABLE access_requests IS 'User requests for trusted-contact role with admin approval workflow';
COMMENT ON COLUMN access_requests.user_id IS 'User requesting access';
COMMENT ON COLUMN access_requests.requested_role IS 'Role being requested (default: trusted-contact)';
COMMENT ON COLUMN access_requests.status IS 'Request status: pending, approved, or rejected';
COMMENT ON COLUMN access_requests.admin_id IS 'Admin who processed the request';

-- ============================================================================
-- RBAC ROLES: Add email verification and trusted contact roles
-- ============================================================================
INSERT INTO roles (name, description) VALUES
    ('email-verified', 'User has verified their email address'),
    ('trusted-contact', 'Trusted contact with access to personal/family content');

-- ============================================================================
-- DATA MIGRATION: Move existing user data to new tables
-- ============================================================================

-- Migrate password_hash from users table to user_credentials
INSERT INTO user_credentials (user_id, password_hash, password_updated_at, created_at)
SELECT
    id,
    password_hash,
    updated_at,
    created_at
FROM users
WHERE password_hash IS NOT NULL
ON CONFLICT (user_id) DO NOTHING;

-- Create user_preferences for all existing users (opt-out model with defaults)
INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list, created_at, updated_at)
SELECT
    id,
    true,
    true,
    created_at,
    updated_at
FROM users
ON CONFLICT (user_id) DO NOTHING;

-- ============================================================================
-- CLEANUP: Drop migrated columns from users table
-- ============================================================================
ALTER TABLE users DROP COLUMN IF EXISTS password_hash;
