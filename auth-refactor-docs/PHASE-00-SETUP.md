# Phase 0: Setup & Baseline

**Estimated Time**: 30-45 minutes
**Prerequisites**: None (this is the first phase)
**Next Phase**: [Phase 1: Database Schema](PHASE-01-DATABASE-SCHEMA.md)

## Objective

Establish a baseline understanding of the current system state before beginning the refactor. This phase ensures:
1. Development environment is properly configured
2. Current schema and performance are documented
3. Test suite is passing
4. Baseline metrics are recorded for comparison

**Why This Matters**: Having clear baselines allows us to measure the refactor's success and quickly identify regressions.

---

## Current State

### Current User Schema
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255),                    -- Will move to user_credentials
    display_name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    active BOOLEAN NOT NULL DEFAULT true,
    real_name VARCHAR(255),                        -- Will move to user_profiles
    google_user_id VARCHAR(255) UNIQUE,            -- Will move to user_external_logins
    timer_is_public BOOLEAN NOT NULL DEFAULT false,-- Will move to user_preferences
    timer_show_in_list BOOLEAN NOT NULL DEFAULT false, -- Will move to user_preferences
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Current Test Statistics
- **Backend Tests**: 227 tests
- **Frontend Tests**: 175 tests
- **Total**: 402 tests

### Current Repositories
- `UserRepository`: Handles all user operations (monolithic)
- `RefreshTokenRepository`: Session management
- `VerificationTokenRepository`: Email verification
- `PasswordResetTokenRepository`: Password recovery
- `IncidentTimerRepository`: Timer management
- `PhraseRepository`: Phrase system

---

## Tasks

### 1. Verify Development Environment

**Objective**: Ensure all services are running and healthy

```bash
# Start development environment
./scripts/dev-start.sh

# Verify health
./scripts/health-check.sh --dev

# Expected output:
# ✅ Nginx: healthy
# ✅ Frontend: healthy
# ✅ Backend: healthy
# ✅ PostgreSQL: healthy
```

**Success Criteria**:
- All services report healthy
- Can access https://localhost
- Backend API responds at http://localhost:8080/backend/health

---

### 2. Run Current Test Suite

**Objective**: Establish baseline test pass rate

```bash
# Set up test database
./scripts/setup-test-db.sh

# Run backend tests
cd backend && cargo test -- --test-threads=4

# Expected: All 227 tests passing
```

**Record Baseline**:
```
Date: ____________________
Backend Tests Passing: ______ / 227
Frontend Tests Passing: ______ / 175
Total Time: __________ seconds
```

**Success Criteria**:
- All tests passing
- No compilation errors
- No warnings related to user models

---

### 3. Document Current Schema

**Objective**: Capture current table structure for comparison

```bash
# Connect to development database
psql postgresql://postgres:postgres@localhost:5432/kennwilliamson

# Describe users table
\d users

# List all foreign keys referencing users
SELECT
    tc.table_name,
    kcu.column_name,
    ccu.table_name AS foreign_table_name,
    ccu.column_name AS foreign_column_name
FROM information_schema.table_constraints AS tc
JOIN information_schema.key_column_usage AS kcu
    ON tc.constraint_name = kcu.constraint_name
JOIN information_schema.constraint_column_usage AS ccu
    ON ccu.constraint_name = tc.constraint_name
WHERE tc.constraint_type = 'FOREIGN KEY'
    AND ccu.table_name = 'users';
```

**Record Foreign Key Dependencies**:
```
Tables referencing users.id:
- user_roles (user_id)
- refresh_tokens (user_id)
- verification_tokens (user_id)
- password_reset_tokens (user_id)
- incident_timers (user_id)
- phrase_suggestions (user_id)
- user_excluded_phrases (user_id)
```

**Success Criteria**:
- Users table structure documented
- Foreign key relationships identified
- Existing indexes recorded

---

### 4. Establish Performance Baselines

