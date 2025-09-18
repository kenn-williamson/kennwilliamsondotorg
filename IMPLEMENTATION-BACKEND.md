# Backend Implementation

## Overview
Full-featured REST API backend built with Rust and Actix-web framework, featuring PostgreSQL integration, JWT authentication, Docker containerization, and comprehensive test coverage.

## Technology Stack
- **Language**: Rust 1.89.0 (stable)
- **Framework**: Actix-web 4.x
- **Database**: SQLx with PostgreSQL 17 + UUIDv7
- **Authentication**: JWT with bcrypt password hashing (cost 12)
- **Serialization**: Serde for JSON handling
- **Environment**: dotenv for configuration
- **Testing**: Integration tests with actix-test + comprehensive endpoint coverage
- **Logging**: env_logger with structured logging

## Project Structure
```
backend/
├── src/
│   ├── main.rs           # Application entry point
│   ├── models/           # Database models
│   │   ├── mod.rs        # Module exports
│   │   ├── user.rs       # User model + auth requests
│   │   ├── incident_timer.rs # Incident timer model
│   │   └── phrase.rs     # Phrase and suggestion models
│   ├── routes/           # API route handlers
│   │   ├── mod.rs        # Route configuration
│   │   ├── auth.rs       # Registration/login endpoints
│   │   ├── incident_timers.rs # CRUD + public endpoints
│   │   ├── phrases.rs    # Phrase management endpoints
│   │   └── admin.rs      # Admin-only endpoints
│   ├── services/         # Business logic
│   │   ├── mod.rs        # Service exports
│   │   ├── auth.rs       # JWT + password validation
│   │   ├── incident_timer.rs # Timer business logic
│   │   └── phrase.rs     # Phrase and suggestion business logic
│   ├── middleware/       # Custom middleware
│   │   ├── mod.rs        # Middleware exports
│   │   └── auth.rs       # JWT validation with role extraction
│   └── utils/            # Utility functions
├── migrations/           # SQLx migrations
│   ├── 20250914134643_initial_schema.sql
│   ├── 20250914134654_add_refresh_tokens_and_user_active.sql
│   └── 20250914134703_add_phrases_system.sql
├── tests/               # Integration tests
│   ├── auth_simple.rs       # Authentication endpoint tests (3 tests)
│   ├── incident_simple.rs   # Incident timer endpoint tests (5 tests)
│   ├── health_simple.rs     # Health endpoint tests (2 tests)
│   └── test_helpers.rs      # Database utilities for testing
├── Cargo.toml           # Dependencies
├── Dockerfile           # Multi-stage container build
└── .env                 # Environment configuration
```

## Current Features
- **Authentication System**: JWT-based auth with registration, login, and role-based middleware
- **Refresh Token System**: Rolling refresh tokens with 1-week expiration and 6-month hard limit
- **User Management**: User creation with bcrypt password hashing and automatic slug generation
- **Slug System**: Real-time slug preview endpoint with collision handling
- **Incident Timer CRUD**: Complete create, read, update, delete operations for authenticated users
- **Public API**: User slug-based public timer access (no authentication required)
- **Phrases System**: Dynamic phrase management with user suggestions and admin approval workflow
- **Phrase Exclusion System**: User-controlled phrase filtering
- **Admin Phrase Management**: Admin endpoints for phrase CRUD and suggestion approval/rejection
- **Database Integration**: PostgreSQL with SQLx, UUIDv7 primary keys, automated timestamp triggers
- **Security**: JWT validation, password hashing, SHA-256 hashed refresh tokens
- **Testing**: Integration tests with proper isolation
- **Route Architecture**: Actix-web routing with selective middleware application

## Architecture Highlights
- **Middleware**: Uses `actix_web::middleware::from_fn()` for JWT validation
- **Role-Based Auth**: Middleware extracts user + roles for authorization
- **Routing**: Single `/backend` scope with selective middleware application
- **Route Structure**: Sub-scopes prevent middleware duplication

## API Endpoints

### Public Endpoints (No Authentication Required)
- `GET /backend/health` - Health check
- `GET /backend/health/db` - Database connectivity check
- `POST /backend/auth/register` - User registration
- `POST /backend/auth/login` - User login
- `POST /backend/auth/refresh` - Token refresh using refresh token
- `POST /backend/auth/preview-slug` - Slug preview for registration form
- `GET /backend/{user_slug}/incident-timer` - Get latest timer by user slug
- `GET /backend/{user_slug}/phrase` - Get random phrase for public display

