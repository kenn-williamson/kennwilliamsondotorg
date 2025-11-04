#!/bin/bash

# health-check.sh - Comprehensive service health verification
# Usage: ./scripts/health-check.sh [--wait] [--service SERVICE] [--dev]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
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

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

failure() {
    echo -e "${RED}❌ $1${NC}"
}

show_help() {
    echo "Usage: $0 [--wait] [--service SERVICE] [--dev] [--local-prod] [--verbose]"
    echo ""
    echo "Verify service health and connectivity"
    echo ""
    echo "Options:"
    echo "  --wait             Wait up to 60s for services to become healthy"
    echo "  --service SERVICE  Check only specific service (postgres, frontend, backend)"
    echo "  --dev              Use development environment"
    echo "  --local-prod       Use local production environment"
    echo "  --verbose          Show detailed logs and diagnostic information"
    echo "  --help, -h         Show this help message"
    echo ""
    echo "Default: Uses production environment"
    echo ""
    echo "Health Checks:"
    echo "  - PostgreSQL: Database connectivity and schema"
    echo "  - Backend: API endpoints and database connection"
    echo "  - Frontend: HTTP response and asset loading"
    echo "  - Docker: Container status and resource usage"
}

# Parse arguments
WAIT_FOR_HEALTHY=false
TARGET_SERVICE=""
EXPECT_SERVICE=false
DEV_MODE=false
LOCAL_PROD_MODE=false
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --wait)
            WAIT_FOR_HEALTHY=true
            shift
            ;;
        --dev)
            DEV_MODE=true
            shift
            ;;
        --local-prod)
            LOCAL_PROD_MODE=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --service)
            EXPECT_SERVICE=true
            shift
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
        *)
            if [[ "$EXPECT_SERVICE" == true ]]; then
                TARGET_SERVICE="$1"
                EXPECT_SERVICE=false
                shift
            else
                error "Unknown argument: $1. Use --help for usage information."
            fi
            ;;
    esac
done

# Change to project root
cd "$PROJECT_ROOT"

# Check if docker-compose is available
if ! command -v docker-compose >/dev/null 2>&1; then
    error "docker-compose not found. Please install Docker Compose."
fi

# Environment detection and validation
source scripts/detect-environment.sh

# Check for environment mismatches
REQUESTED_ENV="production"  # Default
if [[ "$DEV_MODE" == true ]]; then
    REQUESTED_ENV="development"
elif [[ "$LOCAL_PROD_MODE" == true ]]; then
    REQUESTED_ENV="local-prod"
fi

if [[ "$REQUESTED_ENV" != "$DETECTED_ENV" ]]; then
    if ! confirm_environment "$REQUESTED_ENV" "$DETECTED_ENV"; then
        exit 1
    fi
fi

# Load environment based on mode
if [[ "$DEV_MODE" == true ]]; then
    # Development mode - load dev environment
    if [[ -f ".env.development" ]]; then
        export $(grep -v '^#' .env.development | xargs) 2>/dev/null || true
        info "Using development environment"
    else
        warn "Development mode requested but .env.development not found"
    fi
elif [[ "$LOCAL_PROD_MODE" == true ]]; then
    # Local production mode - load production environment
    if [[ -f ".env.production" ]]; then
        export $(grep -v '^#' .env.production | xargs) 2>/dev/null || true
        info "Using local production environment"
    else
        warn "Local production mode requested but .env.production not found"
    fi
else
    # Production mode (default) - load production environment
    if [[ -f ".env.production" ]]; then
        export $(grep -v '^#' .env.production | xargs) 2>/dev/null || true
        info "Using production environment"
    elif [[ -f ".env" ]]; then
        export $(grep -v '^#' .env | xargs) 2>/dev/null || true
        info "Using .env file"
    else
        warn "No production environment file found (.env.production or .env)"
    fi
fi

# Set docker-compose command based on mode
if [[ "$DEV_MODE" == true ]]; then
    COMPOSE_CMD="docker-compose --env-file .env.development -f docker-compose.yml -f docker-compose.development.yml"
elif [[ "$LOCAL_PROD_MODE" == true ]]; then
    COMPOSE_CMD="docker-compose --env-file .env.production -f docker-compose.yml -f docker-compose.local-prod.yml"
else
    COMPOSE_CMD="docker-compose --env-file .env.production -f docker-compose.yml"
fi

