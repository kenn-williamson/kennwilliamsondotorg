# OAuth + Email Verification System Design

## Overview
Comprehensive authentication upgrade adding Google OAuth SSO and email verification with role-based feature gating.

## Goals
1. **Google OAuth Integration**: Allow users to sign in with Google
2. **Email Verification**: Verify email addresses for security and anti-spam
3. **Feature Gating**: Restrict unverified users to view-only access
4. **Account Linking**: Auto-link OAuth accounts by verified email
5. **Identity Management**: Separate real name (from OAuth) from display name (user-controlled)

## Design Decisions

### 1. Role-Based Email Verification (Not Boolean Flag)
**Decision**: Use `email-verified` role instead of `email_verified` boolean field

**Rationale**:
- Maintains consistency with existing RBAC system
- Extensible for future role-based permissions
- Middleware already checks roles
- Easier to manage in admin panel

**Implementation**:
- New role: `email-verified` in roles table
- Automatically assigned on email verification
- Manually assignable by admins
- Required for feature access

### 2. Password Optional for OAuth Users
**Decision**: Make `password_hash` nullable in database

**Rationale**:
- OAuth-only users don't need passwords
- Allows pure Google sign-in flow
- Users can optionally link password later

**Implementation**:
- `ALTER TABLE users ALTER COLUMN password_hash DROP NOT NULL`
- Validation: At least one auth method required (password OR google_user_id)

### 3. Real Name vs Display Name
**Decision**: Two separate fields
- `real_name` (optional): From OAuth, auto-updates, read-only to user
- `display_name` (required): User-controlled, used for slugs

**Rationale**:
- Users want consistent identity across platforms (real_name from Google)
- Users also want custom usernames for the site (display_name)
- Future: Could show "John Doe (@johnd)" format

**Implementation**:
- `real_name` populated from OAuth profile
- Updates on each OAuth login
- `display_name` unchanged by OAuth
- Email/password users have no `real_name`

### 4. Account Linking Strategy
**Decision**: Auto-link by verified email, with fallback to manual linking

**Flow**:
1. User signs in with Google
2. Check if `google_user_id` exists → Login existing user
3. Check if email exists AND email is verified → Link to existing account
4. Otherwise → Create new user with `email-verified` role

**Rationale**:
- Convenience for users with existing accounts
- Only link verified emails (security)
- Prevents account hijacking via unverified emails

### 5. Email Verification Flow
**Decision**: Token-based verification via email link

**Flow**:
1. User registers → Create unverified user
2. Generate random token, hash with SHA-256
3. Send email with verification link: `https://kennwilliamson.org/verify-email?token=XXX`
4. User clicks link → Validate token → Assign `email-verified` role
5. Delete all user's verification tokens

**Token Security**:
- 256-bit random token (hex encoded)
- SHA-256 hashed in database
- 24-hour expiration
- Single-use (deleted on verification)

### 6. Feature Gating
**Decision**: Middleware-based role checking

**Protected Actions** (require `email-verified` role):
- Creating incident timers
- Suggesting phrases
- Any future user-generated content

**Public Actions** (no verification required):
- Viewing public content
- Reading timers/phrases
- Browsing the site

**Implementation**:
- New middleware: `email_verification_middleware`
- Returns 403 with message: "Email not verified. Please check your inbox to verify your email address."
- Applied to specific POST/PUT/DELETE endpoints

### 7. Email Service Architecture
**Decision**: Generic trait with AWS SES implementation

**Rationale**:
- Provider-agnostic interface
- Easy to swap providers later
- Mockable for testing
- SES is cost-effective (62K emails/month free)

**Interface**:
```rust
trait EmailService {
    async fn send_verification_email(to: &str, token: &str) -> Result<()>
}
```

**Implementations**:
- `SesEmailService`: Production (AWS SES)
- `MockEmailService`: Testing (in-memory)
- Future: `SmtpEmailService`, `SendGridService`, etc.

## Data Model Changes

### Users Table
```sql
ALTER TABLE users
  ADD COLUMN real_name VARCHAR(255),
  ADD COLUMN google_user_id VARCHAR(255) UNIQUE,
  ALTER COLUMN password_hash DROP NOT NULL;
```

### Verification Tokens Table
```sql
CREATE TABLE verification_tokens (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
  user_id UUID REFERENCES users(id) ON DELETE CASCADE,
  token_hash VARCHAR(255) UNIQUE NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Roles
```sql
INSERT INTO roles (name, description) VALUES
  ('email-verified', 'User has verified their email address');
