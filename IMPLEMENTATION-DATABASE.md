# Database Implementation

## Overview
PostgreSQL 17 database with UUIDv7 support, automated timestamp triggers, and comprehensive migration system.

## Technology Stack
- **Database**: PostgreSQL 17 running in Docker
- **Extensions**: pg_uuidv7 for UUIDv7 support
- **Migration Tool**: SQLx CLI with 3 migrations
- **Connection**: SQLx connection pooling in Rust backend
- **Development Tools**: Database management scripts

## Database Management Scripts

### Core Operations
```bash
# Reset database with fresh migrations
./scripts/reset-db.sh

# Run migrations (preserves data)
./scripts/setup-db.sh

# Backup database
./scripts/backup-db.sh

# Download backup from remote
./scripts/download-backup.sh
```

## Timestamp Management
All tables use PostgreSQL triggers for `updated_at` timestamps:
- **created_at**: `NOT NULL DEFAULT NOW()` - Set once on insert
- **updated_at**: `NOT NULL DEFAULT NOW()` - Auto-updated by trigger on every UPDATE
- **Trigger Function**: Single `update_updated_at_column()` function shared across all tables

## Migration Files
```
migrations/
├── 20250914134643_initial_schema.up.sql          # Users, roles, incident_timers
├── 20250914134654_add_refresh_tokens_and_user_active.up.sql # Refresh tokens + user.active
└── 20250914134703_add_phrases_system.up.sql      # Phrases system with initial data
```

## Schema Implementation

### Tables
- **users**: Authentication, profile data, public user slugs, and active status
- **roles**: Role-based authorization system (user, admin)
- **user_roles**: Many-to-many user-role relationships
- **incident_timers**: Timer tracking with user association and notes
- **refresh_tokens**: Rolling refresh tokens with SHA-256 hashing and expiration tracking
- **phrases**: Motivational phrases with active status and creator tracking
- **user_excluded_phrases**: User phrase exclusion preferences (exclusion-based filtering)
- **phrase_suggestions**: User phrase suggestions with admin approval workflow

### Key Features
- **UUIDv7 Primary Keys**: Time-ordered UUIDs for better indexing performance
- **Automatic Timestamps**: Database triggers handle `updated_at` updates
- **Foreign Key Constraints**: Proper referential integrity with cascades
- **Unique Constraints**: Email and user_slug uniqueness enforced

### Schema Details
All database schema details are documented in migration files located in `backend/migrations/`.

## Docker Configuration
PostgreSQL 17 container with UUIDv7 extension, health checks, and backup volume mounting.

## Connection Configuration
```env
DATABASE_URL=postgresql://user:password@postgres:5432/kennwilliamson
DB_MAX_CONNECTIONS=10
DB_MIN_CONNECTIONS=2
DB_ACQUIRE_TIMEOUT=30
```

## Performance & Security
- **Indexing**: Primary keys, foreign keys, email uniqueness
- **Connection Pooling**: 10 max connections (AWS free tier constraint)
- **Access Control**: Minimal privileges, Docker network isolation
- **Backups**: Automated via `backup-db.sh` script

## Development
```bash
# Start database
docker-compose up postgres -d

# Run migrations
./scripts/setup-db.sh

# Reset database
./scripts/reset-db.sh
```