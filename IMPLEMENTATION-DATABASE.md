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
├── 20251003192001_add_auth_and_compliance_features.up.sql # OAuth, email verification, password reset, email suppression
├── 20251011000000_create_normalized_auth_tables.up.sql # Normalized auth schema (Phase 1)
└── 20251012222532_drop_old_auth_columns.up.sql   # Data backfill and old column removal (Phase 9)
```

## Schema Design

### Core Tables

#### User Management (Normalized Multi-Table Architecture)
- **users**: Core user identity (id, email, display_name, slug, active status)
- **user_credentials**: Password authentication (password_hash, password_updated_at)
- **user_external_logins**: OAuth provider linkage (provider, provider_user_id, linked_at)
- **user_profiles**: User profile data (real_name, bio, avatar_url, etc.)
- **user_preferences**: User settings (timer_is_public, timer_show_in_list, email preferences)

**Schema Refactor**: Completed January 2025 (Phases 0-9). Previously monolithic `users` table split into normalized structure for better maintainability, multi-provider OAuth support, and GDPR/CCPA compliance.

#### Access Control
- **roles**: System role definitions (user, email-verified, trusted-contact, admin)
- **user_roles**: User-to-role mapping with role-based access control (RBAC)

#### Authentication & Security
- **refresh_tokens**: Secure token storage with expiration and rotation
- **verification_tokens**: Email verification tokens with expiration
- **password_reset_tokens**: Password reset tokens with expiration and usage tracking
- **email_suppressions**: Email suppression list for AWS SES compliance (bounces, complaints, unsubscribes)

#### Features
- **incident_timers**: User timer entries with public sharing
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