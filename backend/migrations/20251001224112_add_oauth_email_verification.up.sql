-- Add OAuth and email verification support
-- Uses role-based verification (email-verified role) for RBAC consistency

-- Add OAuth and user identity fields to users table
ALTER TABLE users
  ADD COLUMN real_name VARCHAR(255),
  ADD COLUMN google_user_id VARCHAR(255) UNIQUE;

-- Make password_hash nullable for OAuth-only users (users who only sign in via Google)
ALTER TABLE users ALTER COLUMN password_hash DROP NOT NULL;

-- Create verification tokens table for email verification flow
CREATE TABLE verification_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_users_google_id ON users(google_user_id);
CREATE INDEX idx_verification_tokens_user_id ON verification_tokens(user_id);
CREATE INDEX idx_verification_tokens_token_hash ON verification_tokens(token_hash);
CREATE INDEX idx_verification_tokens_expires_at ON verification_tokens(expires_at);

-- Add email-verified and trusted-contact roles for RBAC-based verification and access control
INSERT INTO roles (name, description) VALUES
    ('email-verified', 'User has verified their email address'),
    ('trusted-contact', 'Trusted contact with access to personal/family content');

-- Add trigger to automatically update updated_at for verification_tokens
CREATE TRIGGER update_verification_tokens_updated_at
    BEFORE UPDATE ON verification_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
