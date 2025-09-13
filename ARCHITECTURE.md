# KennWilliamson.org Architecture

## System Overview
Full-stack web application with containerized microservices architecture, designed for development environment with production deployment capabilities.

## Current Architecture

### Service Architecture
```
Development Environment (https://localhost)
┌─────────────────┐
│   Developer     │
└─────────────────┘
         │
┌─────────────────┐
│ Nginx Proxy     │ ← SSL Termination, Static Files, Request Routing
│ Port 443        │
└─────────────────┘
         │
    ┌────┴────┐
    │         │
┌───▼───┐ ┌──▼──────┐
│Nuxt.js│ │ Rust    │
│Vue 3  │ │ Actix   │
│SSR    │ │ API     │
│:3000  │ │ :8080   │
└───────┘ └──┬──────┘
              │
        ┌─────▼─────┐
        │PostgreSQL │
        │   :5432   │
        └───────────┘
```

### Current Service Definitions

#### Nginx Reverse Proxy
- **Purpose**: SSL termination, static file serving, request routing
- **SSL**: Self-signed certificates for development HTTPS
- **Routing**: Frontend (/*), Backend API (/backend/*), WebSocket support for HMR
- **Configuration**: nginx/conf.d/default.conf

#### Nuxt.js Frontend
- **Technology**: Nuxt.js 4.0.3 (Vue 3 + SSR + TypeScript)
- **Port**: 3000 (internal)
- **Features**: Server-side rendering, file-based routing, Pinia state management
- **Hot Reload**: Vite HMR with WebSocket support through nginx proxy

#### Rust Backend
- **Technology**: Actix-web 4.x + SQLx for database integration
- **Port**: 8080 (internal)
- **Features**: REST API, JWT authentication, bcrypt password hashing
- **Auto Reload**: cargo-watch for development hot reload

#### PostgreSQL Database
- **Technology**: PostgreSQL 17 with UUIDv7 extension
- **Port**: 5432 (internal only)
- **Storage**: Docker volume for data persistence
- **Features**: Automated timestamp triggers, role-based authorization

## Data Flow Architecture

### Authentication Flow
1. User registration/login → Frontend form validation
2. Frontend → Backend API (/backend/auth/register, /backend/auth/login)
3. API → PostgreSQL (user validation/creation with bcrypt hashing)
4. API → JWT token generation with role claims
5. Frontend → Store JWT in httpOnly cookies
6. Protected requests → JWT validation middleware

### Application Data Flow
1. Browser → HTTPS request to https://localhost
2. Nginx → Route to Frontend (SSR) or Backend API (/backend/*)
3. Frontend SSR → Generate HTML with initial data
4. Client-side hydration → Single Page Application behavior
5. API calls → Rust backend → PostgreSQL with SQLx

### Timer Feature Flow
1. User creates timer → Frontend form → Backend API /backend/incident-timers
2. Timer data stored → PostgreSQL with UUIDv7 and timestamps
3. Real-time updates → Frontend polls/refreshes timer display
4. Public access → /{user_slug}/incident-timer (no auth required)

## Security Architecture

### Current Security Implementation
- **SSL/TLS**: Self-signed certificates for development HTTPS
- **Authentication**: JWT tokens with 24-hour expiration
- **Password Security**: bcrypt hashing with cost factor 12
- **Database Security**: Internal network only, no external access
- **Input Validation**: Request/response validation on both frontend and backend

### Authorization Model
- **JWT Claims**: User ID, email, roles array, issued/expiry timestamps
- **Route Protection**: Middleware-based authentication for protected endpoints
- **Role-Based Access**: User/admin roles with middleware extraction
- **Data Isolation**: Users can only access their own timer data

## Development Environment

### Container Orchestration
- **Docker Compose**: docker-compose.development.yml for development
- **Service Dependencies**: Proper startup order and health checks
- **Volume Mounts**: Code volumes for hot reload, config volumes for nginx
- **Environment Variables**: .env.development for configuration

### Development Workflow
- **Hot Reload**: All services support live code updates
- **Development Scripts**: Automated service management via scripts/
- **Health Monitoring**: Comprehensive service health checking
- **Database Management**: Migration scripts and reset capabilities

## Resource Management

### Development Environment (Typical Development Machine)
- **Nginx**: ~20MB (lightweight alpine)
- **Frontend (Nuxt.js)**: ~150MB (development mode)
- **Backend (Rust)**: ~50MB (debug build)
- **PostgreSQL**: ~100MB (development load)
- **Docker Overhead**: ~50MB

### Production Target (AWS EC2 t2.micro - 1GB RAM)
- **Nginx**: ~50MB
- **Frontend (Nuxt.js)**: ~200MB (production build)
- **Backend (Rust)**: ~150MB (release build)
- **PostgreSQL**: ~300MB (optimized for 1GB constraint)
- **System/Docker**: ~300MB

## Integration Points

### Service Communication
- **Frontend ↔ Backend**: HTTP/HTTPS via nginx proxy
- **Backend ↔ Database**: PostgreSQL protocol via SQLx connection pool
- **Development ↔ Services**: Docker Compose networking

### External Integration Readiness
- **Domain Management**: Ready for DNS configuration
- **SSL Certificate Management**: Ready for Let's Encrypt integration
- **Monitoring**: Health check endpoints implemented
- **Backup**: Database backup procedures ready

## Scalability Considerations

### Current Limitations
- **Single Instance**: All services run as single containers
- **Database**: Single PostgreSQL instance without replication
- **State Management**: JWT tokens are stateless but no distributed session storage

### Architecture Scalability Features
- **Stateless Backend**: Rust API can scale horizontally
- **Database Connection Pooling**: SQLx connection pool ready for multiple backend instances
- **Load Balancer Ready**: Nginx configuration supports upstream backend pools
- **Container Architecture**: Services are independently scalable

---

*This document describes the current system architecture. For production deployment plans and infrastructure scaling, see [ROADMAP.md](ROADMAP.md).*