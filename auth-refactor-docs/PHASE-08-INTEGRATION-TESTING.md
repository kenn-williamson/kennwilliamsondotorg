# Phase 8: Integration Testing & Test Fixes

**Estimated Time**: 3-4 hours
**Prerequisites**: [Phase 7: Data Migration Script](PHASE-07-DATA-MIGRATION.md) complete
**Next Phase**: [Phase 9: Cutover & Cleanup](PHASE-09-CUTOVER.md)

## Objective

Fix all broken tests and add comprehensive integration tests for the new multi-table schema. Ensure all 227 backend tests pass before proceeding to production cutover.

**Key Principle**: All tests must pass before Phase 9. No exceptions.

---

## Test Categories

### Category A: Tests that need TestUserBuilder migration
- Currently use `create_test_user_in_db()` with explicit columns
- Need update to use `TestUserBuilder`

### Category B: Tests that need repository updates
- Currently mock old user_repository
- Need to mock new repositories (credentials, external_logins, etc.)

### Category C: Tests that need assertion updates
- Assert on old User struct fields
- Need to check new table data

---

## Task 1: Run Full Test Suite & Categorize Failures (30 minutes)

```bash
cd backend
cargo test -- --test-threads=4 2>&1 | tee test-failures.txt

# Categorize failures
grep "test result:" test-failures.txt
# Expected: Some tests failing
```

Create `test-failure-categories.md`:

```markdown
# Test Failures by Category

## Category A: Builder Pattern Needed (Est: XX tests)
- test_register_duplicate_email
- test_login_wrong_password
- test_oauth_new_user
- ...

## Category B: Repository Mocks Needed (Est: XX tests)
- test_update_password
- test_link_google_account
- test_get_profile
- ...

## Category C: Assertion Updates Needed (Est: XX tests)
- test_data_export_structure
- test_user_profile_includes_real_name
- ...
```

---

## Task 2: Create Mock Repositories (60 minutes)

Create mock implementations for new repositories:

### MockUserCredentialsRepository

`backend/src/repositories/mocks/mock_user_credentials_repository.rs`:

```rust
use mockall::mock;
use crate::repositories::traits::user_credentials_repository::UserCredentialsRepository;

mock! {
    pub UserCredentialsRepository {}

    #[async_trait]
    impl UserCredentialsRepository for UserCredentialsRepository {
        async fn create(&self, user_id: Uuid, password_hash: String) -> Result<UserCredentials>;
        async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserCredentials>>;
        async fn update_password(&self, user_id: Uuid, new_password_hash: String) -> Result<()>;
        async fn delete(&self, user_id: Uuid) -> Result<()>;
        async fn has_password(&self, user_id: Uuid) -> Result<bool>;
    }
}
```

### MockUserExternalLoginRepository

Similar structure for external logins.

### MockUserProfileRepository

Similar structure for profiles.

### MockUserPreferencesRepository

Similar structure for preferences.

Update `backend/src/repositories/mocks/mod.rs`:

```rust
pub mod mock_user_credentials_repository;
pub mod mock_user_external_login_repository;
pub mod mock_user_profile_repository;
pub mod mock_user_preferences_repository;

pub use mock_user_credentials_repository::MockUserCredentialsRepository;
pub use mock_user_external_login_repository::MockUserExternalLoginRepository;
pub use mock_user_profile_repository::MockUserProfileRepository;
pub use mock_user_preferences_repository::MockUserPreferencesRepository;
```

---

## Task 3: Migrate Tests to TestUserBuilder (60 minutes)

Update tests systematically:

```rust
// BEFORE
let user = create_test_user_in_db(
    &pool,
    "test@example.com",
    &test_password_hash(),
    "Test User",
    "test-user"
).await?;

// AFTER
let user = TestUserBuilder::new(&pool)
    .with_email("test@example.com")
    .with_password("Test123!@#")
    .build()
    .await?;
```

**Strategy**: Fix highest-value tests first
1. Registration tests
2. Login tests
3. OAuth tests
4. Profile tests
5. Admin tests

---

## Task 4: Add Multi-Table Integration Tests (60 minutes)

### Test: Registration creates all tables

```rust
#[tokio::test]
async fn test_registration_creates_multi_table_data() {
    let ctx = TestContext::builder().build().await;

    let register_req = json!({
        "email": "newuser@example.com",
        "password": "Password123!@#",
        "display_name": "New User"
    });

    let resp = ctx.server
        .post("/backend/auth/register")
        .send_json(&register_req)
        .await;

    assert_eq!(resp.status(), 200);

    let body: AuthResponse = resp.json().await;
    let user_id = body.user.id;

    // Verify user created
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
    assert_eq!(user.email, "newuser@example.com");

    // Verify credentials created
    let creds = sqlx::query("SELECT * FROM user_credentials WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&ctx.pool)
        .await;
    assert!(creds.is_ok());

    // Verify preferences created
    let prefs = sqlx::query("SELECT * FROM user_preferences WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&ctx.pool)
        .await;
    assert!(prefs.is_ok());
}
```

### Test: OAuth login creates external_login

