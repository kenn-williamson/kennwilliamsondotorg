# Security Implementation

## Overview
Comprehensive security implementation covering authentication, authorization, data protection, and infrastructure security for the KennWilliamson.org application.

## Authentication System

### JWT-Based Authentication
- **Token Type**: JSON Web Tokens with HS256 algorithm
- **Access Token**: 1-hour expiration, minimal claims (user ID only)
- **Storage**: In-memory via Nuxt session for security
- **Validation**: Signature verification on all protected routes

### Refresh Token System
- **Duration**: 1-week rolling tokens with 6-month hard limit
- **Storage**: SHA-256 hashed in database (never plaintext)
- **Rolling Strategy**: Each refresh generates new JWT + new refresh token
- **Multi-Device**: Separate tokens per login session
- **Revocation**: Individual or bulk token revocation support

### Password Security
- **Hashing**: bcrypt with cost factor 12 (production)
- **Validation**: Frontend validation with backend verification
- **Storage**: Only hashed passwords in database
- **Requirements**: Enforced via VeeValidate + Yup schemas

### OAuth Authentication
- **Provider**: Google OAuth 2.0 with PKCE flow
- **Flow Type**: Authorization Code with Proof Key for Code Exchange (PKCE)
- **State Storage**: Redis-backed state validation with expiration
- **Security**: PKCE prevents authorization code interception attacks
- **User Integration**: Automatic user creation or linking on successful OAuth
- **Implementation**: `backend/src/services/auth/auth_service/oauth.rs`
- **Frontend**: `frontend/app/pages/auth/google/callback.vue`

**PKCE Flow**:
1. Frontend generates code verifier and challenge
2. Backend creates OAuth URL with state stored in Redis (10-minute expiration)
3. User authenticates with Google
4. Google redirects to callback with authorization code
5. Backend validates state from Redis
6. Backend exchanges code for tokens using code verifier
7. Backend fetches user info and creates/links account
8. User receives JWT and refresh token

### Email Verification
- **Service**: AWS SES for email delivery
- **Token Generation**: Secure random tokens stored in database
- **Expiration**: Verification tokens expire after 24 hours
- **Flow**: Registration → Email sent → User clicks link → Token validated → Account verified
- **Implementation**: `backend/src/services/auth/auth_service/email_verification.rs`
- **Frontend**: `frontend/app/composables/useEmailVerification.ts`
- **Database**: `verification_tokens` table with user_id and expiration

## Authorization System

### Role-Based Access Control (RBAC)

The application implements a 4-tier role system stored in the `roles` and `user_roles` tables:

#### Role Definitions

1. **`user`** (Base Role - Immutable)
   - Automatically assigned on registration
   - Cannot be removed or changed via admin interface
   - Represents basic authenticated user status
   - Permissions: View public content, manage account settings

2. **`email-verified`** (Manageable)
   - Assigned after successful email verification
   - Can be manually granted/revoked by admins
   - Required for creating timers and phrases
   - Gates features requiring verified identity

3. **`trusted-contact`** (Manageable)
   - Manually granted by admins after request approval
   - Provides access to personal/family content on about pages
   - Protected content: Origins, Wilderness, Faith, Theology, Life Now, Vision pages
   - Public content: Overview, Professional, AI pages (no authentication required)

4. **`admin`** (Manageable)
   - Full system access including user management
   - Can grant/revoke manageable roles (`email-verified`, `trusted-contact`, `admin`)
   - Cannot remove `user` role (base role is permanent)
   - Access to admin panel and all privileged endpoints

#### Role Management
- **Assignment**: Roles assigned via `user_roles` junction table (many-to-many)
- **Middleware**: JWT validation extracts user context and roles
- **Route Protection**: Automatic role checking for admin and protected endpoints
- **Admin Interface**: Role toggle controls for manageable roles
- **Implementation**: See `backend/src/middleware/auth.rs`

### Protected Resources
- **User Data**: Users can only access their own timers
- **Admin Routes**: Additional role check for admin-only endpoints
- **Public Routes**: Specific endpoints bypass authentication
- **API Structure**: Clear separation of public/protected/admin routes

## Data Protection

### Input Validation
- **Frontend**: VeeValidate + Yup for form validation
- **Backend**: Serde validation on all requests
- **SQL Injection**: Prevented via SQLx parameterized queries
- **XSS Prevention**: Input sanitization and proper escaping

