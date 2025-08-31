# Database Implementation

## Overview
PostgreSQL 17 database with UUIDv7 support, complete schema implementation, automated timestamp triggers, and comprehensive migration system.

## Technology Stack
- **Database**: PostgreSQL 17 running in Docker
- **Extensions**: pg_uuidv7 for UUIDv7 support
- **Migration Tool**: SQLx CLI with 4 applied migrations
- **Connection**: SQLx connection pooling in Rust backend
- **Container**: PostgreSQL 17 Docker container with health checks
- **Development Tools**: Database management scripts

## Database Reset Script

For development, use the automated reset script:

```bash
# Reset database with fresh migrations
./scripts/reset-db.sh

# Future: Reset with seed data (not yet implemented)
# ./scripts/reset-db.sh --seed
```

The script handles:
- Stopping and removing PostgreSQL container
- Clearing data volume
- Starting fresh PostgreSQL 17 container with UUIDv7
- Running all migrations
- Verifying database health

## Timestamp Management Strategy

**Decision: Database Triggers for `updated_at`**

All tables use PostgreSQL triggers to automatically update `updated_at` timestamps:
- **created_at**: `NOT NULL DEFAULT NOW()` - Set once on insert
- **updated_at**: `NOT NULL DEFAULT NOW()` - Auto-updated by trigger on every UPDATE
- **Trigger Function**: Single `update_updated_at_column()` function shared across all tables
- **Benefits**: Guaranteed consistency, no application logic needed, works for all operations

## Migration Management with SQLx

### Installation
```bash
# Install SQLx CLI (from backend setup)
cargo install sqlx-cli --no-default-features --features postgres
```

### Migration Workflow
```bash
# Create new migration
sqlx migrate add create_users_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Check migration status
sqlx migrate info
```

### Migration Files Structure (✅ All Applied)
```
migrations/
├── 20250829024919_create_users_table.sql         # Users table with auth fields ✅
├── 20250829025210_create_roles_table.sql         # Roles + user_roles junction ✅
├── 20250829095648_add_user_slug_to_users.sql     # Added user_slug for public URLs ✅
└── 20250829095731_create_incident_timers_table.sql # Timer tracking table ✅
```

## Schema Implementation

### Current Tables
- **users**: Authentication, profile data, and public user slugs
- **roles**: Role-based authorization system (user, admin)
- **user_roles**: Many-to-many user-role relationships
- **incident_timers**: Timer tracking with user association and notes

### Key Features
- **UUIDv7 Primary Keys**: Time-ordered UUIDs for better indexing performance
- **Automatic Timestamps**: Database triggers handle `updated_at` updates
- **Foreign Key Constraints**: Proper referential integrity with cascades
- **Unique Constraints**: Email and user_slug uniqueness enforced
- **Migration-Driven**: All schema details documented in migration files

### Current Schema
All database schema details are documented in migration files located in `backend/migrations/`. For planned schema extensions, see [ROADMAP.md](ROADMAP.md).

## Docker Configuration

### docker-compose.yml Service
```yaml
postgres:
  image: postgres:15-alpine
  environment:
    POSTGRES_DB: kennwilliamson
    POSTGRES_USER: ${DB_USER}
    POSTGRES_PASSWORD: ${DB_PASSWORD}
  volumes:
    - postgres_data:/var/lib/postgresql/data
    - ./backups:/backups
  ports:
    - "5432:5432"  # Remove in production
  healthcheck:
    test: ["CMD-SHELL", "pg_isready -U ${DB_USER}"]
    interval: 30s
    timeout: 10s
    retries: 3
```

### Initialization Scripts
```bash
# init-scripts/ directory for Docker
CREATE DATABASE kennwilliamson_test; -- Test database
```

## Connection Configuration

### Environment Variables
```env
# Database connection
DATABASE_URL=postgresql://user:password@postgres:5432/kennwilliamson

# Connection pool settings
DB_MAX_CONNECTIONS=10
DB_MIN_CONNECTIONS=2
DB_ACQUIRE_TIMEOUT=30
```

### SQLx Configuration (in backend)
Refer to **IMPLEMENTATION-BACKEND.md** for Rust SQLx pool setup and query examples.

## Backup Strategy

### Automated Backups
```bash
# Daily backup script
#!/bin/bash
BACKUP_DIR="/backups"
DATE=$(date +%Y%m%d_%H%M%S)
FILENAME="kennwilliamson_backup_$DATE.sql"

pg_dump -h postgres -U $DB_USER -d kennwilliamson > "$BACKUP_DIR/$FILENAME"

# Keep last 7 days
find $BACKUP_DIR -name "*.sql" -mtime +7 -delete
```

### Docker Backup Volume
```yaml
volumes:
  postgres_data:
    driver: local
  backup_data:
    driver: local
```

### Custom PostgreSQL Build
The project uses a custom PostgreSQL 17 Docker image with UUIDv7 support:
- Base: `postgres:17-alpine`
- Extension: `pg_uuidv7` for time-ordered UUID generation
- Benefits: Better indexing performance and natural ordering

## Development Setup

### Local Development
```bash
# Start PostgreSQL container
docker-compose up postgres -d

# Run migrations
cd backend && sqlx migrate run

# Seed test data (future)
sqlx migrate run --source ./seeds/
```

### Testing Database
```bash
# Create test database
createdb kennwilliamson_test

# Run migrations on test DB
DATABASE_URL=postgresql://user:password@localhost/kennwilliamson_test sqlx migrate run
```

## Performance Optimization

### Indexing Strategy
- Primary keys (UUID with btree)
- Foreign key constraints with indexes
- Email uniqueness with btree index
- Composite indexes for common queries

### Connection Pooling
- Max connections: 10 (AWS free tier constraint)
- Min connections: 2 (always ready)
- Connection timeout: 30s

### Query Optimization
- Use SQLx compile-time query checking
- Implement proper JOIN strategies
- Monitor slow queries with pg_stat_statements

## Security Configuration

### Access Control
- Database user with minimal privileges
- No superuser access for application
- Network isolation (Docker internal network)
- No external port exposure in production

### Data Protection
- Regular backups with encryption
- Password hashing handled by backend (see **IMPLEMENTATION-BACKEND.md**)
- Audit logging for sensitive operations

## Monitoring & Health Checks

### Health Check Query
```sql
-- Simple connectivity test
SELECT 1;

-- More comprehensive check
SELECT 
    COUNT(*) as user_count,
    NOW() as current_time,
    version() as pg_version;
```

### Container Health Check
```yaml
healthcheck:
  test: ["CMD-SHELL", "pg_isready -U ${DB_USER}"]
  interval: 30s
  timeout: 10s
  retries: 3
```

## Migration Best Practices

### Version Control
- All migrations in Git
- Never edit existing migration files
- Use descriptive migration names
- Test migrations on copy of production data

### Rollback Strategy
- Write reversible migrations when possible
- Test rollback procedures
- Document breaking changes
- Coordinate with backend deployments

## AWS Deployment Notes

### EBS Volume
- Persistent storage for data directory
- Regular snapshots for backup
- Monitor disk space usage

### Security Groups
- Restrict PostgreSQL port to backend container only
- No public internet access
- VPC internal communication only