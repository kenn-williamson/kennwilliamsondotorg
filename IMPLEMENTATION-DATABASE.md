# Database Implementation

## Overview
Database architecture decisions and schema design rationale.

## Technology Stack Decisions

### PostgreSQL 17
**Decision**: PostgreSQL over alternatives

**Why:**
- Robust ACID compliance
- Advanced features (full-text search, JSON, extensions)
- Mature ecosystem
- Free and open source

**Alternatives considered:**
- **MySQL**: Less advanced features
- **MongoDB**: Not suitable for relational data
- **SQLite**: Not production-ready for concurrent access

### UUIDv7 Primary Keys
**Decision**: Use pg_uuidv7 extension for all primary keys

**Why:**
- Time-ordered for better indexing than random UUIDs
- Globally unique (no conflicts in distributed systems)
- No auto-increment collision issues
- 128-bit vs 64-bit integer (future-proof)

**Trade-offs:**
- Slightly larger storage than integers
- Worth it: Distributed system ready, better indexing than UUID4

### SQLx for Database Access
**Decision**: SQLx over ORMs

**Why:**
- Compile-time SQL verification
- Prevents SQL typos in production
- Raw SQL control
- Async support

**Alternatives rejected:**
- **Diesel**: Compile times too slow
- **SeaORM**: Less mature, adds abstraction layer
- **Raw queries**: No compile-time safety

## Schema Design Decisions

### Normalized Auth Architecture
**Decision**: Split user data across 5 tables

**Tables:**
- `users`: Core identity (email, slug, status)
- `user_credentials`: Password authentication
- `user_external_logins`: OAuth providers
- `user_profiles`: Display data
- `user_preferences`: Settings

**Why:**
- Multi-provider OAuth: Users link multiple providers
- GDPR compliance: Clear data boundaries
- Maintainability: Changes to auth don't affect profile
- Extensibility: Add providers without schema changes

**Trade-offs:**
- More joins for full user data
- More tables to maintain
- Worth it: Flexibility and compliance

**Alternative rejected:**
- Monolithic users table: Doesn't scale to multiple OAuth providers

### Automated Timestamps
**Decision**: PostgreSQL triggers for `updated_at`

**Why:**
- Can't forget to update
- Consistent across all code paths
- Database-level guarantee
- No application logic needed

**Pattern:**
- `created_at`: DEFAULT NOW() on insert
- `updated_at`: Trigger updates on every UPDATE

**Alternative rejected:**
- Application-level updates: Easy to forget, inconsistent

### Role-Based Access Control (RBAC)
**Decision**: Separate roles and user_roles tables

**Why:**
- Flexible role assignment
- Many-to-many relationship
- Easy to add new roles
- Query-friendly for permission checks

**Roles:**
- `user`: Base role (immutable)
- `email-verified`: Verified identity
- `trusted-contact`: Personal content access
- `admin`: Full system access

**Trade-offs:**
- Join required for role checks
- Worth it: Flexible, standard pattern

## Migration Strategy

### SQLx Migrations
**Decision**: Version-controlled SQL migrations

**Why:**
- Declarative schema changes
- Rollback capability
- Source control integration
- Database-agnostic SQL

**Pattern:**
- Up migrations only (no down)
- Sequential numbering
- Preserve data by default

**Scripts:**
- `setup-db.sh`: Run migrations (safe)
- `reset-db.sh`: Fresh start (destructive)

### Data Preservation Philosophy
**Decision**: Migrations preserve data by default

**Why:**
- Accidents are expensive
- Development data valuable
- Explicit destructive operations
- Safe by default

## Performance Decisions

### Connection Pooling
**Decision**: 10 max connections, 2 min

**Why:**
- AWS free tier constraint (limited connections)
- Balance availability and resource usage
- Sufficient for single-instance deployment

**Trade-offs:**
- May need to increase for multiple instances
- Worth it: Fits current infrastructure

### Indexing Strategy
**Decision**: Index primary keys, foreign keys, and search fields

**Why:**
- Fast lookups by ID
- Fast joins
- Full-text search performance
- Trigram search for fuzzy matching

**Indexed fields:**
- All primary keys (automatic)
- All foreign keys
- `email` (unique, frequent lookups)
- Phrase text (full-text search GIN index)

### Full-Text Search
**Decision**: PostgreSQL built-in full-text search + trigram fallback

**Why:**
- No external search engine needed
- Good enough performance
- Trigram handles partial matches
- Simpler architecture

**Alternatives considered:**
- **Elasticsearch**: Overkill for scale
- **Meilisearch**: External dependency
- **ILIKE only**: Too slow without trigram

**Trade-offs:**
- Not as powerful as dedicated search
- Worth it: Simple, no external service

## Security Decisions

### Network Isolation
**Decision**: Database only accessible via internal Docker network

**Why:**
- No external exposure
- Services connect via service name
- Automatic DNS resolution

**Trade-offs:**
- Can't connect from host without port forward
- Worth it: Security by default

### Minimal Privileges
**Decision**: Application user has minimal required permissions

**Why:**
- Limit blast radius of compromise
- Can't drop tables from application
- Defense in depth

## Backup Strategy

**Decision**: Script-based backups

**Why:**
- Simple and reliable
- Version controlled process
- Environment-aware
- Manual for now (automated later if needed)

**Script**: `backup-db.sh`

**Trade-offs:**
- Manual execution required
- Worth it: Sufficient for current scale
