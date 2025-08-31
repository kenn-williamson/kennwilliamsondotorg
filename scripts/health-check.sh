#!/bin/bash

# health-check.sh - Comprehensive service health verification
# Usage: ./scripts/health-check.sh [--wait] [--service SERVICE]

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
    echo "Usage: $0 [--wait] [--service SERVICE]"
    echo ""
    echo "Verify service health and connectivity"
    echo ""
    echo "Options:"
    echo "  --wait             Wait up to 60s for services to become healthy"
    echo "  --service SERVICE  Check only specific service (postgres, frontend, backend)"
    echo "  --help, -h         Show this help message"
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

while [[ $# -gt 0 ]]; do
    case $1 in
        --wait)
            WAIT_FOR_HEALTHY=true
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

# Load environment if available
if [[ -f ".env.development" ]]; then
    export $(grep -v '^#' .env.development | xargs) 2>/dev/null || true
elif [[ -f ".env" ]]; then
    export $(grep -v '^#' .env | xargs) 2>/dev/null || true
fi

# Get available services
AVAILABLE_SERVICES=$(docker-compose config --services 2>/dev/null || echo "postgres backend frontend")

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
    if ! docker-compose ps "$service_name" | grep -q "Up"; then
        failure "PostgreSQL container is not running"
        return 1
    fi
    
    # Check database connectivity
    if docker-compose exec -T "$service_name" pg_isready -U postgres >/dev/null 2>&1; then
        success "PostgreSQL is accepting connections"
    else
        failure "PostgreSQL is not accepting connections"
        return 1
    fi
    
    # Check database exists
    if docker-compose exec -T "$service_name" psql -U postgres -lqt | cut -d \| -f 1 | grep -qw kennwilliamson; then
        success "Database 'kennwilliamson' exists"
    else
        failure "Database 'kennwilliamson' not found"
        return 1
    fi
    
    # Check key tables
    local tables=("users" "incident_timers")
    for table in "${tables[@]}"; do
        if docker-compose exec -T "$service_name" psql -U postgres -d kennwilliamson -c "SELECT 1 FROM $table LIMIT 1;" >/dev/null 2>&1; then
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
    if ! docker-compose ps "$service_name" | grep -q "Up"; then
        failure "Backend container is not running"
        return 1
    fi
    
    # Check health endpoint
    if curl -f -s http://localhost:8080/health >/dev/null 2>&1; then
        success "Backend health endpoint responding"
    else
        failure "Backend health endpoint not responding"
        return 1
    fi
    
    # Check database health endpoint
    if curl -f -s http://localhost:8080/api/health/db >/dev/null 2>&1; then
        success "Backend database connectivity OK"
    else
        failure "Backend cannot connect to database"
        return 1
    fi
    
    return 0
}

check_frontend() {
    local service_name="frontend"
    info "Checking Frontend health..."
    
    # Check container status
    if ! docker-compose ps "$service_name" | grep -q "Up"; then
        failure "Frontend container is not running"
        return 1
    fi
    
    # Check HTTP response
    if curl -f -s -o /dev/null http://localhost:3000/; then
        success "Frontend is serving HTTP requests"
    else
        failure "Frontend is not responding to HTTP requests"
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
        info "Try: docker-compose logs [service] for more details"
        return 1
    fi
}

# Run main function
main "$@"