#!/bin/bash
# Database backup and restore script with local testing support

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

# Parse command line arguments
COMMAND=""
ENVIRONMENT="development"
BACKUP_FILE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        backup)
            COMMAND="backup"
            shift
            ;;
        restore)
            COMMAND="restore"
            shift
            ;;
        --prod)
            ENVIRONMENT="production"
            shift
            ;;
        --file)
            BACKUP_FILE="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 <backup|restore> [--prod] [--file BACKUP_FILE]"
            echo ""
            echo "Commands:"
            echo "  backup              Create a database backup"
            echo "  restore             Restore database from backup"
            echo ""
            echo "Options:"
            echo "  --prod              Use production environment"
            echo "  --file FILE         Specify backup file for restore"
            echo ""
            echo "Examples:"
            echo "  $0 backup                                    # Backup development DB"
            echo "  $0 backup --prod                             # Backup production DB"
            echo "  $0 restore --file backup.sql.gz             # Restore dev DB"
            echo "  $0 restore --prod --file backup.sql.gz      # Restore production DB"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use -h or --help for usage information"
            exit 1
            ;;
    esac
done

if [ -z "$COMMAND" ]; then
    error "Please specify backup or restore command. Use -h or --help for usage information"
fi

# Directory validation - critical for container mount
if [ ! -d "$PROJECT_ROOT/backups" ]; then
    error "backups directory not found at $PROJECT_ROOT/backups
    This directory is required for PostgreSQL container mount
    Ensure Docker containers are running:
    - Development: ./scripts/dev-start.sh
    - Local Prod:  ./scripts/setup-local-prod.sh"
fi

# Environment detection and validation
source scripts/detect-environment.sh

# Check for environment mismatches
REQUESTED_ENV="development"  # Default
if [[ "$ENVIRONMENT" == "production" ]]; then
    REQUESTED_ENV="production"
fi

if [[ "$REQUESTED_ENV" != "$DETECTED_ENV" ]]; then
    if ! confirm_environment "$REQUESTED_ENV" "$DETECTED_ENV"; then
        exit 1
    fi
fi

# Auto-detect environment by checking running containers (now for container names)
POSTGRES_CONTAINER=""
if docker ps --format "{{.Names}}" | grep -q "kennwilliamson-postgres-dev"; then
    ENVIRONMENT="development"
    POSTGRES_CONTAINER="kennwilliamson-postgres-dev"
    DB_NAME="kennwilliamson"
    DB_USER="postgres"
elif docker ps --format "{{.Names}}" | grep -q "postgres"; then
    POSTGRES_CONTAINER=$(docker ps --format "{{.Names}}" | grep postgres | head -1)

    # Environment file loading based on --prod flag
    if [ "$ENVIRONMENT" == "production" ]; then
        ENV_FILE="$PROJECT_ROOT/.env.production"
        if [ -f "$ENV_FILE" ]; then
            log "Loading production environment from $ENV_FILE"
            set -a; source "$ENV_FILE"; set +a
        fi
    else
        ENVIRONMENT="local-prod"
        ENV_FILE="$PROJECT_ROOT/.env.production"
        if [ -f "$ENV_FILE" ]; then
            log "Loading local-prod environment from $ENV_FILE"
            set -a; source "$ENV_FILE"; set +a
        fi
    fi
    DB_NAME="${DB_NAME:-kennwilliamson}"
    DB_USER="${DB_USER:-postgres}"
else
    error "No PostgreSQL container found
    Make sure Docker containers are running:
    - Development: ./scripts/dev-start.sh
    - Local Prod:  ./scripts/setup-local-prod.sh"
fi

log "Detected environment: $ENVIRONMENT"
info "PostgreSQL container: $POSTGRES_CONTAINER"

# Backup directory is mounted in containers at /backups
BACKUP_DIR="/backups"

# Execute backup or restore command
if [ "$COMMAND" == "backup" ]; then
    TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
    BACKUP_FILE="kennwilliamson_${ENVIRONMENT}_backup_${TIMESTAMP}.sql.gz"

    log "Creating $ENVIRONMENT database backup..."
    info "Backup file: $BACKUP_FILE"

    # Create compressed backup using docker exec
    docker exec "$POSTGRES_CONTAINER" pg_dump \
        -U "$DB_USER" -d "$DB_NAME" \
        --verbose --clean --no-acl --no-owner | gzip > "./backups/$BACKUP_FILE"

    if [ $? -eq 0 ]; then
        log "✅ Backup created: ./backups/$BACKUP_FILE"
        info "📦 Backup size: $(ls -lh "./backups/$BACKUP_FILE" | awk '{print $5}')"
    else
        error "Backup failed"
    fi

elif [ "$COMMAND" == "restore" ]; then
    if [ -z "$BACKUP_FILE" ]; then
        error "--file option is required for restore
        Use: $0 restore --file backup.sql.gz"
    fi

    # Check if backup file exists (try both relative and absolute paths)
    RESTORE_PATH=""
    if [ -f "$BACKUP_FILE" ]; then
        RESTORE_PATH="$BACKUP_FILE"
    elif [ -f "$PROJECT_ROOT/backups/$BACKUP_FILE" ]; then
        RESTORE_PATH="$PROJECT_ROOT/backups/$BACKUP_FILE"
    else
        error "Backup file not found: $BACKUP_FILE
        Checked paths:
        - $BACKUP_FILE
        - $PROJECT_ROOT/backups/$BACKUP_FILE"
    fi

    warn "This will completely replace the $ENVIRONMENT database!"
    info "Container: $POSTGRES_CONTAINER"
    info "Database: $DB_NAME"
    info "Backup file: $RESTORE_PATH"
    info "File size: $(ls -lh "$RESTORE_PATH" | awk '{print $5}')"
    echo ""
    read -p "Are you sure you want to continue? (yes/no): " CONFIRM

    if [ "$CONFIRM" != "yes" ]; then
        log "Restore cancelled"
        exit 0
    fi

    log "Restoring $ENVIRONMENT database from backup..."

    # Restore from compressed backup using docker exec
    gunzip -c "$RESTORE_PATH" | docker exec -i "$POSTGRES_CONTAINER" \
        psql -U "$DB_USER" -d "$DB_NAME"

    if [ $? -eq 0 ]; then
        log "✅ Database restored successfully"
        info "🔄 Restored from: $RESTORE_PATH"
    else
        error "Restore failed"
    fi
fi