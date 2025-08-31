#!/bin/bash

# Development Environment Logs Script
# View logs from development services

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

echo -e "${BLUE}📋 KennWilliamson.org Development Logs${NC}"
echo ""

# Parse command line arguments
SERVICES=""
FOLLOW_FLAG="-f"
TAIL_LINES=""
TIMESTAMPS=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --no-follow)
            FOLLOW_FLAG=""
            shift
            ;;
        --tail)
            TAIL_LINES="--tail $2"
            shift 2
            ;;
        --timestamps|-t)
            TIMESTAMPS="--timestamps"
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS] [SERVICES]"
            echo ""
            echo "Options:"
            echo "  --no-follow     Don't follow logs (show and exit)"
            echo "  --tail N        Show last N lines from each service"
            echo "  --timestamps    Show timestamps"
            echo "  --help, -h      Show this help message"
            echo ""
            echo "Services (optional):"
            echo "  postgres        Show only PostgreSQL logs"
            echo "  backend         Show only backend API logs"
            echo "  frontend        Show only frontend logs"
            echo ""
            echo "Examples:"
            echo "  $0                    # Follow all development service logs"
            echo "  $0 --no-follow       # Show recent logs and exit"
            echo "  $0 --tail 50         # Show last 50 lines from each service"
            echo "  $0 backend           # Follow only backend logs"
            echo "  $0 --timestamps backend frontend  # Backend and frontend with timestamps"
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
CMD="docker-compose --env-file $ENV_FILE $COMPOSE_FILES logs $FOLLOW_FLAG $TAIL_LINES $TIMESTAMPS $SERVICES"

echo -e "${YELLOW}📋 Running: $CMD${NC}"

# Show what we're doing
if [ -n "$SERVICES" ]; then
    echo -e "${BLUE}📝 Showing logs for:$SERVICES${NC}"
else
    echo -e "${BLUE}📝 Showing logs for: all services${NC}"
fi

if [ -n "$FOLLOW_FLAG" ]; then
    echo -e "${YELLOW}ℹ️  Following logs (Ctrl+C to stop)${NC}"
else
    echo -e "${YELLOW}ℹ️  Showing recent logs${NC}"
fi

echo ""

# Execute the command
eval $CMD