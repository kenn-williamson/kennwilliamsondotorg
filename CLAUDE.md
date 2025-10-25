# CLAUDE.md - Project Context

## Business Purpose

**KennWilliamson.org** serves multiple strategic goals:

1. **Portfolio & Demonstration**: Production full-stack application showcasing professional deployment skills (Rust + Nuxt.js + PostgreSQL + Docker + AWS)
2. **Learning Platform**: Exploring AI-assisted development in unfamiliar stack (Rust/Nuxt) while applying strong architectural knowledge
3. **Public Platform**: Personal testimony and community engagement through incident timers and motivational phrases
4. **Creative Outlet**: Personal project with steampunk aesthetic and unique features

**Deployment**: Live production site at kennwilliamson.org

## Users & Needs

**Primary Audiences:**
- General public wanting to know Kenn
- Potential employers, clients, collaborators
- Community participants (sharing incident timers, contributing phrases)
- Personal network (friends, potential partners)

**Key User Constraints:**
- Public visibility requires professional polish and performance
- SEO critical for discoverability
- Community features require trust and data privacy compliance
- Mobile access expected (responsive design essential)

## Technology Stack Decisions

### Core Technologies
- **Frontend**: Nuxt.js 4.0.3 (Vue 3 + SSR + TypeScript)
- **Backend**: Rust 1.90.0 + Actix-web 4.x
- **Database**: PostgreSQL 17.0
- **Infrastructure**: Docker Compose + Nginx + AWS

### Why Nuxt.js SSR
**Decision**: Server-side rendering with Nuxt.js

**Why**: SEO requirement for public discoverability. Search engines need server-rendered HTML for portfolio and testimony content to be indexed.

**Alternative rejected**: Client-side SPA (React/Vue) - insufficient SEO for public-facing content.

### Why Rust Backend
**Decision**: Rust + Actix-web for API

**Why**:
- Learning goal: AI-assisted development in unfamiliar systems language
- Performance: Stateless Rust API handles high throughput with minimal resources
- Type safety: Catches errors at compile-time (important for learning new stack)

**Trade-off**: Steeper learning curve vs. Node.js, but superior performance and safety.

### Why PostgreSQL
**Decision**: PostgreSQL 17 over alternatives