**Objective**: Record query performance before refactoring

```sql
-- Auth query (email lookup)
EXPLAIN ANALYZE
SELECT * FROM users WHERE email = 'test@example.com';

-- Record:
-- Planning Time: __________ ms
-- Execution Time: __________ ms

-- Profile query (ID lookup)
EXPLAIN ANALYZE
SELECT * FROM users WHERE id = 'UUID_HERE';

-- Record:
-- Planning Time: __________ ms
-- Execution Time: __________ ms

-- Public timer list query
EXPLAIN ANALYZE
SELECT u.id, u.display_name, u.slug, u.created_at, it.reset_timestamp, it.notes
FROM users u
JOIN incident_timers it ON u.id = it.user_id
WHERE u.timer_is_public = true AND u.timer_show_in_list = true
ORDER BY it.reset_timestamp DESC
LIMIT 10;

-- Record:
-- Planning Time: __________ ms
-- Execution Time: __________ ms
```

**Performance Baseline Template**:
```markdown
## Performance Baselines (Phase 0)

Date: ____________________

### Auth Query (email lookup)
- Planning: _____ ms
- Execution: _____ ms
- Rows returned: _____

### Profile Query (ID lookup)
- Planning: _____ ms
- Execution: _____ ms
- Rows returned: _____

### Public Timer List Query
- Planning: _____ ms
- Execution: _____ ms
- Rows returned: _____
```

**Success Criteria**:
- Baseline query times recorded
- Current table row counts recorded
- Index usage documented

---

### 5. Review Current Test Patterns

**Objective**: Understand how tests currently create users

```bash
# Review test helpers
cat backend/tests/test_helpers.rs | grep -A 20 "create_test_user"

# Identify pattern:
# - Direct SQL INSERT
# - Manual field specification
# - SELECT with explicit column list
```

**Document Current Test Pattern**:
```rust
// CURRENT PATTERN (brittle - breaks when adding columns)
pub async fn create_test_user_in_db(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    display_name: &str,
    slug: &str,
) -> Result<User> {
    // Insert with explicit columns
    sqlx::query("INSERT INTO users (email, password_hash, ...) VALUES (...)")
        .execute(pool)
        .await?;

    // SELECT with explicit column list (BREAKS when adding columns)
    sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, display_name, slug, active,
         real_name, google_user_id, timer_is_public, timer_show_in_list,
         created_at, updated_at FROM users WHERE id = $1"
    )
    .fetch_one(pool)
    .await
}
```

**Problem Identified**: Adding any new column to `users` table requires updating:
1. INSERT statement with new column
2. SELECT statement with new column
3. All tests that call `create_test_user_in_db`

**Success Criteria**:
- Current test pattern documented
- Brittleness points identified
- Test helper usage locations noted

---

### 6. Verify SQLx Cache

**Objective**: Ensure SQLx offline mode is working

```bash
# Verify sqlx-data.json exists
ls -lh backend/sqlx-data.json

# Expected output:
# -rw-r--r-- 1 user user 150K Jan 11 12:00 sqlx-data.json

# Regenerate cache to ensure it's current
./scripts/prepare-sqlx.sh --clean

# Expected output:
# ✅ SQLx cache regenerated successfully
```

**Success Criteria**:
- `sqlx-data.json` exists and is current
- Cache regeneration succeeds without errors
- Cargo build succeeds with offline mode

---

### 7. Document Current Service Dependencies

**Objective**: Map which services use `UserRepository`

```bash
# Find all UserRepository usages
rg "UserRepository" backend/src/services/ --type rust

# Expected locations:
# - auth_service/register.rs
# - auth_service/login.rs
# - auth_service/oauth.rs
# - auth_service/profile.rs
# - auth_service/password.rs
# - auth_service/data_export.rs
# - admin/user_management/mod.rs
```

