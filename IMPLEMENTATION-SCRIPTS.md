# Scripts Implementation - Development Automation

## Overview
Modular script architecture for development workflow automation, database management, and Docker orchestration. Designed for flexibility and single-responsibility principles.

## Script Architecture Strategy

### **Design Philosophy: Modular with Orchestrator**
- **Individual scripts**: Single responsibility, focused tasks
- **Orchestrator scripts**: Combine multiple operations for common workflows
- **Parameterized**: Scripts accept flags for different modes
- **Idempotent**: Safe to run multiple times
- **Error handling**: Fail fast with clear error messages

## Script Inventory

### **Core Infrastructure Scripts**
```
scripts/
├── reset-db.sh         # ✅ Complete database reset (existing)
├── setup-db.sh         # ✅ Migrations only (no reset) - COMPLETE  
├── prepare-sqlx.sh     # ✅ SQLx query cache generation (COMPLETE)
└── health-check.sh     # ✅ Service health verification (COMPLETE)
```

**❌ Rejected Scripts:**
- `build-services.sh` - **Rejected**: Docker Compose commands are already simple (`docker-compose build backend`). Script would add complexity without significant value. Use Docker commands directly.

### **Orchestrator Scripts**
```
scripts/
├── dev-setup.sh        # 🚧 New developer onboarding
├── dev-reset.sh        # 🚧 Nuclear reset + rebuild
├── dev-update.sh       # 🚧 Update after code changes
└── deploy.sh           # ✅ Deployment (existing)
```

## Individual Script Specifications

### **setup-db.sh** - Database Migrations Only ✅ COMPLETE
**Purpose**: Run database migrations without destroying existing data (safe, preserves data)
**Use case**: After schema changes, new developer setup with existing DB

```bash
./scripts/setup-db.sh              # Run pending migrations
./scripts/setup-db.sh --verify     # Run migrations + verify schema
```

**✅ Implemented Features:**
- **Smart Environment Loading**: Loads from project root AND backend .env files
- **Database Auto-Start**: Offers to start PostgreSQL if not running (like prepare-sqlx.sh)
- **Migration Status Reporting**: Shows applied/pending count before and after
- **Safe Operation**: Only runs pending migrations, preserves existing data
- **Schema Verification**: `--verify` option checks tables, extensions, triggers
- **SQLx CLI Management**: Installs sqlx-cli if missing
- **Comprehensive Error Handling**: Clear error messages with environment file paths
- **Next Steps Guidance**: Shows what to do after migrations complete

**✅ Test Results:**
- ✅ All migrations current → Reports status, no unnecessary action
- ✅ Pending migrations → Detects and runs them with progress reporting
- ✅ Database OFF → Auto-start workflow with user confirmation
- ✅ Schema verification → Confirms UUIDv7 extension, tables, triggers exist
- ✅ Environment handling → Successfully loads multiple .env file locations

**Value Add**: Significant improvement over `cd backend && sqlx migrate run` by handling environment setup, database connectivity, and providing clear status reporting.

### **prepare-sqlx.sh** - SQLx Query Cache ✅ COMPLETE
**Purpose**: Generate SQLx offline query cache for Docker builds
**Use case**: After changing SQL queries, before Docker builds

```bash
./scripts/prepare-sqlx.sh          # Generate query cache
./scripts/prepare-sqlx.sh --clean  # Clean + regenerate cache
```

**✅ Implemented Features:**
- **Smart Environment Loading**: Auto-detects `.env.development` or `.env`
- **Database Connectivity**: Uses Docker fallback when `pg_isready` unavailable
- **Auto-Start Database**: Offers to start PostgreSQL if not running
- **Migration Validation**: Checks for pending migrations, fails with helpful guidance
- **Clean Regeneration**: `--clean` removes entire cache and regenerates
- **SQLx CLI Management**: Installs sqlx-cli if missing
- **Git Integration**: Warns about uncommitted `.sqlx` changes
- **Error Handling**: Colored output, clear error messages, safe failure modes

**✅ Test Results:**
- ✅ Database OFF → Auto-start workflow
- ✅ Database ON + Current migrations → Successful generation
- ✅ Database ON + Pending migrations → Helpful failure message
- ✅ Clean operation → Complete removal and regeneration
- ✅ Connectivity check → Docker fallback works correctly

**Scope Decision**: Migration check with helpful failure (maintains separation of concerns)

### **health-check.sh** - Service Health Verification ✅ COMPLETE
**Purpose**: Comprehensive service health verification and resource monitoring
**Use case**: After builds, deployments, debugging service issues

```bash
./scripts/health-check.sh                 # Check all services
./scripts/health-check.sh --wait          # Wait up to 60s for services to become healthy
./scripts/health-check.sh --service postgres  # Check specific service only
```

