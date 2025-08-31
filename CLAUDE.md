# CLAUDE.md - Project Context

## Project Overview
**KennWilliamson.org** is a full-stack web application featuring:
- **Frontend**: Nuxt.js 4.0.3 (Vue 3 + SSR + TypeScript + TailwindCSS)
- **Backend**: Rust 1.89.0 + Actix-web 4.x + PostgreSQL 17
- **Development**: Docker Compose + Hot Reload Environment
- **Infrastructure**: Nginx proxy + AWS EC2 deployment ready

**Current State**: ✅ **Fully Functional** - Complete end-to-end application with hot reload development environment.

## Current Architecture
```
kennwilliamsondotorg/
├── PROJECT_HISTORY.md       # Completed phases and implementation history
├── ARCHITECTURE.md          # System architecture and deployment details
├── CLAUDE.md               # Current project context (this file)
├── frontend/               # Nuxt.js 4.0.3 application (✅ Complete)
│   └── app/               # Nuxt 4 directory structure
│       ├── components/    # Vue components
│       ├── pages/        # File-based routing
│       ├── stores/       # Pinia state management
│       ├── services/     # API service layer
│       └── middleware/   # Route protection
├── backend/               # Rust + Actix-web API (✅ Production Ready)
│   ├── src/              # Rust source code
│   ├── tests/            # Integration test suite (11 tests)
│   └── migrations/       # Database migrations (4 applied)
├── scripts/               # Development automation (✅ Complete)
│   ├── dev-start.sh      # Service management
│   ├── dev-stop.sh       # Service shutdown
│   ├── dev-logs.sh       # Log viewing
│   ├── setup-db.sh       # Database migrations
│   ├── prepare-sqlx.sh   # SQLx cache management
│   └── health-check.sh   # Service monitoring
├── nginx/                # Reverse proxy + SSL
├── docker-compose.yml    # Production configuration
└── docker-compose.development.yml # Hot reload environment
```

## Development Environment

### 🚀 **Hot Reload Environment (Ready)**
The project includes a complete hot reload development environment that mimics production:

**Features**:
- **Frontend HMR**: Vue/TypeScript changes update instantly without page refresh
- **Backend Hot Reload**: Rust code changes trigger automatic recompilation
- **SSL Development**: Self-signed HTTPS certificates (`https://localhost`)
- **Production Routing**: Nginx proxy eliminates CORS issues
- **Automated Scripts**: Use scripts instead of manual docker commands

### Quick Start
```bash
# Start entire development environment with hot reload
./scripts/dev-start.sh

# View logs
./scripts/dev-logs.sh

# Stop services
./scripts/dev-stop.sh

# Access application
# Frontend: https://localhost (recommended - nginx proxy)
# Direct: http://localhost:3000 (fallback)
```

## Development Scripts Documentation

### Core Scripts (Always Use These)

#### `./scripts/dev-start.sh` - Service Management
**Purpose**: Start, build, or restart development services  
**When to use**: Instead of manual docker-compose commands

```bash
# Basic usage
./scripts/dev-start.sh                    # Start all services
./scripts/dev-start.sh --logs            # Start and follow logs

# Build options
./scripts/dev-start.sh --build           # Force rebuild all
./scripts/dev-start.sh --rebuild backend # Force recreate backend only
./scripts/dev-start.sh --no-cache frontend # Rebuild frontend without cache

# Service-specific
./scripts/dev-start.sh backend           # Start only backend
./scripts/dev-start.sh postgres frontend # Start only specified services
```

#### `./scripts/dev-logs.sh` - Log Viewing
**Purpose**: View service logs with filtering and formatting  
**When to use**: To monitor service behavior and debug issues

```bash
# Basic usage
./scripts/dev-logs.sh                    # Follow all services
./scripts/dev-logs.sh backend           # Follow specific service
./scripts/dev-logs.sh nginx --tail 20   # Show last 20 lines

# Options
./scripts/dev-logs.sh --no-follow       # Show logs and exit
./scripts/dev-logs.sh --timestamps      # Include timestamps
```

#### `./scripts/dev-stop.sh` - Service Shutdown
**Purpose**: Clean shutdown of development services  
**When to use**: To stop services cleanly

