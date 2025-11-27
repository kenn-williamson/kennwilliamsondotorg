#!/bin/bash

#######################################################
# ci-check.sh - Local CI Validation
#######################################################
# Runs the same checks as GitHub Actions CI locally
# to catch issues before pushing.
#
# Usage:
#   ./scripts/ci-check.sh              # Run all checks
#   ./scripts/ci-check.sh backend      # Backend only
#   ./scripts/ci-check.sh frontend     # Frontend only
#######################################################

set -e  # Exit on first error

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Determine what to check
CHECK_BACKEND=true
CHECK_FRONTEND=true

if [ "$1" = "backend" ]; then
    CHECK_FRONTEND=false
elif [ "$1" = "frontend" ]; then
    CHECK_BACKEND=false
fi

# Track failures
FAILED_CHECKS=()

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Local CI Validation${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

#######################################################
# Backend Checks
#######################################################
if [ "$CHECK_BACKEND" = true ]; then
    echo -e "${BLUE}[1/7] Backend Clippy (--all-targets)${NC}"
    cd "$PROJECT_ROOT/backend"
    if cargo clippy --locked --all-targets --all-features -- -D warnings; then
        echo -e "${GREEN}✓ Backend clippy passed${NC}\n"
    else
        echo -e "${RED}✗ Backend clippy failed${NC}\n"
        FAILED_CHECKS+=("backend-clippy")
    fi

    echo -e "${BLUE}[2/7] Backend Tests (this may take a few minutes...)${NC}"
    # Use pooled testcontainers for faster parallel test execution
    # TESTCONTAINER_POOL_SIZE=8 maintains 8 reusable containers
    # --test-threads=8 runs tests in parallel across those containers
    export TESTCONTAINER_POOL_SIZE=8
    if cargo nextest run --locked --all-features --test-threads=8 || cargo test --locked --all-features -- --test-threads=8; then
        echo -e "${GREEN}✓ Backend tests passed${NC}\n"
    else
        echo -e "${RED}✗ Backend tests failed${NC}\n"
        FAILED_CHECKS+=("backend-tests")
    fi

    echo -e "${BLUE}[3/7] Backend Security Audit${NC}"
    if cargo audit --deny warnings; then
        echo -e "${GREEN}✓ Backend audit passed${NC}\n"
    else
        echo -e "${RED}✗ Backend audit failed${NC}\n"
        FAILED_CHECKS+=("backend-audit")
    fi

    echo -e "${BLUE}[4/7] Backend SQLx Check (optional)${NC}"
    if [ -d "$PROJECT_ROOT/backend/.sqlx" ]; then
        if cargo sqlx prepare --check -- --all-targets 2>/dev/null; then
            echo -e "${GREEN}✓ SQLx metadata is up to date${NC}\n"
        else
            echo -e "${YELLOW}⚠ SQLx metadata may need update (run ./scripts/prepare-sqlx.sh)${NC}\n"
            echo -e "${YELLOW}  Note: CI doesn't validate this, so won't block your push${NC}\n"
        fi
    else
        echo -e "${YELLOW}⚠ SQLx offline mode not configured (skipping)${NC}\n"
    fi
fi

#######################################################
# Frontend Checks
#######################################################
if [ "$CHECK_FRONTEND" = true ]; then
    echo -e "${BLUE}[5/7] Frontend TypeScript Check${NC}"
    cd "$PROJECT_ROOT/frontend"
    if npx vue-tsc --noEmit; then
        echo -e "${GREEN}✓ Frontend TypeScript passed${NC}\n"
    else
        echo -e "${RED}✗ Frontend TypeScript failed${NC}\n"
        FAILED_CHECKS+=("frontend-typescript")
    fi

    echo -e "${BLUE}[6/7] Frontend Tests${NC}"
    # Show output to see test progress
    if npm run test:coverage; then
        echo -e "${GREEN}✓ Frontend tests passed${NC}\n"
    else
        echo -e "${RED}✗ Frontend tests failed${NC}\n"
        FAILED_CHECKS+=("frontend-tests")
    fi

    echo -e "${BLUE}[7/7] Frontend Security Audit${NC}"
    if npm audit --audit-level=high; then
        echo -e "${GREEN}✓ Frontend audit passed${NC}\n"
    else
        echo -e "${RED}✗ Frontend audit failed${NC}\n"
        FAILED_CHECKS+=("frontend-audit")
    fi
fi

#######################################################
# Summary
#######################################################
echo -e "${BLUE}========================================${NC}"
if [ ${#FAILED_CHECKS[@]} -eq 0 ]; then
    echo -e "${GREEN}✅ All CI checks passed!${NC}"
    echo -e "${GREEN}   Safe to push to remote${NC}"
    echo -e "${BLUE}========================================${NC}"
    exit 0
else
    echo -e "${RED}❌ CI checks failed:${NC}"
    for check in "${FAILED_CHECKS[@]}"; do
        echo -e "${RED}   - $check${NC}"
    done
    echo -e "${BLUE}========================================${NC}"
    echo -e "${YELLOW}Fix these issues before pushing${NC}"
    exit 1
fi
