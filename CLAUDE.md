# CLAUDE.md - Project Context

## Project Overview
**KennWilliamson.org** is a fully functional full-stack web application with:
- **Frontend**: Nuxt.js 4.0.3 (Vue 3 + SSR + TypeScript + TailwindCSS)  
- **Backend**: Rust 1.89.0 + Actix-web 4.x + PostgreSQL 17
- **Development**: Docker Compose + Hot Reload Environment
- **Infrastructure**: Nginx reverse proxy with SSL

**Current State**: Complete end-to-end application with comprehensive development tooling.

## Quick Start
```bash
# Start entire development environment
./scripts/dev-start.sh

# View logs and monitor services
./scripts/dev-logs.sh

# Access application at https://localhost
```

## Key Features
- **Authentication**: JWT-based register/login system
- **Incident Timers**: Full CRUD operations with public sharing
- **Responsive UI**: Mobile-first design with TailwindCSS
- **Route Protection**: Middleware-based authentication
- **Hot Reload**: Instant updates for both frontend and backend

# Architecture & Implementation Documentation
- @ARCHITECTURE.md - System architecture and service design
- @UX-LAYOUT.md - Design system and responsive breakpoints  
- @IMPLEMENTATION-AUTH.md - Authentication system design
- @IMPLEMENTATION-BACKEND.md - Rust backend implementation
- @IMPLEMENTATION-DATA-CONTRACTS.md - API request/response schemas
- @IMPLEMENTATION-DATABASE.md - PostgreSQL schema and migrations
- @IMPLEMENTATION-FRONTEND.md - Nuxt.js frontend implementation
- @IMPLEMENTATION-NGINX.md - Reverse proxy configuration
- @IMPLEMENTATION-SCRIPTS.md - Development automation scripts
- @IMPLEMENTATION-TESTING.md - Testing implementation and patterns
- @IMPLEMENTATION-UTILS.md - Development utilities
- @ROADMAP.md - Future development priorities
- @CODING-RULES.md - Development standards and conventions
- @DEVELOPMENT-WORKFLOW.md - Common development workflows

## Development Context7 Usage
When working on this project, use Context7 MCP to look up framework documentation:
- **Nuxt.js**: `resolve-library-id` with "nuxt.js" for SSR and routing
- **Actix-web**: `resolve-library-id` with "actix-web" for Rust web framework
- **PostgreSQL**: `resolve-library-id` with "postgresql" for database operations
- **Docker**: `resolve-library-id` with "docker" for containerization

## Essential Rules
- Always use development scripts instead of manual Docker commands
- Use `.env.development` for all development work
- Check service health after major changes with `./scripts/health-check.sh`
- Run migrations safely with `./scripts/setup-db.sh` (preserves data)
- Update SQLx cache with `./scripts/prepare-sqlx.sh` after SQL changes
- Follow [CODING-RULES.md](CODING-RULES.md) for development standards