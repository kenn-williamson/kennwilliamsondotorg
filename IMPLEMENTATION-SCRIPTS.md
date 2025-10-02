# Scripts Implementation

## Overview
Automation scripts for development, database management, deployment, and environment setup.

## Script Inventory

### Development Scripts
- **`dev-start.sh`**: Start services with build/rebuild options
- **`dev-stop.sh`**: Stop services with optional container removal
- **`dev-logs.sh`**: View and filter service logs
- **`health-check.sh`**: Verify service health and connectivity

### Production Monitoring Scripts
- **`log-monitor.sh`**: Production log monitoring and management

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

## Production Monitoring

### log-monitor.sh

**Purpose**: Monitor and manage Docker container logs in production environment.

**Container Naming**:
- **Production**: Uses Docker Compose default naming (`kennwilliamsondotorg-{service}-1`)
- **Development**: Uses custom naming (`kennwilliamson-{service}-dev`)
- **Note**: This script is designed for production monitoring only

**Testing Before Deployment**:
```bash
# Start local production environment
./scripts/setup-local-prod.sh

# Test log monitoring commands
./scripts/log-monitor.sh status           # View all service log status
./scripts/log-monitor.sh tail backend     # Tail backend logs
./scripts/log-monitor.sh size             # Check log file sizes
./scripts/log-monitor.sh monitor          # Real-time monitoring
```

**Available Commands**:
- `status` - Show log status for all services
- `tail [service]` - Tail logs for specific service (nginx, frontend, backend, postgres, redis)
- `size` - Show Docker log file sizes and system usage
- `rotate` - Force log rotation by restarting containers
- `clean` - Clean old logs and prune Docker system
- `monitor` - Monitor all services in real-time

**Options**:
- `-n, --lines N` - Number of lines to show (default: 100)
- `-f, --follow` - Follow logs in real-time
- `-s, --since T` - Show logs since time (default: 1h)

**Production Usage**:
```bash
# On production server after deployment
./scripts/log-monitor.sh status
./scripts/log-monitor.sh tail backend -n 50
./scripts/log-monitor.sh tail backend -f  # Follow in real-time
```

**Important**: Cannot be used with dev environment due to different container naming. Use `./scripts/dev-logs.sh` for development log viewing.