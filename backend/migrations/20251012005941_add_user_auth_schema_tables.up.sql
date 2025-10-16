-- ============================================================================
-- USER AUTHENTICATION SCHEMA REFACTOR
-- Add new normalized tables for users authentication, profile, and preferences
-- ============================================================================

-- ============================================================================
-- user_credentials: Local password authentication
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
-- user_external_logins: Multi-provider OAuth support
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

COMMENT ON TABLE user_external_logins IS 'OAuth provider links (supports multiple providers per user)';
COMMENT ON COLUMN user_external_logins.provider IS 'OAuth provider name (google, github, microsoft, etc.)';
COMMENT ON COLUMN user_external_logins.provider_user_id IS 'User ID from OAuth provider';
COMMENT ON COLUMN user_external_logins.linked_at IS 'When account was linked';

-- Indexes for performance
CREATE INDEX idx_user_external_logins_user_id ON user_external_logins(user_id);
CREATE INDEX idx_user_external_logins_provider ON user_external_logins(provider, provider_user_id);

-- ============================================================================
-- user_profiles: Optional profile data
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
-- user_preferences: User settings
-- ============================================================================
CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    timer_is_public BOOLEAN NOT NULL DEFAULT false,
    timer_show_in_list BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE user_preferences IS 'User application preferences (can grow without breaking auth)';
COMMENT ON COLUMN user_preferences.timer_is_public IS 'Whether timer is publicly viewable';
COMMENT ON COLUMN user_preferences.timer_show_in_list IS 'Whether timer appears in public list';

-- ============================================================================
-- access_requests: Request trusted-contact role access
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

COMMENT ON TABLE access_requests IS 'User requests for trusted-contact role with admin approval workflow';
COMMENT ON COLUMN access_requests.user_id IS 'User requesting access';
COMMENT ON COLUMN access_requests.requested_role IS 'Role being requested (default: trusted-contact)';
COMMENT ON COLUMN access_requests.status IS 'Request status: pending, approved, or rejected';
COMMENT ON COLUMN access_requests.admin_id IS 'Admin who processed the request';

-- Indexes for performance
CREATE INDEX idx_access_requests_user_id ON access_requests(user_id);
CREATE INDEX idx_access_requests_status ON access_requests(status);
CREATE INDEX idx_access_requests_admin_id ON access_requests(admin_id);
CREATE INDEX idx_access_requests_requested_role ON access_requests(requested_role);

-- Updated timestamp trigger
CREATE TRIGGER update_access_requests_updated_at
    BEFORE UPDATE ON access_requests
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
