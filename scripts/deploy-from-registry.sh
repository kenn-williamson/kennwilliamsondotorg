#!/bin/bash
set -e

echo "🚀 Deploying KennWilliamson.org from GitHub Container Registry..."

# Validate required environment variables
if [ -z "$VERSION" ]; then
    echo "⚠️ VERSION environment variable not set, using 'latest'"
    VERSION="latest"
fi

if [ -z "$GITHUB_USER" ]; then
    echo "❌ GITHUB_USER environment variable not set"
    exit 1
fi

# Check if .env file exists
if [ ! -f .env.production ]; then
    echo "❌ .env.production file not found. Please create it from .env.example"
    exit 1
fi

# Export version for docker-compose
export VERSION
export GITHUB_USER

echo "📦 Deploying version: $VERSION"
echo "👤 GitHub user: $GITHUB_USER"

# Authenticate with GitHub Container Registry (if credentials provided)
if [ -n "$GITHUB_TOKEN" ]; then
    echo "🔐 Authenticating with GitHub Container Registry..."
    echo "$GITHUB_TOKEN" | docker login ghcr.io -u "$GITHUB_USER" --password-stdin
else
    echo "ℹ️  No GITHUB_TOKEN provided, assuming public images or already authenticated"
fi

# Pull latest images from registry
echo "📥 Pulling Docker images from registry..."
docker-compose -f docker-compose.production.yml pull

# Stop existing containers gracefully
echo "🛑 Stopping existing containers..."
docker-compose -f docker-compose.production.yml down --timeout 30

# Start new containers
echo "▶️ Starting containers..."
docker-compose -f docker-compose.production.yml up -d

# Wait for services to be healthy
echo "🏥 Waiting for services to be healthy..."
sleep 30

# Check service health
echo "✅ Checking service health..."
docker-compose -f docker-compose.production.yml ps

# Run database migrations
echo "🗃️ Running database migrations..."
docker-compose -f docker-compose.production.yml run --rm migrations || echo "⚠️ Migration failed or already applied"

# Cleanup old images to save disk space
echo "🧹 Cleaning up old Docker images..."
docker image prune -f

echo "🎉 Deployment completed!"
echo "🌐 Application should be available at https://kennwilliamson.org"
echo "📊 Deployed version: $VERSION"
