# Phase 9: Cutover & Cleanup

**Estimated Time**: 1-2 hours
**Prerequisites**: [Phase 8: Integration Testing & Test Fixes](PHASE-08-INTEGRATION-TESTING.md) complete (ALL TESTS PASSING)
**Next Phase**: None - Refactor Complete!

## Objective

Complete the migration by cleaning up old schema columns and verifying production deployment.

**Key Principle**: Only remove old columns after confirmed stable in production.

---

## ⚠️ Prerequisites Checklist

Before starting this phase, verify:

- [x] All 227 backend tests passing
- [x] All 175 frontend tests passing
- [x] Data export includes all new tables (Phase 5 verified)
- [x] Data migration completed successfully (Phase 7)
- [x] Application runs successfully with new schema
- [x] Smoke tests completed in development environment

**DO NOT proceed unless ALL boxes checked.**

---

## Production Deployment Strategy

### Option A: Blue-Green Deployment (Recommended)

1. Deploy new code to staging
2. Run full test suite on staging
3. Monitor staging for 24 hours
4. Deploy to production (blue-green swap)
5. Monitor production for 48 hours
6. Proceed with cleanup if stable

### Option B: Rolling Deployment

1. Deploy new code to single production instance
2. Monitor for 1 hour
3. Roll out to remaining instances
4. Monitor for 48 hours
5. Proceed with cleanup if stable

---

## Task 1: Pre-Deployment Verification (30 minutes)

### Development Environment Final Check

```bash
# Start clean environment
./scripts/dev-stop.sh --remove
./scripts/dev-start.sh --build

# Health check
./scripts/health-check.sh --dev

# Run full test suite
cd backend && cargo test -- --test-threads=4
# Expected: All 227 tests passing

cd ../frontend && npm test
# Expected: All 175 tests passing

# Manual smoke tests
# 1. Register new user
# 2. Login with password
# 3. Link Google OAuth
# 4. Update profile
# 5. Update preferences
# 6. Export data
# 7. Delete account
```

### Database State Verification

```sql
-- Verify data consistency
SELECT
    'users' as table_name,
    COUNT(*) as count
FROM users
UNION ALL
SELECT 'user_credentials', COUNT(*) FROM user_credentials
UNION ALL
SELECT 'user_external_logins', COUNT(*) FROM user_external_logins
UNION ALL
SELECT 'user_profiles', COUNT(*) FROM user_profiles
UNION ALL
SELECT 'user_preferences', COUNT(*) FROM user_preferences;

-- All user_preferences count should equal users count
```

---

## Task 2: Create Cleanup Migration (15 minutes)

Create `backend/migrations/XXXXXXXX_drop_old_user_columns.up.sql`:

```sql
-- ============================================================================
-- DROP OLD COLUMNS FROM USERS TABLE
-- This completes the auth schema refactor
-- ONLY run after verifying new schema works in production for 48+ hours
-- ============================================================================

-- Drop columns that have been moved to other tables
ALTER TABLE users DROP COLUMN IF EXISTS password_hash;
ALTER TABLE users DROP COLUMN IF EXISTS google_user_id;
ALTER TABLE users DROP COLUMN IF EXISTS real_name;
ALTER TABLE users DROP COLUMN IF EXISTS timer_is_public;
ALTER TABLE users DROP COLUMN IF EXISTS timer_show_in_list;

-- Verify users table now only has core identity fields
-- \d users should show:
-- id, email, display_name, slug, active, created_at, updated_at
```

Create `backend/migrations/XXXXXXXX_drop_old_user_columns.down.sql`:

```sql
-- ============================================================================
-- ROLLBACK: Re-add columns to users table
-- This is for emergency rollback only
-- Data will be lost if columns were already dropped
-- ============================================================================

ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255);
ALTER TABLE users ADD COLUMN IF NOT EXISTS google_user_id VARCHAR(255) UNIQUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS real_name VARCHAR(255);
ALTER TABLE users ADD COLUMN IF NOT EXISTS timer_is_public BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE users ADD COLUMN IF NOT EXISTS timer_show_in_list BOOLEAN NOT NULL DEFAULT false;

-- NOTE: This rollback cannot restore data if columns were already dropped
-- Always backup database before running cleanup migration
```

