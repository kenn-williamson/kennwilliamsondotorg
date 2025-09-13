#!/bin/bash

# Development Environment Startup Script
# Starts all services with proper development configuration

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
ENV_FILE=".env.development"
COMPOSE_FILES="-f docker-compose.yml -f docker-compose.development.yml"

echo -e "${BLUE}🚀 Starting KennWilliamson.org Development Environment${NC}"
echo "   Environment: ${ENV_FILE}"
echo "   Compose files: docker-compose.yml + docker-compose.development.yml"
echo ""

# Check if environment file exists
if [ ! -f "$ENV_FILE" ]; then
    echo -e "${RED}❌ Error: $ENV_FILE not found${NC}"
    echo "   Please create $ENV_FILE with development configuration"
    exit 1
fi

# Check and generate SSL certificates if needed
SSL_CERT="nginx/ssl/localhost.crt"
SSL_KEY="nginx/ssl/localhost.key"
if [ ! -f "$SSL_CERT" ] || [ ! -f "$SSL_KEY" ]; then
    echo -e "${YELLOW}🔒 SSL certificates not found, generating development certificates...${NC}"
    if [ -f "scripts/generate-ssl.sh" ]; then
        ./scripts/generate-ssl.sh dev
        echo -e "${GREEN}✅ Development SSL certificates generated${NC}"
    else
        echo -e "${RED}❌ Error: scripts/generate-ssl.sh not found${NC}"
        echo "   Cannot generate SSL certificates automatically"
        exit 1
    fi
fi

# Parse command line arguments
SERVICES=""
BUILD_FLAG=""
NO_CACHE_FLAG=""
RESTART_FLAG=""
REBUILD_FLAG=""
DETACHED="-d"
LOGS_FLAG=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --build)
            BUILD_FLAG="--build"
            shift
            ;;
        --rebuild)
            REBUILD_FLAG="--rebuild"
            BUILD_FLAG="--build"
            shift
            ;;
        --no-cache)
            NO_CACHE_FLAG="--no-cache"
            BUILD_FLAG="--build"
            shift
            ;;
        --restart)
            RESTART_FLAG="--restart"
            shift
            ;;
        --logs|-f)
            LOGS_FLAG="--logs"
            DETACHED=""  # Don't run detached if showing logs
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS] [SERVICES]"
            echo ""
            echo "Build Options:"
            echo "  --build         Force rebuild of containers"
            echo "  --rebuild       Force recreate containers (--force-recreate)"
            echo "  --no-cache      Build without using cache (removes frontend node_modules volume if frontend is targeted)"
            echo "  --restart       Restart existing containers"
            echo ""
            echo "Runtime Options:"
            echo "  --logs, -f      Show logs after starting (runs in foreground)"
            echo "  --help, -h      Show this help message"
            echo ""
            echo "Services (optional):"
            echo "  postgres        Start only PostgreSQL"
            echo "  backend         Start only backend API"
            echo "  frontend        Start only frontend"
            echo "  nginx           Start only nginx proxy"
            echo ""
            echo "Examples:"
            echo "  $0                          # Start all development services"
            echo "  $0 --build                 # Rebuild and start all services"
            echo "  $0 --rebuild backend       # Force recreate backend container"
            echo "  $0 --no-cache frontend     # Rebuild frontend without cache"
            echo "  $0 --restart postgres      # Restart PostgreSQL only"
            echo "  $0 --logs                  # Start services and show logs"
            echo "  $0 backend frontend        # Start only backend and frontend"
            exit 0
            ;;
        postgres|backend|frontend|nginx)
            SERVICES="$SERVICES $1"
            shift
            ;;
        *)
            echo -e "${RED}❌ Unknown option: $1${NC}"
            echo "   Use --help for usage information"
            exit 1
            ;;
    esac
done

# Handle different operation modes
if [ -n "$RESTART_FLAG" ]; then
    # Restart mode - just restart existing containers
    CMD="docker-compose --env-file $ENV_FILE $COMPOSE_FILES restart $SERVICES"
    echo -e "${YELLOW}🔄 Restarting containers: $CMD${NC}"
