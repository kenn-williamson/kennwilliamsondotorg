# Backend Implementation

## Overview
Rust-based REST API using Actix-web 4.x with PostgreSQL integration and comprehensive test coverage. The backend has been refactored with a modular service architecture, enhanced admin capabilities, and comprehensive testing infrastructure.

## Technology Stack
- **Language**: Rust 1.90.0
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
│   ├── lib.rs            # Library exports
│   ├── models/           # Database models
│   │   ├── mod.rs        # Module exports
│   │   ├── api/          # API models
│   │   │   └── user.rs   # User API models
│   │   ├── oauth/        # OAuth models
│   │   │   ├── mod.rs
│   │   │   └── google_user_info.rs # Google OAuth user info
│   │   ├── user.rs       # User model + auth requests
│   │   ├── incident_timer.rs # Incident timer model
│   │   └── phrase.rs     # Phrase and suggestion models
│   ├── repositories/     # Repository layer (3-layer architecture)
│   │   ├── mod.rs        # Repository exports
│   │   ├── traits/       # Repository trait definitions
│   │   │   ├── mod.rs
│   │   │   ├── user_repository.rs
│   │   │   ├── refresh_token_repository.rs
│   │   │   ├── verification_token_repository.rs
│   │   │   ├── incident_timer_repository.rs
│   │   │   ├── phrase_repository.rs
│   │   │   └── admin_repository.rs
│   │   ├── postgres/     # PostgreSQL implementations
│   │   │   ├── mod.rs
│   │   │   ├── postgres_user_repository.rs
│   │   │   ├── postgres_refresh_token_repository.rs
│   │   │   ├── postgres_verification_token_repository.rs
│   │   │   ├── postgres_incident_timer_repository.rs
│   │   │   ├── postgres_phrase_repository.rs
│   │   │   └── postgres_admin_repository.rs
│   │   └── mocks/        # Mock implementations for testing
│   │       ├── mod.rs
│   │       ├── mock_user_repository.rs
│   │       ├── mock_refresh_token_repository.rs
│   │       ├── mock_verification_token_repository.rs
│   │       ├── mock_incident_timer_repository.rs
│   │       ├── mock_phrase_repository.rs
│   │       └── mock_admin_repository.rs
│   ├── routes/           # API route handlers
│   │   ├── mod.rs        # Route configuration with public/protected/admin scoping
│   │   ├── auth.rs       # Authentication endpoints
│   │   ├── incident_timers.rs # Timer CRUD + public endpoints
│   │   ├── phrases.rs    # Phrase management endpoints
│   │   ├── admin.rs      # Admin user management and phrase moderation endpoints
│   │   └── health.rs     # Health check endpoints
│   ├── services/         # Business logic layer
│   │   ├── mod.rs        # Service exports
│   │   ├── container.rs  # ServiceContainer for dependency injection
│   │   ├── auth/         # Authentication services (modularized)
│   │   │   ├── mod.rs    # Auth service exports
│   │   │   ├── jwt.rs    # JWT token management
│   │   │   ├── oauth/    # OAuth service abstraction
│   │   │   │   ├── mod.rs
│   │   │   │   ├── google_oauth_service.rs # Google OAuth implementation
│   │   │   │   └── mock_google_oauth_service.rs # Mock for testing
│   │   │   └── auth_service/ # Auth service modules
│   │   │       ├── mod.rs
│   │   │       ├── register.rs # User registration
│   │   │       ├── login.rs    # User login
│   │   │       ├── refresh_token.rs # Token refresh logic
│   │   │       ├── profile.rs  # Profile management
│   │   │       ├── password.rs # Password operations
│   │   │       ├── slug.rs     # Username slug generation
│   │   │       ├── email_verification.rs # Email verification logic
│   │   │       └── oauth.rs    # Google OAuth PKCE flow
│   │   ├── email/         # Email service abstraction
│   │   │   ├── mod.rs     # EmailService trait
│   │   │   ├── mock_email_service.rs # Mock for testing
│   │   │   └── ses_email_service.rs  # AWS SES implementation
│   │   ├── cleanup/       # Token cleanup service
│   │   │   └── mod.rs     # CleanupService for expired tokens
│   │   ├── incident_timer/ # Timer business logic (modularized)
│   │   │   ├── mod.rs
│   │   │   ├── create.rs
│   │   │   ├── read.rs
│   │   │   ├── update.rs
│   │   │   └── delete.rs
│   │   ├── phrase/       # Phrase and suggestion business logic (modularized)
│   │   │   ├── mod.rs
│   │   │   ├── public_access.rs
│   │   │   ├── user_management.rs
│   │   │   ├── admin_management.rs
│   │   │   ├── exclusions.rs
│   │   │   └── suggestions.rs
│   │   └── admin/        # Admin services (modularized)
│   │       ├── mod.rs    # Admin service exports
│   │       ├── user_management/ # User management service modules
│   │       │   └── mod.rs # UserManagementService + 7 unit tests
│   │       ├── phrase_moderation/ # Phrase moderation service modules
│   │       │   └── mod.rs # PhraseModerationService + 5 unit tests
│   │       └── stats/    # Statistics service modules
│   │           └── mod.rs # StatsService + 3 unit tests
│   ├── middleware/       # Custom middleware
│   │   ├── mod.rs        # Middleware exports
│   │   ├── auth.rs       # JWT validation with role extraction
│   │   └── admin.rs      # Admin role validation middleware
│   └── utils/            # Utility functions
├── migrations/           # SQLx migrations
│   ├── 20250914134643_initial_schema.sql
│   ├── 20250914134654_add_refresh_tokens_and_user_active.sql
│   └── 20250914134703_add_phrases_system.sql
├── tests/               # Test suite
│   ├── mod.rs           # Test module organization
│   ├── test_helpers.rs  # Test utilities
│   ├── refresh_token_validation.rs # Refresh token validation tests
│   └── api/             # API endpoint tests
│       ├── mod.rs
│       ├── testcontainers_auth_api_tests.rs # Authentication API tests
│       ├── testcontainers_incident_timer_api_tests.rs # Incident timer API tests
│       ├── testcontainers_phrase_api_tests.rs # Phrase API tests
│       ├── testcontainers_admin_api_tests.rs # Admin API tests
│       └── testcontainers_health_api_tests.rs # Health API tests
├── Cargo.toml           # Dependencies
├── Dockerfile           # Multi-stage container build
└── .env                 # Environment configuration
```

## Core Features
- **Authentication**: JWT-based with refresh tokens (see [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md#authentication-system))
- **OAuth Integration**: Google OAuth 2.0 with PKCE flow and Redis-backed state management
- **Email Verification**: Token-based email verification with role-based access control
- **Password Reset**: Secure token-based password reset with expiration and usage tracking
- **Token Cleanup**: Automatic background cleanup of expired tokens (refresh + verification)
- **User Management**: Registration, login, slug generation, profile updates
- **Profile Management**: Display name and slug editing, password changes
- **Account Management**: Self-service account deletion and data export (GDPR/CCPA compliance)
- **Email Suppression**: AWS SES compliance with bounce/complaint/unsubscribe handling
- **Admin Management**: User deactivation, password reset, user promotion, system statistics
- **Incident Timers**: CRUD operations with user ownership and public sharing
- **Phrases System**: Dynamic phrases with user suggestions and admin approval
- **Enhanced Search**: PostgreSQL full-text search with `ts_rank` ranking and ILIKE fallback
- **Public API**: Unauthenticated access to user timers and phrases
- **Database Integration**: SQLx with compile-time query verification
- **3-Layer Architecture**: Clean separation with repository pattern and dependency injection
- **Modular Services**: Auth service split into focused modules (register, login, refresh, profile, password, slug, email_verification, oauth)
- **Modular Incident Timer Service**: Split into focused modules (create, read, update, delete)
- **Modular Phrase Service**: Split into focused modules (public_access, user_management, admin_management, exclusions, suggestions)
- **Modular Admin Services**: Split into focused modules (user_management, phrase_moderation, stats)
- **Background Tasks**: Scheduled cleanup service for token expiration management
- **Route Scoping**: Public/protected/admin route organization with appropriate middleware
- **Testing**: See [IMPLEMENTATION-TESTING.md](IMPLEMENTATION-TESTING.md) for comprehensive testing details

## Architecture Patterns

### 3-Layer Architecture
- **API Layer**: HTTP handlers in `routes/` directory with public/protected/admin scoping
- **Service Layer**: Business logic in `services/` directory using repository traits
- **Repository Layer**: Data access in `repositories/` directory with trait-based design

### Repository Pattern
- **Traits**: Abstract interfaces for all data operations
- **PostgreSQL Implementations**: Concrete implementations using SQLx
- **Mock Implementations**: Complete mock implementations for unit testing
- **Dependency Injection**: ServiceContainer manages all dependencies
- **Admin Repository**: Dedicated repository for admin operations and system statistics
- **Search Implementation**: PostgreSQL full-text search with `ts_rank` ranking and ILIKE fallback for phrase search

### Service Layer
- **Repository Dependencies**: All services use repository traits instead of direct database access
- **Business Logic**: Clean separation from data access concerns
- **Error Handling**: Consistent error responses across endpoints
- **Testing**: Easy unit testing with mock repositories
- **Modular Design**: Auth service split into focused modules for maintainability
- **Admin Services**: Dedicated service layer for administrative operations

### API Layer
- **Route Handlers**: Use service layer exclusively
- **Middleware**: JWT validation and admin role validation via `actix_web::middleware::from_fn()`
- **Route Structure**: Three-tier scoping (public/protected/admin) with appropriate middleware
- **Error Handling**: Consistent error responses across endpoints
- **Service Injection**: Services injected via ServiceContainer for dependency management

### Service Container Pattern
- **Dependency Injection**: Centralized service management and configuration
- **Environment Awareness**: Development, testing, and production configurations
- **Repository Abstraction**: Services depend on traits, not concrete implementations
- **Testing Support**: Easy switching between real and mock implementations

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

See [IMPLEMENTATION-TESTING.md](IMPLEMENTATION-TESTING.md) for comprehensive testing documentation.

## Token Cleanup Service

### Overview
Automated background service that removes expired tokens from the database to prevent unbounded growth and maintain system performance.

### Architecture
- **Service**: `CleanupService` in `services/cleanup/mod.rs`
- **Repository Dependencies**: Uses `RefreshTokenRepository` and `VerificationTokenRepository` traits
- **Background Task**: Spawned via `actix_web::rt::spawn` on application startup
- **Scheduling**: Uses `actix_web::rt::time::interval` for periodic execution

### Behavior
- **Startup**: Runs immediately on application startup to clean any existing expired tokens
- **Interval**: Runs every N hours (configurable via `CLEANUP_INTERVAL_HOURS`, default: 24)
- **Target**: Cleans both refresh tokens and verification tokens in a single operation
- **Logging**: Logs cleanup results (count of deleted tokens) at INFO level

### Configuration
```bash
# .env.development
CLEANUP_INTERVAL_HOURS=24  # Run cleanup every 24 hours
```

### Implementation Details
```rust
// Spawned in main.rs on startup
actix_web::rt::spawn(async move {
    let mut interval = actix_web::rt::time::interval(
        std::time::Duration::from_secs(cleanup_interval_hours * 3600)
    );

    loop {
        interval.tick().await;  // First tick completes immediately
        // Cleanup logic...
    }
});
```

### Testing
- **Unit Tests**: 5 tests with mock repositories (service creation, repository calls, error handling)
- **Integration Tests**: 5 tests with testcontainers (expired refresh tokens, verification tokens, both types, no expired tokens, empty database)

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