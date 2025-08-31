# CLAUDE.md - Project Context

## Project Overview
This project is transitioning to a **full-stack web application** for kennwilliamson.org. The current simple Vue.js counter app will be replaced with a comprehensive architecture featuring:
- **Frontend**: Nuxt.js 3 (Vue 3 + SSR + routing)
- **Backend**: Rust with Actix-web framework
- **Database**: PostgreSQL
- **Infrastructure**: AWS EC2 free tier with Docker + Docker Compose
- **Reverse Proxy**: Nginx with Let's Encrypt SSL

See **ARCHITECTURE.md** for detailed system design and future roadmap.

## Current Structure (Legacy)
```
vue-project/                 # LEGACY: Will be replaced
├── src/
│   ├── components/          # Vue components (HeaderComponent, CounterBanner)
│   ├── assets/             # Static assets and CSS
│   ├── App.vue            # Main application component
│   └── main.ts            # Application entry point
├── public/                 # Public static files
├── package.json           # Dependencies and scripts
├── vite.config.ts         # Vite configuration
├── tsconfig.json          # TypeScript configuration
└── eslint.config.ts       # ESLint configuration
```

## New Architecture Structure (Planned)
```
kennwilliamsondotorg/
├── ARCHITECTURE.md          # System architecture documentation
├── CLAUDE.md               # Project context (this file)
├── docker-compose.yml      # Service orchestration
├── frontend/               # Nuxt.js application
│   ├── pages/              # File-based routing
│   ├── components/         # Vue components
│   ├── composables/        # Shared logic
│   └── package.json        # Frontend dependencies
├── backend/                # Rust Actix-web API
│   ├── src/                # Rust source code
│   ├── Cargo.toml          # Rust dependencies
│   └── Dockerfile          # Backend container
├── nginx/                  # Reverse proxy configuration
└── scripts/                # Deployment and utility scripts
```

## Technology Stack

### Current (Legacy)
- **Vue 3.5.18** - Progressive JavaScript framework
- **TypeScript 5.8** - Type-safe JavaScript
- **Vite 7.0.6** - Fast build tool and dev server
- **ESLint + Prettier** - Code quality and formatting

### New Full-Stack Architecture
- **Frontend**: Nuxt.js 3 with Vue 3, TypeScript, SSR
- **Backend**: Rust 1.70+ with Actix-web 4.x
- **Database**: PostgreSQL 15
- **Infrastructure**: Docker, Docker Compose, Nginx
- **Deployment**: AWS EC2 t2.micro (free tier)
- **SSL**: Let's Encrypt with automated renewal

## Context7 MCP Server Usage
When working on this project, use the Context7 MCP server to look up APIs and documentation:

### Current/Legacy Stack
1. **For Vue.js APIs**: Use `resolve-library-id` with "vue" to get Vue.js documentation
2. **For TypeScript**: Use `resolve-library-id` with "typescript" for TypeScript language features
3. **For Vite**: Use `resolve-library-id` with "vite" for build tool configuration

### New Full-Stack APIs
1. **For Nuxt.js**: Use `resolve-library-id` with "nuxt.js" for SSR, routing, and framework features
2. **For Rust/Actix-web**: Use `resolve-library-id` with "actix-web" for web framework documentation
3. **For PostgreSQL**: Use `resolve-library-id` with "postgresql" for database operations
4. **For Docker**: Use `resolve-library-id` with "docker" for containerization

## Development Notes

### Current Legacy App
- The app uses Vue 3 Composition API with `<script setup>` syntax
- Components are located in `src/components/` with a clean, modular structure
- TypeScript is configured for strict type checking
- Vite provides hot module replacement for fast development
- The project follows Vue.js best practices and conventions

### New Architecture Guidelines
- **Node.js 20+**: Required for Nuxt.js (even-numbered versions recommended)
- **Rust 1.70+**: Required for modern async/await and latest features
- **Docker Compose**: Orchestrates all services with resource constraints for AWS free tier
- **Security-First**: JWT authentication, bcrypt hashing, HTTPS everywhere
- **Performance**: Optimized for 1GB RAM constraint on EC2 t2.micro

## Development Status

### 🎉 Backend API Complete (Phase 1 Finished)
The Rust backend is **production-ready** with all core functionality implemented:
- **Authentication**: JWT-based registration/login with bcrypt password hashing
- **Authorization**: Role-based middleware (user/admin) ready for future use
- **Database**: PostgreSQL 17 + UUIDv7 with automated timestamp triggers
- **API Endpoints**: Full CRUD operations + public incident timer endpoint
- **Testing**: Comprehensive integration test suite covering all endpoints
- **Security**: Proper JWT validation, password hashing, and route protection

