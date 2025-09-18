# Authentication Implementation

## Overview
JWT-based authentication system with email/password registration and secure session management using Nuxt Auth Utils. Integrates with [IMPLEMENTATION-FRONTEND.md](IMPLEMENTATION-FRONTEND.md), [IMPLEMENTATION-BACKEND.md](IMPLEMENTATION-BACKEND.md), and [IMPLEMENTATION-DATABASE.md](IMPLEMENTATION-DATABASE.md).

## Authentication Flow

### Registration Process
1. Frontend form validation (VeeValidate + Yup)
2. Backend password hashing with bcrypt (cost factor 12)
3. User creation in database with UUIDv7 and auto-generated slug
4. JWT token + refresh token generation and return
5. Frontend session state update via Nuxt Auth Utils

### Login Process
1. Frontend login form submission
2. Backend credential verification against database
3. Password hash comparison with bcrypt
4. JWT token + refresh token generation with minimal claims
5. Frontend session state update and redirect

### Session Management
- **JWT Storage**: In-memory via Nuxt session (`session.secure.jwtToken`)
- **Refresh Token Storage**: Server-side session via Nuxt Auth Utils (`session.secure.refreshToken`)
- **JWT Expiration**: 1-hour with automatic refresh
- **Refresh Token Expiration**: 1-week rolling tokens
- **Hard Limit**: 6-month maximum token age
- **Logout**: Token revocation and session clearing

## Security Implementation

### Password Security
- **Hashing**: bcrypt with cost factor 12
- **Validation**: VeeValidate with Yup schemas on frontend
- **Storage**: Only hashed passwords stored in database

### JWT Security
- **Secret**: Environment variable configuration
- **Algorithm**: HS256 (HMAC with SHA-256)
- **Claims**: Minimal payload (user ID only) for performance
- **Expiration**: 1-hour with automatic refresh
- **Validation**: Signature verification on protected routes

### Refresh Token Security
- **Storage**: SHA-256 hashed in database, never plaintext
- **Rolling**: Each refresh generates new JWT + new refresh token
- **Expiration**: 1-week rolling tokens
- **Hard Limit**: 6-month maximum age from creation
- **Device Support**: Optional device_info JSONB field
- **Clean Revocation**: Tokens deleted immediately on refresh

## Database Integration
- User table with email uniqueness constraint
- Role-based authorization with user and admin roles
- UUIDv7 primary keys for performance
- Automated timestamp triggers for audit tracking
- **Refresh tokens table**: Stores hashed tokens with user association and expiration

For complete schema details, see [IMPLEMENTATION-DATABASE.md](IMPLEMENTATION-DATABASE.md).

## API Endpoints

### Authentication Endpoints
- `POST /backend/auth/register` - User registration with slug generation
- `POST /backend/auth/login` - User login with JWT response
- `POST /backend/auth/preview-slug` - Real-time slug preview for registration
- `POST /backend/auth/refresh` - Token refresh using refresh token
- `GET /backend/auth/me` - Get current user profile (protected)
- `POST /backend/auth/revoke` - Revoke specific refresh token (protected)
- `POST /backend/auth/revoke-all` - Revoke all user's refresh tokens (protected)

### Protected Routes
- JWT validation middleware for `/backend/incident-timers/*`, `/backend/phrases/*`, `/backend/admin/*`
- Automatic user context injection for protected handlers
- Role-based authorization for admin features

## Frontend Integration

### Session Management (Nuxt Auth Utils)
- User state management via `useUserSession()`
- Login/register operations with error handling
- Automatic token refresh via server-side API routes
- Session-based logout functionality

### Route Protection
- Middleware-based authentication for protected pages
- Automatic redirect to login for unauthenticated users
- Conditional navigation based on authentication state

### Form Implementation
- Registration form with real-time slug preview
- Login form with validation and error display
- VeeValidate integration with Yup validation schemas

## Backend Integration

### JWT Middleware
- Token validation for protected routes
- User context extraction and injection
- Role-based authorization support
- Comprehensive error handling for invalid tokens

### Authentication Service
- User registration with bcrypt password hashing
- Login credential verification
- JWT token generation with proper claims
- Rolling refresh token management
- Token revocation and cleanup

## Security Considerations

### Attack Prevention
- **SQL Injection**: Parameterized queries via SQLx
- **XSS**: Input sanitization and validation
- **CSRF**: Proper origin validation
- **Password Security**: bcrypt with appropriate cost factor

### Error Handling
- **Invalid Credentials**: Generic error messages prevent information leakage
- **Account Not Found**: Same response as invalid credentials
- **Validation Errors**: Clear frontend validation with server confirmation
- **Server Errors**: Generic error responses without sensitive information

## Testing Coverage

### Integration Tests
- User registration flow with unique email validation
- Login authentication with credential verification
- JWT token validation for protected endpoints
- Error handling for invalid authentication attempts

For complete test details, see [IMPLEMENTATION-TESTING.md](IMPLEMENTATION-TESTING.md).

---

*For future OAuth integration plans, see [ROADMAP.md](ROADMAP.md).*