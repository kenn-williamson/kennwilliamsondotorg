#!/bin/bash
# Environment Detection Script - Shared logic for multi-environment scripts
# Usage: source scripts/detect-environment.sh
# Provides: DETECTED_ENV variable and confirm_environment() function

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Environment detection function
detect_environment() {
    if docker ps --format "{{.Names}}" | grep -q "kennwilliamson-postgres-dev"; then
        DETECTED_ENV="development"
        DETECTED_REASON="kennwilliamson-postgres-dev container found"
    elif docker ps --format "{{.Names}}" | grep -q "postgres"; then
        # Check if other dev containers are running
        if docker ps --format "{{.Names}}" | grep -q "kennwilliamson.*-dev"; then
            DETECTED_ENV="development"
            DETECTED_REASON="development containers found (kennwilliamson-*-dev pattern)"
        else
            # Could be local-prod or production
            # Check for local-prod specific patterns or default to production
            if docker ps --format "{{.Names}}" | grep -q "local-prod\|localprod"; then
                DETECTED_ENV="local-prod"
                DETECTED_REASON="local-prod containers found"
            else
                DETECTED_ENV="production"
                DETECTED_REASON="postgres container without dev naming pattern"
            fi
        fi
    else
        DETECTED_ENV="unknown"
        DETECTED_REASON="no PostgreSQL containers found"
    fi

    echo -e "${BLUE}🔍 Detected: $DETECTED_ENV environment ($DETECTED_REASON)${NC}"
}

# Environment mismatch confirmation function
confirm_environment() {
    local requested="$1"
    local detected="$2"

    echo -e "${YELLOW}⚠️  Requested: $requested environment${NC}"
    echo -e "${YELLOW}⚠️  Detected: $detected environment${NC}"
    echo ""

    # Provide helpful guidance based on detected environment
    if [[ "$detected" == "development" && "$requested" != "development" ]]; then
        echo -e "${BLUE}💡 Tip: Use --dev flag for development containers${NC}"
    elif [[ "$detected" == "production" && "$requested" != "production" ]]; then
        echo -e "${BLUE}💡 Tip: Remove --dev flag for production containers${NC}"
    elif [[ "$detected" == "local-prod" && "$requested" != "local-prod" ]]; then
        echo -e "${BLUE}💡 Tip: Use --local-prod flag for local production testing${NC}"
    fi
    echo ""

    if [[ "$detected" == "unknown" ]]; then
        echo -e "${RED}Cannot proceed: No PostgreSQL containers detected${NC}"
        echo "Start containers first:"
        echo "  Development: ./scripts/dev-start.sh"
        echo "  Local Prod:  ./scripts/setup-local-prod.sh"
        return 1
    fi

    read -p "Continue with $requested configuration against $detected containers? (y/n): " -r
    echo ""

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${GREEN}✅ Proceeding with $requested environment as requested${NC}"
        return 0
    else
        echo -e "${YELLOW}❌ Operation cancelled by user${NC}"
        return 1
    fi
}

# Run detection automatically when sourced
detect_environment

# Export variables for use in other scripts
export DETECTED_ENV
export DETECTED_REASON