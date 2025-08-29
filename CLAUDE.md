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

### Current Applications
- **Legacy Vue App** (vue-project/): Simple counter app, preserved as reference
- **Frontend** (frontend/): Nuxt.js 4.0.3 + Docker ready (awaiting backend integration)
- **Backend** (backend/): ✅ **PRODUCTION READY** - Full API implementation complete
- **Infrastructure**: Docker Compose + Nginx + SSL automation ready

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

# Architecture & Implementation Documentation
- @ARCHITECTURE.md
- @IMPLEMENTATION-FRONTEND.md
- @IMPLEMENTATION-BACKEND.md
- @IMPLEMENTATION-DATABASE.md
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
- ✅ **Authentication system COMPLETE** (JWT-based register/login with role middleware)
- ✅ **Incident timer CRUD** (Full create, read, update, delete + public endpoint)
- ✅ **Comprehensive testing** (Integration tests covering all endpoints)
- 🔄 Deploy to AWS EC2 with SSL

### Phase 2: Enhancement
- Add OAuth integration (Google, GitHub)
- Implement GitHub Actions CI/CD pipeline
- Add comprehensive testing (unit, integration, e2e)
- Set up staging environment

### Phase 3: Advanced Features  
- Enhanced monitoring and logging
- Performance optimization and caching
- Advanced user features and CRUD operations
- Backup automation and disaster recovery


# RULES
 - Use idomatic code for the appropriate language framework.
 - Documentation should not include code that should go in actual implemented files.
 - Documentation should be succinct and clear
 - Code should prefer clarity over cleverness
 - When encountering technical challenges (like UUIDv7 PostgreSQL extension), use web search tools to research solutions before suggesting alternatives or compromises. Many specific technical issues have existing solutions that can be found through targeted research.