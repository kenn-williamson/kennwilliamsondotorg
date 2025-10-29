#!/bin/bash
set -e

echo "⏮️ Rolling back KennWilliamson.org deployment..."

# Check if a version is specified
if [ -z "$1" ]; then
    echo "❌ Usage: $0 <version>"
    echo "   Example: $0 v1.0.0"
    echo ""
    echo "   Available versions can be found at:"
    echo "   https://github.com/$GITHUB_USER/kennwilliamsondotorg/pkgs/container/kennwilliamsondotorg-backend"
    exit 1
fi

ROLLBACK_VERSION=$1

# Validate version format (should start with 'v')
if [[ ! $ROLLBACK_VERSION =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "⚠️ Warning: Version '$ROLLBACK_VERSION' doesn't match semantic versioning format (vX.Y.Z)"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Rollback cancelled"
        exit 1
    fi
fi

# Check if .env.production file exists
if [ ! -f .env.production ]; then
    echo "❌ .env.production file not found"
    exit 1
fi

# Set environment variables
export VERSION=$ROLLBACK_VERSION
export GITHUB_USER=${GITHUB_USER:-kenn}

echo "📦 Rolling back to version: $VERSION"
echo "👤 GitHub user: $GITHUB_USER"

# Confirm rollback
read -p "⚠️ This will rollback to $VERSION. Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Rollback cancelled"
    exit 1
fi

# Authenticate with GitHub Container Registry (if credentials provided)
if [ -n "$GITHUB_TOKEN" ]; then
    echo "🔐 Authenticating with GitHub Container Registry..."
    echo "$GITHUB_TOKEN" | docker login ghcr.io -u "$GITHUB_USER" --password-stdin
fi

# Pull specified version from registry
echo "📥 Pulling version $VERSION from registry..."
docker-compose -f docker-compose.production.yml pull

# Stop existing containers
echo "🛑 Stopping current containers..."
docker-compose -f docker-compose.production.yml down --timeout 30

# Start containers with rollback version
echo "▶️ Starting containers with version $VERSION..."
docker-compose -f docker-compose.production.yml up -d

# Wait for services to be healthy
echo "🏥 Waiting for services to be healthy..."
sleep 30

# Check service health
echo "✅ Checking service health..."
docker-compose -f docker-compose.production.yml ps

# Run database migrations (in case rollback version has different schema)
echo "🗃️ Running database migrations for version $VERSION..."
docker-compose -f docker-compose.production.yml run --rm migrations || echo "⚠️ Migration failed or already applied"

echo "🎉 Rollback to version $VERSION completed!"
echo "🌐 Application should be available at https://kennwilliamson.org"

# Log rollback for audit trail
echo "$(date -u +%Y-%m-%dT%H:%M:%SZ) - Rolled back to $VERSION" >> deployment-history.log