**✅ Implemented Features:**
- **PostgreSQL Health**: Connection, database existence, table accessibility
- **Backend Health**: API endpoints + database connectivity via `/api/health/db`
- **Frontend Health**: HTTP response verification (ready for when frontend works)
- **Resource Monitoring**: Memory/CPU usage warnings with configurable thresholds
- **Wait Functionality**: `--wait` waits up to 60s with progress reporting
- **Selective Checking**: `--service NAME` for targeted verification
- **Docker Integration**: Container status + resource usage monitoring
- **Comprehensive Reporting**: Clear success/failure with actionable guidance

**✅ Test Results:**
- ✅ Individual service checks → Detailed validation for postgres, backend
- ✅ All services check → Reports healthy and missing services correctly
- ✅ Resource monitoring → Memory/CPU usage warnings at appropriate thresholds
- ✅ Wait functionality → Successfully waits for backend startup
- ✅ Help and argument parsing → Clear usage and proper flag handling

**Value Add**: Significant improvement over manual checking - provides automated verification, integration testing (not just container status), resource monitoring, and actionable error reporting in a single command.

## Orchestrator Script Workflows

### **dev-setup.sh** - New Developer Onboarding
**Purpose**: Complete development environment setup from scratch
**Target user**: New developer, fresh clone

```bash
./scripts/dev-setup.sh                    # Full setup
./scripts/dev-setup.sh --skip-docker     # Setup without Docker builds
```

**Workflow:**
1. Verify prerequisites (Docker, Rust, Node.js)
2. Copy .env.development if needed
3. Run database setup (setup-db.sh)
4. Generate SQLx cache (prepare-sqlx.sh) 
5. Build all services (build-services.sh)
6. Verify health (health-check.sh)
7. Display next steps

### **dev-reset.sh** - Nuclear Option
**Purpose**: Complete reset and rebuild (when things are broken)
**Target user**: Developer with corrupted state

```bash
./scripts/dev-reset.sh             # Full reset + rebuild
./scripts/dev-reset.sh --fast      # Skip some verification steps
```

**Workflow:**
1. Stop all services
2. Reset database (reset-db.sh)
3. Clean Docker images/volumes
4. Full rebuild (dev-setup.sh workflow)

### **dev-update.sh** - Incremental Updates
**Purpose**: Update development environment after code changes
**Target user**: Daily development workflow

```bash
./scripts/dev-update.sh            # Smart update based on changes
./scripts/dev-update.sh --force    # Force all updates
```

**Workflow:**
1. Detect what changed (git diff analysis)
2. Run appropriate subset of scripts:
   - Schema changes → setup-db.sh + prepare-sqlx.sh
   - Rust changes → prepare-sqlx.sh + build backend
   - Frontend changes → build frontend
   - Docker changes → full rebuild

## Error Handling Strategy

### **Common Patterns**
- `set -e`: Exit on first error
- Colored output: 🟢 Success, 🟡 Warning, 🔴 Error
- Cleanup functions: Always clean up on exit
- Progress indicators: Show current step
- Rollback capability: Undo on failure where possible

### **Logging**
- Timestamped logs to `logs/scripts.log`
- Separate error log `logs/errors.log`
- Docker logs integration
- Debug mode with `--verbose` flag

## Integration Points

### **CLAUDE.md Updates**
```markdown
## Development Scripts (Phase 1 Complete)
- ✅ **Modular Architecture**: Individual focused scripts + orchestrators
- ✅ **Database Management**: Automated migrations, resets, SQLx prepare
- ✅ **Docker Integration**: Service builds, health checks, environment management
- ✅ **Developer Onboarding**: Single command setup for new developers
```

### **Docker Integration**
- Scripts use `.env.development` automatically
- Health checks integrated with Docker Compose
- Build optimization with layer caching
- Multi-service orchestration

### **Git Integration**
- SQLx cache committed automatically
- Git hooks for script automation (future)
- Change detection for smart updates
- Pre-commit script validation

## Usage Examples

### **New Developer Workflow**
```bash
git clone repo
cd kennwilliamsondotorg
./scripts/dev-setup.sh
# ☕ Coffee break - everything sets up automatically
```

### **Daily Development**  
```bash
git pull
./scripts/dev-update.sh    # Smart update based on changes
# Ready to develop
```

### **After Database Schema Change**
```bash
# Create migration
cd backend && sqlx migrate add new_feature
# Update environment
./scripts/setup-db.sh && ./scripts/prepare-sqlx.sh
./scripts/build-services.sh backend
```

### **When Things Break**
```bash
./scripts/dev-reset.sh     # Nuclear option
# Everything clean and rebuilt
```

## Future Enhancements

### **Phase 2: CI/CD Integration**
- GitHub Actions integration
- Automated testing workflows
- Production deployment scripts
- Rollback capabilities

### **Phase 3: Advanced Features**
- Parallel execution optimization
- Resource usage monitoring
- Automated performance testing
- Multi-environment support (staging, prod)

### **Phase 4: Developer Experience**
- Interactive script menus
- Configuration validation
- Dependency checking
- Auto-completion support

---

**Status**: Design Complete, Implementation In Progress
**Next**: Create individual scripts following this specification