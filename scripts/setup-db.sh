#!/bin/bash

# setup-db.sh - Database migrations without reset (preserves existing data)
# Usage: ./scripts/setup-db.sh [--verify] [--dev]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory for relative paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BACKEND_DIR="$PROJECT_ROOT/backend"

log() {
    echo -e "${GREEN}[$(date +'%H:%M:%S')] $1${NC}"
}

info() {
    echo -e "${BLUE}[$(date +'%H:%M:%S')] $1${NC}"
}

warn() {
    echo -e "${YELLOW}[$(date +'%H:%M:%S')] WARNING: $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%H:%M:%S')] ERROR: $1${NC}"
    exit 1
}

show_help() {
    echo "Usage: $0 [--verify] [--dev]"
    echo ""
    echo "Setup database with migrations (preserves existing data)"
    echo ""
    echo "Options:"
    echo "  --verify      Run migrations + verify schema state"
    echo "  --dev         Use development environment (.env.development)"
    echo "  --help, -h    Show this help message"
    echo ""
    echo "Default: Uses production environment (.env.production or .env)"
    echo ""
    echo "Features:"
    echo "  - Auto-starts PostgreSQL if not running"
    echo "  - Loads proper environment files"
    echo "  - Shows migration status before/after"
    echo "  - Verifies database extensions (UUIDv7)"
    echo "  - Safe: only runs pending migrations"
}

# Parse arguments
VERIFY_SCHEMA=false
DEV_MODE=false

for arg in "$@"; do
    case $arg in
        --verify)
            VERIFY_SCHEMA=true
            ;;
        --dev)
            DEV_MODE=true
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
        *)
            error "Unknown argument: $arg. Use --help for usage information."
            ;;
    esac
done

# Change to project root directory
cd "$PROJECT_ROOT"

# Check if we're in the right directory
if [[ ! -f "$BACKEND_DIR/Cargo.toml" ]]; then
    error "Backend directory not found at $BACKEND_DIR"
fi

# Initialize Docker Compose command
COMPOSE_CMD="docker-compose"

# Load environment based on mode
if [[ "$DEV_MODE" == true ]]; then
    # Development mode - load dev environment
    if [[ -f "$PROJECT_ROOT/.env.development" ]]; then
        export $(grep -v '^#' "$PROJECT_ROOT/.env.development" | xargs) 2>/dev/null || true
        log "Using development environment"
        COMPOSE_CMD="$COMPOSE_CMD --env-file .env.development -f docker-compose.yml -f docker-compose.development.yml"
    else
        error "Development mode requested but .env.development not found"
    fi
else
    # Production mode (default) - load production environment
    if [[ -f "$PROJECT_ROOT/.env.production" ]]; then
        export $(grep -v '^#' "$PROJECT_ROOT/.env.production" | xargs) 2>/dev/null || true
        log "Using production environment"
        COMPOSE_CMD="$COMPOSE_CMD --env-file .env.production -f docker-compose.yml"
    elif [[ -f "$PROJECT_ROOT/.env" ]]; then
        export $(grep -v '^#' "$PROJECT_ROOT/.env" | xargs) 2>/dev/null || true
        log "Using .env file"
        COMPOSE_CMD="$COMPOSE_CMD --env-file .env -f docker-compose.yml"
    else
        error "No production environment file found (.env.production or .env)"
    fi
fi

# Note: Backend .env file is not used - Docker Compose provides all environment variables

# Verify DATABASE_URL is set
if [[ -z "$DATABASE_URL" ]]; then
    error "DATABASE_URL not set. Check environment files:
  - $PROJECT_ROOT/.env.development
  - $PROJECT_ROOT/.env.production
  - $PROJECT_ROOT/.env"
fi

# Convert Docker network hostname to localhost for host-side script execution
# Docker containers use 'postgres' hostname, but host scripts need 'localhost'
if [[ "$DATABASE_URL" == *"@postgres:"* ]]; then
    SCRIPT_DATABASE_URL="${DATABASE_URL/@postgres:/@localhost:}"
    info "Converting DATABASE_URL for host-side execution: postgres -> localhost"
else
    SCRIPT_DATABASE_URL="$DATABASE_URL"
fi

# Function to test database connectivity
test_db_connection() {
    # Extract database details from SCRIPT_DATABASE_URL for connection test
    DB_HOST=$(echo "$SCRIPT_DATABASE_URL" | sed -n 's|.*@\([^:]*\):.*|\1|p')
    DB_PORT=$(echo "$SCRIPT_DATABASE_URL" | sed -n 's|.*:\([0-9]*\)/.*|\1|p')
    DB_NAME=$(echo "$SCRIPT_DATABASE_URL" | sed -n 's|.*/\([^?]*\).*|\1|p')
    
    # Try direct connection if pg_isready is available
    if command -v pg_isready >/dev/null 2>&1; then
        pg_isready -h "$DB_HOST" -p "$DB_PORT" -d "$DB_NAME" >/dev/null 2>&1
    else
        # Use Docker to test connectivity
        cd "$PROJECT_ROOT"
        $COMPOSE_CMD exec -T postgres pg_isready -U postgres -h localhost -p 5432 >/dev/null 2>&1
    fi
}

