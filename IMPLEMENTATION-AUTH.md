# User Authentication System Design

## Overview
JWT-based authentication system with email/password registration, secure session management, and future OAuth integration. Cross-references **IMPLEMENTATION-FRONTEND.md**, **IMPLEMENTATION-BACKEND.md**, and **IMPLEMENTATION-DATABASE.md**.

## Authentication Flow

### Registration Process
1. Frontend form validation (email format, password strength)
2. Backend receives registration request
3. Password hashing with bcrypt (high cost factor)
4. User creation in database with UUIDv7
5. JWT token generation and return
6. Frontend stores token in httpOnly cookie

### Login Process
1. Frontend login form submission
2. Backend credential verification against database
3. Password hash comparison with bcrypt
4. JWT token generation with user claims
5. Token returned to frontend via httpOnly cookie
6. Frontend redirects to protected area

### Session Management
- **Token Storage**: httpOnly cookies (XSS protection)
- **Token Expiration**: 24-hour default, configurable
- **Refresh Strategy**: Silent refresh before expiration
- **Logout**: Token invalidation, cookie clearing

## Security Architecture

### Password Security
- **Hashing**: bcrypt with salt rounds (cost factor 12+)
- **Requirements**: Minimum 8 characters, complexity rules
- **Storage**: Only hashed passwords in database

### JWT Security
- **Secret**: Strong random secret (environment variable)
- **Algorithm**: HS256 (HMAC with SHA-256)
- **Claims**: User ID, email, roles array, issued/expiry timestamps
- **Validation**: Signature verification on every request

### Cookie Security
- **httpOnly**: Prevents JavaScript access
- **Secure**: HTTPS-only transmission
- **SameSite**: CSRF protection
- **Path**: Scoped to application routes

## Database Schema Integration
- User table with email uniqueness constraint
- Password hash storage (see **IMPLEMENTATION-DATABASE.md**)
- Role-based authorization with roles and user_roles tables
- Default roles: 'user' (standard permissions), 'admin' (full permissions)
- Optional session tracking table for enhanced security
- OAuth provider linkage tables (future)

## API Endpoints

### Authentication Endpoints
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login
- `POST /api/auth/refresh` - Token refresh
- `POST /api/auth/logout` - Session termination
- `GET /api/auth/me` - Current user info (protected)

### Protected Routes
- **Frontend**: Middleware-based route protection
- **Backend**: JWT validation middleware
- **Redirection**: Automatic login page redirect for unauthenticated

## Frontend Integration

### State Management
- Auth store with user state and authentication status
- Reactive authentication checks
- Automatic token refresh handling

### Route Protection
- Navigation guards for protected pages
- Conditional rendering based on auth state
- Login/logout UI components

### Form Handling
- Client-side validation with server confirmation
- Error message display and handling
- Loading states during authentication

## Backend Integration

### Middleware
- JWT validation middleware for protected routes
- Request enrichment with user context and roles
- Role-based route protection (admin-only endpoints)
- Error handling for invalid/expired tokens

### User Context
- User information injection into request handlers
- Role-based permission checking utilities
- Secure user data retrieval with role information

## Future OAuth Integration (Phase 2)

### OAuth Providers
- **Google OAuth 2.0**: Social login option
- **GitHub OAuth**: Developer-focused authentication
- **Account Linking**: Connect OAuth to existing email accounts

### OAuth Flow
1. Frontend OAuth button redirects to provider
2. Provider authentication and consent
3. Backend receives OAuth callback with code
4. Token exchange with provider for user info
5. User matching/creation in database
6. JWT token generation for application session
7. Account linking for existing users

### Database Extensions
- OAuth provider table linking
- Multiple authentication methods per user
- Provider-specific user data storage

## Security Considerations

### Attack Prevention
- **Brute Force**: Rate limiting on login endpoints
- **CSRF**: SameSite cookies and origin validation
- **XSS**: httpOnly cookies, input sanitization
- **Injection**: Parameterized queries, input validation

### Monitoring & Auditing
- Failed login attempt tracking
- Successful authentication logging
- Suspicious activity detection
- Account lockout mechanisms (future)

## Error Handling
- **Invalid Credentials**: Generic error messages
- **Account Not Found**: Same response as invalid credentials
- **Token Expiry**: Automatic refresh or login redirect
- **Server Errors**: Generic error without sensitive information

## Testing Strategy
- **Unit Tests**: Password hashing, JWT generation/validation
- **Integration Tests**: Full authentication flow
- **Security Tests**: Token manipulation, unauthorized access
- **E2E Tests**: Complete user registration and login flows