### Database Security
- **Connection**: Internal Docker network only
- **Credentials**: Environment variables, never in code
- **Query Safety**: SQLx compile-time SQL verification
- **Access Control**: Minimal database privileges

### Session Security
- **JWT Storage**: Client memory (not localStorage)
- **Refresh Tokens**: httpOnly cookies on server
- **CSRF Protection**: Origin validation
- **Session Cleanup**: Automatic expired token removal

## Infrastructure Security

### SSL/TLS Configuration
- **Development**: Self-signed certificates for HTTPS testing
- **Production**: Let's Encrypt with automatic renewal
- **Configuration**: See [IMPLEMENTATION-NGINX.md](IMPLEMENTATION-NGINX.md#ssl-certificate-management)
- **Enforcement**: HTTP automatically redirects to HTTPS

### Security Headers
- **Implementation**: Nginx security headers in production
- **CORS**: Environment-specific configuration
- **Rate Limiting**: Configured in nginx for API protection
- **Details**: See [IMPLEMENTATION-NGINX.md](IMPLEMENTATION-NGINX.md#security-features)

### Container Security
- **Non-Root**: Containers run as non-root users
- **Network Isolation**: Internal Docker network
- **Minimal Images**: Alpine-based for reduced attack surface
- **Health Checks**: Built-in container monitoring

## API Security

### Endpoint Protection Levels
1. **Public Endpoints** (No auth required):
   - Health checks: `/backend/health`
   - Authentication: `/backend/auth/register`, `/backend/auth/login`
   - Public data: `/backend/{user_slug}/incident-timer`

2. **Protected Endpoints** (JWT required):
   - User resources: `/backend/incident-timers/*`
   - Profile: `/backend/auth/me`
   - Phrases: `/backend/phrases/*`

3. **Admin Endpoints** (Admin role required):
   - Admin panel: `/backend/admin/*`
   - User management: `/backend/admin/users/*`
   - Phrase moderation: `/backend/admin/suggestions/*`

### Error Handling
- **Generic Responses**: No information leakage in errors
- **Consistent Format**: Standardized error response structure
- **Logging**: Detailed server logs without sensitive data
- **Status Codes**: Appropriate HTTP status for each error type

## Security Implementation Details

### Backend Security (Rust/Actix-web)
- **Middleware**: `backend/src/middleware/auth.rs` for JWT validation
- **Services**: `backend/src/services/auth.rs` for authentication logic
- **Password Hashing**: bcrypt implementation in auth service
- **Token Generation**: JWT creation with proper claims

### Frontend Security (Nuxt/Vue)
- **Auth Utils**: Nuxt auth-utils for session management
- **Route Guards**: `app/middleware/auth.ts` for protected pages
- **API Client**: `app/composables/useBackendFetch.ts` with auth headers
- **Form Validation**: Comprehensive client-side validation

### Database Security
- **Schema**: Role-based tables with proper constraints
- **Migrations**: Version-controlled schema changes
- **Triggers**: Automated timestamp management
- **Details**: See [IMPLEMENTATION-DATABASE.md](IMPLEMENTATION-DATABASE.md)

## Security Best Practices

### Development Security
- **Environment Files**: Never commit `.env` files
- **Secrets Management**: Use environment variables
- **Development Data**: Use separate test credentials
- **Code Reviews**: Security-focused review process

### Production Security
- **Secret Generation**: Cryptographically secure secrets
- **Monitoring**: Log analysis for security events
- **Updates**: Regular dependency updates
- **Backups**: Encrypted database backups

### Incident Response
- **Token Revocation**: Immediate revocation capability
- **User Deactivation**: Account disable functionality
- **Audit Logging**: Track security-relevant events
- **Recovery**: Documented recovery procedures

## Testing Security

### Security Test Coverage
- **Authentication**: Login/registration edge cases
- **Authorization**: Permission boundary testing
- **Input Validation**: Malformed input handling
- **Token Security**: Expiration and validation tests
- **Details**: See [IMPLEMENTATION-TESTING.md](IMPLEMENTATION-TESTING.md#backend-testing)

---

*For security-related API contracts, see [IMPLEMENTATION-DATA-CONTRACTS.md](IMPLEMENTATION-DATA-CONTRACTS.md#authentication-contracts). For deployment security, see [IMPLEMENTATION-DEPLOYMENT.md](IMPLEMENTATION-DEPLOYMENT.md#security-configuration).*
