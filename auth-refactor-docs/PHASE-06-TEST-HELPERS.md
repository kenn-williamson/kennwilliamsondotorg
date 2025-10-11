# Phase 6: Test Helpers & Builder Pattern

**Estimated Time**: 2-3 hours
**Prerequisites**: [Phase 5: Data Export (CRITICAL)](PHASE-05-DATA-EXPORT.md) complete
**Next Phase**: [Phase 7: Data Migration Script](PHASE-07-DATA-MIGRATION.md)

## Objective

Implement builder pattern for test user creation to eliminate test brittleness. When we add new preference fields in the future, tests won't break.

**Key Principle**: Tests should use builder pattern, not raw SQL with explicit column lists.

---

## Problem

Current test pattern (from Phase 0):

```rust
// BRITTLE: Breaks when adding columns
pub async fn create_test_user_in_db(...) -> Result<User> {
    sqlx::query("INSERT INTO users (email, password_hash, display_name, slug, active, real_name, google_user_id, timer_is_public, timer_show_in_list, ...) VALUES (...)")
        .execute(pool).await?;

    // SELECT with ALL columns explicitly listed
    sqlx::query_as::<_, User>("SELECT id, email, password_hash, display_name, slug, active, real_name, google_user_id, timer_is_public, timer_show_in_list, created_at, updated_at FROM users WHERE id = $1")
        .fetch_one(pool).await
}
```

**Problem**: Adding any new column requires updating:
1. INSERT statement
2. SELECT statement
3. Every test that calls this helper

---

## Solution: Builder Pattern

```rust
// RESILIENT: Adding columns doesn't break tests
let user = TestUserBuilder::new(&pool)
    .with_email("test@example.com")
    .with_password("password123")
    .timer_public()
    .build()
    .await?;
```

---

## Task 1: Implement TestUserBuilder (90 minutes)

Create in `backend/tests/test_helpers.rs`:

```rust
/// Builder for creating test users with flexible configuration
pub struct TestUserBuilder<'a> {
    pool: &'a PgPool,
    email: String,
    display_name: String,
    slug: String,
    password: Option<String>,
    google_id: Option<String>,
    github_id: Option<String>,
    real_name: Option<String>,
    bio: Option<String>,
    timer_public: bool,
    timer_in_list: bool,
}

impl<'a> TestUserBuilder<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        let rand_id = Uuid::new_v4();
        Self {
            pool,
            email: format!("test-{}@example.com", rand_id),
            display_name: format!("Test User {}", rand_id),
            slug: format!("test-{}", rand_id),
            password: Some("Test123!@#".to_string()),
            google_id: None,
            github_id: None,
            real_name: None,
            bio: None,
            timer_public: false,
            timer_in_list: false,
        }
    }

    /// Set custom email
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    /// Set custom password
    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Create OAuth-only user (no password)
    pub fn oauth_only(mut self) -> Self {
        self.password = None;
        self
    }

    /// Link Google account
    pub fn with_google(mut self, google_id: impl Into<String>) -> Self {
        self.google_id = Some(google_id.into());
        self
    }

    /// Link GitHub account
    pub fn with_github(mut self, github_id: impl Into<String>) -> Self {
        self.github_id = Some(github_id.into());
        self
    }

    /// Set real name
    pub fn with_real_name(mut self, real_name: impl Into<String>) -> Self {
        self.real_name = Some(real_name.into());
        self
    }

    /// Set bio
    pub fn with_bio(mut self, bio: impl Into<String>) -> Self {
        self.bio = Some(bio.into());
        self
    }

    /// Make timer public
    pub fn timer_public(mut self) -> Self {
        self.timer_public = true;
        self
    }

    /// Show timer in public list
    pub fn timer_in_list(mut self) -> Self {
        self.timer_public = true;  // Required for list
        self.timer_in_list = true;
        self
    }

    /// Build the test user (creates in database)
    pub async fn build(self) -> Result<User> {
        // 1. Create user (core identity)
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, display_name, slug) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&self.email)
        .bind(&self.display_name)
        .bind(&self.slug)
        .fetch_one(self.pool)
        .await?;

        // 2. Create credentials if password provided
        if let Some(password) = self.password {
            let password_hash = bcrypt::hash(password, 4)?;  // Low cost for tests
            sqlx::query("INSERT INTO user_credentials (user_id, password_hash) VALUES ($1, $2)")
                .bind(user.id)
                .bind(password_hash)
                .execute(self.pool)
                .await?;
        }

        // 3. Create external logins if provided
        if let Some(google_id) = self.google_id {
            sqlx::query("INSERT INTO user_external_logins (user_id, provider, provider_user_id) VALUES ($1, 'google', $2)")
                .bind(user.id)
                .bind(google_id)
                .execute(self.pool)
                .await?;
        }

        if let Some(github_id) = self.github_id {
            sqlx::query("INSERT INTO user_external_logins (user_id, provider, provider_user_id) VALUES ($1, 'github', $2)")
                .bind(user.id)
                .bind(github_id)
                .execute(self.pool)
                .await?;
        }

        // 4. Create profile if data provided
        if self.real_name.is_some() || self.bio.is_some() {
            sqlx::query("INSERT INTO user_profiles (user_id, real_name, bio) VALUES ($1, $2, $3)")
                .bind(user.id)
                .bind(&self.real_name)
                .bind(&self.bio)
                .execute(self.pool)
                .await?;
        }

        // 5. Create preferences (always)
        sqlx::query("INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list) VALUES ($1, $2, $3)")
            .bind(user.id)
            .bind(self.timer_public)
            .bind(self.timer_in_list)
            .execute(self.pool)
            .await?;

        Ok(user)
    }
}
```

