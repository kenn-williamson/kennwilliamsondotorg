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
│   │   └── incident_timer.rs # Incident timer model
│   ├── routes/           # API route handlers
│   │   ├── mod.rs        # Route configuration
│   │   ├── auth.rs       # Registration/login endpoints
│   │   └── incident_timers.rs # CRUD + public endpoints
│   ├── services/         # Business logic
│   │   ├── mod.rs        # Service exports
│   │   ├── auth.rs       # JWT + password validation
│   │   └── incident_timer.rs # Timer business logic
│   ├── middleware/       # Custom middleware
│   │   ├── mod.rs        # Middleware exports
│   │   └── auth.rs       # JWT validation with role extraction
│   └── utils/            # Utility functions
├── migrations/           # SQLx migrations
│   ├── 20250829024919_create_users_table.sql
│   ├── 20250829025210_create_roles_table.sql
│   ├── 20250829095648_add_user_slug_to_users.sql
│   └── 20250829095731_create_incident_timers_table.sql
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
- **Authentication System**: Full JWT-based auth with registration, login, and role-based middleware
- **User Management**: User creation with bcrypt password hashing and role assignment + automatic slug generation
- **Slug System**: Real-time slug preview endpoint with collision handling and URL-safe generation
- **Incident Timer CRUD**: Complete create, read, update, delete operations for authenticated users
- **Public API**: User slug-based public timer access (no authentication required)
- **Database Integration**: PostgreSQL with SQLx, UUIDv7 primary keys, automated timestamp triggers
- **Security**: Proper JWT validation, password hashing, role extraction middleware
- **Testing**: Comprehensive integration tests (11 tests) with fast execution and proper isolation
- **Clean Route Architecture**: Idiomatic Actix-web routing with selective middleware application
- **Request Logging**: Comprehensive request logging middleware for debugging

## Architecture Highlights
- **Modern Middleware**: Uses `actix_web::middleware::from_fn()` for clean JWT validation
- **Role-Based Auth**: Middleware extracts user + roles for authorization
- **Idiomatic Routing**: Single `/api` scope with selective middleware application
- **Clean Route Structure**: Sub-scopes prevent middleware duplication
- **Request Logging**: Built-in debugging with comprehensive request/response logging
- **Production Ready**: Proper error handling, logging, and security best practices

## API Endpoints

### Public Endpoints (No Authentication Required)
- `GET /health` - Health check
- `GET /health/db` - Database connectivity check
- `POST /auth/register` - User registration
- `POST /auth/login` - User login
- `POST /auth/preview-slug` - Slug preview for registration form
- `GET /backend/{user_slug}/incident-timer` - Get latest timer by user slug

### Protected Endpoints (JWT Authentication Required)
- `POST /backend/incident-timers` - Create new timer
- `GET /backend/incident-timers` - List current user's timers
- `PUT /backend/incident-timers/{id}` - Update timer entry
- `DELETE /backend/incident-timers/{id}` - Delete timer entry

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
Comprehensive integration test suite with 11 tests covering all endpoints:
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
- JWT tokens with secure signing and validation
- bcrypt password hashing with appropriate cost factor
- Role-based authorization middleware for future admin features

### Input Validation & Security
- Request/response validation with Serde
- SQL injection prevention through SQLx parameterized queries
- CORS configuration for cross-origin requests
- Comprehensive input sanitization

### Error Handling
- Structured error responses with appropriate HTTP status codes
- Security-conscious error messages (no information leakage)
- Comprehensive logging for debugging without exposing sensitive data

## Docker Configuration

### Multi-Stage Build
The Dockerfile implements efficient multi-stage building:
- Build stage: Rust compilation with dependency caching
- Runtime stage: Minimal Alpine Linux container
- Security: Non-root user execution
- Health checks: Built-in container health monitoring

### Container Integration
Designed to work seamlessly with Docker Compose development environment and production deployment.

---

*This document describes the current backend implementation. For future enhancements and planned features, see [ROADMAP.md](ROADMAP.md).*