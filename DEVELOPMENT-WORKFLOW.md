# Development Workflow Guide

## Overview
This guide provides comprehensive workflows for daily development on the KennWilliamson.org project, focusing on using the automated development scripts and following established patterns.

## Getting Started

### Initial Setup
For new developers setting up the project:

```bash
# Clone the repository
git clone <repository-url>
cd kennwilliamsondotorg

# Start development environment
./scripts/dev-start.sh

# Verify services are healthy
./scripts/health-check.sh

# Access the application
# https://localhost (recommended - nginx proxy with SSL)
# http://localhost:3000 (direct frontend access if needed)
```

## Daily Development Workflows

### Starting Your Work Day

```bash
# Start the complete development environment
./scripts/dev-start.sh

# Optional: Verify all services are running properly
./scripts/health-check.sh

# Begin coding - changes will hot reload automatically:
# - Frontend: Vue/TypeScript changes update instantly (HMR)
# - Backend: Rust changes trigger automatic recompilation
```

### Making Code Changes

#### Frontend Development
- Edit files in `frontend/app/`
- Changes automatically trigger Hot Module Replacement (HMR)
- No manual restart required for most changes
- View changes instantly in browser at `https://localhost`

#### Backend Development
- Edit files in `backend/src/`
- Cargo-watch detects changes and rebuilds automatically
- Service restarts automatically after successful compilation
- Monitor rebuild status with `./scripts/dev-logs.sh backend`

### Database Development Workflow

#### After Schema Changes
When you've modified database migrations or SQL queries:

```bash
# Run new migrations (safe - preserves existing data)
./scripts/setup-db.sh

# Update SQLx query cache for Docker builds
./scripts/prepare-sqlx.sh --clean

# Restart backend to pick up schema changes
./scripts/dev-start.sh --rebuild backend
```

#### Creating New Migrations
```bash
# Create a new migration file
cd backend
sqlx migrate add your_migration_name

# Edit the generated migration file
# Then follow the "After Schema Changes" workflow above
```

## Development Scripts Reference

### Core Service Management

#### `./scripts/dev-start.sh`
Primary service management script with flexible options:

```bash
# Basic usage
./scripts/dev-start.sh                    # Start all services
./scripts/dev-start.sh --logs            # Start and follow logs

# Build options
./scripts/dev-start.sh --build           # Force rebuild all services
./scripts/dev-start.sh --rebuild backend # Force recreate specific service
./scripts/dev-start.sh --no-cache frontend # Rebuild without cache

# Service-specific startup
./scripts/dev-start.sh backend           # Start only backend
./scripts/dev-start.sh postgres frontend # Start multiple specific services
```

#### `./scripts/dev-logs.sh`
Monitor service behavior and debug issues:

```bash
# Basic log viewing
./scripts/dev-logs.sh                    # Follow all services
./scripts/dev-logs.sh backend           # Follow specific service
./scripts/dev-logs.sh nginx --tail 20   # Show last 20 lines

# Advanced options
./scripts/dev-logs.sh --no-follow       # Show logs and exit
./scripts/dev-logs.sh --timestamps      # Include timestamps
```

#### `./scripts/dev-stop.sh`
Clean service shutdown:

```bash
./scripts/dev-stop.sh                   # Stop all services
./scripts/dev-stop.sh --remove          # Stop and remove containers
./scripts/dev-stop.sh backend          # Stop specific service
```

### Database Management

#### `./scripts/setup-db.sh`
Safe database migration management:

```bash
./scripts/setup-db.sh                  # Run pending migrations
./scripts/setup-db.sh --verify         # Run migrations + verify schema
```

**Key Features:**
- Preserves existing data (safe to run repeatedly)
- Auto-starts PostgreSQL if needed
- Shows migration status before and after
- Comprehensive error handling

#### `./scripts/prepare-sqlx.sh`
SQLx query cache generation for Docker builds:

```bash
./scripts/prepare-sqlx.sh              # Generate cache
./scripts/prepare-sqlx.sh --clean      # Clean + regenerate
```

**When to use:**
- After changing SQL queries in Rust code
- Before Docker builds to ensure query validation
- When SQLx compilation errors occur

### Health and Debugging

#### `./scripts/health-check.sh`
Comprehensive service health verification:

