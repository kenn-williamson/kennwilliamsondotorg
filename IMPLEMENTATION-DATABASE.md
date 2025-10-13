# Database Implementation

## Overview
PostgreSQL 17 with pg_uuidv7 extension, automated timestamps, and SQLx migrations.

## Technology Stack
- **Database**: PostgreSQL 17.0
- **Extensions**: pg_uuidv7
- **Migrations**: SQLx CLI
- **Connection**: SQLx pooling
- **Container**: Docker with health checks

## Database Management
Scripts for database operations:
- **Migrations**: `./scripts/setup-db.sh` (preserves data)
- **Reset**: `./scripts/reset-db.sh` (fresh start)
- **Backup**: `./scripts/backup-db.sh`

See [DEVELOPMENT-WORKFLOW.md](DEVELOPMENT-WORKFLOW.md#database-development-workflow) for usage.

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
├── 20250914134703_add_phrases_system.up.sql      # Phrases system with initial data
├── 20250927212836_add_search_and_performance_optimization.up.sql # Full-text search + trigram + indexes
└── 20251003192001_add_auth_and_compliance_features.up.sql # OAuth, email verification, password reset, email suppression
```

## Schema Design

### Core Tables
- **users**: User accounts with authentication data (OAuth support, timer visibility settings)
- **roles** & **user_roles**: Role-based access control (user, email-verified, trusted-contact, admin)
- **incident_timers**: User timer entries
- **refresh_tokens**: Secure token storage with expiration
- **verification_tokens**: Email verification tokens with expiration
- **password_reset_tokens**: Password reset tokens with expiration and usage tracking
- **email_suppressions**: Email suppression list for AWS SES compliance (bounces, complaints, unsubscribes)
- **phrases**: Motivational phrase system with full-text search
- **user_excluded_phrases**: Phrase filtering preferences
- **phrase_suggestions**: User submission workflow

### Design Patterns
- **Primary Keys**: UUIDv7 for time-ordered indexing
- **Timestamps**: Automated via triggers
- **Constraints**: Foreign keys, unique indexes
- **Schema Definition**: See migration files in `backend/migrations/`

## Container Configuration
- **Image**: PostgreSQL 17 official
- **Health Checks**: Built-in connectivity verification
- **Volumes**: Persistent data storage
- **Network**: Internal Docker network only

## Connection Configuration
```env
DATABASE_URL=postgresql://user:password@postgres:5432/kennwilliamson
DB_MAX_CONNECTIONS=10
DB_MIN_CONNECTIONS=2
DB_ACQUIRE_TIMEOUT=30
```

## Performance & Security
- **Indexing**: Primary keys, foreign keys, email uniqueness, full-text search (GIN), trigram search, composite indexes
- **Search Optimization**: PostgreSQL full-text search with ts_rank ranking + pg_trgm for ILIKE fallback
- **Connection Pooling**: 10 max connections (AWS free tier constraint)
- **Access Control**: Minimal privileges, Docker network isolation
- **Backups**: Automated via `backup-db.sh` script
- **Extensions**: pg_uuidv7 (time-ordered UUIDs), pg_trgm (trigram similarity search)

## Development Integration
See [DEVELOPMENT-WORKFLOW.md](DEVELOPMENT-WORKFLOW.md#database-development-workflow) for database development workflows.