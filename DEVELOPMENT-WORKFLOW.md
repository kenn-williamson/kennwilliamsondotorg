# Development Workflow Guide

## Quick Start

```bash
# Start development environment
./scripts/dev-start.sh
./scripts/health-check.sh --dev

# Access: https://localhost
```

## Daily Workflows

### Development
- **Start**: `./scripts/dev-start.sh`
- **Health Check**: `./scripts/health-check.sh --dev`
- **Logs**: `./scripts/dev-logs.sh [service]`
- **Stop**: `./scripts/dev-stop.sh`

### Code Changes
- **Frontend**: Edit `frontend/app/` - HMR updates instantly
- **Backend**: Edit `backend/src/` - cargo-watch rebuilds automatically

### Database
```bash
# After schema changes
./scripts/setup-db.sh              # Run migrations
./scripts/prepare-sqlx.sh --clean  # Update query cache
./scripts/dev-start.sh --rebuild backend
```

## Scripts Reference

### Core Scripts
- **`dev-start.sh`**: Start services (`--build`, `--rebuild service`, `--logs`)
- **`dev-stop.sh`**: Stop services (`--remove` to remove containers)
- **`dev-logs.sh`**: View logs (`[service]`, `--tail N`, `--timestamps`)
- **`health-check.sh`**: Verify health (`--dev`, `--local-prod`, `--wait`, `--service SERVICE`)

### Database Scripts
- **`setup-db.sh`**: Run migrations (preserves data)
- **`prepare-sqlx.sh`**: Generate query cache (`--clean` to regenerate)
- **`reset-db.sh`**: Fresh database start
- **`backup-db.sh`**: Backup/restore database
- **`setup-test-db.sh`**: Create test database with migrations
- **`cleanup-test-db.sh`**: Drop test database

## Troubleshooting

### Quick Fixes
1. **Health Check**: `./scripts/health-check.sh --dev`
2. **View Logs**: `./scripts/dev-logs.sh [service] --tail 50`
3. **Restart Service**: `./scripts/dev-start.sh --rebuild [service]`
4. **Full Reset**: `./scripts/dev-stop.sh --remove && ./scripts/dev-start.sh --build`

### Common Issues
- **SQLx errors**: `./scripts/prepare-sqlx.sh --clean`
- **Database issues**: `./scripts/setup-db.sh --verify`
- **Port conflicts**: `./scripts/dev-stop.sh --remove && docker system prune`

## Local Production Testing

```bash
# Set up local production environment
./scripts/setup-local-prod.sh

# Test production config
curl -k https://localhost

# Stop when done
docker-compose --env-file .env.production -f docker-compose.yml -f docker-compose.local-prod.yml down
```

## Testing Workflow

### Test Database Setup
```bash
# Start development environment (if not running)
./scripts/dev-start.sh

# Create test database with migrations
./scripts/setup-test-db.sh

# Run tests
cd backend && cargo test -- --test-threads 1

# Clean up test database when done
./scripts/cleanup-test-db.sh
```

### Test Database Details
- **Test DB**: `kennwilliamson_test` (separate from dev)
- **Connection**: `postgresql://postgres:password@localhost:5432/kennwilliamson_test`
- **Isolation**: Complete separation from development data

## Access Points
- **Main App**: https://localhost
- **Backend API**: http://localhost:8080/backend/
- **Database**: postgresql://postgres:postgres@localhost:5432/kennwilliamson

## Best Practices
- Use scripts instead of manual Docker commands
- Check health after major changes
- Update SQLx cache after SQL changes
---

*This workflow guide should be your primary reference for daily development activities. It emphasizes using the project's automated scripts and established patterns for consistency and reliability.*