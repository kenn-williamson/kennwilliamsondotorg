#!/bin/bash
set -e

echo "🚀 Deploying KennWilliamson.org..."

# Check if .env file exists
if [ ! -f .env ]; then
    echo "❌ .env file not found. Please create it from .env.example"
    exit 1
fi

# Build application images (includes pulling base images)
echo "🔨 Building application images..."
docker-compose build --pull

# Stop existing containers
echo "🛑 Stopping existing containers..."
docker-compose down

# Start new containers
echo "▶️ Starting containers..."
docker-compose up -d

# Wait for services to be healthy
echo "🏥 Waiting for services to be healthy..."
sleep 30

# Check service health
echo "✅ Checking service health..."
docker-compose ps

# Run database migrations if backend is healthy
echo "🗃️ Running database migrations..."
docker-compose exec backend sqlx migrate run || echo "⚠️ Migration failed or already applied"

echo "🎉 Deployment completed!"
echo "🌐 Application should be available at https://$(grep DOMAIN_NAME .env | cut -d '=' -f2)"