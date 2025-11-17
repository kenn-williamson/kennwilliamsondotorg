use backend::test_utils::UserBuilder;
use sqlx::PgPool;
use uuid::Uuid;

/// Fixture helper: Create a user with email/password (for testing account linking)
/// Uses UserBuilder pattern for resilient test fixtures
async fn create_email_password_user(pool: &PgPool, email: &str, verified: bool) -> Uuid {
    let user = UserBuilder::new()
        .with_email(email)
        .with_slug(format!("test-{}", Uuid::new_v4()))
        .with_display_name("Test User")
        .with_password("$2b$12$test_hash")
        .persist(pool)
        .await
        .expect("Failed to create email/password user");

    // Assign email-verified role if requested
    if verified {
        crate::fixtures::assign_email_verified_role(pool, &user.id.to_string()).await;
    }

    user.id
}

/// Fixture helper: Create OAuth user directly in DB (simulates existing Google user)
/// Uses UserBuilder pattern for resilient test fixtures
async fn create_oauth_user(pool: &PgPool, google_user_id: &str, email: &str) -> Uuid {
    let user = UserBuilder::new()
        .oauth(google_user_id, "Real Name")
        .with_email(email)
        .with_slug(format!("oauth-{}", Uuid::new_v4()))
        .with_display_name("OAuth User")
        .persist(pool)
        .await
        .expect("Failed to create OAuth user");

    user.id
}

// ==================== Integration Tests ====================

/// Test: New user via Google OAuth creates user with email-verified role
#[actix_web::test]
async fn test_oauth_new_user_creates_account_with_verified_role() {
    let ctx = crate::fixtures::TestContext::builder().build().await;

    // This test verifies the complete flow for a new OAuth user:
    // 1. User authenticates with Google (simulated via mock)
    // 2. System creates new user account
    // 3. User automatically gets 'user' and 'email-verified' roles
    // 4. System returns valid JWT tokens
    // 5. real_name from Google is stored in database

    // TODO: Implement once google_oauth_callback is available
    // Expected behavior per design doc section 4:
    // - Check google_user_id doesn't exist (new user)
    // - Check email doesn't exist (new user)
    // - Create OAuth user via UserRepository.create_oauth_user()
    // - Verify user.google_user_id = "google_123"
    // - Verify user.real_name = "New User"
    // - Verify user has roles: ["user", "email-verified"]
    // - Verify JWT claims include user_id and roles
    // - Verify refresh token created

    let _pool = &ctx.pool;
}

/// Test: Existing user with google_user_id logs in (no new account created)
#[actix_web::test]
async fn test_oauth_existing_google_user_logs_in() {
    let ctx = crate::fixtures::TestContext::builder().build().await;

    // Pre-create OAuth user to simulate someone who has logged in before
    let google_user_id = "existing_google_123";
    let email = "existing@example.com";
    let existing_user_id = create_oauth_user(&ctx.pool, google_user_id, email).await;

    // This test verifies returning user flow per design doc section 4:
    // When user logs in again with Google:
    // 1. System finds existing user by google_user_id (primary check)
    // 2. NO new user is created
    // 3. real_name is updated from Google profile (auto-sync)
    // 4. New JWT tokens issued for existing user
    // 5. New refresh token created

    // TODO: Implement once google_oauth_callback is available
    // Expected behavior:
    // - UserRepository.find_by_google_user_id("existing_google_123") returns existing user
    // - User count before = User count after (no duplicate)
    // - AuthResponse.user.id == existing_user_id
    // - User.real_name updated to "Updated Name From Google"
    // - User.display_name unchanged (user-controlled field)
    // - JWT claims.user_id == existing_user_id
    // - Refresh token created in refresh_tokens table

    let _existing_id = existing_user_id;
}

