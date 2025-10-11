# Phase 4B: Login & Authentication Service

**Estimated Time**: 2-3 hours
**Prerequisites**: [Phase 4A: Registration Service](PHASE-04A-REGISTRATION-SERVICE.md) complete
**Next Phase**: [Phase 4C: OAuth Service Updates](PHASE-04C-OAUTH-SERVICE.md)

## Objective

Update login service to query `user_credentials` table for password verification instead of checking `password_hash` in the `users` table.

**Key Principle**: Support both local auth users (with password) and OAuth-only users (without password).

---

## TDD Approach

1. **Red**: Write tests for login scenarios (password, OAuth-only, etc.)
2. **Green**: Update login logic to use credentials repository
3. **Refactor**: Handle edge cases

---

## Current Login Flow

From `backend/src/services/auth/auth_service/login.rs`:

```rust
// CURRENT: Password hash stored in users table
pub async fn login(&self, email: &str, password: &str) -> Result<AuthResponse> {
    let user = self.user_repository
        .find_by_email(email)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;

    // Verify password from users.password_hash
    if let Some(password_hash) = &user.password_hash {
        if !verify_password(password, password_hash)? {
            return Err(anyhow::anyhow!("Invalid credentials"));
        }
    } else {
        return Err(anyhow::anyhow!("No password set"));
    }

    // Generate tokens...
}
```

---

## New Login Flow

```rust
// NEW: Password hash stored in user_credentials table
pub async fn login(&self, email: &str, password: &str) -> Result<AuthResponse> {
    // 1. Find user by email
    let user = self.user_repository
        .find_by_email(email)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;

    // 2. Check if user has password credentials
    let credentials = self.credentials_repository
        .find_by_user_id(user.id)
        .await?;

    match credentials {
        Some(creds) => {
            // User has password - verify it
            if !verify_password(password, &creds.password_hash)? {
                return Err(anyhow::anyhow!("Invalid credentials"));
            }
        }
        None => {
            // OAuth-only user - cannot login with password
            return Err(anyhow::anyhow!(
                "This account uses OAuth authentication. Please login with Google."
            ));
        }
    }

    // Check if account is active
    if !user.active {
        return Err(anyhow::anyhow!("Account is deactivated"));
    }

    // Generate tokens...
}
```

---

## Key Changes

### Update login.rs

The main change is querying `user_credentials` table:

```rust
// Add credentials_repository field to AuthService (already done in Phase 4A)

// Update login method
impl AuthService {
    pub async fn login(&self, credentials: LoginRequest) -> Result<AuthResponse> {
        let user = self.user_repository
            .find_by_email(&credentials.email)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;

        // NEW: Query credentials table
        let user_credentials = self.credentials_repository
            .find_by_user_id(user.id)
            .await?;

        // Handle OAuth-only users
        let password_hash = user_credentials
            .ok_or_else(|| anyhow::anyhow!(
                "This account uses OAuth. Please login with your OAuth provider."
            ))?
            .password_hash;

        // Verify password
        if !bcrypt::verify(&credentials.password, &password_hash)? {
            return Err(anyhow::anyhow!("Invalid credentials"));
        }

        // Rest of login flow...
    }
}
```

---

## Test Scenarios

### Test 1: Local auth user login success

```rust
#[tokio::test]
async fn test_login_with_password_success() {
    let mut user_repo = MockUserRepository::new();
    let mut creds_repo = MockUserCredentialsRepository::new();

    let user_id = Uuid::new_v4();
    let password_hash = bcrypt::hash("password123", 4).unwrap();

    // User exists
    user_repo.expect_find_by_email()
        .returning(move |_| Ok(Some(test_user(user_id))));

    // User has credentials
    creds_repo.expect_find_by_user_id()
        .returning(move |_| Ok(Some(test_credentials(user_id, password_hash.clone()))));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .credentials_repository(Box::new(creds_repo))
        .build();

    let result = auth_service.login("test@example.com", "password123").await;
    assert!(result.is_ok());
}
```

### Test 2: OAuth-only user cannot login with password

