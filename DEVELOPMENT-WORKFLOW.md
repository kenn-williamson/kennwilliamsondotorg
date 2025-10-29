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

# Run tests (see IMPLEMENTATION-TESTING.md for detailed testing documentation)
cd backend && cargo test -- --test-threads=4

# Clean up test database when done
./scripts/cleanup-test-db.sh
```

### Test Database Details
- **Test DB**: `kennwilliamson_test` (separate from dev)
- **Connection**: `postgresql://postgres:password@localhost:5432/kennwilliamson_test`
- **Isolation**: Complete separation from development data

## Release & Deployment Workflow

### Automated Release Process (Release-Please)

**Pattern**: Conventional commits trigger automated versioning and releases

#### How It Works

1. **Develop with Conventional Commits**:
   ```bash
   git commit -m "feat: add user dashboard"
   git commit -m "fix: resolve login bug"
   git commit -m "feat!: breaking API change"
   git push origin master
   ```

2. **Release-Please Creates/Updates PR Automatically**:
   - Bot creates PR titled: "chore: release vX.Y.Z"
   - PR shows auto-generated CHANGELOG
   - Includes all commits since last release
   - Version calculated from conventional commits
   - PR updates with each new commit

3. **Release When Ready**:
   - Review the Release PR on GitHub
   - Check version number is correct
   - Review CHANGELOG and included commits
   - **Merge PR when ready to deploy**

4. **Automatic Deployment**:
   - Merge triggers tag creation (e.g., v1.0.0)
   - Tag triggers CD pipeline automatically
   - GitHub Actions → "CD Pipeline - Deploy to Production"
   - Watch build-and-push job (~5-10 minutes)
   - Watch deploy job (~2-5 minutes)
   - Verify at https://kennwilliamson.org

### Conventional Commit Format

**Format**: `<type>: <description>`

**Types and Version Bumps**:
- **feat**: New feature → MINOR version bump (v1.1.0)
- **fix**: Bug fix → PATCH version bump (v1.0.1)
- **feat!**: Breaking change → MAJOR version bump (v2.0.0)
- **BREAKING CHANGE**: In commit body → MAJOR version bump
- **docs**, **chore**, **style**, **refactor**: No version bump

**Examples**:
```bash
feat: add Google OAuth login          # v1.1.0
fix: resolve timer display bug        # v1.0.1
feat!: change API response format     # v2.0.0
docs: update README                   # no version bump

# Breaking change in body
feat: new authentication system

BREAKING CHANGE: JWT token format changed  # v2.0.0
```

### Semantic Versioning Guidelines

- **MAJOR** (v2.0.0): Breaking changes, API changes, database schema incompatibilities
- **MINOR** (v1.1.0): New features, backwards-compatible enhancements
- **PATCH** (v1.0.1): Bug fixes, small improvements, security patches

### Manual Release (Alternative)

**If Release-Please is down or you need immediate release**:
```bash
# Manually create tag
git tag v1.0.0
git push origin v1.0.0
# CD pipeline triggers automatically
```

### Rollback Process

**Option 1: Rollback Script (Fastest)**:
```bash
# On production server
ssh <ec2-host>
cd /home/<user>/app
export GITHUB_USER=<your-github-username>
./scripts/rollback.sh v1.0.0  # Specify version to rollback to
```

**Option 2: Re-tag Previous Commit**:
```bash
# On local machine
git checkout <previous-commit-hash>
git tag v1.0.1  # New tag for rollback version
git push origin v1.0.1
# CD pipeline deploys previous code as new version
```

### CI/CD Status Checks

**Before Merging PRs**:
- ✅ Backend tests passed (cargo test + clippy + audit)
- ✅ Frontend tests passed (vitest + TypeScript + npm audit)
- ✅ Docker builds passed (both backend and frontend)

**Before Creating Release Tag**:
- ✅ All CI checks passed on main
- ✅ Code reviewed (if team grows)
- ✅ Manual testing completed
- ✅ Database migrations tested locally

**See IMPLEMENTATION-CICD.md for complete CI/CD architecture and troubleshooting**

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