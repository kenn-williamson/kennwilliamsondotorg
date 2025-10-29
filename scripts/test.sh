#!/bin/bash

# test.sh - Run tests with cargo-nextest
# Usage: ./scripts/test.sh [nextest options]
#
# Examples:
#   ./scripts/test.sh                    # Run all tests with retries
#   ./scripts/test.sh --no-fail-fast     # Continue after first failure
#   ./scripts/test.sh test_name          # Run specific test
#   ./scripts/test.sh -- --nocapture     # Pass args to test binary

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Script directory for relative paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Change to backend directory
cd "$PROJECT_ROOT/backend"

# Check if cargo-nextest is installed
if ! command -v cargo-nextest >/dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  cargo-nextest not found${NC}"
    echo ""
    echo "Installing cargo-nextest..."
    cargo install cargo-nextest
    echo ""
fi

echo -e "${BLUE}🧪 Running tests with cargo-nextest${NC}"
echo -e "${BLUE}===================================${NC}"
echo ""
echo "Configuration:"
echo "  - Retries: 2 (per test)"
echo "  - Threads: 8"
echo "  - Failure output: immediate"
echo ""

# Run tests with nextest
# The config is read from .config/nextest.toml automatically
cargo nextest run "$@"

# Check exit code
EXIT_CODE=$?

echo ""

if [ $EXIT_CODE -eq 0 ]; then
    echo -e "${GREEN}✅ All tests passed!${NC}"
else
    echo -e "${RED}❌ Some tests failed${NC}"
    echo ""
    echo "Flaky test report (if any):"
    echo "  Check output above for tests marked as 'FLAKY'"
    echo ""
    echo "To investigate specific test:"
    echo "  cargo nextest run <test_name> --nocapture"
fi

exit $EXIT_CODE