```

## API Endpoints

### Email Verification
- `POST /backend/auth/send-verification` (protected) - Send/resend verification email
- `GET /backend/auth/verify-email?token=XXX` (public) - Verify email via token

### Google OAuth
- `GET /backend/auth/google/url` (public) - Get Google OAuth authorization URL
- `POST /backend/auth/google/callback` (public) - Handle OAuth callback with code

### Updated Endpoints
- `POST /backend/auth/register` - Now sends verification email
- `GET /backend/auth/me` - Returns `email_verified` and `real_name`

## Frontend Changes

### New Components
- `GoogleOAuthButton.vue` - "Sign in with Google" button
- `EmailVerificationBanner.vue` - Persistent banner for unverified users
- `VerifyEmailPage.vue` - Email verification success/error page

### Updated Components
- `LoginPage.vue` - Add Google OAuth button
- `RegisterPage.vue` - Add Google OAuth button + verification notice
- `ProfilePage.vue` - Show verification status, real_name, linked providers

### New Composables
- `useEmailVerification()` - Send/resend verification emails
- `useGoogleOAuth()` - OAuth flow management

## Security Considerations

### Token Security
- Cryptographically secure random tokens (256-bit)
- SHA-256 hashing before database storage
- 24-hour expiration
- Single-use tokens (deleted on verification)

### OAuth Security
- State parameter for CSRF protection
- Validate OAuth tokens with Google
- Verify email from Google (already verified by Google)
- Only link to verified accounts

### Email Security
- SPF/DKIM/DMARC for AWS SES
- Rate limiting on verification emails (max 3/hour per user)
- Prevent email enumeration (generic success messages)

### Feature Gating
- Middleware enforces verification requirement
- Clear error messages for UX
- No data leakage in error responses

## Migration Strategy

### Existing Users
**Decision**: Create admin tool for manual verification, handle case-by-case

**Options**:
1. **Grandfather all**: `UPDATE users SET ... WHERE created_at < NOW()` + assign role
2. **Admin verification**: Admins manually verify trusted users
3. **Require verification**: All users must verify (may disrupt existing users)

**Recommendation**: Start with admin verification tool, then decide based on user count

### Deployment Steps
1. Run database migrations (adds columns, tables, role)
2. Deploy backend with new endpoints
3. Deploy frontend with OAuth/verification UI
4. Configure AWS SES (verify domain, production access)
5. Set up Google OAuth app
6. Update environment variables
7. Test verification flow end-to-end
8. Monitor email deliverability

## Testing Strategy

### Backend Tests
- Email verification token generation/validation
- OAuth callback flow with mock Google API
- Account linking scenarios (verified/unverified)
- Feature gating middleware (verified/unverified users)
- Email service mock tests

### Frontend Tests
- OAuth button redirects correctly
- Verification banner shows for unverified users
- Verification page handles success/error states
- Profile shows verification status

### Integration Tests
- Full OAuth flow: Google → callback → session
- Full verification flow: register → email → verify → access
- Account linking: OAuth → existing account

### Manual Testing
- Real Google OAuth flow
- Real AWS SES email delivery
- Mobile responsiveness
- Error states (expired tokens, invalid codes)

## Configuration Requirements

### Environment Variables
```bash
# Google OAuth
GOOGLE_CLIENT_ID=xxx
GOOGLE_CLIENT_SECRET=xxx
GOOGLE_REDIRECT_URI=https://kennwilliamson.org/auth/google/callback

# AWS SES
AWS_REGION=us-east-1
AWS_ACCESS_KEY_ID=xxx
AWS_SECRET_ACCESS_KEY=xxx
SES_FROM_EMAIL=noreply@kennwilliamson.org
SES_REPLY_TO_EMAIL=support@kennwilliamson.org