/// Test: Existing verified email links Google account
#[actix_web::test]
async fn test_oauth_existing_verified_email_links_account() {
    let ctx = crate::fixtures::TestContext::builder().build().await;

    // Pre-create verified email/password user (simulates user who registered with email)
    let email = crate::fixtures::unique_test_email();
    let existing_user_id = create_email_password_user(&ctx.pool, &email, true).await;

    // This test verifies account linking for verified users per design doc section 4:
    // User registered with email/password, verified their email, now wants to add Google login.
    // System should link Google to existing account (convenience + security).

    // TODO: Implement once google_oauth_callback is available
    // Expected behavior:
    // - UserRepository.find_by_google_user_id() returns None (new Google ID)
    // - UserRepository.find_by_email(email) returns existing user
    // - UserRepository.has_role(existing_user_id, "email-verified") returns true
    // - UserRepository.link_google_account(existing_user_id, "new_google_id", "Real Name")
    // - NO new user created
    // - Existing user now has google_user_id field populated
    // - Existing user now has real_name from Google
    // - AuthResponse.user.id == existing_user_id (linked account)
    // - User can now log in with either email/password OR Google
    // - email-verified role already present (no duplicate)

    let _existing_id = existing_user_id;
}

/// Test: Existing unverified email does NOT link (security)
#[actix_web::test]
async fn test_oauth_existing_unverified_email_creates_new_account_security() {
    let ctx = crate::fixtures::TestContext::builder().build().await;

    // Pre-create UNVERIFIED email/password user
    let email = "unverified@example.com";
    let unverified_user_id = create_email_password_user(&ctx.pool, email, false).await;

    // CRITICAL SECURITY TEST per design doc section 4:
    // Scenario: Someone registers with email but never verifies.
    // Attacker could register same email with Google and gain access if we blindly link.
    // Solution: Only link to VERIFIED email accounts.

    // TODO: Implement once google_oauth_callback is available
    // Expected behavior (SECURITY):
    // - UserRepository.find_by_google_user_id() returns None
    // - UserRepository.find_by_email(email) returns unverified user
    // - UserRepository.has_role(unverified_user_id, "email-verified") returns FALSE
    // - System creates NEW separate OAuth user (does NOT link)
    // - Database now has 2 users with same email:
    //   - User 1 (unverified_user_id): email/password, no google_user_id, no email-verified role
    //   - User 2 (new OAuth user): Google login, has google_user_id, HAS email-verified role
    // - AuthResponse.user.id != unverified_user_id (new user created)
    // - Original unverified account remains untouched
    // - Prevents account hijacking: attacker can't take over unverified account via OAuth

    // Note: This means one email can have multiple accounts (intentional for security).
    // Alternative design would reject OAuth with error "email exists but unverified",
    // but creating separate account is more user-friendly.

    let _unverified_id = unverified_user_id;
}

/// Test: OAuth user created with correct roles in database
#[actix_web::test]
async fn test_oauth_user_has_correct_roles_in_database() {
    let ctx = crate::fixtures::TestContext::builder().build().await;

    // This test verifies database-level role assignment for OAuth users.
    // OAuth users should get email-verified role automatically because Google
    // has already verified the email (trusted provider).

    // TODO: Implement once google_oauth_callback is available
    // Expected behavior:
    // - Create new OAuth user via google_oauth_callback
    // - Query: SELECT r.name FROM roles r JOIN user_roles ur ON r.id = ur.role_id WHERE ur.user_id = ?
    // - Verify roles = ["user", "email-verified"]
    // - Verify exactly 2 roles (no extras like "admin")
    // - This differs from email/password registration which only gets "user" initially
    // - email-verified role enables feature access immediately (no verification email needed)

    let _pool = &ctx.pool;
}

