#!/bin/bash

# Database Reset Script for KennWilliamson.org
# Tears down existing PostgreSQL container and rebuilds with fresh migrations

set -e  # Exit on any error

echo "🗑️  Stopping and removing PostgreSQL container..."
docker-compose down postgres 2>/dev/null || true

echo "🧹 Removing old database volume..."
docker volume rm kennwilliamsondotorg_postgres_data 2>/dev/null || true

echo "🚀 Starting fresh PostgreSQL container..."
docker-compose up postgres -d

echo "⏳ Waiting for PostgreSQL to be ready..."
sleep 15

# Wait for healthy status
echo "🔍 Checking PostgreSQL health..."
max_attempts=30
attempt=1
while [ $attempt -le $max_attempts ]; do
    if docker-compose ps postgres | grep -q "healthy"; then
        echo "✅ PostgreSQL is healthy!"
        break
    fi
    
    if [ $attempt -eq $max_attempts ]; then
        echo "❌ PostgreSQL failed to become healthy after $max_attempts attempts"
        docker-compose logs postgres --tail 10
        exit 1
    fi
    
    echo "⏳ Attempt $attempt/$max_attempts - waiting for PostgreSQL..."
    sleep 2
    ((attempt++))
done

echo "🏗️  Running database migrations..."
cd backend && sqlx migrate run && cd ..

echo "🧪 Testing UUIDv7 extension..."
docker-compose exec -T postgres psql -U postgres -d kennwilliamson -c "SELECT uuid_generate_v7() as test_uuid;" > /dev/null

echo "📋 Listing database tables..."
docker-compose exec -T postgres psql -U postgres -d kennwilliamson -c "\dt"

echo "👤 Creating test user..."
# Generate hash for "Password123!" using our utility (cost 4 for faster development)
echo "🔑 Generating password hash..."
cd utils/hash_gen
if ! TEST_PASSWORD_HASH=$(cargo run --quiet Password123! 2>/dev/null); then
    echo "❌ Failed to generate password hash"
    cd ../..
    exit 1
fi
cd ../..

if [ -z "$TEST_PASSWORD_HASH" ]; then
    echo "❌ Password hash generation returned empty result"
    exit 1
fi

echo "✅ Password hash generated successfully"

docker-compose exec -T postgres psql -U postgres -d kennwilliamson <<EOF
-- Insert test user
INSERT INTO users (email, password_hash, display_name, slug)
VALUES ('kenn@seqtek.com', '$TEST_PASSWORD_HASH', 'Kenn', 'kenn')
ON CONFLICT (email) DO NOTHING;

-- Assign user role
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id FROM users u, roles r 
WHERE u.email = 'kenn@seqtek.com' AND r.name = 'user'
ON CONFLICT (user_id, role_id) DO NOTHING;

-- Verify test user creation
SELECT email, display_name, slug FROM users WHERE email = 'kenn@seqtek.com';
EOF

echo ""
echo "✅ Database reset complete!"
echo "📊 Database is ready with:"
echo "   • PostgreSQL 17 with UUIDv7 support"
echo "   • All migrations applied"
echo "   • Triggers for updated_at timestamps"
echo ""

# Future: Add seed data loading here
# if [ "$1" == "--seed" ]; then
#     echo "🌱 Loading seed data..."
#     # Add seed data commands here
# fi