# Email Verification
VERIFICATION_TOKEN_EXPIRY_HOURS=24
FRONTEND_URL=https://kennwilliamson.org
```

### External Setup Required
1. **Google Cloud Console**: OAuth app configuration
2. **AWS Console**: SES domain verification, production access
3. **DNS**: SPF/DKIM records for SES

## User Flows

### New User - Email/Password Registration
1. User fills registration form
2. Backend creates user (no `email-verified` role)
3. Backend sends verification email
4. User sees "Check your email" message
5. User clicks link in email
6. Backend assigns `email-verified` role
7. User can now create timers, suggest phrases

### New User - Google OAuth
1. User clicks "Sign in with Google"
2. Redirect to Google consent screen
3. Google redirects back with code
4. Backend exchanges code for profile
5. Backend creates user with `email-verified` role (Google emails are verified)
6. User immediately has full access

### Existing User - Linking Google
1. User logs in with email/password
2. User clicks "Link Google Account" in profile
3. OAuth flow completes
4. Backend adds `google_user_id` to user
5. Backend updates `real_name` from Google
6. User can now sign in with either method

### Unverified User - Restricted Access
1. Unverified user tries to create timer
2. Middleware checks for `email-verified` role
3. Returns 403: "Email not verified. Please check your inbox..."
4. Frontend shows verification banner
5. User clicks "Resend verification email"
6. User verifies and gains access

## Future Enhancements

### Additional OAuth Providers
- GitHub OAuth (needs `oauth_provider_id` tracking, not just email)
- Discord OAuth (same as GitHub)
- Generic OAuth2 provider support

### Enhanced Verification
- SMS verification option
- Two-factor authentication
- Email change verification

### Account Management
- Unlink OAuth providers
- Multiple OAuth providers per account
- Password reset flow for OAuth users

### Email Features
- Welcome email series
- Notification preferences
- Email digest of activity

## Open Questions

1. **Email content**: Plain text or HTML? (Decision: Plain text to start, HTML later with designer assets)
2. **Error messages**: Specific or generic? (Decision: Specific "Email not verified" message)
3. **Existing users**: Auto-verify or manual? (Decision: Admin tool for case-by-case)
4. **Rate limiting**: How many verification emails per hour? (Recommendation: 3/hour)
5. **Token length**: 32 bytes (256 bits) or 64 bytes? (Recommendation: 32 bytes hex = 64 chars)

## Success Criteria

1. **Functionality**: Users can register, verify email, and sign in with Google
2. **Security**: Tokens are secure, OAuth is validated, feature gating works
3. **UX**: Clear messaging for verification status, easy OAuth flow
4. **Reliability**: Emails deliver consistently via SES
5. **Testing**: Comprehensive test coverage for all flows
6. **Documentation**: Setup guides for Google OAuth and AWS SES

---

## Implementation Status

### ✅ Completed (TDD Approach)

**Architecture Decision: Builder Pattern for AuthService**
- Implemented `AuthServiceBuilder` for flexible dependency injection
- Scales cleanly for future features (OAuth, 2FA, etc.)
- Legacy `new()` constructor maintained for backward compatibility
- Production uses builder with all dependencies

**Database Layer:**
- ✅ Migration `20251001224112_add_oauth_email_verification.up.sql` created
- ✅ Added `real_name` and `google_user_id` to users table
- ✅ Made `password_hash` nullable for OAuth-only users
- ✅ Created `verification_tokens` table with indexes
- ✅ Added `email-verified` role

**Repository Layer:**
- ✅ `VerificationTokenRepository` trait defined
- ✅ `PostgresVerificationTokenRepository` implementation complete
- ✅ `MockVerificationTokenRepository` for testing

**Service Layer - Email Verification:**
- ✅ `send_verification_email()` implemented with TDD
  - Generates secure 256-bit random tokens
  - SHA-256 hashing before storage
  - 24-hour token expiration
  - Email sending via EmailService trait
- ✅ `verify_email()` implemented with TDD
  - Token validation and expiration checking
  - Role assignment (`email-verified`)
  - Automatic token cleanup
- ✅ 8 unit tests passing (test-first approach)

**Email Service:**
- ✅ `EmailService` trait (provider-agnostic)
- ✅ `MockEmailService` for testing
- ✅ `SesEmailService` skeleton (ready for AWS SDK integration)

**Service Container:**
- ✅ Updated to use builder pattern
- ✅ Injects verification repositories and email service in production
- ✅ Uses mocks for testing environment

**Registration Flow:**
- ✅ Updated `register()` method to send verification emails
- ✅ Added `frontend_url` parameter to `register()` method
- ✅ Implemented graceful degradation (works without email service)
- ✅ Route handler updated to pass `FRONTEND_URL` from environment
- ✅ Added `FRONTEND_URL=https://localhost` to `.env.development`
- ✅ 5 unit tests passing (TDD approach)
- ✅ All 58 auth tests passing (no breaking changes)

**API Routes - Email Verification:**
- ✅ `POST /backend/protected/auth/send-verification` - Resend verification email (protected)
- ✅ `GET /backend/public/auth/verify-email?token=XXX` - Verify email with token (public)
- ✅ Route handlers implemented with error handling
- ✅ Routes registered in mod.rs
- ✅ All tests passing (58 auth tests)

**Token Cleanup Service:**
- ✅ `CleanupService` implemented with TDD approach
- ✅ Cleans up expired refresh tokens and verification tokens
- ✅ Background task runs on app startup, then every 24 hours (configurable)
- ✅ Configuration: `CLEANUP_INTERVAL_HOURS` environment variable (default: 24)
- ✅ Integrated into ServiceContainer with dependency injection
- ✅ 5 unit tests with mock repositories (test service creation, repository calls, error handling)
- ✅ 5 integration tests with testcontainers (test expired refresh tokens, verification tokens, both types, no expired tokens, empty database)
- ✅ All 170 tests passing (165 unit + 5 integration)

### 🚧 In Progress
(None)

### 📋 Remaining Tasks

**Service Layer - Google OAuth:**
- Implement `get_google_oauth_url()`
- Implement `google_oauth_callback()`
- Account linking logic
- OAuth unit tests (TDD)

**Middleware:**
- Create `email_verification_middleware`
- Apply to protected endpoints (timers, phrase suggestions)
- Unit tests for middleware

**API Routes - Google OAuth:**
- `GET /backend/public/auth/google/url`
- `POST /backend/public/auth/google/callback`

**Integration Tests:**
- Email verification flow (testcontainers)
- OAuth flow (testcontainers)
- Feature gating scenarios

**Infrastructure:**
- Run database migrations
- Update SQLx query cache
- Configure AWS SES environment variables
- Configure Google OAuth credentials

**Frontend Changes:**
- Not yet started (see original design doc)

---

**Status**: Partial Implementation - Email Verification Complete, OAuth Pending
**Next Steps**: Complete register() update, then implement OAuth with TDD
