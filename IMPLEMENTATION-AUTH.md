# Authentication Implementation

## Overview
JWT-based authentication system with email/password registration and secure session management. Integrates with [IMPLEMENTATION-FRONTEND.md](IMPLEMENTATION-FRONTEND.md), [IMPLEMENTATION-BACKEND.md](IMPLEMENTATION-BACKEND.md), and [IMPLEMENTATION-DATABASE.md](IMPLEMENTATION-DATABASE.md).

## Authentication Flow

### Registration Process
1. Frontend form validation (VeeValidate + Yup)
2. Password hashing with bcrypt (cost factor 12)
3. User creation in database with UUIDv7 and auto-generated slug
4. JWT token generation and return
5. Frontend authentication state update

### Login Process
1. Frontend login form submission
2. Backend credential verification against database
3. Password hash comparison with bcrypt
4. JWT token generation with user claims
5. Frontend authentication state update and redirect

### Session Management
- **Token Storage**: Client-side authentication state management
- **Token Expiration**: 24-hour default
- **Logout**: Token invalidation and state clearing

## Security Implementation

### Password Security
- **Hashing**: bcrypt with cost factor 12
- **Validation**: VeeValidate with Yup schemas on frontend
- **Storage**: Only hashed passwords stored in database

### JWT Security
- **Secret**: Environment variable configuration
- **Algorithm**: HS256 (HMAC with SHA-256)
- **Claims**: User ID, email, roles array, timestamps
- **Validation**: Signature verification on protected routes

## Database Integration
- User table with email uniqueness constraint
- Role-based authorization with user and admin roles
- UUIDv7 primary keys for performance
- Automated timestamp triggers for audit tracking

For complete schema details, see [IMPLEMENTATION-DATABASE.md](IMPLEMENTATION-DATABASE.md).

## API Endpoints

### Authentication Endpoints
- `POST /api/auth/register` - User registration with slug generation
- `POST /api/auth/login` - User login with JWT response
- `POST /api/auth/preview-slug` - Real-time slug preview for registration

### Protected Routes
- JWT validation middleware for `/api/incident-timers/*`
- Automatic user context injection for protected handlers
- Role-based authorization ready for future admin features

## Frontend Integration

### Authentication Store (Pinia)
- User state management with authentication status
- Login/register operations with error handling
- Token management and logout functionality

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