-- Data Integrity Verification Script for Auth Schema Migration
-- Phase 7: Verify backfill data integrity and consistency
--
-- This script checks for data integrity issues after backfilling.
-- All verification queries should return 0 for "count" column.

\echo ''
\echo '=== Data Integrity Verification ==='
\echo ''

-- Check 1: Orphaned credentials (credentials without a user)
\echo 'Check 1: Orphaned credentials (should be 0)'
SELECT 'Orphaned credentials' as issue, COUNT(*) as count
FROM user_credentials uc
LEFT JOIN users u ON uc.user_id = u.id
WHERE u.id IS NULL;

-- Check 2: Orphaned external logins (external logins without a user)
\echo 'Check 2: Orphaned external logins (should be 0)'
SELECT 'Orphaned external logins' as issue, COUNT(*) as count
FROM user_external_logins uel
LEFT JOIN users u ON uel.user_id = u.id
WHERE u.id IS NULL;

-- Check 3: Orphaned profiles (profiles without a user)
\echo 'Check 3: Orphaned profiles (should be 0)'
SELECT 'Orphaned profiles' as issue, COUNT(*) as count
FROM user_profiles up
LEFT JOIN users u ON up.user_id = u.id
WHERE u.id IS NULL;

-- Check 4: Orphaned preferences (preferences without a user)
\echo 'Check 4: Orphaned preferences (should be 0)'
SELECT 'Orphaned preferences' as issue, COUNT(*) as count
FROM user_preferences upr
LEFT JOIN users u ON upr.user_id = u.id
WHERE u.id IS NULL;

-- Check 5: Users without preferences (should be 0 - every user should have preferences)
\echo 'Check 5: Users without preferences (should be 0)'
SELECT 'Users without preferences' as issue, COUNT(*) as count
FROM users u
LEFT JOIN user_preferences up ON u.id = up.user_id
WHERE up.user_id IS NULL;

-- Check 6: Password hash mismatch (credentials table should match users table)
\echo 'Check 6: Password hash mismatch (should be 0)'
SELECT 'Password hash mismatch' as issue, COUNT(*) as count
FROM users u
JOIN user_credentials uc ON u.id = uc.user_id
WHERE u.password_hash != uc.password_hash;

-- Check 7: Google ID mismatch (external_logins table should match users table)
\echo 'Check 7: Google ID mismatch (should be 0)'
SELECT 'Google ID mismatch' as issue, COUNT(*) as count
FROM users u
JOIN user_external_logins uel ON u.id = uel.user_id
WHERE u.google_user_id != uel.provider_user_id
AND uel.provider = 'google';

-- Check 8: Real name mismatch (profiles table should match users table)
\echo 'Check 8: Real name mismatch (should be 0)'
SELECT 'Real name mismatch' as issue, COUNT(*) as count
FROM users u
JOIN user_profiles up ON u.id = up.user_id
WHERE u.real_name != up.real_name;

-- Check 9: Timer preferences mismatch (preferences should match users table)
\echo 'Check 9: Timer preferences mismatch (should be 0)'
SELECT 'Timer preferences mismatch' as issue, COUNT(*) as count
FROM users u
JOIN user_preferences upr ON u.id = upr.user_id
WHERE u.timer_is_public != upr.timer_is_public
OR u.timer_show_in_list != upr.timer_show_in_list;

-- Check 10: Users with password but no credentials record
\echo 'Check 10: Users with password but no credentials (should be 0)'
SELECT 'Users with password but no credentials' as issue, COUNT(*) as count
FROM users u
WHERE u.password_hash IS NOT NULL
AND NOT EXISTS (SELECT 1 FROM user_credentials uc WHERE uc.user_id = u.id);

-- Check 11: Users with Google ID but no external login record
\echo 'Check 11: Users with Google ID but no external login (should be 0)'
SELECT 'Users with Google ID but no external login' as issue, COUNT(*) as count
FROM users u
WHERE u.google_user_id IS NOT NULL
AND NOT EXISTS (SELECT 1 FROM user_external_logins uel WHERE uel.user_id = u.id AND uel.provider = 'google');

-- Check 12: Users with real_name but no profile record
\echo 'Check 12: Users with real_name but no profile (should be 0)'
SELECT 'Users with real_name but no profile' as issue, COUNT(*) as count
FROM users u
WHERE u.real_name IS NOT NULL
AND NOT EXISTS (SELECT 1 FROM user_profiles up WHERE up.user_id = u.id);

\echo ''
\echo '=== Summary Statistics ==='
\echo ''

-- Summary: Show overall counts
SELECT
    (SELECT COUNT(*) FROM users) as total_users,
    (SELECT COUNT(*) FROM user_credentials) as users_with_credentials,
    (SELECT COUNT(*) FROM user_external_logins) as users_with_external_logins,
    (SELECT COUNT(*) FROM user_profiles) as users_with_profiles,
    (SELECT COUNT(*) FROM user_preferences) as users_with_preferences;

\echo ''
\echo '=== Expected vs Actual Counts ==='
\echo ''

-- Expected counts based on users table
SELECT
    'Expected credentials' as metric,
    COUNT(*) as count
FROM users
WHERE password_hash IS NOT NULL
UNION ALL
SELECT
    'Actual credentials' as metric,
    COUNT(*) as count
FROM user_credentials
UNION ALL
SELECT
    'Expected external logins' as metric,
    COUNT(*) as count
FROM users
WHERE google_user_id IS NOT NULL
UNION ALL
SELECT
    'Actual external logins' as metric,
    COUNT(*) as count
FROM user_external_logins
UNION ALL
SELECT
    'Expected profiles' as metric,
    COUNT(*) as count
FROM users
WHERE real_name IS NOT NULL
UNION ALL
SELECT
    'Actual profiles' as metric,
    COUNT(*) as count
FROM user_profiles
UNION ALL
SELECT
    'Expected preferences' as metric,
    COUNT(*) as count
FROM users
UNION ALL
SELECT
    'Actual preferences' as metric,
    COUNT(*) as count
FROM user_preferences;

\echo ''
\echo 'Verification complete! All "count" values should be 0 for integrity checks.'
\echo ''
