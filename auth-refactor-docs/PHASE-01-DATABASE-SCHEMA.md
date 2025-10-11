# Phase 1: Database Schema & Migrations

**Estimated Time**: 2-3 hours
**Prerequisites**: [Phase 0: Setup & Baseline](PHASE-00-SETUP.md) complete
**Next Phase**: [Phase 2A: Core User Models](PHASE-02A-CORE-MODELS.md)

## Objective

Create the new normalized database schema with 4 new tables while maintaining the existing `users` table intact. This phase establishes the foundation for the multi-table architecture without breaking existing functionality.

**Key Principle**: Additive changes only - we're creating new tables, not modifying existing ones yet.

---

## TDD Approach

Since this is a database migration phase, we'll adapt TDD:
1. **Red**: Write migration with expected structure
2. **Green**: Run migration successfully
3. **Refactor**: Verify constraints and indexes work correctly

We'll use SQL schema tests rather than traditional unit tests.

---

## New Tables Overview

### user_credentials
**Purpose**: Local password authentication (optional for OAuth-only users)
```sql
CREATE TABLE user_credentials (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    password_hash VARCHAR(255) NOT NULL,
    password_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### user_external_logins
**Purpose**: Multi-provider OAuth support
```sql
CREATE TABLE user_external_logins (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    linked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_user_id)
);
CREATE INDEX idx_user_external_logins_user_id ON user_external_logins(user_id);
CREATE INDEX idx_user_external_logins_provider ON user_external_logins(provider, provider_user_id);
```

### user_profiles
**Purpose**: Optional profile data (bio, avatar, etc.)
```sql
CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    real_name VARCHAR(255),
    bio TEXT,
    avatar_url VARCHAR(500),
    location VARCHAR(255),
    website VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### user_preferences
**Purpose**: User settings (can grow without breaking auth)
```sql
CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    timer_is_public BOOLEAN NOT NULL DEFAULT false,
    timer_show_in_list BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

---

## Task 1: Create UP Migration (1 hour)

### Step 1: Create migration file

```bash
# Create new migration
cd backend
sqlx migrate add add_user_auth_schema_tables

# Expected output:
# Creating migrations/XXXXXXXX_add_user_auth_schema_tables.up.sql
# Creating migrations/XXXXXXXX_add_user_auth_schema_tables.down.sql
```

### Step 2: Write UP migration SQL

Edit `migrations/XXXXXXXX_add_user_auth_schema_tables.up.sql`:

```sql
-- ============================================================================
-- USER AUTHENTICATION SCHEMA REFACTOR
-- Add new normalized tables for users authentication, profile, and preferences
-- ============================================================================

-- ============================================================================
-- user_credentials: Local password authentication
-- ============================================================================
CREATE TABLE user_credentials (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    password_hash VARCHAR(255) NOT NULL,
    password_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE user_credentials IS 'Local password authentication credentials (optional for OAuth-only users)';
COMMENT ON COLUMN user_credentials.user_id IS 'Foreign key to users table (primary key)';
COMMENT ON COLUMN user_credentials.password_hash IS 'bcrypt hashed password';
COMMENT ON COLUMN user_credentials.password_updated_at IS 'Last password change timestamp';

-- ============================================================================
-- user_external_logins: Multi-provider OAuth support
-- ============================================================================
CREATE TABLE user_external_logins (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    linked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_user_id)
);

COMMENT ON TABLE user_external_logins IS 'OAuth provider links (supports multiple providers per user)';
COMMENT ON COLUMN user_external_logins.provider IS 'OAuth provider name (google, github, microsoft, etc.)';
COMMENT ON COLUMN user_external_logins.provider_user_id IS 'User ID from OAuth provider';
COMMENT ON COLUMN user_external_logins.linked_at IS 'When account was linked';

-- Indexes for performance
CREATE INDEX idx_user_external_logins_user_id ON user_external_logins(user_id);
CREATE INDEX idx_user_external_logins_provider ON user_external_logins(provider, provider_user_id);

-- ============================================================================
-- user_profiles: Optional profile data
-- ============================================================================
CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    real_name VARCHAR(255),
    bio TEXT,
    avatar_url VARCHAR(500),
    location VARCHAR(255),
    website VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE user_profiles IS 'Optional user profile data (bio, avatar, etc.)';
