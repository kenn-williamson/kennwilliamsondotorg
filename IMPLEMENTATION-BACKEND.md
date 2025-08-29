# Backend Implementation Plan - Rust + Actix-web

## Overview
Create REST API backend using Rust and Actix-web framework with PostgreSQL integration, JWT authentication, and Docker containerization.

## Technology Stack
- **Language**: Rust 1.70+ (latest stable)
- **Framework**: Actix-web 4.x
- **Database**: SQLx with PostgreSQL driver
- **Authentication**: JWT with bcrypt password hashing
- **Serialization**: Serde for JSON handling
- **Environment**: dotenv for configuration
- **Testing**: Tokio-test + sqlx-test

## Rust Installation
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version

# Add useful tools
cargo install cargo-watch  # Auto-reload during development
cargo install sqlx-cli     # Database migrations
```

## Project Structure
```
backend/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library root
│   ├── config.rs         # Configuration management
│   ├── routes/           # API route handlers
│   │   ├── mod.rs
│   │   ├── auth.rs       # Authentication endpoints
│   │   ├── users.rs      # User management
│   │   └── health.rs     # Health check
│   ├── models/           # Database models
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── auth.rs
│   ├── services/         # Business logic
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   └── user.rs
│   ├── middleware/       # Custom middleware
│   │   ├── mod.rs
│   │   ├── auth.rs       # JWT validation
│   │   └── cors.rs       # CORS handling
│   └── utils/            # Utility functions
│       ├── mod.rs
│       ├── crypto.rs     # Password hashing
│       └── jwt.rs        # JWT utilities
├── migrations/           # Database migrations
├── tests/               # Integration tests
├── Cargo.toml           # Dependencies
├── Dockerfile           # Container build
└── .env.example         # Environment template
```

## Key Dependencies (Cargo.toml)
```toml
[dependencies]
actix-web = "4"
actix-cors = "0.7"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "migrate"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
jsonwebtoken = "9"
bcrypt = "0.15"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
dotenv = "0.15"
env_logger = "0.11"
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
actix-rt = "2"
sqlx-test = "0.7"
```

## Setup Commands
```bash
# Create new Rust project
cargo new backend --name backend
cd backend

# Add dependencies (update Cargo.toml with above deps)
cargo check

# Install database CLI
cargo install sqlx-cli

# Setup database (requires PostgreSQL running)
sqlx database create
sqlx migrate add create_users_table
```

## Core Features
- **REST API**: JSON endpoints with proper HTTP status codes
- **Authentication**: Registration, login, JWT token management
- **Database**: PostgreSQL with SQLx query builder
- **Validation**: Request/response validation with custom errors
- **Security**: CORS, rate limiting, input sanitization
- **Logging**: Structured logging with env_logger
- **Health Checks**: Database connectivity and service status

## API Endpoints
```
POST   /api/auth/register    # User registration
POST   /api/auth/login       # User login
POST   /api/auth/refresh     # Token refresh
GET    /api/auth/me          # Get current user (protected)
GET    /api/health           # Health check
GET    /api/users            # List users (protected)
```

## Database Integration
- PostgreSQL connection via SQLx with connection pooling
- Database schema and migrations managed separately (see **IMPLEMENTATION-DATABASE.md**)
- Async database operations with compile-time query checking
- Transaction support for complex operations

## Docker Configuration
```dockerfile
# Multi-stage build for optimization
FROM rust:1.70-alpine AS builder

WORKDIR /app
RUN apk add --no-cache musl-dev openssl-dev

# Build dependencies first (cache layer)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Build application
COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM alpine:latest
RUN apk add --no-cache ca-certificates
WORKDIR /app

COPY --from=builder /app/target/release/backend /app/
COPY migrations ./migrations

USER 1000:1000
EXPOSE 8080

CMD ["./backend"]
```

## Environment Variables
```env
# Database
DATABASE_URL=postgresql://user:password@localhost/kennwilliamson

# Authentication
JWT_SECRET=your-super-secret-jwt-key
JWT_EXPIRES_IN=24h

# Server
HOST=0.0.0.0
PORT=8080
RUST_LOG=backend=info,actix_web=info

# CORS (development)
CORS_ORIGIN=http://localhost:3000
```

## Development Workflow
```bash
# Run with auto-reload
cargo watch -x run

# Run tests
cargo test

# Database migrations
sqlx migrate run

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Security Considerations
- **Password Hashing**: bcrypt with proper cost factor
- **JWT Security**: Secure secret, proper expiration
- **Input Validation**: Sanitize all user inputs
- **CORS**: Restrict origins in production
- **Rate Limiting**: Prevent brute force attacks
- **SQL Injection**: SQLx compile-time query checking

## Performance Optimizations
- **Connection Pooling**: SQLx connection pool
- **Async Operations**: Full async/await with Tokio
- **JSON Streaming**: Efficient serialization with Serde
- **Docker Optimization**: Multi-stage build for smaller images
- **Memory Management**: Rust's zero-cost abstractions