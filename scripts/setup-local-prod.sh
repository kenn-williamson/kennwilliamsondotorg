#!/bin/bash

# setup-local-prod.sh - Set up local production environment for testing
# Usage: ./scripts/setup-local-prod.sh [--build] [--stop-first] [--help]

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
    echo "Usage: $0 [--build] [--stop-first] [--logs] [--help]"
    echo ""
    echo "Set up local production environment for testing production configs locally"
    echo ""
    echo "Options:"
    echo "  --build       Force rebuild of all containers"
    echo "  --stop-first  Stop existing services before starting"
    echo "  --logs        Follow logs after starting services"
    echo "  --help, -h    Show this help message"
    echo ""
    echo "What this script does:"
    echo "  1. Generate self-signed SSL certificates for domain testing"
    echo "  2. Set up local production Docker environment"
    echo "  3. Run database migrations"
    echo "  4. Verify all services are healthy"
    echo ""
    echo "Access points after setup:"
    echo "  • https://localhost (works immediately)"
    echo "  • https://kennwilliamson.org (requires /etc/hosts entry)"
    echo "  • Backend API: https://localhost/api/"
    echo "  • Health check: https://localhost/health"
}

# Parse arguments
BUILD_FLAG=""
STOP_FIRST=false
FOLLOW_LOGS=false

for arg in "$@"; do
    case $arg in
        --build)
            BUILD_FLAG="--build"
            ;;
        --stop-first)
            STOP_FIRST=true
            ;;
        --logs)
            FOLLOW_LOGS=true
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

# Change to project root
cd "$PROJECT_ROOT"

# Check required files
REQUIRED_FILES=(
    ".env.production"
    "docker-compose.yml"
    "docker-compose.local-prod.yml"
    "nginx/conf.d/default.local-prod.conf"
    "nginx/nginx.local-prod.conf"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [[ ! -f "$file" ]]; then
        error "Required file missing: $file"
    fi
done

# Header
echo ""
log "🏭 Setting up Local Production Environment"
echo ""
info "This creates a production-like environment locally for testing:"
echo "  • Production Docker configuration"
echo "  • Production environment variables"
echo "  • Self-signed SSL certificates for HTTPS"
echo "  • All production services and middleware"
echo ""

# Step 1: Generate SSL certificates
log "Step 1: Generating SSL certificates..."
if [[ ! -f "$PROJECT_ROOT/scripts/generate-ssl.sh" ]]; then
    error "SSL generation script not found: scripts/generate-ssl.sh"
fi

"$PROJECT_ROOT/scripts/generate-ssl.sh" local-prod

# Step 2: Stop existing services if requested
if [[ "$STOP_FIRST" == true ]]; then
    log "Step 2: Stopping existing services..."
    docker-compose --env-file .env.production down --remove-orphans || true
fi

# Step 3: Start local production services
log "Step 3: Starting local production services..."
COMPOSE_CMD="docker-compose --env-file .env.production -f docker-compose.yml -f docker-compose.local-prod.yml"

if [[ -n "$BUILD_FLAG" ]]; then
    info "Building containers from scratch..."
    $COMPOSE_CMD build --no-cache
fi

info "Starting services..."
$COMPOSE_CMD up -d

# Step 4: Wait for services to be ready
log "Step 4: Waiting for services to start..."
sleep 5

# Check if containers are running
CONTAINERS=("postgres" "backend" "frontend" "nginx")
for container in "${CONTAINERS[@]}"; do
    if ! docker ps --filter "name=kennwilliamsondotorg-${container}-1" --filter "status=running" | grep -q "${container}"; then
        warn "Container ${container} may not be running properly"
        info "Checking logs for ${container}:"
        docker logs "kennwilliamsondotorg-${container}-1" --tail 10 || true
    else
        log "✅ Container ${container} is running"
    fi
done

# Step 5: Run database migrations
log "Step 5: Setting up database..."
if ! "$PROJECT_ROOT/scripts/setup-db.sh" >/dev/null 2>&1; then
    warn "Database setup may have issues. Running with verbose output:"
    "$PROJECT_ROOT/scripts/setup-db.sh"
else
    log "✅ Database migrations completed"
fi

# Step 6: Health check
log "Step 6: Running health checks..."
sleep 3

if "$PROJECT_ROOT/scripts/health-check.sh" >/dev/null 2>&1; then
    log "✅ All health checks passed!"
else
    warn "Some health checks failed. Running verbose health check:"
    "$PROJECT_ROOT/scripts/health-check.sh"
fi

# Final status
echo ""
log "🚀 Local Production Environment Ready!"
echo ""
info "Access your application:"
echo "  • HTTPS (recommended): https://localhost"
echo "  • HTTP (redirects):     http://localhost"
echo "  • Backend API:         https://localhost/api/"
echo "  • Health check:        https://localhost/health"
echo ""
info "For domain testing (kennwilliamson.org), add to /etc/hosts:"
echo "  127.0.0.1 kennwilliamson.org"
echo "  127.0.0.1 www.kennwilliamson.org"
echo ""
warn "Note: Self-signed certificates will show browser warnings (this is expected)"
echo ""
info "Useful commands:"
echo "  • View logs:      docker-compose --env-file .env.production logs -f"
echo "  • Stop services:  docker-compose --env-file .env.production down"
echo "  • Restart nginx:  docker-compose --env-file .env.production restart nginx"
echo "  • Health check:   ./scripts/health-check.sh"

# Follow logs if requested
if [[ "$FOLLOW_LOGS" == true ]]; then
    echo ""
    log "Following logs (Ctrl+C to exit)..."
    $COMPOSE_CMD logs -f
fi