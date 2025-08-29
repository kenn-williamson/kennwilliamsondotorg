# Database Implementation Plan - PostgreSQL

## Overview
PostgreSQL database setup with SQLx migrations, Docker containerization, and backup automation. See **IMPLEMENTATION-BACKEND.md** for Rust integration details.

## Technology Stack
- **Database**: PostgreSQL 15
- **Migration Tool**: SQLx CLI (integrates with Rust backend)
- **Connection**: SQLx with connection pooling
- **Container**: Official PostgreSQL Docker image
- **Backup**: pg_dump with automated scheduling

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

### Migration Files Structure (✅ Completed)
```
migrations/
├── 20250829024919_create_users_table.sql     # Users with UUIDv7 + timestamps
├── 20250829025210_create_roles_table.sql     # Roles + user_roles junction
└── [future migrations...]
```

## Schema Design

### Core Tables (Phase 1)
- **users**: Basic user authentication and profile data
- **roles**: User roles (user, admin) for authorization
- **user_roles**: Many-to-many junction table linking users to roles
- **Indexes**: Email uniqueness, role lookups, performance optimization

### Future Tables (Phase 2+)
- **oauth_providers**: Google/GitHub authentication linkage
- **sessions**: Session management (if JWT cookies insufficient)
- **Additional tables**: Based on CRUD feature requirements

### Schema Principles
- **UUIDv7 primary keys** for distributed scalability and better indexing performance
- Proper foreign key relationships with cascading deletes
- Timestamp tracking (created_at, updated_at)
- Indexing strategy optimized for UUIDv7 time-ordering

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