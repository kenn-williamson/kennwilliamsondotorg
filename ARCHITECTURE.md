# KennWilliamson.org Architecture

## System Overview
Containerized microservices architecture with Nginx reverse proxy, Nuxt.js frontend, Rust backend, and PostgreSQL database.

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

### Service Architecture

#### Nginx Reverse Proxy
- **Purpose**: SSL termination, request routing, static file serving
- **Ports**: 443 (HTTPS), 80 (HTTP redirect)
- **Configuration**: Environment-specific nginx configs
- **Details**: See [IMPLEMENTATION-NGINX.md](IMPLEMENTATION-NGINX.md)

#### Nuxt.js Frontend
- **Framework**: Nuxt.js 4.0.3 with Vue 3 and TypeScript
- **Port**: 3000 (internal)
- **Architecture**: SSR with file-based routing
- **Details**: See [IMPLEMENTATION-FRONTEND.md](IMPLEMENTATION-FRONTEND.md)

#### Rust Backend API
- **Framework**: Actix-web 4.x with 3-layer architecture
- **Port**: 8080 (internal)
- **Architecture**: Repository pattern with dependency injection
- **Database**: SQLx for PostgreSQL integration
- **Details**: See [IMPLEMENTATION-BACKEND.md](IMPLEMENTATION-BACKEND.md)

#### PostgreSQL Database
- **Version**: PostgreSQL 17 with pg_uuidv7
- **Port**: 5432 (internal only)
- **Storage**: Persistent Docker volumes
- **Details**: See [IMPLEMENTATION-DATABASE.md](IMPLEMENTATION-DATABASE.md)

## Data Flow Architecture

### 3-Layer Backend Architecture

**API Layer** (`routes/`):
- HTTP request/response handling
- Route validation and middleware
- Service layer delegation

**Service Layer** (`services/`):
- Business logic and orchestration
- Repository trait dependencies
- Error handling and validation

**Repository Layer** (`repositories/`):
- Data access abstraction
- PostgreSQL and mock implementations
- Database query execution

### Authentication Flow
Hybrid JWT/refresh token architecture with secure session management. See [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md#authentication-system) for details.

### Hybrid API Architecture

**SSR Proxy Pattern** (`/api/*`):
- Server-side data fetching for initial page loads
- Session-based authentication handling
- Nuxt server acts as proxy to backend

**Direct Backend Pattern** (`/backend/*`):
- Client-side API calls for mutations
- JWT authentication in request headers
- Direct routing through nginx to backend

See [IMPLEMENTATION-FRONTEND.md](IMPLEMENTATION-FRONTEND.md#architecture-patterns) for implementation details.

### Data Flow Examples

**Timer Creation**:
1. Frontend form submission → API Layer (`routes/incident_timers.rs`)
2. Service Layer (`services/incident_timer.rs`) → Repository Layer (`repositories/postgres/`)
3. PostgreSQL with automatic timestamps
4. Response propagated back through layers to frontend

**Public Access**:
- Direct URL access without authentication
- API Layer → Service Layer → Repository Layer
- Returns public-safe data only

**Service Dependencies**:
- All services use repository traits for data access
- ServiceContainer manages dependency injection
- Mock repositories enable unit testing
- Clean separation of concerns across all layers

## Security Architecture
Comprehensive security implementation across all layers. See [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md) for detailed security measures including:
- Authentication and authorization
- Data protection and validation
- Infrastructure security
- API endpoint protection

## Development Environment

### Container Orchestration
- **Docker Compose**: Environment-specific compose files
- **Service Dependencies**: Health checks and startup ordering
- **Volume Mounts**: Code and configuration persistence
- **Scripts**: Automated workflows via `scripts/`

See [DEVELOPMENT-WORKFLOW.md](DEVELOPMENT-WORKFLOW.md) for detailed development processes.

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

### Current State
- **Single Instance**: All services run as single containers
- **Database**: PostgreSQL without replication
- **State**: JWT tokens provide stateless authentication

### Scalability Design
- **Stateless API**: Horizontal scaling ready
- **Connection Pooling**: Multi-instance support
- **Load Balancing**: Nginx upstream configuration
- **Container Independence**: Services scale individually

For planned scalability enhancements, see [ROADMAP.md](ROADMAP.md#infrastructure-scaling).