---

## Task 2: Usage Examples & Documentation (30 minutes)

Add documentation to test_helpers.rs:

```rust
/// # Test User Builder Pattern
///
/// Use `TestUserBuilder` to create test users with flexible configuration.
///
/// ## Examples
///
/// ```
/// // Simple user with password
/// let user = TestUserBuilder::new(&pool)
///     .build()
///     .await?;
///
/// // OAuth-only user (Google)
/// let oauth_user = TestUserBuilder::new(&pool)
///     .with_email("oauth@example.com")
///     .with_google("google_12345")
///     .oauth_only()
///     .build()
///     .await?;
///
/// // User with both password and OAuth
/// let hybrid_user = TestUserBuilder::new(&pool)
///     .with_password("password123")
///     .with_google("google_123")
///     .build()
///     .await?;
///
/// // User with complete profile
/// let complete_user = TestUserBuilder::new(&pool)
///     .with_real_name("John Doe")
///     .with_bio("Software developer")
///     .timer_public()
///     .build()
///     .await?;
///
/// // User with multiple OAuth providers
/// let multi_oauth = TestUserBuilder::new(&pool)
///     .with_google("google_123")
///     .with_github("github_456")
///     .oauth_only()
///     .build()
///     .await?;
/// ```
///
/// ## Benefits
///
/// - **Resilient**: Adding new preference fields doesn't break tests
/// - **Readable**: Clear what each test user represents
/// - **Flexible**: Easy to create different user scenarios
/// - **Default values**: Sensible defaults for common cases
```

---

## Task 3: Migrate Existing Tests (60 minutes)

Update a few key tests to use builder pattern as examples:

### Example 1: Simple test migration

```rust
// BEFORE
let user = create_test_user_in_db(
    &pool,
    "test@example.com",
    "password_hash",
    "Test User",
    "test-user"
).await?;

// AFTER
let user = TestUserBuilder::new(&pool)
    .with_email("test@example.com")
    .build()
    .await?;
```

### Example 2: OAuth test migration

```rust
// BEFORE
let oauth_user = create_oauth_user(&pool, "oauth@example.com", "oauth-user", "google_123").await?;

// AFTER
let oauth_user = TestUserBuilder::new(&pool)
    .with_email("oauth@example.com")
    .with_google("google_123")
    .oauth_only()
    .build()
    .await?;
```

### Example 3: Admin user migration

```rust
// BEFORE
let admin = create_test_user_in_db(&pool, "admin@test.com", "hash", "Admin", "admin").await?;
add_admin_role_to_user(&pool, admin.id).await?;

// AFTER
let admin = TestUserBuilder::new(&pool)
    .with_email("admin@test.com")
    .build()
    .await?;
