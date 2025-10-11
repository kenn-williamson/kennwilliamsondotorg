# Phase 4A: Registration Service Updates

**Estimated Time**: 2-3 hours
**Prerequisites**: [Phase 3B: Profile & Preferences Repositories](PHASE-03B-PROFILE-REPOS.md) complete
**Next Phase**: [Phase 4B: Login & Authentication Service](PHASE-04B-LOGIN-SERVICE.md)

## Objective

Update registration service to create entries in multiple tables (users, user_credentials, user_preferences) within a transaction.

---

## TDD Approach

1. **Red**: Write integration test for multi-table registration
2. **Green**: Update `register.rs` to use new repositories
3. **Refactor**: Ensure transaction rollback on failure

---

## Current Registration Flow

`backend/src/services/auth/auth_service/register.rs`:

```rust
// CURRENT: Creates user with password_hash in users table
pub async fn register(&self, data: RegisterRequest) -> Result<AuthResponse> {
    let password_hash = bcrypt::hash(data.password, 12)?;

    let user = self.user_repository.create_user(&CreateUserData {
        email: data.email,
        password_hash,
        display_name: data.display_name,
        slug,
    }).await?;

    // Generate tokens...
}
```

---

## New Registration Flow

```rust
// NEW: Creates user + credentials + preferences in transaction
pub async fn register(&self, data: RegisterRequest) -> Result<AuthResponse> {
    let password_hash = bcrypt::hash(data.password, 12)?;

    // 1. Create user (core identity) - password_hash temporarily still in users table
    let user = self.user_repository.create_user(&CreateUserData {
        email: data.email,
        password_hash: password_hash.clone(), // Temp: still write to old location
        display_name: data.display_name,
        slug,
    }).await?;

    // 2. Create credentials (NEW)
    self.credentials_repository
        .create(user.id, password_hash)
        .await?;

    // 3. Create preferences (NEW)
    self.preferences_repository
        .create(user.id)
        .await?;

    // Generate tokens...
}
```

---

## Key Changes

### Update AuthService struct

Add new repository fields to `backend/src/services/auth/auth_service/mod.rs`:

```rust
pub struct AuthService {
    jwt_service: JwtService,
    user_repository: Box<dyn UserRepository>,
    credentials_repository: Box<dyn UserCredentialsRepository>,  // ADD
    preferences_repository: Box<dyn UserPreferencesRepository>,  // ADD
    // ... existing fields
}
```

### Update AuthServiceBuilder

In `backend/src/services/auth/auth_service/builder.rs`:

```rust
pub struct AuthServiceBuilder {
    credentials_repository: Option<Box<dyn UserCredentialsRepository>>,  // ADD
    preferences_repository: Option<Box<dyn UserPreferencesRepository>>,  // ADD
    // ... existing fields
}

impl AuthServiceBuilder {
    pub fn credentials_repository(mut self, repo: Box<dyn UserCredentialsRepository>) -> Self {
        self.credentials_repository = Some(repo);
        self
    }

    pub fn preferences_repository(mut self, repo: Box<dyn UserPreferencesRepository>) -> Self {
        self.preferences_repository = Some(repo);
        self
    }
}
```

---

## Integration Test

In `register.rs`:

```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_register_creates_credentials_and_preferences() {
        // Setup mocks
        let mut user_repo = MockUserRepository::new();
        let mut creds_repo = MockUserCredentialsRepository::new();
        let mut prefs_repo = MockUserPreferencesRepository::new();

        // Expectations
        user_repo.expect_create_user().returning(|_| Ok(test_user()));
        creds_repo.expect_create().returning(|_, _| Ok(test_credentials()));
        prefs_repo.expect_create().returning(|_| Ok(test_preferences()));

        let auth_service = AuthService::builder()
            .user_repository(Box::new(user_repo))
            .credentials_repository(Box::new(creds_repo))
            .preferences_repository(Box::new(prefs_repo))
            .build();

        let result = auth_service.register(test_register_request()).await;
        assert!(result.is_ok());
    }
}
```

---

## Transaction Handling (Future)

For now, we're using individual repository calls. In Phase 7 (Data Migration), we'll add proper transaction handling:

```rust
// Future: Use transactions for atomicity
let mut tx = self.pool.begin().await?;

// Create user
// Create credentials
// Create preferences

tx.commit().await?;
```

---

## Deliverables

1. **Updated AuthService**: New repository fields
2. **Updated AuthServiceBuilder**: Builder methods for new repos
3. **Updated register.rs**: Multi-table creation
4. **Integration tests**: Verify all tables created
5. **Rollback tests**: Verify failure handling

---

## Success Criteria

- [ ] AuthService has credentials_repository field
- [ ] AuthService has preferences_repository field
- [ ] AuthServiceBuilder supports new repositories
- [ ] register() creates user + credentials + preferences
- [ ] Integration tests verify all tables created
- [ ] Tests pass with mock repositories
- [ ] Full test suite still passes (227 tests)

**Time Check**: 2-3 hours

---

## Next Steps

```bash
git add backend/src/services/auth/auth_service/mod.rs
git add backend/src/services/auth/auth_service/builder.rs
git add backend/src/services/auth/auth_service/register.rs
git commit -m "feat(services): update registration for multi-table schema (Phase 4A)"
```

**Next Phase**: [Phase 4B: Login & Authentication Service](PHASE-04B-LOGIN-SERVICE.md)