**DO NOT RUN THIS MIGRATION YET**

---

## Task 3: Production Deployment (Variable)

This depends on your deployment infrastructure. General steps:

```bash
# 1. Create full database backup
# (Your production backup procedure)

# 2. Deploy new code
git tag v2.0.0-auth-refactor
git push origin v2.0.0-auth-refactor

# 3. Deploy to production
# (Your deployment procedure)

# 4. Verify health
curl https://kennwilliamson.org/backend/health
# Expected: {"status": "healthy"}

# 5. Monitor logs
# (Your monitoring procedure)

# 6. Run smoke tests on production
# - Register new account
# - Login
# - OAuth flow
# - Profile updates
# - Data export
```

---

## Task 4: Observation Period (48 hours)

### Monitoring Checklist

**Hour 1**: Intensive monitoring
- [ ] No errors in application logs
- [ ] All API endpoints responding
- [ ] Database queries executing successfully
- [ ] No user-reported issues

**Hour 6**: Regular monitoring
- [ ] Performance metrics normal
- [ ] Error rates normal
- [ ] User activity normal

**Hour 24**: Daily check
- [ ] All systems stable
- [ ] No anomalies detected

**Hour 48**: Final check before cleanup
- [ ] Confirmed stable for 48 hours
- [ ] No rollback needed
- [ ] Ready for cleanup migration

---

## Task 5: Run Cleanup Migration (15 minutes)

**ONLY after 48 hour observation period**:

```bash
# Full database backup FIRST
# (Your backup procedure)

# Run cleanup migration
cd backend
sqlx migrate run

# Verify columns dropped
psql $DATABASE_URL
\d users

# Expected output - users table should only have:
# id, email, display_name, slug, active, created_at, updated_at

# Verify old columns gone
\d users
# Should NOT show: password_hash, google_user_id, real_name, timer_is_public, timer_show_in_list
```

---

## Task 6: Post-Cleanup Verification (15 minutes)

### Verify Application Still Works

```bash
# Run tests against production schema
cargo test -- --test-threads=4
# Expected: All 227 tests passing

# Manual verification
# 1. Register new user → Works
# 2. Login → Works
# 3. OAuth → Works
# 4. Profile update → Works
# 5. Data export → Works (version 2.0)
```

### Verify Database Schema

```sql
-- Verify final schema
SELECT
    table_name,
    column_name,
    data_type
FROM information_schema.columns
WHERE table_schema = 'public'
    AND table_name IN ('users', 'user_credentials', 'user_external_logins', 'user_profiles', 'user_preferences')
ORDER BY table_name, ordinal_position;

-- Should show clean separation:
-- users: id, email, display_name, slug, active, created_at, updated_at
-- user_credentials: user_id, password_hash, password_updated_at, created_at
-- user_external_logins: id, user_id, provider, provider_user_id, linked_at, created_at, updated_at
-- user_profiles: user_id, real_name, bio, avatar_url, location, website, created_at, updated_at
-- user_preferences: user_id, timer_is_public, timer_show_in_list, created_at, updated_at
```

---

## Task 7: Documentation Updates (30 minutes)

Update project documentation:

### IMPLEMENTATION-DATABASE.md

Update schema documentation to reflect new tables.

### DESIGN-USER-AUTH-SCHEMA-REFACTOR.md

Add completion status:

```markdown
## Implementation Status

**Status**: ✅ COMPLETE
**Completion Date**: [DATE]
**Version**: 2.0.0

All phases completed successfully:
- [x] Phase 0: Setup & Baseline
- [x] Phase 1: Database Schema
- [x] Phase 2A: Core User Models
- [x] Phase 2B: New Table Models
- [x] Phase 3A: Credentials & External Login Repositories
- [x] Phase 3B: Profile & Preferences Repositories
- [x] Phase 4A: Registration Service
- [x] Phase 4B: Login Service
- [x] Phase 4C: OAuth Service
- [x] Phase 4D: Profile Management Service
- [x] Phase 5: Data Export (CRITICAL)
- [x] Phase 6: Test Helpers & Builder Pattern
- [x] Phase 7: Data Migration
- [x] Phase 8: Integration Testing
- [x] Phase 9: Cutover & Cleanup

**Results**:
- All 227 backend tests passing
- All 175 frontend tests passing
- Data export GDPR/CCPA compliant (version 2.0)
- Zero downtime deployment
- Clean multi-table architecture
```