add_admin_role_to_user(&pool, admin.id).await?;  // Keep role assignment separate
```

**Note**: Don't migrate ALL tests in this phase. Just update 3-5 tests as examples. Full migration happens in Phase 8.

---

## Task 4: Test the Builder (30 minutes)

Add builder tests:

```rust
#[tokio::test]
async fn test_user_builder_default() {
    let pool = setup_test_pool().await;
    let user = TestUserBuilder::new(&pool).build().await.unwrap();

    // Has user record
    assert!(!user.email.is_empty());

    // Has credentials
    let creds = sqlx::query_as::<_, UserCredentials>(
        "SELECT * FROM user_credentials WHERE user_id = $1"
    )
    .bind(user.id)
    .fetch_one(&pool)
    .await;
    assert!(creds.is_ok());

    // Has preferences
    let prefs = sqlx::query_as::<_, UserPreferences>(
        "SELECT * FROM user_preferences WHERE user_id = $1"
    )
    .bind(user.id)
    .fetch_one(&pool)
    .await;
    assert!(prefs.is_ok());
}

#[tokio::test]
async fn test_user_builder_oauth_only() {
    let pool = setup_test_pool().await;
    let user = TestUserBuilder::new(&pool)
        .with_google("google_123")
        .oauth_only()
        .build()
        .await
        .unwrap();

    // NO credentials
    let creds = sqlx::query("SELECT * FROM user_credentials WHERE user_id = $1")
        .bind(user.id)
        .fetch_optional(&pool)
        .await
        .unwrap();
    assert!(creds.is_none());

    // HAS external login
    let login = sqlx::query("SELECT * FROM user_external_logins WHERE user_id = $1 AND provider = 'google'")
        .bind(user.id)
        .fetch_one(&pool)
        .await;
    assert!(login.is_ok());
}

#[tokio::test]
async fn test_user_builder_multiple_providers() {
    let pool = setup_test_pool().await;
    let user = TestUserBuilder::new(&pool)
        .with_google("google_123")
        .with_github("github_456")
        .oauth_only()
        .build()
        .await
        .unwrap();

    // HAS both external logins
    let logins = sqlx::query_as::<_, UserExternalLogin>(
        "SELECT * FROM user_external_logins WHERE user_id = $1"
    )
    .bind(user.id)
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(logins.len(), 2);
}
```

---

## Deliverables

1. **TestUserBuilder**: Complete builder implementation
2. **Documentation**: Usage examples and benefits
3. **Example migrations**: 3-5 tests updated to use builder
4. **Builder tests**: Comprehensive builder verification

---

## Success Criteria

- [ ] TestUserBuilder implemented with fluent API
- [ ] Supports password users, OAuth users, and hybrid users
- [ ] Supports multiple OAuth providers
- [ ] Documentation with clear examples
- [ ] 3-5 tests migrated to use builder
- [ ] Builder unit tests pass
- [ ] Full test suite still passes (227 tests)

**Time Check**: 2-3 hours

---

## Migration Strategy

**Don't migrate all tests now!** Phase 8 will handle bulk test migration.

This phase establishes the pattern. Future preference additions:

```rust
// Future: Add notification_email preference
pub fn with_email_notifications(mut self, enabled: bool) -> Self {
    self.notification_email = enabled;
    self
}

// In build():
sqlx::query("INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list, notification_email) VALUES ($1, $2, $3, $4)")
    .bind(user.id)
    .bind(self.timer_public)
    .bind(self.timer_in_list)
    .bind(self.notification_email)  // NEW
    .execute(self.pool)
    .await?;

// Existing tests DON'T break!
```

---

## Next Steps

```bash
git add backend/tests/test_helpers.rs
git commit -m "feat(tests): add TestUserBuilder with builder pattern (Phase 6)

- Implement fluent builder API for test user creation
- Support password users, OAuth users, and hybrid scenarios
- Support multiple OAuth providers (Google, GitHub)
- Add comprehensive documentation and examples
- Migrate 3-5 tests to demonstrate usage
- Future-proof: Adding preferences won't break tests
"
```

**Next Phase**: [Phase 7: Data Migration Script](PHASE-07-DATA-MIGRATION.md)