**Why**: Full-text search, JSONB flexibility, ACID compliance, mature ecosystem. See [IMPLEMENTATION-DATABASE.md](IMPLEMENTATION-DATABASE.md#technology-stack-decisions) for detailed rationale.

## Architectural Decisions

### Hybrid API Architecture
**Decision**: Two API patterns - `/api/*` SSR proxy vs `/backend/*` direct

**Why**:
- **SEO**: Nuxt SSR renders initial page load (search engine indexing)
- **Security/UX**: `/api/*` leverages stateful Nuxt for session handling and cookie management
- **Performance**: `/backend/*` leverages stateless Rust speed for mutations and API calls

**Trade-off**: Architectural complexity vs. optimizing each use case appropriately.

See [ARCHITECTURE.md](ARCHITECTURE.md#hybrid-api-architecture) for implementation details.

### 3-Layer Backend Architecture
**Decision**: API → Service → Repository pattern

**Why**: Testability critical for AI-assisted development. Clear separation enables unit testing with mocks, integration testing with real database, and confidence when exploring unfamiliar patterns.

See [IMPLEMENTATION-BACKEND.md](IMPLEMENTATION-BACKEND.md#architecture-decisions) for detailed patterns and rationale.

### Normalized Auth Schema
**Decision**: Split user data across 5 tables (users, credentials, external_logins, profiles, preferences)

**Why**:
- Multi-provider OAuth support (future-proofing)
- GDPR/CCPA compliance (clear data boundaries)
- Maintainability (auth changes don't affect profile)

See [IMPLEMENTATION-DATABASE.md](IMPLEMENTATION-DATABASE.md#normalized-auth-architecture) for schema design decisions.

## Constraints & Requirements

### Budget: AWS t3.small (2GB RAM)
**Impact**: All resource allocation decisions driven by 2GB RAM constraint
- Connection pooling: Limited to 10 database connections
- PostgreSQL optimization: Tuned for 2GB environment
- Stateless design: Enables horizontal scaling without memory penalties

See [ARCHITECTURE.md](ARCHITECTURE.md#resource-management) for resource targets.

### GDPR/CCPA Compliance
**Impact**: Data privacy requirements drive architecture
- Self-service account deletion and data export
- Email suppression list (AWS SES compliance)
- Clear data boundaries in normalized schema
- Audit logging for compliance demonstration

### Performance Targets
**Impact**: Fast response times despite resource constraints
- Stateless Rust API for throughput
- Connection pooling and query optimization
- SSR for initial page load, client hydration for interactivity

### Learning Goals
**Impact**: Exploring AI-assisted development patterns
- 3-layer architecture for testability and confidence
- Comprehensive test coverage (~620 tests)
- Documentation-driven development for AI context

## What Makes This Unique

- **Steampunk Aesthetic**: Personal creative expression in UI design
- **Public Incident Timers**: Community transparency feature (share "days since incident")
- **AI-Assisted Production**: Real-world exploration of Claude-assisted development
- **Hybrid Platform**: Portfolio + testimony + community engagement in one application
- **Normalized Auth**: Multi-table architecture unusual for small projects (learning investment)

## Core Documentation

**Read First** (strategic context and patterns):
- **ARCHITECTURE.md**: System architecture and design decisions
- **CODING-RULES.md**: Development standards and conventions
- **DEVELOPMENT-WORKFLOW.md**: Daily development workflows and scripts

**Implementation Details** (decision rationale by component):
- **IMPLEMENTATION-BACKEND.md**: Rust backend architecture decisions
- **IMPLEMENTATION-FRONTEND.md**: Nuxt.js frontend architecture decisions
- **IMPLEMENTATION-DATABASE.md**: PostgreSQL schema design decisions
- **IMPLEMENTATION-SECURITY.md**: Authentication and security decisions
- **IMPLEMENTATION-TESTING.md**: Testing strategy and paradigms
- **IMPLEMENTATION-NGINX.md**: Reverse proxy configuration decisions
- **IMPLEMENTATION-SCRIPTS.md**: Development automation scripts
- **IMPLEMENTATION-DEPLOYMENT.md**: Production deployment decisions
- **IMPLEMENTATION-LOGGING.md**: Logging and observability decisions
- **IMPLEMENTATION-UTILS.md**: Development utilities

**Additional Documentation**:
- **UX-LAYOUT.md**: Design system and responsive breakpoints
- **ROADMAP.md**: Future development priorities

**Note**: IMPLEMENTATION-* documents follow "Decision/Why/Alternatives/Trade-offs" pattern. Focus on decisions and rationale, not current code structure (code is discoverable).

## Development Context

### AI Assistant Usage
When working on this project, use Context7 MCP for framework documentation:
- **Nuxt.js**: `resolve-library-id` with "nuxt.js" for SSR and routing patterns
- **Actix-web**: `resolve-library-id` with "actix-web" for Rust web framework patterns
- **PostgreSQL**: `resolve-library-id` with "postgresql" for database operations
- **Docker**: `resolve-library-id` with "docker" for containerization

### Essential Development Practices

**CRITICAL: ALWAYS use development scripts - NEVER use Docker commands directly**

**Common Development Scripts:**
- `./scripts/dev-start.sh` - Start all services (not `docker-compose up`)
- `./scripts/dev-stop.sh` - Stop all services (not `docker-compose down`)
- `./scripts/dev-logs.sh` - View service logs (not `docker logs`)
- `./scripts/health-check.sh` - Verify system health
- `./scripts/setup-db.sh` - Run migrations (preserves data, safe to run repeatedly)
- `./scripts/reset-db.sh` - Fresh database start (DESTRUCTIVE)
- `./scripts/prepare-sqlx.sh` - Update SQLx cache after SQL query changes
- `./scripts/backup-db.sh` - Backup/restore database

**Why scripts matter:**
- Consistent interface across all developers
- Handle environment detection automatically
- Proper service ordering and error handling
- User-friendly output with progress indicators

**See [IMPLEMENTATION-SCRIPTS.md](IMPLEMENTATION-SCRIPTS.md) for complete script reference**

**Other Essentials:**
- **Environment**: `.env.development` for all development work
- **Testing**: `cargo test -- --test-threads=4` (prevents Docker resource exhaustion)
- **Standards**: Follow [CODING-RULES.md](CODING-RULES.md) for language-specific conventions

See [DEVELOPMENT-WORKFLOW.md](DEVELOPMENT-WORKFLOW.md) for comprehensive workflow guidance.

## Project History

**Recent Major Update** (January 2025): Auth schema refactor completed (Phases 0-9). Migrated from monolithic `users` table to normalized multi-table architecture for multi-provider OAuth support and GDPR/CCPA compliance enhancement.