**Service Dependency Map**:
```
AuthService
├── registration (creates user)
├── login (queries user by email, validates password_hash)
├── oauth (queries by google_user_id, links accounts)
├── profile (updates display_name, slug, real_name)
├── password (updates password_hash)
└── data_export (queries all user data for GDPR)

UserManagementService (Admin)
├── list users (queries with roles)
├── deactivate user (sets active = false)
└── delete user (cascades to all related tables)
```

**Success Criteria**:
- All services using `UserRepository` identified
- User CRUD operations mapped to services
- Data export dependencies noted (CRITICAL for Phase 5)

---

## Deliverables

At the end of this phase, you should have:

1. **Environment Status Report**
   - All services healthy
   - Test suite passing
   - SQLx cache current

2. **Baseline Metrics Document** (`baseline-metrics.md`)
   ```markdown
   # Baseline Metrics (Pre-Refactor)

   Date: [DATE]

   ## Test Suite
   - Backend: 227/227 passing
   - Frontend: 175/175 passing
   - Total execution time: XX seconds

   ## Performance
   [Performance metrics from Task 4]

   ## Database
   - users table: XX rows
   - Foreign key dependencies: [list from Task 3]

   ## Known Issues
   - Test brittleness: Adding columns breaks XX tests
   - OAuth: Only supports Google (no multi-provider)
   ```

3. **Current Schema Documentation** (`current-schema.sql`)
   - Copy of current `users` table DDL
   - Foreign key relationships
   - Index definitions

4. **Test Pattern Analysis** (`test-pattern-analysis.md`)
   - Current test helper usage
   - Brittleness points
   - Test files that will need updates

---

## Success Criteria

**Before proceeding to Phase 1**, verify:

- [ ] Development environment healthy (all services running)
- [ ] All 227 backend tests passing
- [ ] Baseline performance metrics recorded
- [ ] Current schema documented
- [ ] Foreign key dependencies mapped
- [ ] SQLx cache current and working
- [ ] Service dependencies identified
- [ ] Test pattern brittleness documented

**Time Check**: This phase should take 30-45 minutes. If significantly longer, troubleshoot environment issues before proceeding.

---

## Common Issues

### Issue: Tests failing before starting refactor
**Cause**: Pre-existing test failures or environment issues
**Solution**:
1. Run `./scripts/dev-stop.sh --remove`
2. Run `./scripts/dev-start.sh --build`
3. Run `./scripts/setup-test-db.sh`
4. Retry tests

### Issue: Health check failing
**Cause**: Services not fully started or port conflicts
**Solution**:
1. Check logs: `./scripts/dev-logs.sh [service]`
2. Verify ports are free: `netstat -tuln | grep -E '(443|3000|8080|5432)'`
3. Restart: `./scripts/dev-start.sh --rebuild [service]`

### Issue: SQLx cache out of date
**Cause**: Recent database migrations not reflected
**Solution**:
1. Run `./scripts/setup-db.sh`
2. Run `./scripts/prepare-sqlx.sh --clean`
3. Rebuild backend: `./scripts/dev-start.sh --rebuild backend`

---

## Next Steps

Once all success criteria are met:

1. **Commit your baseline documentation**:
   ```bash
   git add auth-refactor-docs/baseline-metrics.md
   git add auth-refactor-docs/current-schema.sql
   git add auth-refactor-docs/test-pattern-analysis.md
   git commit -m "docs: establish refactor baseline (Phase 0)"
   ```

2. **Proceed to Phase 1**:
   - Read [PHASE-01-DATABASE-SCHEMA.md](PHASE-01-DATABASE-SCHEMA.md)
   - Create new table migrations
   - Begin TDD cycle

---

**Phase Status**: ⬜ Not Started → **Continue when ready**

**Estimated Completion Time**: 30-45 minutes

**Next Phase**: [Phase 1: Database Schema & Migrations](PHASE-01-DATABASE-SCHEMA.md)