```rust
#[tokio::test]
async fn test_oauth_creates_external_login() {
    // Mock OAuth service
    let mut mock_oauth = MockGoogleOAuthService::new();
    mock_oauth.expect_get_user_info()
        .returning(|| Ok(GoogleUserInfo {
            sub: "google_12345".to_string(),
            email: "oauth@example.com".to_string(),
            name: "OAuth User".to_string(),
            picture: "https://example.com/pic.jpg".to_string(),
        }));

    let ctx = TestContext::builder()
        .with_oauth(mock_oauth)
        .build()
        .await;

    // Simulate OAuth callback
    let resp = ctx.server
        .get("/backend/auth/google/callback?code=test_code&state=test_state")
        .await;

    assert_eq!(resp.status(), 200);

    let body: AuthResponse = resp.json().await;
    let user_id = body.user.id;

    // Verify external_login created
    let login = sqlx::query_as::<_, UserExternalLogin>(
        "SELECT * FROM user_external_logins WHERE user_id = $1 AND provider = 'google'"
    )
    .bind(user_id)
    .fetch_one(&ctx.pool)
    .await
    .unwrap();

    assert_eq!(login.provider, "google");
    assert_eq!(login.provider_user_id, "google_12345");
}
```

### Test: Data export includes all new tables

```rust
#[tokio::test]
async fn test_data_export_includes_all_new_tables() {
    let ctx = TestContext::builder().build().await;

    // Create user with complete data
    let user = TestUserBuilder::new(&ctx.pool)
        .with_email("complete@example.com")
        .with_password("password123")
        .with_google("google_123")
        .with_real_name("Complete User")
        .timer_public()
        .build()
        .await
        .unwrap();

    // Export data
    let resp = ctx.server
        .get(&format!("/backend/auth/export-data"))
        .insert_header(("Authorization", format!("Bearer {}", create_test_jwt(&user).await.unwrap())))
        .await;

    assert_eq!(resp.status(), 200);

    let export: UserDataExport = resp.json().await;

    // Verify version
    assert_eq!(export.export_version, "2.0");

    // Verify authentication data
    assert!(export.authentication.has_password);
    assert!(export.authentication.password_last_changed.is_some());

    // Verify external logins
    assert_eq!(export.external_logins.len(), 1);
    assert_eq!(export.external_logins[0].provider, "google");

    // Verify profile
    assert!(export.profile.is_some());
    assert_eq!(export.profile.as_ref().unwrap().real_name, Some("Complete User".to_string()));

    // Verify preferences
    assert!(export.preferences.is_some());
    assert_eq!(export.preferences.as_ref().unwrap().timer_is_public, true);
}
```

---

## Task 5: Fix Test Helpers (30 minutes)

Update `TestContext` to use new repositories:

```rust
impl TestContext {
    pub async fn create_verified_user(&self, email: &str, slug: &str) -> User {
        TestUserBuilder::new(&self.pool)
            .with_email(email)
            .build()
            .await
            .unwrap()
    }

    pub async fn create_oauth_user(&self, email: &str, google_id: &str) -> User {
        TestUserBuilder::new(&self.pool)
            .with_email(email)
            .with_google(google_id)
            .oauth_only()
            .build()
            .await
            .unwrap()
    }
}
```

---

## Task 6: Run Full Test Suite Until All Pass (30 minutes)

```bash
# Run tests iteratively
cargo test -- --test-threads=4

# Fix failures one by one
# Re-run after each fix

# Target: All 227 tests passing
```

---

## Deliverables

1. **Mock repositories**: All 4 new repositories mocked
2. **Migrated tests**: All tests use TestUserBuilder
3. **Integration tests**: Multi-table scenarios covered
4. **All tests passing**: 227/227 backend tests green

---

## Success Criteria

- [ ] MockUserCredentialsRepository created
- [ ] MockUserExternalLoginRepository created
- [ ] MockUserProfileRepository created
- [ ] MockUserPreferencesRepository created
- [ ] All tests migrated to TestUserBuilder
- [ ] Registration multi-table test added
- [ ] OAuth multi-table test added
- [ ] Data export multi-table test added
- [ ] All 227 backend tests passing
- [ ] No compilation errors
- [ ] No warnings

**Time Check**: 3-4 hours

---

## Test Failure Troubleshooting

### Common Patterns

**Pattern 1**: "Column not found"
- **Cause**: Test queries old User struct
- **Fix**: Update to query new tables or use builder

**Pattern 2**: "User has no password_hash field"
- **Cause**: Checking user.password_hash directly
- **Fix**: Query user_credentials table

**Pattern 3**: "google_user_id not found"
- **Cause**: Checking user.google_user_id directly
- **Fix**: Query user_external_logins table

**Pattern 4**: Mock expectation not set
- **Cause**: Test uses new repository not mocked
- **Fix**: Add mock expectations for new repositories

---

## Next Steps

**ONLY proceed when all tests pass**:

```bash
git add backend/src/repositories/mocks/
git add backend/tests/
git commit -m "feat(tests): update all tests for multi-table schema (Phase 8)

- Create mock repositories for new tables
- Migrate all tests to TestUserBuilder
- Add multi-table integration tests
- Fix all test failures
- All 227 backend tests passing
"
```

**Next Phase**: [Phase 9: Cutover & Cleanup](PHASE-09-CUTOVER.md)

⚠️ **BLOCKER**: Cannot proceed to Phase 9 until all tests pass.