# Check if database is accessible
log "Verifying database connectivity..."
if ! test_db_connection; then
    warn "Cannot connect to database"
    
    # Check if Docker Compose is available
    if command -v docker-compose >/dev/null 2>&1; then
        echo "Would you like to start PostgreSQL? (y/N)"
        read -r response
        if [[ "$response" =~ ^[Yy]$ ]]; then
            log "Starting PostgreSQL service..."
            cd "$PROJECT_ROOT"
            $COMPOSE_CMD up postgres -d
            
            # Wait for database to be ready
            log "Waiting for database to be ready..."
            for i in {1..30}; do
                if test_db_connection; then
                    log "Database is ready!"
                    break
                fi
                if [[ $i -eq 30 ]]; then
                    error "Database failed to start after 30 seconds"
                fi
                sleep 1
            done
            cd "$BACKEND_DIR"
        else
            error "Database is required for migrations. Start it with: $COMPOSE_CMD up postgres -d"
        fi
    else
        error "Cannot connect to database and docker-compose not available"
    fi
fi

# Change to backend directory for SQLx operations
cd "$BACKEND_DIR"

# Set DATABASE_URL for SQLx operations (using the host-accessible version)
export DATABASE_URL="$SCRIPT_DATABASE_URL"

# Check if sqlx-cli is installed
if ! command -v sqlx >/dev/null 2>&1; then
    log "SQLx CLI not found, installing..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Show current migration status
log "Checking current migration status..."
if ! sqlx migrate info >/dev/null 2>&1; then
    error "Cannot check migration status. Ensure DATABASE_URL is correct."
fi

PENDING_COUNT=$(sqlx migrate info | grep -c "pending" || true)
APPLIED_COUNT=$(sqlx migrate info | grep -c "applied" || true)

info "Migration Status:"
echo "  - Applied: $APPLIED_COUNT migrations"
if [[ "$PENDING_COUNT" -gt 0 ]]; then
    echo "  - Pending: $PENDING_COUNT migrations"
    
    log "Running pending migrations..."
    if sqlx migrate run; then
        log "✅ Migrations completed successfully"
        
        # Show new status
        NEW_APPLIED=$(sqlx migrate info | grep -c "applied" || true)
        COMPLETED=$((NEW_APPLIED - APPLIED_COUNT))
        info "Applied $COMPLETED new migrations"
    else
        error "Migration failed. Check database logs and connection."
    fi
else
    log "✅ All migrations are already applied"
fi

# Verify database extensions and schema
if [[ "$VERIFY_SCHEMA" == true ]]; then
    log "Verifying database schema..."
    
    # Extract database name from SCRIPT_DATABASE_URL
    DB_NAME=$(echo "$SCRIPT_DATABASE_URL" | sed -n 's|.*/\([^?]*\).*|\1|p')
    
    # Check UUIDv7 extension
    if $COMPOSE_CMD exec -T postgres psql -U postgres -d "$DB_NAME" -c "SELECT 1 FROM pg_extension WHERE extname = 'pg_uuidv7';" >/dev/null 2>&1; then
        log "✅ UUIDv7 extension is installed"
    else
        warn "⚠️  UUIDv7 extension not found"
    fi
    
    # Check key tables exist
    TABLES=("users" "roles" "user_roles" "incident_timers")
    for table in "${TABLES[@]}"; do
        if $COMPOSE_CMD exec -T postgres psql -U postgres -d "$DB_NAME" -c "SELECT 1 FROM information_schema.tables WHERE table_name = '$table';" >/dev/null 2>&1; then
            log "✅ Table '$table' exists"
        else
            warn "⚠️  Table '$table' not found"
        fi
    done
    
    # Check timestamp triggers
    if $COMPOSE_CMD exec -T postgres psql -U postgres -d "$DB_NAME" -c "SELECT 1 FROM information_schema.triggers WHERE trigger_name LIKE '%updated_at%';" >/dev/null 2>&1; then
        log "✅ Timestamp triggers are configured"
    else
        warn "⚠️  Timestamp triggers not found"
    fi
fi

# Final status
log "🚀 Database setup complete!"
echo ""
info "Next steps:"
echo "  - Generate SQLx cache: ./scripts/prepare-sqlx.sh"
echo "  - Start all services: $COMPOSE_CMD up -d"
echo "  - Reset database: ./scripts/reset-db.sh (destroys data!)"
echo "  - Check service health: ./scripts/health-check.sh (when available)"