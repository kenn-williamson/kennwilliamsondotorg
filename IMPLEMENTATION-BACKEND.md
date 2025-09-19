# Backend Implementation

## Overview
Rust-based REST API using Actix-web 4.x with PostgreSQL integration and comprehensive test coverage.

## Technology Stack
- **Language**: Rust 1.89.0
- **Framework**: Actix-web 4.x
- **Database**: SQLx with PostgreSQL
- **Serialization**: Serde
- **Environment**: dotenv
- **Testing**: actix-test
- **Logging**: env_logger

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
│   ├── auth_simple.rs       # Authentication endpoint tests (4 tests)
│   ├── refresh_token_tests.rs # Refresh token tests (4 tests)
│   ├── incident_simple.rs   # Incident timer endpoint tests (5 tests)
│   ├── health_simple.rs     # Health endpoint tests (2 tests)
│   └── test_helpers.rs      # Database utilities for testing
├── Cargo.toml           # Dependencies
├── Dockerfile           # Multi-stage container build
└── .env                 # Environment configuration
```

## Core Features
- **Authentication**: JWT-based with refresh tokens (see [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md#authentication-system))
- **User Management**: Registration, login, slug generation, profile updates
- **Profile Management**: Display name and slug editing, password changes
- **Incident Timers**: CRUD operations with user ownership
- **Phrases System**: Dynamic phrases with user suggestions and admin approval
- **Public API**: Unauthenticated access to user timers and phrases
- **Database Integration**: SQLx with compile-time query verification
- **Testing**: Comprehensive integration test suite

## Architecture Patterns
- **Middleware**: JWT validation via `actix_web::middleware::from_fn()`
- **Routing**: Single `/backend` scope with sub-scopes for middleware control
- **Service Layer**: Business logic separated from route handlers
- **Error Handling**: Consistent error responses across endpoints

## API Endpoints

### API Endpoints
For complete endpoint documentation and request/response contracts, see [IMPLEMENTATION-DATA-CONTRACTS.md](IMPLEMENTATION-DATA-CONTRACTS.md#api-endpoints).

For authentication and security details, see [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md#api-security).

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
Configuration via `backend/.env`. See [IMPLEMENTATION-DEPLOYMENT.md](IMPLEMENTATION-DEPLOYMENT.md#environment-configuration) for production setup.

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
Comprehensive integration test suite with 15 tests covering all endpoints. See [IMPLEMENTATION-TESTING.md](IMPLEMENTATION-TESTING.md#backend-testing) for details.

## Database Integration
- **Connection**: SQLx with connection pooling
- **Migrations**: Managed via SQLx CLI
- **Schema**: See [IMPLEMENTATION-DATABASE.md](IMPLEMENTATION-DATABASE.md)
- **Query Safety**: Compile-time SQL verification

## Security
See [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md) for authentication, authorization, and security implementation details.

## Docker Configuration
- **Multi-Stage Build**: Compilation + minimal Alpine runtime
- **Security**: Non-root user execution
- **Health Checks**: Built-in monitoring endpoints
- **Optimization**: Dependency caching for faster rebuilds