# Backend Implementation

## Overview
Rust-based REST API using Actix-web 4.x with PostgreSQL integration and comprehensive test coverage.

## Technology Stack Decisions

### Core Technologies
- **Language**: Rust 1.90.0
  - Why: Type safety, performance, fearless concurrency
- **Framework**: Actix-web 4.x
  - Why: High performance, async by default, mature ecosystem
- **Database**: SQLx with PostgreSQL
  - Why: Compile-time query verification, async support
- **Serialization**: Serde
  - Why: Industry standard, zero-copy deserialization
- **Testing**: actix-test + testcontainers
  - Why: Integration tests with real database, full request/response cycle testing

## Architecture Decisions

### 3-Layer Architecture
Clean separation of concerns with testability via dependency injection:

**API Layer (routes/)**
- HTTP request/response handling
- Middleware application (auth, RBAC)
- Route scoping: public / protected / admin
- Minimal business logic - delegates to services

**Service Layer (services/)**
- Business logic and orchestration
- Depends on repository traits (not concrete implementations)
- Modular design: Split large services into focused modules
- Error handling and validation

**Repository Layer (repositories/)**
- Data access abstraction
- Trait-based design for testability
- PostgreSQL implementations for production
- Mock implementations for unit testing

**Why 3 layers:**
- Testability: Services testable with mock repositories
- Maintainability: Changes to data layer don't cascade
- Flexibility: Can swap database implementations
- Clear boundaries: Each layer has single responsibility

### Repository Pattern
Trait-based abstraction for all data operations:

**Design:**
- `traits/` directory defines interfaces
- `postgres/` directory implements for production
- `mocks/` directory implements for testing
- Services depend on traits, not concrete types

**Benefits:**
- Unit test services without database
- Swap implementations (e.g., Redis cache layer)
- Compile-time verification of dependencies
- Clear contracts between layers

**Trade-offs:**
- More upfront code (trait + postgres + mock)
- Worth it: Enables comprehensive unit testing

### Service Container Pattern
Centralized dependency injection for services:

**Purpose:**
- Manage service lifecycle and configuration
- Inject dependencies based on environment
- Abstract repository creation from routes

**Environments:**
- Development: Real repositories + logging
- Testing: Mock repositories + no cleanup service
- Production: Real repositories + background tasks

**Why:**
- Single place to wire up dependencies
- Easy environment-specific configuration
- Simplifies route handlers (just extract from container)

### Normalized Auth Schema
Multi-table user authentication architecture:

**Design:**
- `users`: Core identity (id, email, active, verified)
- `user_credentials`: Password auth
- `user_external_logins`: OAuth providers
- `user_profiles`: Display data
- `user_preferences`: Settings

**Why:**
- Multi-provider OAuth: Users can link multiple providers
- GDPR compliance: Clear data boundaries
- Maintainability: Changes to one auth method don't affect others
- Extensibility: Add new providers without schema changes

**Trade-offs:**
- More joins for full user data
- Worth it: Flexibility and compliance benefits

### Modular Service Design
Large services split into focused submodules:

**Pattern:**
- `auth/auth_service/` contains: register, login, refresh, profile, password, oauth
- `incident_timer/` contains: create, read, update, delete
- `phrase/` contains: public_access, user_management, admin_management
- `admin/` contains: user_management, phrase_moderation, stats

**Why:**
- Easier navigation: Find auth registration logic in `register.rs`
- Smaller files: Each module <200 lines
- Clear boundaries: Each module has single responsibility
- Testability: Test modules independently

## Security Architecture
See [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md) for authentication, authorization, and security decisions.

## Testing Strategy
See [IMPLEMENTATION-TESTING.md](IMPLEMENTATION-TESTING.md) for testing philosophy and paradigm-based approach.

## Database Integration
See [IMPLEMENTATION-DATABASE.md](IMPLEMENTATION-DATABASE.md) for schema decisions and migration strategy.

## Key Patterns

### Token Cleanup Service
**Decision**: Background task removes expired tokens automatically

**Why:**
- Prevents unbounded database growth
- Maintains query performance
- No manual maintenance required

**Trade-offs:**
- Additional background process
- Worth it: Set it and forget it

### Route Scoping
**Decision**: Three-tier route organization (public / protected / admin)

**Why:**
- Clear security boundaries
- Middleware applied at scope level
- Self-documenting API structure

### Error Handling
**Decision**: Services return `Result<T, anyhow::Error>`, routes map to HTTP status codes

**Why:**
- Services don't know about HTTP
- Routes handle HTTP concerns
- Clear error propagation

## Development Environment

### Hot Reload
Use development scripts for automatic rebuilds:
```bash
./scripts/dev-start.sh backend
```

### Database Migrations
SQLx manages schema migrations with compile-time verification:
```bash
./scripts/setup-db.sh              # Run migrations
./scripts/prepare-sqlx.sh --clean  # Update query cache
```

## Docker Configuration
Multi-stage build for minimal production images:
- Compilation stage: Full Rust toolchain
- Runtime stage: Alpine with just the binary
- Non-root user for security
- Health check endpoints for orchestration
