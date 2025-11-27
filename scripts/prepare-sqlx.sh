#!/bin/bash

# prepare-sqlx.sh - Generate SQLx offline query cache for Docker builds
# Usage: ./scripts/prepare-sqlx.sh [--clean]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Script directory for relative paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BACKEND_DIR="$PROJECT_ROOT/backend"

log() {
    echo -e "${GREEN}[$(date +'%H:%M:%S')] $1${NC}"
}

warn() {
    echo -e "${YELLOW}[$(date +'%H:%M:%S')] WARNING: $1${NC}"
}

error() {
    echo -e "${RED}[$(date +'%H:%M:%S')] ERROR: $1${NC}"
    exit 1
}

# Check if we're in the right directory
if [[ ! -f "$BACKEND_DIR/Cargo.toml" ]]; then
    error "Backend directory not found at $BACKEND_DIR"
fi

# Change to backend directory
cd "$BACKEND_DIR"

# Load environment and construct DATABASE_URL
if [[ -f "$PROJECT_ROOT/.env.development" ]]; then
    log "Loading environment from .env.development file"
    source "$PROJECT_ROOT/.env.development"
elif [[ -f "$PROJECT_ROOT/.env" ]]; then
    log "Loading environment from .env file"
    source "$PROJECT_ROOT/.env"
else
    error "No environment file found (.env or .env.development required)"
fi

# Construct DATABASE_URL if not provided (for host-side execution)
if [[ -z "$DATABASE_URL" ]]; then
    if [[ -n "$DB_USER" && -n "$DB_PASSWORD" ]]; then
        DATABASE_URL="postgresql://${DB_USER}:${DB_PASSWORD}@localhost:5432/kennwilliamson"
        log "Constructed DATABASE_URL from environment variables"
    else
        error "DATABASE_URL not set and cannot construct from DB_USER/DB_PASSWORD"
    fi
fi

# Convert Docker network hostname to localhost for host-side script execution
if [[ "$DATABASE_URL" == *"@postgres:"* ]]; then
    DATABASE_URL="${DATABASE_URL/@postgres:/@localhost:}"
    log "Converting DATABASE_URL for host-side execution: postgres -> localhost"
fi

export DATABASE_URL

# Check if database is accessible
log "Verifying database connectivity..."

# Function to test database connectivity
test_db_connection() {
    # Try direct connection if pg_isready is available
    if command -v pg_isready >/dev/null 2>&1; then
        pg_isready -d "$DATABASE_URL" >/dev/null 2>&1
    else
        # Use Docker to test connectivity
        docker compose exec -T postgres pg_isready -U postgres >/dev/null 2>&1
    fi
}

if ! test_db_connection; then
    warn "Cannot connect to database"
    
    # Check if Docker Compose is available
    if docker compose version >/dev/null 2>&1; then
        echo "Would you like to start PostgreSQL? (y/N)"
        read -r response
        if [[ "$response" =~ ^[Yy]$ ]]; then
            log "Starting PostgreSQL service..."
            cd "$PROJECT_ROOT"
            docker compose up postgres -d
            
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
            error "Database is required for SQLx preparation. Start it with: docker compose up postgres -d"
        fi
    else
        error "Cannot connect to database and docker compose not available"
    fi
fi

# Check if migrations are current
log "Checking migration status..."
cd "$BACKEND_DIR"
if ! sqlx migrate info >/dev/null 2>&1; then
    error "Cannot check migration status. Ensure DATABASE_URL is correct and sqlx-cli is installed."
fi

# Check for pending migrations
PENDING_MIGRATIONS=$(sqlx migrate info | grep -c "pending" || true)
if [[ "$PENDING_MIGRATIONS" -gt 0 ]]; then
    error "❌ Database has $PENDING_MIGRATIONS pending migrations. SQLx cache requires current schema.

Run migrations first:
  cd backend && sqlx migrate run

Or use the setup script:
  ./scripts/setup-db.sh

Then retry:
  ./scripts/prepare-sqlx.sh"
fi

log "✅ Database migrations are current"

# Clean option - remove existing cache
if [[ "$1" == "--clean" ]]; then
    log "Cleaning existing SQLx query cache..."
    if [[ -d ".sqlx" ]]; then
        rm -rf .sqlx
        log "Removed .sqlx directory"
    fi
fi

# Check if sqlx-cli is installed
if ! command -v sqlx >/dev/null 2>&1; then
    log "SQLx CLI not found, installing..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Generate query cache
log "Generating SQLx offline query cache..."
if cargo sqlx prepare --workspace; then
    log "✅ SQLx query cache generated successfully"
else
    error "Failed to generate SQLx query cache"
fi

# Verify cache was created
if [[ ! -d ".sqlx" ]]; then
    error "SQLx cache directory not found after generation"
fi

# Count cached queries
QUERY_COUNT=$(find .sqlx -name "*.json" | wc -l)
log "Generated cache for $QUERY_COUNT queries"

# Check if cache should be committed
if [[ -d "$PROJECT_ROOT/.git" ]]; then
    cd "$PROJECT_ROOT"
    if git status --porcelain | grep -q "backend/.sqlx"; then
        warn "SQLx cache has changes. Consider committing to git:"
        echo "  git add backend/.sqlx"
        echo "  git commit -m '[CHORE] Update SQLx query cache'"
    else
        log "SQLx cache is up to date in git"
    fi
fi

log "🚀 SQLx preparation complete! Ready for Docker builds."