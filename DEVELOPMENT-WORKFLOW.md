# Development Workflow Guide

## Overview
Daily development workflows using automated scripts and established patterns.

## Getting Started

```bash
# Clone and setup
git clone <repository-url>
cd kennwilliamsondotorg

# Start environment
./scripts/dev-start.sh
./scripts/health-check.sh

# Access: https://localhost
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

### Code Changes
- **Frontend**: Edit `frontend/app/` - HMR updates instantly
- **Backend**: Edit `backend/src/` - cargo-watch rebuilds automatically
- **Monitoring**: Use `./scripts/dev-logs.sh [service]` to track changes

### Database Workflow

```bash
# After schema changes
./scripts/setup-db.sh              # Run migrations
./scripts/prepare-sqlx.sh --clean  # Update query cache
./scripts/dev-start.sh --rebuild backend

# Create new migration
cd backend && sqlx migrate add <name>
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
curl http://localhost:8080/backend/health

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

### Development Best Practices
- Use scripts instead of manual Docker commands
- Check health after major changes
- Let auto-reload handle most restarts
- Update SQLx cache after SQL changes
- Use `.env.development` consistently

## Local Production Environment

### Setting Up Local Production Testing

When you need to test production-like configurations locally (e.g., debugging production-only issues, testing SSL configuration, validating production environment variables):

```bash
# One-command setup of local production environment
./scripts/setup-local-prod.sh

# Optional: Force rebuild and follow logs
./scripts/setup-local-prod.sh --build --logs

# Optional: Stop existing services first
./scripts/setup-local-prod.sh --stop-first
```

### Local Production Workflow

```bash
# 1. Set up local production environment
./scripts/setup-local-prod.sh

# 2. Verify all services are healthy
./scripts/health-check.sh --local-prod

# 3. Test production configuration
curl -k https://localhost  # Browser shows cert warning (expected)

# 4. Optional: Add domain testing to /etc/hosts
echo "127.0.0.1 kennwilliamson.org" | sudo tee -a /etc/hosts
echo "127.0.0.1 www.kennwilliamson.org" | sudo tee -a /etc/hosts

# 5. Test with production domain
curl -k https://kennwilliamson.org

# 6. When done, stop local production environment
docker-compose --env-file .env.production -f docker-compose.yml -f docker-compose.local-prod.yml down
```

### Local Production Features

**What You Get:**
- **Production Environment Variables**: Same `.env.production` as real production
- **Production-Like SSL**: Domain certificates with production-grade security
- **Rate Limiting**: Same rate limiting rules as production
- **Security Headers**: Full production security header configuration
- **Isolated Database**: Separate `postgres_data_local_prod` volume
- **Domain Testing**: Support for testing with production domain names

**Access Points:**
- **HTTPS (Recommended)**: https://localhost
- **Domain Testing**: https://kennwilliamson.org (requires /etc/hosts entry)
- **Backend API**: https://localhost/api/
- **Health Check**: https://localhost/health

**Common Use Cases:**
- Debug production-only configuration issues
- Test SSL certificate handling
- Validate production environment variables
- Test rate limiting and security headers
- Integration testing with production-like setup

### Switching Between Environments

```bash
# Development environment (default)
./scripts/dev-start.sh
./scripts/health-check.sh --dev

# Local production environment
./scripts/setup-local-prod.sh
./scripts/health-check.sh --local-prod

# Production environment (deployment only)
./scripts/health-check.sh  # defaults to production mode
```

## Environment Access Points

### Primary Development URLs
- **Main Application:** `https://localhost` (recommended - full nginx proxy)
- **Frontend Direct:** `http://localhost:3000` (fallback for debugging)
- **Backend API:** `http://localhost:8080/backend/` (direct API access)
- **PostgreSQL:** `localhost:5432` (for database tools)

### Service Endpoints
- **Health Check:** `http://localhost:8080/backend/health`
- **Database Health:** `http://localhost:8080/backend/health/db`
- **Public Timer Example:** `http://localhost:8080/backend/user-slug/incident-timer`

## Integration with Development Tools

### IDE Integration
- **VS Code:** Works seamlessly with hot reload
- **File watching:** Changes detected automatically by both frontend and backend
- **Debugging:** Use IDE debugging tools with running services

### Database Tools
- **Connection:** `postgresql://postgres:postgres@localhost:5432/kennwilliamson`
- **pgAdmin, DBeaver, etc.:** Can connect to local PostgreSQL instance
- **Migrations:** Always use `./scripts/setup-db.sh` instead of direct SQLx commands


## Troubleshooting

### Docker Configuration Issues
**Important**: If you're following a clear tutorial or documentation and something isn't working as expected, **always check the Docker configuration first**.

Common issues:
- **TypeScript types not recognized**: Check if required directories are mounted in `docker-compose.development.yml`
- **File changes not detected**: Verify volume mounts include all necessary directories
- **Import errors**: Ensure all source directories are properly mounted
- **Build failures**: Check if all required files are included in the build context

**Example**: When setting up nuxt-auth-utils types, the `shared/` directory wasn't mounted in Docker, causing TypeScript to not recognize the type definitions in the container.

### Quick Docker Checks
```bash
# Check if containers are running
docker ps

# Check volume mounts
docker inspect kennwilliamson-frontend-dev | grep -A 10 "Mounts"

# Restart containers after config changes
docker-compose down && docker-compose up -d
```

### Reviewing Our Work
- If asked to review git diff `git diff --staged | cat` so that you can correctly see the work staged.
---

*This workflow guide should be your primary reference for daily development activities. It emphasizes using the project's automated scripts and established patterns for consistency and reliability.*