# Get available services
AVAILABLE_SERVICES=$($COMPOSE_CMD config --services 2>/dev/null || echo "postgres backend frontend")

# Validate target service
if [[ -n "$TARGET_SERVICE" ]]; then
    if ! echo "$AVAILABLE_SERVICES" | grep -q "\b$TARGET_SERVICE\b"; then
        error "Unknown service: $TARGET_SERVICE. Available: $AVAILABLE_SERVICES"
    fi
fi

# Health check functions
check_postgres() {
    local service_name="postgres"
    info "Checking PostgreSQL health..."
    
    # Check container status
    if ! $COMPOSE_CMD ps "$service_name" | grep -q "Up"; then
        failure "PostgreSQL container is not running"
        return 1
    fi
    
    # Check database connectivity
    if $COMPOSE_CMD exec -T "$service_name" pg_isready -U postgres >/dev/null 2>&1; then
        success "PostgreSQL is accepting connections"
    else
        failure "PostgreSQL is not accepting connections"
        return 1
    fi
    
    # Check database exists
    if $COMPOSE_CMD exec -T "$service_name" psql -U postgres -lqt | cut -d \| -f 1 | grep -qw kennwilliamson; then
        success "Database 'kennwilliamson' exists"
    else
        failure "Database 'kennwilliamson' not found"
        return 1
    fi
    
    # Check key tables
    local tables=("users" "incident_timers")
    for table in "${tables[@]}"; do
        if $COMPOSE_CMD exec -T "$service_name" psql -U postgres -d kennwilliamson -c "SELECT 1 FROM $table LIMIT 1;" >/dev/null 2>&1; then
            success "Table '$table' is accessible"
        else
            failure "Table '$table' is not accessible"
            return 1
        fi
    done
    
    return 0
}

check_backend() {
    local service_name="backend"
    info "Checking Backend API health..."

    # Check container status
    if ! $COMPOSE_CMD ps "$service_name" | grep -q "Up"; then
        failure "Backend container is not running"
        if [[ "$VERBOSE" == true ]]; then
            echo ""
            echo "Container status:"
            $COMPOSE_CMD ps "$service_name"
            echo ""
            echo "Recent backend logs:"
            $COMPOSE_CMD logs --tail=50 "$service_name"
        fi
        return 1
    fi

    if [[ "$VERBOSE" == true ]]; then
        echo ""
        echo "Container status:"
        $COMPOSE_CMD ps "$service_name"
        echo ""
        echo "Recent backend logs (last 30 lines):"
        $COMPOSE_CMD logs --tail=30 "$service_name"
        echo ""
        echo "Testing health endpoint..."
    fi

    # Check health endpoint - try multiple methods
    # 1. Try nginx proxy (production only)
    # 2. Try direct host access (development with exposed ports)
    # 3. Try from inside container (always works)

    if curl -f -s -k https://localhost/backend/public/health >/dev/null 2>&1; then
        success "Backend health endpoint responding (via nginx proxy)"
    elif curl -f -s http://localhost:8080/backend/public/health >/dev/null 2>&1; then
        success "Backend health endpoint responding (direct access)"
    elif $COMPOSE_CMD exec -T "$service_name" wget -q --spider http://127.0.0.1:8080/backend/public/health 2>&1; then
        success "Backend health endpoint responding (from inside container)"
    else
        failure "Backend health endpoint not responding"
        if [[ "$VERBOSE" == true ]]; then
            echo ""
            echo "Attempting to reach health endpoint from inside container:"
            $COMPOSE_CMD exec -T "$service_name" wget -O- http://127.0.0.1:8080/backend/public/health 2>&1 || echo "Failed"
            echo ""
            echo "Full backend logs:"
            $COMPOSE_CMD logs "$service_name"
        fi
        return 1
    fi

    # Check database health endpoint - try multiple methods
    if curl -f -s -k https://localhost/backend/public/health/db >/dev/null 2>&1; then
        success "Backend database connectivity OK (via nginx proxy)"
    elif curl -f -s http://localhost:8080/backend/public/health/db >/dev/null 2>&1; then
        success "Backend database connectivity OK (direct access)"
    elif $COMPOSE_CMD exec -T "$service_name" wget -q --spider http://127.0.0.1:8080/backend/public/health/db 2>&1; then
        success "Backend database connectivity OK (from inside container)"
    else
        failure "Backend cannot connect to database"
        if [[ "$VERBOSE" == true ]]; then
            echo ""
            echo "Full backend logs:"
            $COMPOSE_CMD logs "$service_name"
        fi
        return 1
    fi

    return 0
}

