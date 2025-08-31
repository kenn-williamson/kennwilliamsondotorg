#!/bin/bash

# Development Environment Stop Script
# Stops development services cleanly

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

echo -e "${BLUE}🛑 Stopping KennWilliamson.org Development Environment${NC}"
echo ""

# Parse command line arguments
SERVICES=""
REMOVE_FLAG=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --remove)
            REMOVE_FLAG="--remove-orphans"
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS] [SERVICES]"
            echo ""
            echo "Options:"
            echo "  --remove       Remove containers and networks"
            echo "  --help, -h     Show this help message"
            echo ""
            echo "Services (optional):"
            echo "  postgres       Stop only PostgreSQL"
            echo "  backend        Stop only backend API"
            echo "  frontend       Stop only frontend"
            echo ""
            echo "Examples:"
            echo "  $0                # Stop all development services"
            echo "  $0 --remove       # Stop and remove all containers"
            echo "  $0 backend        # Stop only backend service"
            exit 0
            ;;
        postgres|backend|frontend)
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

# Build command
if [ -n "$REMOVE_FLAG" ]; then
    CMD="docker-compose --env-file $ENV_FILE $COMPOSE_FILES down $REMOVE_FLAG $SERVICES"
    echo -e "${YELLOW}🗑️  Stopping and removing: $CMD${NC}"
else
    CMD="docker-compose --env-file $ENV_FILE $COMPOSE_FILES stop $SERVICES"
    echo -e "${YELLOW}⏹️  Stopping: $CMD${NC}"
fi

echo ""

# Execute the command
if eval $CMD; then
    echo ""
    echo -e "${GREEN}✅ Development environment stopped successfully!${NC}"
    
    if [ -n "$SERVICES" ]; then
        echo -e "${YELLOW}ℹ️  Stopped only:$SERVICES${NC}"
        echo "   Other services may still be running"
    fi
else
    echo -e "${RED}❌ Failed to stop development environment${NC}"
    exit 1
fi