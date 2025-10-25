# Scripts Implementation

## Overview
Automation philosophy and script architecture for development and deployment workflows.

## Automation Philosophy

### Why Scripts Over Manual Commands
**Decision**: Automate everything repeatable

**Why:**
- Consistency: Same process every time
- Documentation: Scripts are executable docs
- Reliability: Less human error
- Speed: Faster than typing commands

**What gets scripted:**
- Development workflows
- Database operations
- Deployment processes
- SSL management
- Health checks

### Script Design Principles

**Safety First:**
- Preserve data by default
- Confirm destructive operations
- Fail fast on errors (set -e)
- Validate before acting

**Environment Awareness:**
- Auto-detect dev/local-prod/production
- Appropriate defaults per environment
- Clear output about what's happening

**User-Friendly:**
- Colored output for clarity
- Help text with --help
- Meaningful error messages
- Progress indicators

## Script Architecture

### Development Scripts
**Purpose**: Daily development workflows

#### dev-start.sh - Start services
**Usage:** `./scripts/dev-start.sh [OPTIONS] [SERVICES]`

**Build Options:**
- `--build` - Force rebuild of containers
- `--rebuild` - Force recreate containers (--force-recreate)
- `--no-cache` - Build without using cache (removes frontend node_modules volume if frontend is targeted)
- `--restart` - Restart existing containers

**Runtime Options:**
- `--logs, -f` - Show logs after starting (runs in foreground)
- `--help, -h` - Show help message

**Services (optional):**
- `postgres`, `backend`, `frontend`, `nginx` - Start specific services only

**Examples:**
- `./scripts/dev-start.sh` - Start all services
- `./scripts/dev-start.sh --restart backend` - Restart just backend
- `./scripts/dev-start.sh --rebuild backend` - Force recreate backend container
- `./scripts/dev-start.sh --build` - Rebuild all services

#### Other Development Scripts
- `dev-logs.sh` - View logs (run `--help` for options)
- `dev-stop.sh` - Stop services
- `health-check.sh` - Verify health

**Why separate from Docker commands:**
- Consistent interface
- Handle environment detection
- Proper service ordering
- User-friendly output
- Self-documenting (--help flag)

### Database Scripts
**Purpose**: Safe database operations

**Key Decision**: Migrations preserve data by default

**Why:**
- Accidents are expensive
- Development data is valuable
- `reset-db.sh` for clean slate
- `setup-db.sh` for safe migrations

**Scripts:**
- `setup-db.sh` - Run migrations (safe)
- `reset-db.sh` - Fresh start (destructive)
- `backup-db.sh` - Backup/restore
- `prepare-sqlx.sh` - Query cache

### Environment Scripts
**Purpose**: Setup different environments

**Why needed:**
- Development: Self-signed SSL
- Local production: Test production config
- Production: Let's Encrypt SSL

**Scripts:**
- `generate-ssl.sh` - Dev/local-prod certs
- `ssl-manager.sh` - Production SSL
- `setup-production-env.sh` - Generate production config
- `setup-local-prod.sh` - Test production locally

### Shared Utilities
**Decision**: Common logic in shared scripts

**Example**: `detect-environment.sh`

**Why:**
- DRY principle
- Consistent detection logic
- Update one place

## Key Script Patterns

### Error Handling
**Pattern**: Set -e for fail-fast

**Why:**
- Stop on first error
- Don't continue with bad state
- Easier to debug

### Environment Detection
**Pattern**: Auto-detect based on files/directories

**Why:**
- No manual flags needed
- Appropriate defaults
- Hard to run wrong environment

### Colored Output
**Pattern**: Green for success, red for errors, yellow for warnings

**Why:**
- Quick visual feedback
- Easy to spot issues
- Better UX

### Docker Compose Abstraction
**Decision**: Scripts hide docker-compose complexity

**Why:**
- `dev-start.sh` vs `docker-compose --env-file .env.development up -d`
- Easier to remember
- Can add logic (health checks, ordering)
- User-friendly

## SSL Management Strategy

### Multiple Certificate Types
**Decision**: Different scripts for different environments

**Why:**
- Development: Quick, self-signed
- Production: Let's Encrypt with renewal
- Different requirements, different tools

**Scripts:**
- `generate-ssl.sh` - Dev/local-prod
- `ssl-manager.sh` - Production with Let's Encrypt

### Automatic Renewal
**Decision**: Cron-based SSL renewal

**Why:**
- Let's Encrypt expires in 90 days
- Manual renewal error-prone
- Automated = reliable

## Maintenance Philosophy

### Scripts as Documentation
**Principle**: Scripts are executable documentation

**Why:**
- Always up-to-date
- Can't be wrong (if it runs, it works)
- New developers run scripts to learn

### Regular Review
**When to update:**
- New features added
- Workflows change
- Errors discovered
- Performance issues

**How:**
- Test scripts regularly
- Update documentation
- Refactor when complex
- Remove unused scripts

## Future Enhancements

**When team grows:**
- GitHub Actions integration
- Automated testing in CI
- Deployment automation
- Health check dashboards

**Current approach sufficient for:**
- Single developer
- Manual deployment
- Interactive workflows
- Rapid iteration
