# Phase 7: Data Migration Script

**Estimated Time**: 1-2 hours
**Prerequisites**: [Phase 6: Test Helpers & Builder Pattern](PHASE-06-TEST-HELPERS.md) complete
**Next Phase**: [Phase 8: Integration Testing & Test Fixes](PHASE-08-INTEGRATION-TESTING.md)

## Objective

Backfill existing data from `users` table to new tables. Implement dual-write to keep both schemas in sync during transition.

**Key Principle**: Zero-downtime migration with rollback capability.

---

## Migration Strategy

### Phase 7A: Data Backfill (1 hour)

Create migration script to copy existing data:

```sql
-- Backfill user_credentials (for users with passwords)
INSERT INTO user_credentials (user_id, password_hash, password_updated_at, created_at)
SELECT id, password_hash, updated_at, created_at
FROM users
WHERE password_hash IS NOT NULL
ON CONFLICT (user_id) DO NOTHING;

-- Backfill user_external_logins (for users with Google)
INSERT INTO user_external_logins (user_id, provider, provider_user_id, linked_at, created_at)
SELECT id, 'google', google_user_id, created_at, created_at
FROM users
WHERE google_user_id IS NOT NULL
ON CONFLICT (provider, provider_user_id) DO NOTHING;

-- Backfill user_profiles (for users with profile data)
INSERT INTO user_profiles (user_id, real_name, created_at, updated_at)
SELECT id, real_name, created_at, updated_at
FROM users
WHERE real_name IS NOT NULL
ON CONFLICT (user_id) DO NOTHING;

-- Backfill user_preferences (ALL users)
INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list, created_at, updated_at)
SELECT id, timer_is_public, timer_show_in_list, created_at, updated_at
FROM users
ON CONFLICT (user_id) DO NOTHING;

-- Verify counts
SELECT
    (SELECT COUNT(*) FROM users) as total_users,
    (SELECT COUNT(*) FROM user_credentials) as users_with_password,
    (SELECT COUNT(*) FROM user_external_logins) as users_with_oauth,
    (SELECT COUNT(*) FROM user_profiles) as users_with_profile,
    (SELECT COUNT(*) FROM user_preferences) as users_with_prefs;
```

### Phase 7B: Dual-Write Implementation (Optional - 30 minutes)

**Purpose**: Keep both old and new schemas in sync during transition.

Update repositories to write to both locations:

```rust
// Example: UserCredentialsRepository
async fn update_password(&self, user_id: Uuid, new_hash: &str) -> Result<()> {
    // Write to NEW table
    sqlx::query("UPDATE user_credentials SET password_hash = $1 WHERE user_id = $2")
        .bind(new_hash)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

    // Write to OLD table (during transition)
    sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
        .bind(new_hash)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

    Ok(())
}
```

**Note**: Dual-write is optional if you're confident in the migration. For production with live users, dual-write provides safety.

---

## Tasks

### Task 1: Run Backfill Script (30 minutes)

```bash
# Create script
cat > backend/scripts/backfill-auth-schema.sql << 'EOF'
-- [SQL from above]
EOF

# Run on development database
psql postgresql://postgres:postgres@localhost:5432/kennwilliamson < backend/scripts/backfill-auth-schema.sql

# Verify
psql postgresql://postgres:postgres@localhost:5432/kennwilliamson
SELECT
    (SELECT COUNT(*) FROM users) as total,
    (SELECT COUNT(*) FROM user_credentials) as creds,
    (SELECT COUNT(*) FROM user_external_logins) as oauth,
    (SELECT COUNT(*) FROM user_profiles) as profiles,
    (SELECT COUNT(*) FROM user_preferences) as prefs;
```

### Task 2: Verify Data Integrity (30 minutes)

```sql
-- Check for orphaned data
SELECT 'Orphaned credentials' as issue, COUNT(*)
FROM user_credentials uc
LEFT JOIN users u ON uc.user_id = u.id
WHERE u.id IS NULL;

SELECT 'Orphaned external logins' as issue, COUNT(*)
FROM user_external_logins uel
LEFT JOIN users u ON uel.user_id = u.id
WHERE u.id IS NULL;

-- Check for missing preferences (should be 0)
SELECT 'Users without preferences' as issue, COUNT(*)
FROM users u
LEFT JOIN user_preferences up ON u.id = up.user_id
WHERE up.user_id IS NULL;

-- Verify password hashes match
SELECT 'Password mismatch' as issue, COUNT(*)
FROM users u
JOIN user_credentials uc ON u.id = uc.user_id
WHERE u.password_hash != uc.password_hash;

-- Verify Google IDs match
SELECT 'Google ID mismatch' as issue, COUNT(*)
FROM users u
JOIN user_external_logins uel ON u.id = uel.user_id
WHERE u.google_user_id != uel.provider_user_id
AND uel.provider = 'google';
```

All counts should be 0.

### Task 3: Implement Rollback Procedure (15 minutes)

```sql
-- Rollback script (if needed)
DELETE FROM user_preferences;
DELETE FROM user_profiles;
DELETE FROM user_external_logins;
DELETE FROM user_credentials;

-- Verify clean
SELECT
    (SELECT COUNT(*) FROM user_credentials) as creds,
    (SELECT COUNT(*) FROM user_external_logins) as oauth,
    (SELECT COUNT(*) FROM user_profiles) as profiles,
    (SELECT COUNT(*) FROM user_preferences) as prefs;
-- All should be 0
```

---

## Deliverables

1. **Backfill script**: SQL to copy existing data
2. **Verification queries**: Data integrity checks
3. **Rollback procedure**: Documented steps
4. **Data checksums**: Verified match

---

## Success Criteria

- [ ] Backfill script created
- [ ] All existing user data copied to new tables
- [ ] No orphaned data
- [ ] No missing preferences
- [ ] Password hashes match between tables
- [ ] Google IDs match between tables
- [ ] Rollback procedure tested
- [ ] Data counts verified

**Time Check**: 1-2 hours

---

## Production Checklist

Before running in production:

1. **Full database backup**
2. **Test backfill on production copy**
3. **Run during low-traffic window**
4. **Monitor for errors**
5. **Verify data counts**
6. **Keep old columns** (don't drop yet - Phase 9)

---

## Next Steps

```bash
git add backend/scripts/backfill-auth-schema.sql
git commit -m "feat(migration): add data backfill script for auth schema (Phase 7)

- Backfill user_credentials from users.password_hash
- Backfill user_external_logins from users.google_user_id
- Backfill user_profiles from users.real_name
- Backfill user_preferences from users.timer_* fields
- Add data integrity verification queries
- Document rollback procedure
"
```

**Next Phase**: [Phase 8: Integration Testing & Test Fixes](PHASE-08-INTEGRATION-TESTING.md)
