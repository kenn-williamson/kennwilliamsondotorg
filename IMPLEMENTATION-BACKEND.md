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
│   ├── repositories/     # Repository layer (3-layer architecture)
│   │   ├── mod.rs        # Repository exports
│   │   ├── traits/       # Repository trait definitions
│   │   │   ├── mod.rs
│   │   │   ├── user_repository.rs
│   │   │   ├── refresh_token_repository.rs
│   │   │   ├── incident_timer_repository.rs
│   │   │   └── phrase_repository.rs
│   │   ├── postgres/     # PostgreSQL implementations
│   │   │   ├── mod.rs
│   │   │   ├── postgres_user_repository.rs
│   │   │   ├── postgres_refresh_token_repository.rs
│   │   │   ├── postgres_incident_timer_repository.rs
│   │   │   └── postgres_phrase_repository.rs
│   │   └── mocks/        # Mock implementations for testing
│   │       ├── mod.rs
│   │       ├── mock_user_repository.rs
│   │       ├── mock_refresh_token_repository.rs
│   │       ├── mock_incident_timer_repository.rs
│   │       └── mock_phrase_repository.rs
│   ├── routes/           # API route handlers
│   │   ├── mod.rs        # Route configuration
│   │   ├── auth.rs       # Registration/login endpoints
│   │   ├── incident_timers.rs # CRUD + public endpoints
│   │   ├── phrases.rs    # Phrase management endpoints
│   │   └── admin.rs      # Admin user management and phrase moderation endpoints
│   ├── services/         # Business logic layer
│   │   ├── mod.rs        # Service exports
│   │   ├── container.rs  # ServiceContainer for dependency injection
│   │   ├── auth.rs       # JWT + password validation
│   │   ├── incident_timer.rs # Timer business logic
│   │   ├── phrase.rs     # Phrase and suggestion business logic
│   │   └── admin/        # Admin services
│   │       ├── mod.rs    # Admin service exports
│   │       ├── user_management.rs # User management operations
│   │       ├── phrase_moderation.rs # Phrase suggestion moderation
│   │       └── stats.rs  # System statistics
│   ├── middleware/       # Custom middleware
│   │   ├── mod.rs        # Middleware exports
│   │   └── auth.rs       # JWT validation with role extraction
│   └── utils/            # Utility functions
├── migrations/           # SQLx migrations
│   ├── 20250914134643_initial_schema.sql
│   ├── 20250914134654_add_refresh_tokens_and_user_active.sql
│   └── 20250914134703_add_phrases_system.sql
├── tests/               # Integration tests
│   ├── refresh_token_validation.rs # Refresh token validation tests
│   └── test_helpers.rs      # Database utilities for testing
├── Cargo.toml           # Dependencies
├── Dockerfile           # Multi-stage container build
└── .env                 # Environment configuration
```

## Core Features
- **Authentication**: JWT-based with refresh tokens (see [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md#authentication-system))
- **User Management**: Registration, login, slug generation, profile updates
- **Profile Management**: Display name and slug editing, password changes
- **Admin Management**: User deactivation, password reset, user promotion, system statistics
- **Incident Timers**: CRUD operations with user ownership
- **Phrases System**: Dynamic phrases with user suggestions and admin approval
- **Public API**: Unauthenticated access to user timers and phrases
- **Database Integration**: SQLx with compile-time query verification
- **3-Layer Architecture**: Clean separation with repository pattern and dependency injection
- **Testing**: 20 unit tests for repository layer, integration tests in progress

## Architecture Patterns

### 3-Layer Architecture
- **API Layer**: HTTP handlers in `routes/` directory
- **Service Layer**: Business logic in `services/` directory using repository traits
- **Repository Layer**: Data access in `repositories/` directory with trait-based design

### Repository Pattern
- **Traits**: Abstract interfaces for all data operations
- **PostgreSQL Implementations**: Concrete implementations using SQLx
- **Mock Implementations**: Complete mock implementations for unit testing
- **Dependency Injection**: ServiceContainer manages all dependencies

### Service Layer
- **Repository Dependencies**: All services use repository traits instead of direct database access
- **Business Logic**: Clean separation from data access concerns
- **Error Handling**: Consistent error responses across endpoints
- **Testing**: Easy unit testing with mock repositories

### API Layer
- **Route Handlers**: Use service layer exclusively
- **Middleware**: JWT validation via `actix_web::middleware::from_fn()`
- **Route Structure**: Public routes (no auth) and protected routes (JWT required) with clear separation
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

### Current Test Status
- **Repository Layer**: 20 unit tests passing for all mock implementations
- **Integration Tests**: 2 test files remaining (refresh_token_validation.rs, test_helpers.rs)
- **Service Layer**: Unit tests needed for business logic
- **API Layer**: Integration tests needed for all endpoints

### Repository Testing
- **Mock Implementations**: Complete test coverage for all repository traits
- **Unit Tests**: Fast, isolated testing without database dependencies
- **Error Handling**: Comprehensive error scenario testing
- **Coverage**: All CRUD operations and helper methods tested

### Service Testing (In Progress)
- **Mock Dependencies**: Services use mock repositories for unit testing
- **Business Logic**: Focused testing of service layer logic
- **Error Scenarios**: Testing error handling and validation

### Integration Testing (In Progress)
- **API Endpoints**: Full request/response cycle testing
- **Database Integration**: Real database operations with test data
- **Authentication**: Complete auth flow testing

See [IMPLEMENTATION-TESTING.md](IMPLEMENTATION-TESTING.md#backend-testing) for detailed testing implementation.

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