### CHANGELOG.md

Add entry:

```markdown
## [2.0.0] - [DATE]

### Major Changes

**User Authentication Schema Refactor**

Refactored user authentication from monolithic `users` table to normalized multi-table architecture.

**New Tables:**
- `user_credentials`: Local password authentication
- `user_external_logins`: Multi-provider OAuth support (Google, GitHub, Microsoft, etc.)
- `user_profiles`: Optional profile data (bio, avatar, location, website)
- `user_preferences`: User settings (scalable for future features)

**Benefits:**
- Adding preference fields no longer breaks tests
- Multi-provider OAuth support (prepare for GitHub, Microsoft, LinkedIn)
- Improved data organization and separation of concerns
- Better performance (narrow tables, targeted queries)
- GDPR/CCPA compliant data export (version 2.0)

**Breaking Changes:**
- Data export format changed from version 1.0 to 2.0
- Internal API changes (service layer uses multiple repositories)

**Migration Notes:**
- Existing data automatically migrated to new tables
- Old columns removed from users table
- All tests updated and passing
```

---

## Deliverables

1. **Cleanup migration**: SQL to drop old columns
2. **Production deployment**: Successful deployment with monitoring
3. **Updated documentation**: Database schema and changelog
4. **Final verification**: All tests passing with clean schema

---

## Success Criteria

**Refactor is complete when**:

- [ ] Application deployed to production successfully
- [ ] 48 hour observation period completed without issues
- [ ] Cleanup migration run successfully
- [ ] Old columns dropped from users table
- [ ] All 227 backend tests passing
- [ ] All 175 frontend tests passing
- [ ] Documentation updated
- [ ] Data export working (version 2.0)
- [ ] No user-facing issues reported

**Time Check**: 1-2 hours (plus 48 hour observation)

---

## Rollback Procedure

If critical issues discovered:

### Before Cleanup Migration

```bash
# Revert code deployment
# (Your rollback procedure)

# Database already has both old and new data
# Old code can still work with old columns
```

### After Cleanup Migration

```sql
-- Restore from backup
# (Your restore procedure)

-- OR run DOWN migration (loses data in dropped columns)
sqlx migrate revert
```

**Always prefer backup restore over DOWN migration.**

---

## Celebration Checklist 🎉

- [ ] Tag release: `v2.0.0-auth-refactor`
- [ ] Update project README with new architecture
- [ ] Document lessons learned
- [ ] Share success with team
- [ ] Archive phase documents for future reference

---

## Post-Refactor Benefits

**Immediate:**
- ✅ Clean separation of concerns
- ✅ Multi-provider OAuth ready
- ✅ Test brittleness eliminated
- ✅ GDPR/CCPA compliant export

**Future:**
- Adding messaging preferences: Add columns to `user_preferences` (tests don't break!)
- Adding GitHub OAuth: Add rows to `user_external_logins` (same pattern as Google)
- Adding profile fields: Add columns to `user_profiles` (no impact on auth)
- Scaling users: Narrow tables perform better

---

## Next Steps

```bash
git add backend/migrations/XXXXXXXX_drop_old_user_columns.*.sql
git add IMPLEMENTATION-DATABASE.md
git add CHANGELOG.md
git add DESIGN-USER-AUTH-SCHEMA-REFACTOR.md
git commit -m "feat(schema): complete auth schema refactor (Phase 9)

- Drop old columns from users table
- Update documentation for new schema
- Add changelog entry for v2.0.0
- Mark refactor complete

BREAKING CHANGE: users table now only contains core identity fields
All auth/profile/preference data moved to dedicated tables
"

git tag v2.0.0
git push origin main --tags
```

---

**🎉 REFACTOR COMPLETE! 🎉**

**Total Time Invested**: 20-28 hours over 1-2 weeks

**Value Delivered**:
- Future-proof architecture
- Multi-provider OAuth support
- Eliminated test brittleness
- GDPR/CCPA compliance
- Better performance and scalability

**Well done!** 🚀