```bash
./scripts/dev-stop.sh                   # Stop all services
./scripts/dev-stop.sh --remove          # Stop and remove containers
./scripts/dev-stop.sh backend          # Stop specific service
```

### Database Scripts

#### `./scripts/setup-db.sh` - Safe Migrations
**Purpose**: Run database migrations safely (preserves data)  
**When to use**: After schema changes, new developer setup

```bash
./scripts/setup-db.sh                  # Run pending migrations
./scripts/setup-db.sh --verify         # Run migrations + verify schema
```

#### `./scripts/prepare-sqlx.sh` - SQLx Cache
**Purpose**: Generate SQLx query cache for Docker builds  
**When to use**: After changing SQL queries, before Docker builds

```bash
./scripts/prepare-sqlx.sh              # Generate cache
./scripts/prepare-sqlx.sh --clean      # Clean + regenerate
```

### Health & Maintenance Scripts

#### `./scripts/health-check.sh` - Service Monitoring
**Purpose**: Verify all services are healthy and responsive  
**When to use**: After builds, deployments, debugging issues

```bash
./scripts/health-check.sh              # Check all services
./scripts/health-check.sh --wait       # Wait up to 60s for startup
./scripts/health-check.sh --service postgres # Check specific service
```

## Daily Development Workflows

### Starting Work
```bash
# 1. Start development environment
./scripts/dev-start.sh

# 2. Verify everything is healthy (optional)
./scripts/health-check.sh

# 3. Open https://localhost and start coding
# - Frontend changes update instantly (HMR)
# - Backend changes trigger automatic rebuild
```

### After Database Schema Changes
```bash
# 1. Run new migrations
./scripts/setup-db.sh

# 2. Update SQLx cache for Docker builds
./scripts/prepare-sqlx.sh --clean

# 3. Restart backend to pick up changes
./scripts/dev-start.sh --rebuild backend
```

### When Things Break
```bash
# Nuclear option - reset everything
./scripts/dev-stop.sh --remove
./scripts/dev-start.sh --build

# Or check what's wrong first
./scripts/health-check.sh
./scripts/dev-logs.sh backend --tail 50
```

### Debugging Issues
```bash
# View logs for specific service
./scripts/dev-logs.sh nginx           # Web server issues
./scripts/dev-logs.sh backend        # API issues  
./scripts/dev-logs.sh frontend       # Build issues

# Check service health
./scripts/health-check.sh --service backend

# Test direct service access
curl http://localhost:8080/api/health    # Backend API
curl http://localhost:3000               # Frontend direct
```

## Current Features

### Working Applications
- **Authentication**: JWT-based register/login with secure tokens
- **Incident Timers**: Full CRUD operations with real-time display
- **Public Sharing**: Timer URLs at `/{user_slug}/incident-timer`
- **Responsive UI**: Works on mobile, tablet, desktop
- **Route Protection**: Protected pages require authentication

### API Endpoints (All Working)
```
# Public
GET  /health                    # Service health
GET  /{user_slug}/incident-timer # Public timer display

# Authentication (Public)
POST /api/auth/register         # User registration
POST /api/auth/login           # User login

# Timers (Protected - requires JWT)
GET    /api/incident-timers     # User's timers
POST   /api/incident-timers     # Create timer
PUT    /api/incident-timers/{id} # Update timer
DELETE /api/incident-timers/{id} # Delete timer
```

## Technology Stack Details

### Frontend Stack
- **Framework**: Nuxt.js 4.0.3 with Vue 3 Composition API
- **Styling**: TailwindCSS with custom design system
- **Forms**: VeeValidate + Yup for validation
- **State**: Pinia stores with proper SSR handling
- **Types**: TypeScript with strict mode

### Backend Stack
- **Language**: Rust 1.89.0 with async/await
- **Framework**: Actix-web 4.x with middleware
- **Database**: PostgreSQL 17 + UUIDv7 + SQLx
- **Auth**: JWT + bcrypt password hashing
- **Tests**: 11 integration tests covering all endpoints

### Development Stack
- **Containers**: Docker + Docker Compose
- **Hot Reload**: Cargo-watch (backend) + Vite HMR (frontend)
- **Proxy**: Nginx with self-signed SSL
- **Environment**: `.env.development` with debug logging

