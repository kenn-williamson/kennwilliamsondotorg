# Scripts Implementation

## Overview
Automation scripts for development, database management, deployment, and environment setup.

## Script Inventory

### Development Scripts
- **`dev-start.sh`**: Start services with build/rebuild options
- **`dev-stop.sh`**: Stop services with optional container removal
- **`dev-logs.sh`**: View and filter service logs
- **`health-check.sh`**: Verify service health and connectivity

### Database Scripts
- **`setup-db.sh`**: Run migrations (preserves existing data)
- **`reset-db.sh`**: Drop and recreate database with fresh data
- **`backup-db.sh`**: Backup/restore database with environment detection
- **`download-backup.sh`**: Download backups from remote servers
- **`prepare-sqlx.sh`**: Generate SQLx query cache for builds
- **`update-migrations-table.sh`**: Update migration tracking table

### Environment Scripts
- **`setup-production-env.sh`**: Generate secure production configuration
- **`generate-ssl.sh`**: Create SSL certificates (dev/local-prod)
- **`setup-local-prod.sh`**: Configure local production testing
- **`ssl-manager.sh`**: Manage Let's Encrypt certificates
- **`detect-environment.sh`**: Shared environment detection utility

### Deployment Scripts
- **`deploy.sh`**: Execute production deployment
- **`test-auth.sh`**: Verify authentication endpoints

## Script Features

### Safety and Intelligence
- **Data Preservation**: Migration scripts protect existing data
- **Environment Awareness**: Auto-detect dev/local-prod/production
- **Error Handling**: Comprehensive validation and recovery
- **Dependency Management**: Proper service startup ordering

### SSL Certificate Support
- **Multi-Environment**: Development, local production, and production certificates
- **Automatic Renewal**: Cron integration for Let's Encrypt
- **Fallback Support**: Temporary certificates if primary generation fails

### Database Operations
- **Safe Migrations**: Non-destructive schema updates
- **Backup Integration**: Environment-aware backup/restore
- **SQLx Support**: Query verification for compiled builds

## Script Organization

### Location
All scripts located in `scripts/` directory at project root.

### Conventions
- **Executable**: All scripts have execute permissions
- **Bash**: Written in bash for portability
- **Error Handling**: Set -e for fail-fast behavior
- **Output**: Colored output for clarity

### Usage
For detailed usage examples and workflows, see [DEVELOPMENT-WORKFLOW.md](DEVELOPMENT-WORKFLOW.md#development-scripts-reference).