check_frontend() {
    local service_name="frontend"
    info "Checking Frontend health..."

    # Check container status
    if ! $COMPOSE_CMD ps "$service_name" | grep -q "Up"; then
        failure "Frontend container is not running"
        if [[ "$VERBOSE" == true ]]; then
            echo ""
            echo "Container status:"
            $COMPOSE_CMD ps "$service_name"
            echo ""
            echo "Recent frontend logs:"
            $COMPOSE_CMD logs --tail=50 "$service_name"
        fi
        return 1
    fi

    # Check HTTP response - try multiple methods
    # 1. Try nginx proxy (production only)
    # 2. Try direct host access (development with exposed ports)
    # 3. Try from inside container (always works)

    if curl -f -s -k -o /dev/null https://localhost/; then
        success "Frontend is serving HTTP requests (via nginx proxy)"
    elif curl -f -s -o /dev/null http://localhost:3000/; then
        success "Frontend is serving HTTP requests (direct access)"
    elif $COMPOSE_CMD exec -T "$service_name" wget -q --spider http://localhost:3000/ 2>&1; then
        success "Frontend is serving HTTP requests (from inside container)"
    else
        failure "Frontend is not responding to HTTP requests"
        if [[ "$VERBOSE" == true ]]; then
            echo ""
            echo "Full frontend logs:"
            $COMPOSE_CMD logs "$service_name"
        fi
        return 1
    fi

    return 0
}

check_service_resources() {
    info "Checking service resource usage..."
    
    # Memory usage
    local high_memory_services=$(docker stats --no-stream --format "{{.Container}}: {{.MemUsage}}" | grep -E '([5-9][0-9]{2}MiB|[0-9]\.?[0-9]*GiB)')
    if [[ -n "$high_memory_services" ]]; then
        warn "High memory usage detected:"
        echo "$high_memory_services"
    fi
    
    # CPU usage  
    local high_cpu_services=$(docker stats --no-stream --format "{{.Container}}: {{.CPUPerc}}" | grep -E '([8-9][0-9]\.|100\.)')
    if [[ -n "$high_cpu_services" ]]; then
        warn "High CPU usage detected:"
        echo "$high_cpu_services"
    fi
}

# Wait for services function
wait_for_service() {
    local service="$1"
    local max_attempts=30
    local attempt=1
    
    info "Waiting for $service to become healthy..."
    
    while [[ $attempt -le $max_attempts ]]; do
        if check_"$service" >/dev/null 2>&1; then
            log "$service is healthy!"
            return 0
        fi
        
        if [[ $attempt -eq $max_attempts ]]; then
            error "$service failed to become healthy after ${max_attempts}s"
        fi
        
        sleep 2
        ((attempt++))
    done
    
    return 1
}

# Main health check execution
main() {
    log "🏥 Starting health check..."
    echo ""
    
    local overall_status=0
    local services_to_check
    
    if [[ -n "$TARGET_SERVICE" ]]; then
        services_to_check=("$TARGET_SERVICE")
    else
        # Default check order: dependencies first
        services_to_check=("postgres")
        if echo "$AVAILABLE_SERVICES" | grep -q "backend"; then
            services_to_check+=("backend")
        fi
        if echo "$AVAILABLE_SERVICES" | grep -q "frontend"; then
            services_to_check+=("frontend")
        fi
    fi
    
    for service in "${services_to_check[@]}"; do
        if [[ "$WAIT_FOR_HEALTHY" == true ]]; then
            if ! wait_for_service "$service"; then
                overall_status=1
            fi
        else
            if ! check_"$service"; then
                overall_status=1
            fi
        fi
        echo ""
    done
    
    # Resource check (only if checking all services)
    if [[ -z "$TARGET_SERVICE" ]]; then
        check_service_resources
        echo ""
    fi
    
    # Summary
    if [[ $overall_status -eq 0 ]]; then
        log "🚀 All health checks passed!"
        echo ""
        info "Services are healthy and ready for use"
    else
        error "❌ Some health checks failed!"
        echo ""
        info "Check the output above for specific issues"
        info "Try: $COMPOSE_CMD logs [service] for more details"
        return 1
    fi
}

# Run main function
main "$@"