```rust
#[tokio::test]
async fn test_login_oauth_only_user_rejects_password() {
    let mut user_repo = MockUserRepository::new();
    let mut creds_repo = MockUserCredentialsRepository::new();

    let user_id = Uuid::new_v4();

    // User exists (OAuth-only)
    user_repo.expect_find_by_email()
        .returning(move |_| Ok(Some(test_user(user_id))));

    // User has NO credentials (OAuth-only)
    creds_repo.expect_find_by_user_id()
        .returning(|_| Ok(None));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .credentials_repository(Box::new(creds_repo))
        .build();

    let result = auth_service.login("oauth@example.com", "anypassword").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("OAuth"));
}
```

### Test 3: User with both password and OAuth

```rust
#[tokio::test]
async fn test_login_user_with_password_and_oauth() {
    // User has both password and OAuth linked
    // Password login should work
    let mut user_repo = MockUserRepository::new();
    let mut creds_repo = MockUserCredentialsRepository::new();

    let user_id = Uuid::new_v4();
    let password_hash = bcrypt::hash("password123", 4).unwrap();

    user_repo.expect_find_by_email()
        .returning(move |_| Ok(Some(test_user(user_id))));

    creds_repo.expect_find_by_user_id()
        .returning(move |_| Ok(Some(test_credentials(user_id, password_hash.clone()))));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .credentials_repository(Box::new(creds_repo))
        .build();

    let result = auth_service.login("both@example.com", "password123").await;
    assert!(result.is_ok());
}
```

### Test 4: Wrong password

```rust
#[tokio::test]
async fn test_login_wrong_password() {
    let mut user_repo = MockUserRepository::new();
    let mut creds_repo = MockUserCredentialsRepository::new();

    let user_id = Uuid::new_v4();
    let password_hash = bcrypt::hash("correct_password", 4).unwrap();

    user_repo.expect_find_by_email()
        .returning(move |_| Ok(Some(test_user(user_id))));

    creds_repo.expect_find_by_user_id()
        .returning(move |_| Ok(Some(test_credentials(user_id, password_hash.clone()))));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .credentials_repository(Box::new(creds_repo))
        .build();

    let result = auth_service.login("test@example.com", "wrong_password").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid credentials"));
}
```

### Test 5: Non-existent user

```rust
#[tokio::test]
async fn test_login_user_not_found() {
    let mut user_repo = MockUserRepository::new();
    let creds_repo = MockUserCredentialsRepository::new();

    // User doesn't exist
    user_repo.expect_find_by_email()
        .returning(|_| Ok(None));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .credentials_repository(Box::new(creds_repo))
        .build();

    let result = auth_service.login("nonexistent@example.com", "password").await;
    assert!(result.is_err());
}
```

---

## Deliverables

1. **Updated login.rs**: Queries user_credentials table
2. **OAuth-only user handling**: Clear error message
3. **Integration tests**: All login scenarios covered
4. **Error messages**: User-friendly OAuth guidance

---

## Success Criteria

**Before proceeding to Phase 4C**, verify:

- [ ] login() queries user_credentials table
- [ ] OAuth-only users cannot login with password
- [ ] Users with both password and OAuth can login with password
- [ ] Wrong password rejected
- [ ] Non-existent user rejected
- [ ] Clear error messages for OAuth-only users
- [ ] All login tests pass
- [ ] Full test suite still passes (227 tests)

**Time Check**: 2-3 hours

---

## Common Issues

### Issue: OAuth-only user gets confusing error
**Cause**: Generic "Invalid credentials" message
**Solution**: Check for None credentials and provide OAuth-specific message

### Issue: Tests fail with "user has no password"
**Cause**: Test helper creates user without credentials
**Solution**: Create credentials in test setup for local auth tests

### Issue: Password verification always fails
**Cause**: Comparing plaintext password to hash incorrectly
**Solution**: Use `bcrypt::verify(plaintext, hash)` not the reverse

---

## Next Steps

```bash
git add backend/src/services/auth/auth_service/login.rs
git commit -m "feat(services): update login to use user_credentials table (Phase 4B)

- Query user_credentials table for password verification
- Handle OAuth-only users (no password) with clear error message
- Support users with both password and OAuth
- Add comprehensive login scenario tests
"
```

**Next Phase**: [Phase 4C: OAuth Service Updates](PHASE-04C-OAUTH-SERVICE.md)