### 🎉 Frontend Complete (Phase 2 - 100% COMPLETE)
The Nuxt.js 4.0.3 frontend is **fully functional** and integrated with backend:
- **Authentication System**: Complete login/register with VeeValidate + Yup ✅
- **User Interface**: Responsive design with gothic construction theme ✅
- **Incident Timers**: Full CRUD operations with real-time updates ✅
- **Service Layer**: Clean architecture with proper API separation ✅
- **Asset Management**: Correct Nuxt 4 structure with build-time optimization ✅
- **Pinia Integration**: No more context errors, proper store initialization ✅
- **Form Validation**: VeeValidate working properly without conflicts ✅
- **CORS Integration**: Backend API calls working from frontend ✅
- **Type Alignment**: Frontend/backend contracts aligned per DATA-CONTRACTS.md ✅

### Current Applications
- **Legacy Vue App** (vue-project/): Simple counter app, preserved as reference
- **Frontend** (frontend/): ✅ **COMPLETE** - Nuxt.js 4.0.3 + Backend integration working
- **Backend** (backend/): ✅ **PRODUCTION READY** - Full API implementation complete
- **Infrastructure**: Docker Compose + Nginx + SSL automation ready
- **Integration Status**: ✅ **WORKING** - User registration, login, timer CRUD all functional

## New Full-Stack Application (Planned)
The new application will be a personal playground website featuring:
- **About Me Page**: Personal information and portfolio
- **User Management**: Registration/login system with email/password auth
- **CRUD Operations**: Various data management features
- **Future OAuth**: Google and GitHub authentication with account linking
- **Responsive SSR**: Server-side rendered Vue.js with Nuxt.js
- **API-First**: RESTful API built with Rust and Actix-web

## Commit Convention
This project uses conventional commits with prefixes:
- **[FEATURE]**: New features and enhancements
- **[FIX]**: Bug fixes and corrections  
- **[CHORE]**: Maintenance, documentation, and tooling
- **[REFACTOR]**: Code restructuring without functional changes

## Development Workflow

### Docker Service Management
Use standard Docker Compose commands for service management:

```bash
# Build services
docker-compose build                 # Build all services
docker-compose build backend        # Build specific service
docker-compose build --no-cache     # Force rebuild

# Start services  
docker-compose up -d                 # Start all services
docker-compose up -d backend        # Start specific service

# Combined build and start
docker-compose up -d --build backend

# ⚠️  IMPORTANT: Always use .env.development for development
docker-compose --env-file .env.development up -d
docker-compose --env-file .env.development up backend -d
docker-compose --env-file .env.development build backend

# Service status and logs
docker-compose ps                    # Service status
docker-compose --env-file .env.development logs backend         # View service logs with proper config
docker-compose --env-file .env.development logs -f backend      # Follow logs with proper config
docker-compose logs backend         # View service logs (basic, may miss debug logging)
docker-compose logs -f backend      # Follow logs (basic, may miss debug logging)
```

### Development Scripts
- `./scripts/reset-db.sh` - Complete database reset with fresh migrations
- `./scripts/setup-db.sh` - Run pending migrations safely (preserves data)
- `./scripts/prepare-sqlx.sh` - Generate SQLx query cache for Docker builds
- `./scripts/health-check.sh` - Comprehensive service health verification

### ⚠️ Environment File Usage (Critical for Debug Logging)
**Important**: For proper debug logging (HTTP requests, JWT middleware), containers must be started with the environment file:
```bash
# Start with proper environment configuration
docker-compose --env-file .env.development up -d backend

# Then view logs (now includes debug output)
docker-compose logs -f backend
```

### Common Development Workflows

**New Developer Setup:**
```bash
# 1. Start database
docker-compose --env-file .env.development up postgres -d

# 2. Run migrations
./scripts/setup-db.sh --verify

# 3. Prepare SQLx cache
./scripts/prepare-sqlx.sh

# 4. Build and start backend
docker-compose --env-file .env.development build backend
docker-compose --env-file .env.development up -d backend

# 5. Verify everything is healthy
./scripts/health-check.sh
```

**After Database Schema Changes:**
```bash
# 1. Run new migrations
./scripts/setup-db.sh

# 2. Update SQLx cache
./scripts/prepare-sqlx.sh --clean

# 3. Rebuild backend with new cache
docker-compose --env-file .env.development build backend
docker-compose --env-file .env.development up -d backend
```

**When Things Are Broken (Nuclear Reset):**
```bash
# 1. Reset database completely
./scripts/reset-db.sh

# 2. Prepare SQLx cache
./scripts/prepare-sqlx.sh

# 3. Rebuild everything
docker-compose --env-file .env.development build
docker-compose --env-file .env.development up -d

# 4. Verify health
./scripts/health-check.sh --wait
```

**Daily Development:**
```bash
# Quick health check
./scripts/health-check.sh

# View logs when debugging
docker-compose logs -f backend

# Run tests
cd backend && cargo test
```

# Architecture & Implementation Documentation
- @ARCHITECTURE.md
- @UX-LAYOUT.md
- @DATA-CONTRACTS.md
- @IMPLEMENTATION-FRONTEND.md
- @IMPLEMENTATION-BACKEND.md
- @IMPLEMENTATION-DATABASE.md
- @IMPLEMENTATION-SCRIPTS.md
- @IMPLEMENTATION-NGINX.md
- @IMPLEMENTATION-DEPLOYMENT.md
- @IMPLEMENTATION-AUTH.md