## Context7 MCP Server Usage
When working on this project, use the Context7 MCP server to look up APIs and documentation:

### Framework Documentation
1. **Nuxt.js**: Use `resolve-library-id` with "nuxt.js" for SSR, routing, and framework features
2. **Rust/Actix-web**: Use `resolve-library-id` with "actix-web" for web framework documentation  
3. **PostgreSQL**: Use `resolve-library-id` with "postgresql" for database operations
4. **Docker**: Use `resolve-library-id` with "docker" for containerization

## Important Rules & Conventions

### Development Workflow Rules
1. **Always use development scripts** instead of manual docker commands
2. **Use `.env.development`** for all development work
3. **Check service health** after major changes
4. **Run migrations safely** with `setup-db.sh` (preserves data)
5. **Update SQLx cache** after SQL query changes

### Commit Conventions
- **[FEATURE]**: New features and enhancements
- **[FIX]**: Bug fixes and corrections
- **[CHORE]**: Maintenance, documentation, and tooling
- **[REFACTOR]**: Code restructuring without functional changes

### Code Standards
- **TypeScript**: Strict mode, proper typing
- **Rust**: Follow Rust conventions, comprehensive error handling
- **Vue**: Composition API with `<script setup>` syntax
- **Testing**: Add tests for new API endpoints

# Architecture & Implementation Documentation
- @ARCHITECTURE.md - System design, AWS deployment strategy, service architecture, resource allocation
- @UX-LAYOUT.md - Design system, responsive breakpoints, aesthetic themes, component patterns
- @DATA-CONTRACTS.md - API type alignment, request/response DTOs, frontend/backend contract fixes
- @IMPLEMENTATION-FRONTEND.md - Nuxt.js patterns, component structure, state management, form validation
- @IMPLEMENTATION-BACKEND.md - Rust/Actix-web patterns, authentication flow, database integration, testing
- @IMPLEMENTATION-DATABASE.md - PostgreSQL schema, migrations, UUIDv7 setup, backup strategy
- @IMPLEMENTATION-SCRIPTS.md - Development automation architecture, script design patterns
- @IMPLEMENTATION-UTILS.md - Development utilities, password hashing, data generation tools
- @IMPLEMENTATION-NGINX.md - Reverse proxy setup, SSL configuration, routing rules
- @IMPLEMENTATION-DEPLOYMENT.md - AWS EC2 deployment, Docker orchestration, CI/CD planning
- @IMPLEMENTATION-AUTH.md - JWT authentication design, security patterns, OAuth planning
- @PROJECT_HISTORY.md - Completed phases, lessons learned, architectural insights, milestones

# RULES
 - Use idomatic code for the appropriate language or framework.
 - Documentation should not include code that should go in actual implemented files.
 - Documentation should be succinct and clear
 - Documentation should maintain separation of concerns between documents so we are not repeating ourselves or creating possible documentation mismatching and confusion.
 - Code should prefer clarity over cleverness
 - When repeatedly encountering errors use context7 MCP to look up documentation to make sure you are correctly implementing code according to the packages intentions.
 - When encountering technical challenges (like UUIDv7 PostgreSQL extension), use web search tools to research solutions before suggesting alternatives or compromises. Many specific technical issues have existing solutions that can be found through targeted research.
 - ASK QUESTIONS INSTEAD OF ASSUMING: it is better to ask me than make an assumption. But also don't ask about every little detail. I know this is confusing but let's try to strike a balance. The more core the question is to the work the more likely you need to ask a question and not assume.
 - CHALLENGE BAD REQUESTS: If it seems like I (the user) am making a mistake. Challenge that mistake and make me defend the reasoning. If I override the objections then let it go.
- When a bash command fails because it has "No such file or directory" but you think that directory or file should exist. First check `pwd` to make sure you are where you think you are before you try anything else.
- ALWAYS use development scripts instead of manual docker commands. Update scripts rather than fall back to direct CLI commands.

---

**Status**: ✅ **Production-ready foundation with hot reload development environment**  
**Latest**: ✅ **Backend routing architecture cleaned and tested - all auth endpoints working**
**Next**: Fix frontend auth persistence, then ready for deployment