/// Test: real_name persists and updates correctly
#[actix_web::test]
async fn test_oauth_real_name_persists_and_updates() {
    let ctx = crate::fixtures::TestContext::builder().build().await;

    // This test verifies the real_name vs display_name distinction per design doc section 3:
    // - real_name: From OAuth provider, auto-updates on login, read-only to user
    // - display_name: User-controlled, used for slugs, never auto-updated
    //
    // Use case: User changes their name on Google. We want to reflect that change
    // without breaking their @username slug or custom display name preference.

    // TODO: Implement once google_oauth_callback is available
    // Expected behavior:
    // 1. First OAuth login with name "John Doe"
    //    - user.real_name = "John Doe"
    //    - user.display_name = generated from email or "John Doe"
    //    - user.slug = "john-doe" or similar
    //
    // 2. Query database: SELECT real_name, display_name FROM users WHERE id = ?
    //    - Verify real_name = "John Doe"
    //
    // 3. Second OAuth login (same google_user_id) with name "Jane Smith" (user changed name on Google)
    //    - UserRepository.update_real_name(user_id, "Jane Smith")
    //    - user.real_name updated to "Jane Smith"
    //    - user.display_name UNCHANGED (user's choice preserved)
    //    - user.slug UNCHANGED (permalinks stay stable)
    //
    // 4. Query database again
    //    - Verify real_name = "Jane Smith" (updated)
    //    - Verify display_name still equals original value (unchanged)
    //
    // This separation allows showing "Jane Smith (@john-doe)" in UI while
    // preserving user's chosen username/slug.

    let _pool = &ctx.pool;
}

#[cfg(test)]
mod account_linking_edge_cases {
    use super::*;

    /// Test: Duplicate google_user_id returns error (data integrity)
    #[actix_web::test]
    async fn test_duplicate_google_user_id_returns_error() {
        let ctx = crate::fixtures::TestContext::builder().build().await;

        // Pre-create OAuth user
        let google_user_id = "duplicate_test_123";
        let existing_id = create_oauth_user(&ctx.pool, google_user_id, "first@example.com").await;

        // This test verifies database integrity constraints.
        // google_user_id should be UNIQUE (database constraint).
        // This should never happen in practice (Google IDs are unique),
        // but we test error handling for data integrity.

        // TODO: Implement once google_oauth_callback is available
        // Expected behavior:
        // - Attempt OAuth callback with same google_user_id but different email
        // - UserRepository.find_by_google_user_id() finds existing user
        // - System should log them in as existing user (not try to create duplicate)
        // - AuthResponse.user.id == existing_id
        // - OR if implementation attempts create: database UNIQUE constraint fails
        // - No data corruption: only one user with this google_user_id exists

        let _existing = existing_id;
    }

    /// Test: Email case-insensitivity in account linking
    #[actix_web::test]
    async fn test_email_case_insensitive_linking() {
        let ctx = crate::fixtures::TestContext::builder().build().await;

        // Create user with lowercase email
        let email_lower = "testuser@example.com";
        let existing_user_id = create_email_password_user(&ctx.pool, email_lower, true).await;

        // This test verifies email matching is case-insensitive per RFC 5321.
        // Email addresses should be treated as case-insensitive for the local part
        // (though technically some servers allow case-sensitive local parts, Gmail/Google doesn't).
        //
        // User registered as "testuser@example.com", Google returns "TESTUSER@EXAMPLE.COM"
        // System should recognize as same user and link accounts.

        // TODO: Implement once google_oauth_callback is available
        // Expected behavior:
        // - OAuth callback with email "TESTUSER@EXAMPLE.COM" (uppercase)
        // - UserRepository.find_by_email() should do case-insensitive search
        //   (PostgreSQL ILIKE or LOWER() comparison)
        // - Finds existing user (testuser@example.com)
        // - Links Google account to existing user
        // - NO duplicate user created
        // - AuthResponse.user.id == existing_user_id
        // - Database still stores email in original case (testuser@example.com)
        //
        // Note: Our database schema likely stores emails as-is but queries should be case-insensitive.
        // This prevents: alice@example.com vs ALICE@example.com being treated as different users.

        let _existing = existing_user_id;
    }
}