## Implementation Phases

### Phase 0: Planning & Documentation (Completed)
- ✅ Architecture design with future roadmap
- ✅ Frontend implementation plan (fresh Nuxt.js approach)
- ✅ Backend implementation plan (Rust + Actix-web)
- ✅ Database implementation plan (PostgreSQL + SQLx migrations)
- ✅ Nginx reverse proxy and SSL configuration plan
- ✅ Deployment strategy (AWS EC2 + Docker Compose)
- ✅ Authentication system design (JWT + future OAuth)

### Phase 1: Foundation (✅ COMPLETED)
- ✅ **Rust toolchain installed** (1.89.0 + cargo-watch + sqlx-cli)
- ✅ **Frontend created** (Nuxt.js 4.0.3 + TypeScript + TailwindCSS + Pinia)
- ✅ **Backend COMPLETE** (Rust + Actix-web 4 + full API implementation)
- ✅ **Database schema COMPLETE** (users, roles, user_roles, incident_timers with UUIDv7)
- ✅ **Docker infrastructure** (Compose + Dockerfiles + Nginx config)
- ✅ **PostgreSQL 17 + UUIDv7** (running with migrations applied + database triggers)
- ✅ **Database reset script** (`./scripts/reset-db.sh` for development)
- ✅ **Database migration script** (`./scripts/setup-db.sh` for safe migrations without reset)
- ✅ **SQLx cache script** (`./scripts/prepare-sqlx.sh` for Docker builds with migration validation)
- ✅ **Health check script** (`./scripts/health-check.sh` for service verification and monitoring)
- ✅ **Authentication system COMPLETE** (JWT-based register/login with role middleware)
- ✅ **Incident timer CRUD** (Full create, read, update, delete + public endpoint)
- ✅ **Comprehensive testing** (Integration tests covering all endpoints)
- 🔄 Deploy to AWS EC2 with SSL

### Phase 2: Frontend Development (✅ 98% COMPLETE)
- ✅ **Nuxt.js 4.0.3 setup** with proper directory structure
- ✅ **Authentication pages** (login/register) with VeeValidate
- ✅ **Responsive header** with mobile navigation
- ✅ **Homepage** with gothic construction theme
- ✅ **Incident management** with CRUD operations
- ✅ **Service layer architecture** for API calls
- ✅ **Pinia stores** with proper context handling
- ✅ **Asset management** with build-time optimization
- ✅ **Form validation** working without conflicts
- 🔄 Final timer calculation enhancement (2% remaining)

### Phase 3: Enhancement
- Add OAuth integration (Google, GitHub)
- Implement GitHub Actions CI/CD pipeline
- Add comprehensive testing (unit, integration, e2e)
- Set up staging environment

### Phase 4: Advanced Features  
- Enhanced monitoring and logging
- Performance optimization and caching
- Advanced user features and CRUD operations
- Backup automation and disaster recovery

## Future Development Environment (Desired State)

### Enhanced Development Container Setup
**Goal**: Full development environment with production-like architecture but optimized for development workflow.

**Architecture:**
```
Development Stack:
┌─────────────────┐
│ Nginx Proxy     │ ← Port 80/443 (dev SSL)
│ :80 → :3000     │
│ :80/api → :8080 │
└─────────────────┘
         │
    ┌────┴────┐
    │         │
┌───▼───┐ ┌──▼──────┐
│Nuxt.js│ │ Rust    │
│Dev    │ │ API     │
│Server │ │ :8080   │
│:3000  │ │         │
└───────┘ └──┬──────┘
              │
        ┌─────▼─────┐
        │PostgreSQL │
        │   :5432   │
        └───────────┘
```

**Features:**
- **Volume Mounting**: Source code directories mounted for instant file changes
- **Hot Reload**: Nuxt dev server with HMR, Rust with cargo-watch auto-rebuild
- **Production Simulation**: Nginx proxy eliminates CORS issues, matches production routing
- **SSL Development**: Self-signed certificates for HTTPS development
- **Network Isolation**: Docker internal networking with exposed dev ports

**Development Experience:**
- Single `docker-compose up -d` starts entire development stack
- Code changes reflect immediately (no container rebuilds)
- Production-like URL structure: `https://localhost/api/health`
- Proper SSL certificate handling for auth testing
- No CORS configuration needed (same-origin requests)

**Implementation Plan:**
- **docker-compose.dev.yml**: Development-specific compose file
- **Volume Mounts**: Source directories attached to containers
- **nginx/dev.conf**: Development proxy configuration with self-signed SSL
- **Frontend**: Nuxt dev mode with `--host 0.0.0.0` for container access
- **Backend**: cargo-watch for auto-rebuild on file changes

**Benefits:**
- Eliminates CORS complexity in development
- Matches production environment architecture  
- Faster development cycle (no rebuilds)
- Easier onboarding for new developers
- Production-like SSL/routing behavior


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