### Protected Endpoints (JWT Authentication Required)
- `GET /backend/auth/me` - Get current user profile information
- `POST /backend/auth/revoke` - Revoke specific refresh token
- `POST /backend/auth/revoke-all` - Revoke all user's refresh tokens
- `POST /backend/incident-timers` - Create new timer
- `GET /backend/incident-timers` - List current user's timers
- `PUT /backend/incident-timers/{id}` - Update timer entry
- `DELETE /backend/incident-timers/{id}` - Delete timer entry

### Phrases Endpoints (JWT Authentication Required)
- `GET /backend/phrases/random` - Get random phrase for authenticated user
- `GET /backend/phrases/user` - Get all phrases with user exclusion status
- `POST /backend/phrases/exclude/{id}` - Exclude phrase from user's feed
- `DELETE /backend/phrases/exclude/{id}` - Remove phrase exclusion
- `POST /backend/phrases/suggestions` - Submit phrase suggestion
- `GET /backend/phrases/suggestions` - Get user's phrase suggestions

### Admin Endpoints (Admin Role Required)
- `GET /backend/admin/phrases` - Get all phrases (admin view)
- `POST /backend/admin/phrases` - Create new phrase (admin only)
- `PUT /backend/admin/phrases/{id}` - Update phrase (admin only)
- `DELETE /backend/admin/phrases/{id}` - Deactivate phrase (admin only)
- `GET /backend/admin/suggestions` - Get all pending suggestions (admin only)
- `POST /backend/admin/suggestions/{id}/approve` - Approve suggestion (admin only)
- `POST /backend/admin/suggestions/{id}/reject` - Reject suggestion (admin only)

For detailed API contracts, see [IMPLEMENTATION-DATA-CONTRACTS.md](IMPLEMENTATION-DATA-CONTRACTS.md).

## Development Environment

### Installation Requirements
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add useful tools
cargo install cargo-watch  # Auto-reload during development
cargo install sqlx-cli     # Database migrations
```

### Environment Configuration
Located in `backend/.env`:
- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - JWT token signing secret
- `RUST_LOG` - Logging level configuration
- Additional variables for CORS, host, port settings

### Running the Backend
The backend is typically run through development scripts:
```bash
# Start with hot reload (recommended)
./scripts/dev-start.sh backend

# View backend logs
./scripts/dev-logs.sh backend

# Direct cargo commands (if needed)
cd backend
cargo run
cargo test
```

## Testing

### Test Architecture
Integration test suite covering all endpoints:
- **Authentication Tests**: Registration, login, credential validation
- **Timer Tests**: CRUD operations, public access, ownership validation
- **Health Tests**: Service and database connectivity

### Running Tests
```bash
# Run all tests (sequential for database isolation)
cargo test -- --test-threads 1

# Run specific test categories
cargo test --test auth_simple
cargo test --test incident_simple
cargo test --test health_simple
```

For detailed testing information, see [IMPLEMENTATION-TESTING.md](IMPLEMENTATION-TESTING.md).

## Database Integration

### Connection Management
- SQLx connection pooling for efficient database access
- UUIDv7 primary keys for better performance and ordering
- Automated timestamp triggers for consistent `updated_at` handling

### Migration Management
Database schema managed through SQLx migrations:
```bash
# Run migrations
./scripts/setup-db.sh

# Create new migration
cd backend && sqlx migrate add migration_name
```

For detailed database information, see [IMPLEMENTATION-DATABASE.md](IMPLEMENTATION-DATABASE.md).

## Security Implementation

### Authentication & Authorization
- JWT tokens with 1-hour expiration
- Rolling refresh tokens with 1-week expiration and 6-month hard limit
- bcrypt password hashing with cost factor 12
- Role-based authorization middleware

### Refresh Token Security
- **Secure Storage**: SHA-256 hashed tokens in database (never plaintext)
- **Rolling Expiration**: Each refresh generates new JWT + new refresh token
- **Automatic Cleanup**: Expired tokens automatically removed from database
- **Multiple Device Support**: Separate refresh tokens per login session
- **Simple Revocation**: Individual token revocation or revoke-all functionality

### Input Validation & Security
- Request/response validation with Serde
- SQL injection prevention through SQLx parameterized queries
- CORS configuration for cross-origin requests

### Error Handling
- Structured error responses with appropriate HTTP status codes
- Security-conscious error messages (no information leakage)

## Docker Configuration

### Multi-Stage Build
- Build stage: Rust compilation with dependency caching
- Runtime stage: Minimal Alpine Linux container
- Security: Non-root user execution
- Health checks: Built-in container health monitoring

---

*This document describes the current backend implementation. For future enhancements and planned features, see [ROADMAP.md](ROADMAP.md).*