else
    # Normal start/build mode
    DOCKER_FLAGS="$BUILD_FLAG"
    
    # Add force recreate flag if rebuild requested
    if [ -n "$REBUILD_FLAG" ]; then
        DOCKER_FLAGS="$DOCKER_FLAGS --force-recreate"
    fi
    
    # Handle no-cache builds (requires separate build step)
    if [ -n "$NO_CACHE_FLAG" ]; then
        echo -e "${YELLOW}🏗️  Building without cache...${NC}"
        
        # Remove frontend node_modules volume if frontend is being rebuilt with --no-cache
        if [[ "$SERVICES" == *"frontend"* ]] || [ -z "$SERVICES" ]; then
            FRONTEND_VOLUME="kennwilliamsondotorg_frontend_node_modules_dev"
            if docker volume ls -q | grep -q "^${FRONTEND_VOLUME}$"; then
                echo -e "${YELLOW}🗑️  Removing frontend node_modules volume for clean rebuild...${NC}"
                
                # Stop and remove frontend container if running
                FRONTEND_CONTAINER="kennwilliamson-frontend-dev"
                if docker ps -a --format "table {{.Names}}" | grep -q "^${FRONTEND_CONTAINER}$"; then
                    echo "   Stopping and removing frontend container..."
                    docker stop "$FRONTEND_CONTAINER" 2>/dev/null || true
                    docker rm "$FRONTEND_CONTAINER" 2>/dev/null || true
                fi
                
                # Remove the volume
                if docker volume rm "$FRONTEND_VOLUME" 2>/dev/null; then
                    echo -e "${GREEN}✅ Frontend node_modules volume removed${NC}"
                else
                    echo -e "${YELLOW}⚠️  Could not remove frontend node_modules volume (may not exist)${NC}"
                fi
            fi
        fi
        
        BUILD_CMD="docker-compose --env-file $ENV_FILE $COMPOSE_FILES build --no-cache $SERVICES"
        echo "   Running: $BUILD_CMD"
        
        if ! eval $BUILD_CMD; then
            echo -e "${RED}❌ Build failed${NC}"
            exit 1
        fi
        echo ""
    fi
    
    # Build the main command
    CMD="docker-compose --env-file $ENV_FILE $COMPOSE_FILES up $DETACHED $DOCKER_FLAGS $SERVICES"
    echo -e "${YELLOW}📦 Starting services: $CMD${NC}"
fi

echo ""

# Execute the command
if eval $CMD; then
    echo ""
    if [ -n "$DETACHED" ]; then
        echo -e "${GREEN}✅ Development environment started successfully!${NC}"
        echo ""
        echo -e "${BLUE}🔗 Services:${NC}"
        echo "   🌐 Main Site:  https://localhost (nginx proxy - recommended)"
        echo "   🖥️  Frontend:   http://localhost:3000 (direct access)"
        echo "   🔧 Backend:    http://localhost:8080 (direct access)"
        echo "   🗄️  Database:   localhost:5432"
        echo ""
        echo -e "${BLUE}📋 Useful commands:${NC}"
        echo "   ./scripts/dev-logs.sh           # View logs"
        echo "   ./scripts/dev-stop.sh           # Stop services"
        echo "   docker-compose --env-file $ENV_FILE ps  # Check status"
        echo ""
        
        # Show if we started specific services
        if [ -n "$SERVICES" ]; then
            echo -e "${YELLOW}ℹ️  Started only:$SERVICES${NC}"
            echo "   Other services may need to be started separately"
        fi
    fi
    
    # If logs requested, show them
    if [ -n "$LOGS_FLAG" ]; then
        echo -e "${BLUE}📋 Following logs (Ctrl+C to stop):${NC}"
        docker-compose --env-file $ENV_FILE $COMPOSE_FILES logs -f $SERVICES
    fi
else
    echo -e "${RED}❌ Failed to start development environment${NC}"
    exit 1
fi