```bash
./scripts/health-check.sh                    # Check all services
./scripts/health-check.sh --wait             # Wait up to 60s for startup
./scripts/health-check.sh --service postgres # Check specific service
```

**Checks performed:**
- PostgreSQL connectivity and database access
- Backend API health endpoints
- Frontend HTTP response
- Docker container status and resource usage

## Troubleshooting Workflows

### When Things Break
Follow this troubleshooting hierarchy:

#### 1. Check Service Health
```bash
./scripts/health-check.sh
```

#### 2. View Recent Logs
```bash
# Check specific service logs
./scripts/dev-logs.sh backend --tail 50    # API issues
./scripts/dev-logs.sh frontend --tail 50   # Build issues
./scripts/dev-logs.sh nginx --tail 50      # Proxy issues
```

#### 3. Test Direct Service Access
```bash
# Test backend API directly
curl http://localhost:8080/api/health

# Test frontend directly (bypassing nginx)
curl http://localhost:3000
```

#### 4. Restart Problematic Services
```bash
# Restart specific service
./scripts/dev-start.sh --rebuild backend

# Or restart everything
./scripts/dev-stop.sh
./scripts/dev-start.sh
```

#### 5. Nuclear Option (Complete Reset)
```bash
# Stop everything and remove containers
./scripts/dev-stop.sh --remove

# Rebuild everything from scratch
./scripts/dev-start.sh --build
```

### Common Issues and Solutions

#### SQLx Query Cache Issues
**Symptoms:** Docker build fails with SQLx query validation errors
**Solution:**
```bash
./scripts/prepare-sqlx.sh --clean
./scripts/dev-start.sh --rebuild backend
```

#### Database Schema Out of Sync
**Symptoms:** Database-related errors in backend logs
**Solution:**
```bash
./scripts/setup-db.sh --verify
./scripts/dev-start.sh --rebuild backend
```

#### Frontend Hot Reload Not Working
**Symptoms:** Changes don't appear in browser automatically
**Solution:**
```bash
# Check if frontend service is running
./scripts/health-check.sh --service frontend

# Restart frontend if needed
./scripts/dev-start.sh --rebuild frontend
```

#### Port Conflicts
**Symptoms:** Services fail to start due to port already in use
**Solution:**
```bash
# Stop all services first
./scripts/dev-stop.sh --remove

# Check for lingering processes
docker ps -a
docker system prune

# Restart services
./scripts/dev-start.sh
```

## Development Best Practices

### Script Usage Rules
1. **Always use development scripts** instead of manual Docker commands
2. **Update scripts** rather than falling back to direct CLI commands
3. **Check service health** after major changes
4. **Use `.env.development`** for all development work

### Code Change Patterns
1. **Backend changes:** Let cargo-watch handle rebuilds automatically
2. **Schema changes:** Always run migrations and update SQLx cache
3. **Frontend changes:** Rely on HMR for instant feedback
4. **Environment changes:** Restart affected services

### Debugging Workflow
1. **Start with health checks** to identify problematic services
2. **Use targeted logging** to focus on specific issues
3. **Test direct access** to isolate networking vs application issues
4. **Escalate systematically** from service restart to complete rebuild

## Environment Access Points

### Primary Development URLs
- **Main Application:** `https://localhost` (recommended - full nginx proxy)
- **Frontend Direct:** `http://localhost:3000` (fallback for debugging)
- **Backend API:** `http://localhost:8080/api/` (direct API access)
- **PostgreSQL:** `localhost:5432` (for database tools)

### Service Endpoints
- **Health Check:** `http://localhost:8080/api/health`
- **Database Health:** `http://localhost:8080/api/health/db`
- **Public Timer Example:** `http://localhost:8080/api/user-slug/incident-timer`

## Integration with Development Tools

### IDE Integration
- **VS Code:** Works seamlessly with hot reload
- **File watching:** Changes detected automatically by both frontend and backend
- **Debugging:** Use IDE debugging tools with running services

### Database Tools
- **Connection:** `postgresql://postgres:postgres@localhost:5432/kennwilliamson`
- **pgAdmin, DBeaver, etc.:** Can connect to local PostgreSQL instance
- **Migrations:** Always use `./scripts/setup-db.sh` instead of direct SQLx commands

---

*This workflow guide should be your primary reference for daily development activities. It emphasizes using the project's automated scripts and established patterns for consistency and reliability.*