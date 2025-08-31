# Hot Reload Dockerized Development Environment Setup

## 🎯 **Objective**: Create a complete hot reload development environment with nginx proxy and self-signed SSL

**Context**: Full-stack application (Rust backend + Nuxt frontend) is fully functional but requires rebuilds for code changes. Need to eliminate rebuild cycles for faster development iteration.

## 📋 **Requirements**

### 1. Development Dockerfiles
Create development-specific Dockerfiles that enable hot reloading:

**Backend (`backend/Dockerfile.dev`)**:
- Base: Rust with development tools
- Install `cargo-watch` for auto-recompilation
- Volume mount source code (read-only mounts where possible)
- Preserve `target/` directory for build cache
- Command: `cargo watch -x run`
- Expose port 8080

**Frontend (`frontend/Dockerfile.dev`)**:
- Base: Node.js 20 alpine
- Install dependencies once, then volume mount source
- Preserve `node_modules` directory
- Command: `npm run dev -- --host 0.0.0.0`
- Expose ports 3000 (app) and 24678 (DevTools)
- Enable `CHOKIDAR_USEPOLLING=true` for container file watching

### 2. Container Naming Strategy
Update `docker-compose.development.yml` to avoid prod/dev image conflicts:
- Use separate image names (e.g., `kennwilliamson-backend-dev`, `kennwilliamson-frontend-dev`)
- Ensure production builds don't overwrite development images

### 3. Nginx Development Proxy
Create nginx configuration for development that matches production structure:

**Goals**:
- **HTTPS**: Self-signed SSL certificates for localhost
- **Routing**: `https://localhost/` → frontend, `https://localhost/api/` → backend  
- **CORS Elimination**: Same-origin requests eliminate CORS complexity
- **Production-like**: Matches production nginx setup for consistency

**Components Needed**:
- `nginx/conf.d/development.conf` - Development nginx configuration
- Self-signed SSL certificate generation (script or Docker volume)
- Nginx service in `docker-compose.development.yml`

### 4. Volume Mount Strategy
Optimize volume mounts for maximum performance and convenience:

**Backend**:
- Source code: `./backend/src:/app/src:ro` (read-only)
- Config files: `./backend/Cargo.toml:/app/Cargo.toml:ro`
- Build cache: `backend_target_dev:/app/target` (persistent volume)

**Frontend**:
- Source code: `./frontend/app:/app/app:ro` (read-only)
- Config files: Package.json, nuxt.config.ts (read-only mounts)
- Dependencies: `frontend_node_modules_dev:/app/node_modules` (persistent volume)

### 5. Development Script Integration
Update existing development scripts to work with hot reload environment:
- `./scripts/dev-start.sh` should use development configuration by default
- Add options for production vs development mode selection
- Ensure all script options (--build, --rebuild, --no-cache) work with new setup

## 🏗️ **Architecture Target**

### Development Stack Flow
```
Developer changes code
       ↓
Files detected by watchers (cargo-watch / Nuxt HMR)
       ↓ 
Backend: Auto-recompile + restart | Frontend: Hot module replacement
       ↓
Changes visible immediately at https://localhost
```

### Network Architecture
```
Browser → https://localhost (nginx) 
           ├── / → frontend:3000 (Nuxt HMR)
           └── /api/ → backend:8080 (cargo-watch)
```

### Container Architecture  
```
nginx-dev (port 443)
├── frontend-dev:3000 (volume mounted source)
├── backend-dev:8080 (volume mounted source) 
└── postgres (shared with prod)
```

## 📁 **File Structure to Create**

```
kennwilliamsondotorg/
├── backend/
│   └── Dockerfile.dev              # New: Hot reload backend
├── frontend/
│   └── Dockerfile.dev              # New: Hot reload frontend
├── nginx/
│   ├── conf.d/
│   │   └── development.conf        # New: Dev nginx config
│   └── ssl/
│       ├── localhost.crt           # New: Self-signed cert
│       └── localhost.key           # New: Private key
├── scripts/
│   └── generate-dev-certs.sh       # New: SSL cert generation
└── docker-compose.development.yml  # Update: Add nginx, volumes, naming
```

## ⚡ **Success Criteria**

After implementation, the development experience should be:

1. **Single Command Start**: `./scripts/dev-start.sh` starts entire environment with hot reload
2. **Instant Updates**: 
   - Rust code changes → automatic recompilation and server restart (5-10 seconds)
   - Vue/TypeScript changes → instant browser updates via HMR (<1 second)
3. **HTTPS Development**: Access via `https://localhost` with valid SSL (no browser warnings)
4. **No CORS Issues**: Same-origin requests eliminate CORS complexity
5. **Production Similarity**: Development environment matches production nginx structure

## 🔧 **Implementation Notes**

### Key Technical Decisions
- Use nginx as the primary development entry point (not direct service access)
- Self-signed certificates valid for `localhost` (not `127.0.0.1`)
- Read-only source mounts where possible (prevent accidental container writes)
- Separate persistent volumes for build caches (faster rebuilds when needed)
- Preserve existing production docker-compose.yml unchanged

### Security Considerations
- Self-signed certificates for development only
- Nginx configuration optimized for development (not production security)
- Volume permissions should match host user (1000:1000)

### Performance Optimization
- Persistent volumes for build caches (`target/`, `node_modules`)
- File watching optimized for container environments
- Minimize image rebuild frequency through strategic layering

## 🚀 **Expected Developer Workflow After Implementation**

```bash
# Start development environment (all services with hot reload)
./scripts/dev-start.sh

# Open browser to https://localhost
# - Frontend loads instantly
# - Make code changes
# - See changes immediately without manual rebuilds
# - Backend changes restart automatically
# - Frontend changes update via HMR

# View logs from all services
./scripts/dev-logs.sh

# Stop when done
./scripts/dev-stop.sh
```

---

**Priority**: High - This will significantly improve development velocity and eliminate the rebuild friction currently slowing down iteration cycles.