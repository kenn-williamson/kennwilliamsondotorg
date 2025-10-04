#!/bin/bash
# Cleanup test containers script
# Removes all pg_uuidv7 containers except the development environment container

set -e

echo "🧹 Cleaning up test containers..."

# Get all containers using the pg_uuidv7 image
CONTAINERS=$(docker ps -a --filter "ancestor=ghcr.io/fboulnois/pg_uuidv7:1.6.0" --format "{{.ID}} {{.Names}}" | grep -v "kennwilliamson-postgres-dev" || true)

if [ -z "$CONTAINERS" ]; then
    echo "✅ No test containers to clean up"
    exit 0
fi

echo "Found test containers to remove:"
echo "$CONTAINERS"
echo ""

# Extract just the container IDs
CONTAINER_IDS=$(echo "$CONTAINERS" | awk '{print $1}')

# Count containers
COUNT=$(echo "$CONTAINER_IDS" | wc -l)
echo "📊 Removing $COUNT test containers..."

# Stop and remove each container
echo "$CONTAINER_IDS" | while read -r CONTAINER_ID; do
    if [ -n "$CONTAINER_ID" ]; then
        echo "  🛑 Stopping and removing container $CONTAINER_ID..."
        docker stop "$CONTAINER_ID" 2>/dev/null || true
        docker rm "$CONTAINER_ID" 2>/dev/null || true
    fi
done

echo "✅ Test container cleanup complete!"