COMMENT ON COLUMN user_profiles.real_name IS 'User real name (from OAuth or user input)';
COMMENT ON COLUMN user_profiles.bio IS 'User bio/description';
COMMENT ON COLUMN user_profiles.avatar_url IS 'URL to user avatar image';

-- ============================================================================
-- user_preferences: User settings
-- ============================================================================
CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    timer_is_public BOOLEAN NOT NULL DEFAULT false,
    timer_show_in_list BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE user_preferences IS 'User application preferences (can grow without breaking auth)';
COMMENT ON COLUMN user_preferences.timer_is_public IS 'Whether timer is publicly viewable';
COMMENT ON COLUMN user_preferences.timer_show_in_list IS 'Whether timer appears in public list';
```

### Step 3: Write DOWN migration SQL

Edit `migrations/XXXXXXXX_add_user_auth_schema_tables.down.sql`:

```sql
-- ============================================================================
-- ROLLBACK: USER AUTHENTICATION SCHEMA REFACTOR
-- Drop all new tables (data will be lost - use with caution)
-- ============================================================================

-- Drop tables in reverse order (respects foreign keys)
DROP TABLE IF EXISTS user_preferences CASCADE;
DROP TABLE IF EXISTS user_profiles CASCADE;
DROP TABLE IF EXISTS user_external_logins CASCADE;
DROP TABLE IF EXISTS user_credentials CASCADE;
```

**Success Criteria**:
- Migration files created
- UP migration has all 4 tables with proper constraints
- DOWN migration drops tables in correct order
- Comments added for documentation

---

## Task 2: Test Migration (30 minutes)

### Step 1: Run UP migration

```bash
# Run migrations on development database
./scripts/setup-db.sh

# Expected output:
# ✅ Applying migration: XXXXXXXX_add_user_auth_schema_tables
```

### Step 2: Verify table creation

```sql
-- Connect to development database
psql postgresql://postgres:postgres@localhost:5432/kennwilliamson

-- Verify all tables exist
\dt user_*

-- Expected output:
-- user_credentials
-- user_external_logins
-- user_profiles
-- user_preferences

-- Describe each table
\d user_credentials
\d user_external_logins
\d user_profiles
\d user_preferences
```

### Step 3: Verify constraints

```sql
-- Check foreign key constraints
SELECT
    tc.table_name,
    tc.constraint_name,
    tc.constraint_type,
    kcu.column_name,
    ccu.table_name AS foreign_table_name,
    ccu.column_name AS foreign_column_name
FROM information_schema.table_constraints AS tc
JOIN information_schema.key_column_usage AS kcu
    ON tc.constraint_name = kcu.constraint_name
JOIN information_schema.constraint_column_usage AS ccu
    ON ccu.constraint_name = tc.constraint_name
WHERE tc.table_name IN (
    'user_credentials',
    'user_external_logins',
    'user_profiles',
    'user_preferences'
)
ORDER BY tc.table_name;

-- Expected:
-- All 4 tables have foreign key to users(id)
-- All foreign keys have ON DELETE CASCADE
```

### Step 4: Verify indexes

```sql
-- Check indexes
SELECT
    tablename,
    indexname,
    indexdef
FROM pg_indexes
WHERE tablename IN (
    'user_credentials',
    'user_external_logins',
    'user_profiles',
    'user_preferences'
)
ORDER BY tablename, indexname;

-- Expected indexes:
-- user_credentials_pkey (PRIMARY KEY on user_id)
-- user_external_logins_pkey (PRIMARY KEY on id)
-- idx_user_external_logins_user_id
-- idx_user_external_logins_provider
-- user_external_logins_provider_provider_user_id_key (UNIQUE)
-- user_profiles_pkey (PRIMARY KEY on user_id)
-- user_preferences_pkey (PRIMARY KEY on user_id)
```

**Success Criteria**:
- All 4 tables created
- All foreign keys reference users(id) with CASCADE
- All indexes created
- UNIQUE constraint on (provider, provider_user_id)

---

## Task 3: Test Rollback (30 minutes)

### Step 1: Test DOWN migration

```bash
# Rollback the migration
cd backend
sqlx migrate revert

# Expected output:
# ✅ Reverting migration: XXXXXXXX_add_user_auth_schema_tables
```

### Step 2: Verify tables dropped

```sql
-- Connect to development database
psql postgresql://postgres:postgres@localhost:5432/kennwilliamson

