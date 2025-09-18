#!/bin/bash
# Update _sqlx_migrations table with local testing support

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

# Logging functions
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

ENVIRONMENT="development"
DRY_RUN=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --prod)
            ENVIRONMENT="production"
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [--prod] [--dry-run]"
            echo ""
            echo "Options:"
            echo "  --prod     Use production database"
            echo "  --dry-run  Show SQL without executing"
            echo ""
            echo "Examples:"
            echo "  $0                    # Update development database"
            echo "  $0 --dry-run         # Show SQL for development"
            echo "  $0 --prod --dry-run  # Show SQL for production"
            echo "  $0 --prod            # Update production database"
            exit 0
            ;;
        *)
            echo "Usage: $0 [--prod] [--dry-run]"
            echo "  --prod     Use production database"
            echo "  --dry-run  Show SQL without executing"
            exit 1
            ;;
    esac
done

# Calculate checksum for initial schema migration
MIGRATION_FILE="$PROJECT_ROOT/backend/migrations/20250914134643_initial_schema.up.sql"
if [ ! -f "$MIGRATION_FILE" ]; then
    error "Migration file not found: $MIGRATION_FILE
    Please run this script from the project root directory"
fi

# SQLx uses SHA-384 for checksums, stored as hex
CHECKSUM=$(sha384sum "$MIGRATION_FILE" | cut -d' ' -f1)
info "📝 Calculated checksum: $CHECKSUM"

# Auto-detect environment by checking running containers
POSTGRES_CONTAINER=""
if docker ps --format "{{.Names}}" | grep -q "kennwilliamson-postgres-dev"; then
    ENVIRONMENT="development"
    POSTGRES_CONTAINER="kennwilliamson-postgres-dev"
    DB_NAME="kennwilliamson"
    DB_USER="postgres"
elif docker ps --format "{{.Names}}" | grep -q "postgres"; then
    # Could be local-prod or production
    POSTGRES_CONTAINER=$(docker ps --format "{{.Names}}" | grep postgres | head -1)
    if [ "$ENVIRONMENT" == "development" ]; then
        ENVIRONMENT="local-prod"  # Override if containers suggest otherwise
    fi
    DB_NAME="${DB_NAME:-kennwilliamson}"
    DB_USER="${DB_USER:-postgres}"
else
    error "No PostgreSQL container found
    Make sure Docker containers are running:
    - Development: ./scripts/dev-start.sh
    - Local Prod:  ./scripts/setup-local-prod.sh"
fi

info "🔍 Detected environment: $ENVIRONMENT"
info "📦 PostgreSQL container: $POSTGRES_CONTAINER"
info "🗄️ Target database: $DB_NAME"

# Generate SQL
SQL_SCRIPT=$(cat <<EOF
-- Show current migrations
SELECT 'Current migrations:' as info;
SELECT version, description, installed_on FROM _sqlx_migrations ORDER BY version;

-- Clear existing migration records (consolidating)
DELETE FROM _sqlx_migrations;

-- Insert consolidated initial schema migration as applied
INSERT INTO _sqlx_migrations (version, description, installed_on, success, checksum, execution_time)
VALUES (
    20250914134643,
    'initial_schema',
    NOW(),
    true,
    decode('$CHECKSUM', 'hex'),
    0
);

-- Show updated migrations table
SELECT 'Updated migrations:' as info;
SELECT version, description, installed_on FROM _sqlx_migrations ORDER BY version;
EOF
)

if [ "$DRY_RUN" == "true" ]; then
    info "🧪 DRY RUN - SQL that would be executed:"
    echo "$SQL_SCRIPT"
else
    info "▶️ Executing migration table update..."
    echo "$SQL_SCRIPT" | docker exec -i "$POSTGRES_CONTAINER" \
        psql -U "$DB_USER" -d "$DB_NAME"

    if [ $? -eq 0 ]; then
        log "✅ Migration table updated successfully"
        info "🚀 Ready for: sqlx migrate run (migrations 2 & 3)"
    else
        error "Migration table update failed"
    fi
fi