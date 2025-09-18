# Scripts Implementation

## Overview
Development automation scripts for workflow management, database operations, and Docker orchestration.

## Scripts by Category

### Core Development
- **`dev-start.sh`**: Service management with flexible startup options
- **`dev-stop.sh`**: Clean service shutdown with removal options
- **`dev-logs.sh`**: Log viewing with filtering and formatting

### Database Management
- **`setup-db.sh`**: Safe database migration management (preserves data)
- **`reset-db.sh`**: Complete database reset for development
- **`backup-db.sh`**: Database backup and restore with environment detection
- **`download-backup.sh`**: Download database backups from remote servers
- **`prepare-sqlx.sh`**: SQLx query cache generation for Docker builds
- **`update-migrations-table.sh`**: Update SQLx migrations table

### Health & Monitoring
- **`health-check.sh`**: Comprehensive service health verification
- **`test-auth.sh`**: Test authentication endpoints

### Environment Setup
- **`setup-production-env.sh`**: Secure production environment generation
- **`generate-ssl.sh`**: SSL certificate generation for development and local production
- **`setup-local-prod.sh`**: Complete local production environment setup
- **`ssl-manager.sh`**: Let's Encrypt SSL certificate management

### Deployment
- **`deploy.sh`**: Production deployment script

### Utilities
- **`detect-environment.sh`**: Shared environment detection logic

## Key Features

### Database Management
- **Migration Safety**: `setup-db.sh` preserves existing data
- **Environment Detection**: Automatic detection of dev/prod environments
- **Backup/Restore**: `backup-db.sh` and `download-backup.sh` for data management
- **SQLx Integration**: Query cache management for Docker builds

### Health Monitoring
- **Service Health**: PostgreSQL, backend API, and frontend verification
- **Resource Monitoring**: Container resource usage tracking
- **Environment Validation**: Mismatch detection and warnings

### SSL Certificate Management
- **Development**: `generate-ssl.sh` for localhost certificates
- **Local Production**: Domain certificates for production testing
- **Production**: `ssl-manager.sh` for Let's Encrypt integration

### Environment Setup
- **Production**: `setup-production-env.sh` for secure environment generation
- **Local Production**: `setup-local-prod.sh` for complete local production setup

## Usage Examples

### Development Workflow
```bash
# Start development environment
./scripts/dev-start.sh

# Check service health
./scripts/health-check.sh --dev

# Run database migrations
./scripts/setup-db.sh --dev

# View logs
./scripts/dev-logs.sh backend
```

### Database Operations
```bash
# Reset database
./scripts/reset-db.sh

# Backup database
./scripts/backup-db.sh

# Download backup from remote
./scripts/download-backup.sh user@server:/path/to/backup.sql
```

### SSL Management
```bash
# Generate development certificates
./scripts/generate-ssl.sh

# Generate local production certificates
./scripts/generate-ssl.sh local-prod

# Manage production SSL
./scripts/ssl-manager.sh generate
```

## Error Handling
- Environment validation before execution
- Graceful failure with clear error messages
- Colored output for status indication
- Comprehensive error reporting