-- Verify tables are gone
\dt user_*

-- Expected output:
-- Should NOT show:
--   - user_credentials
--   - user_external_logins
--   - user_profiles
--   - user_preferences

-- Original users table should still exist
SELECT COUNT(*) FROM users;
-- Should return row count (not error)
```

### Step 3: Re-apply migration

```bash
# Re-run the migration
./scripts/setup-db.sh

# Expected output:
# ✅ Applying migration: XXXXXXXX_add_user_auth_schema_tables
```

**Success Criteria**:
- DOWN migration successfully removes all 4 tables
- Original `users` table unaffected
- UP migration can be re-applied successfully
- No orphaned constraints or indexes

---

## Task 4: Test CASCADE Deletion (30 minutes)

This is critical - we need to verify that deleting a user properly cascades to all new tables.

### Step 1: Create test data

```sql
-- Connect to development database
psql postgresql://postgres:postgres@localhost:5432/kennwilliamson

-- Create a test user
INSERT INTO users (email, display_name, slug)
VALUES ('test-cascade@example.com', 'Test User', 'test-cascade')
RETURNING id;

-- Note the returned UUID (e.g., '123e4567-e89b-12d3-a456-426614174000')
-- Use that UUID in the following commands (replace YOUR_UUID)

-- Insert credentials
INSERT INTO user_credentials (user_id, password_hash)
VALUES ('YOUR_UUID', '$2b$04$test_hash');

-- Insert external login
INSERT INTO user_external_logins (user_id, provider, provider_user_id)
VALUES ('YOUR_UUID', 'google', 'google_12345');

-- Insert profile
INSERT INTO user_profiles (user_id, real_name)
VALUES ('YOUR_UUID', 'Test User Real Name');

-- Insert preferences
INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list)
VALUES ('YOUR_UUID', true, false);

-- Verify all data exists
SELECT 'credentials' AS table_name, COUNT(*) FROM user_credentials WHERE user_id = 'YOUR_UUID'
UNION ALL
SELECT 'external_logins', COUNT(*) FROM user_external_logins WHERE user_id = 'YOUR_UUID'
UNION ALL
SELECT 'profiles', COUNT(*) FROM user_profiles WHERE user_id = 'YOUR_UUID'
UNION ALL
SELECT 'preferences', COUNT(*) FROM user_preferences WHERE user_id = 'YOUR_UUID';

-- Expected: All counts should be 1
```

### Step 2: Test cascade deletion

```sql
-- Delete the user
DELETE FROM users WHERE id = 'YOUR_UUID';

-- Verify all related data was deleted
SELECT 'credentials' AS table_name, COUNT(*) FROM user_credentials WHERE user_id = 'YOUR_UUID'
UNION ALL
SELECT 'external_logins', COUNT(*) FROM user_external_logins WHERE user_id = 'YOUR_UUID'
UNION ALL
SELECT 'profiles', COUNT(*) FROM user_profiles WHERE user_id = 'YOUR_UUID'
UNION ALL
SELECT 'preferences', COUNT(*) FROM user_preferences WHERE user_id = 'YOUR_UUID';

-- Expected: All counts should be 0
```

**Success Criteria**:
- Creating user and related data succeeds
- Deleting user cascades to all 4 new tables
- No orphaned rows remain
- No constraint violations

---

## Task 5: Update SQLx Cache (15 minutes)

### Step 1: Regenerate SQLx cache

```bash
# Clean and regenerate SQLx cache
./scripts/prepare-sqlx.sh --clean

# Expected output:
# ✅ SQLx cache regenerated successfully
```

### Step 2: Verify compilation

```bash
# Test backend compilation
cd backend && cargo check

# Expected output:
# Finished dev [unoptimized + debuginfo] target(s) in X.XXs
# (No errors)
```

**Success Criteria**:
- SQLx cache includes new tables
- Backend compiles without errors
- No query verification failures

---

## Task 6: Document Migration (15 minutes)

### Create migration documentation

Create `auth-refactor-docs/phase-01-migration-details.md`:

```markdown
# Phase 1: Migration Details

## Migration File
- **Timestamp**: XXXXXXXX
- **Name**: add_user_auth_schema_tables
- **Applied**: [DATE]

