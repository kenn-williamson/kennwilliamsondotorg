# CLAUDE.md - Project Context

## Project Overview
**KennWilliamson.org** is a production-deployed full-stack web application with:
- **Frontend**: Nuxt.js 4.0.3 (Vue 3 + SSR + TypeScript + TailwindCSS)
- **Backend**: Rust 1.90.0 + Actix-web 4.x + PostgreSQL 17.0
- **Development**: Docker Compose + Hot Reload Environment
- **Infrastructure**: Nginx reverse proxy with SSL
- **Deployment**: Live at kennwilliamson.org

**Current State**: Production application with complete feature set including authentication, OAuth, email verification, incident timers, phrases system, admin panel, GDPR/CCPA compliance, 3-layer architecture refactor, and comprehensive test coverage (~620 tests total: ~445 backend + 175 frontend).

## Quick Start
```bash
# Start entire development environment
./scripts/dev-start.sh

# OR start local production environment for production testing
./scripts/setup-local-prod.sh

# View logs and monitor services
./scripts/dev-logs.sh

# Access application at https://localhost
```

## Key Features
- **Authentication**: JWT + refresh token system with session management and OAuth (Google)
- **Email Verification**: Token-based email verification with role-based access control
- **Password Reset**: Secure token-based password reset with expiration tracking
- **Incident Timers**: Full CRUD operations with public sharing and steampunk UI
- **Phrases System**: Random motivational phrases with user suggestions, filtering, and full-text search
- **Admin Panel**: User management, phrase moderation, RBAC, and system statistics
- **Profile Management**: Display name, slug editing, password changes, and account settings
- **Data Privacy**: Self-service account deletion and data export (GDPR/CCPA compliant)
- **Email Suppression**: AWS SES compliance with bounce/complaint/unsubscribe handling
- **Public Display**: Public timer and phrase endpoints (no auth required)
- **Testing**: ~620 tests (~445 backend unit/integration + 175 frontend) with paradigm-based documentation
- **Architecture**: 3-layer backend (API/Service/Repository) with dependency injection
- **Frontend Architecture**: Stores with embedded actions for SSR hydration
- **Responsive UI**: Mobile-first design with TailwindCSS and steampunk aesthetics
- **Route Protection**: Middleware-based authentication with role-based access (RBAC)
- **Hot Reload**: Instant updates for both frontend and backend

# Core Documentation (Loaded in Memory)
- @ARCHITECTURE.md - System architecture and service design
- @CODING-RULES.md - Development standards and conventions
- @DEVELOPMENT-WORKFLOW.md - Common development workflows

## Implementation Documentation (Reference as Needed)
- IMPLEMENTATION-AUTH.md - Authentication system design
- IMPLEMENTATION-BACKEND.md - Rust backend implementation
- IMPLEMENTATION-DATA-CONTRACTS.md - API request/response schemas
- IMPLEMENTATION-DATABASE.md - PostgreSQL schema and migrations
- IMPLEMENTATION-FRONTEND.md - Nuxt.js frontend implementation
- IMPLEMENTATION-NGINX.md - Reverse proxy configuration
- IMPLEMENTATION-SCRIPTS.md - Development automation scripts
- IMPLEMENTATION-TESTING.md - Testing implementation and patterns
- IMPLEMENTATION-UTILS.md - Development utilities

## Additional Documentation
- UX-LAYOUT.md - Design system and responsive breakpoints
- ROADMAP.md - Future development priorities

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