## Tables Created
1. user_credentials (PRIMARY KEY on user_id)
2. user_external_logins (PRIMARY KEY on id, UNIQUE on provider+provider_user_id)
3. user_profiles (PRIMARY KEY on user_id)
4. user_preferences (PRIMARY KEY on user_id)

## Foreign Keys
All tables have:
- Foreign key: user_id → users(id)
- ON DELETE CASCADE

## Indexes
- user_credentials_pkey (implicit)
- user_external_logins_pkey (implicit)
- idx_user_external_logins_user_id
- idx_user_external_logins_provider (composite on provider, provider_user_id)
- user_external_logins_provider_provider_user_id_key (UNIQUE)
- user_profiles_pkey (implicit)
- user_preferences_pkey (implicit)

## Rollback Verified
- DOWN migration successfully removes all 4 tables
- Original users table unaffected
- UP migration can be re-applied

## CASCADE Deletion Verified
- Deleting a user cascades to all 4 new tables
- No orphaned rows
- No constraint violations

## Performance Impact
- No impact to existing queries (new tables not used yet)
- Baseline metrics unchanged
```

**Success Criteria**:
- Migration details documented
- Rollback procedure verified
- CASCADE behavior confirmed

---

## Deliverables

At the end of this phase, you should have:

1. **Migration Files**
   - `XXXXXXXX_add_user_auth_schema_tables.up.sql`
   - `XXXXXXXX_add_user_auth_schema_tables.down.sql`

2. **Documentation**
   - `phase-01-migration-details.md`

3. **Verified Database State**
   - 4 new tables created
   - All foreign keys with CASCADE
   - All indexes in place
   - UNIQUE constraint on provider+provider_user_id
   - Rollback tested and working
   - CASCADE deletion verified

---

## Success Criteria

**Before proceeding to Phase 2A**, verify:

- [ ] UP migration runs successfully
- [ ] All 4 tables created with correct structure
- [ ] All foreign keys reference users(id) with ON DELETE CASCADE
- [ ] All indexes created (2 on user_external_logins)
- [ ] UNIQUE constraint on (provider, provider_user_id) works
- [ ] DOWN migration successfully removes all tables
- [ ] UP migration can be re-applied after rollback
- [ ] CASCADE deletion verified with test data
- [ ] SQLx cache regenerated successfully
- [ ] Backend compiles without errors
- [ ] Migration documented

**Time Check**: This phase should take 2-3 hours. Most time spent on verification.

---

## Common Issues

### Issue: Foreign key constraint fails
**Cause**: Missing ON DELETE CASCADE
**Solution**: Ensure all foreign keys have `ON DELETE CASCADE` in UP migration

### Issue: UNIQUE constraint not working
**Cause**: Incorrect constraint definition
**Solution**: Verify `UNIQUE(provider, provider_user_id)` syntax

### Issue: Index creation fails
**Cause**: Incorrect column names or types
**Solution**: Verify column names match exactly, check data types

### Issue: SQLx cache errors after migration
**Cause**: Cache not regenerated
**Solution**: Run `./scripts/prepare-sqlx.sh --clean`

### Issue: CASCADE deletion not working
**Cause**: Foreign keys missing CASCADE clause
**Solution**: Verify `REFERENCES users(id) ON DELETE CASCADE` in all tables

---

## Next Steps

Once all success criteria are met:

1. **Commit the migration**:
   ```bash
   git add backend/migrations/XXXXXXXX_add_user_auth_schema_tables.*.sql
   git add auth-refactor-docs/phase-01-migration-details.md
   git commit -m "feat(db): add user auth schema tables (Phase 1)

   - Add user_credentials table for local authentication
   - Add user_external_logins for multi-provider OAuth
   - Add user_profiles for optional profile data
   - Add user_preferences for user settings
   - All tables have CASCADE delete to users
   - Verified rollback and CASCADE behavior
   "
   ```

2. **Proceed to Phase 2A**:
   - Read [PHASE-02A-CORE-MODELS.md](PHASE-02A-CORE-MODELS.md)
   - Update core User model
   - Begin TDD cycle for models

---

**Phase Status**: ⬜ Not Started → **Continue when ready**

**Estimated Completion Time**: 2-3 hours

**Next Phase**: [Phase 2A: Core User Models](PHASE-02A-CORE-MODELS.md)
