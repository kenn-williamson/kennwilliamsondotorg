# Phase 3A: Credentials & External Login Repositories

**Estimated Time**: 2-3 hours
**Prerequisites**: [Phase 2B: New Table Models](PHASE-02B-NEW-MODELS.md) complete
**Next Phase**: [Phase 3B: Profile & Preferences Repositories](PHASE-03B-PROFILE-REPOS.md)

## Objective

Create repository traits and implementations for `user_credentials` and `user_external_logins` tables. This phase establishes the data access layer for authentication-related operations.

**Key Principle**: Repositories abstract database operations. They have trait definitions, Postgres implementations, and mock implementations for testing.

---

## TDD Approach

For each repository:
1. **Red**: Write trait definition and integration tests (failing)
2. **Green**: Implement Postgres repository to make tests pass
3. **Refactor**: Create mock implementation for unit tests

---

## Task 1: UserCredentialsRepository Trait (30 minutes)

### Step 1: Create trait file

Create `backend/src/repositories/traits/user_credentials_repository.rs`:

```rust
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::db::user_credentials::UserCredentials;

/// Repository trait for user credentials (local password authentication)
#[async_trait]
pub trait UserCredentialsRepository: Send + Sync {
    /// Create credentials for a user (during registration or password set)
    async fn create(&self, user_id: Uuid, password_hash: String) -> Result<UserCredentials>;

    /// Find credentials by user ID
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserCredentials>>;

    /// Update password hash (during password change)
    async fn update_password(&self, user_id: Uuid, new_password_hash: String) -> Result<()>;

    /// Delete credentials (during account deletion or password removal)
    async fn delete(&self, user_id: Uuid) -> Result<()>;

    /// Check if user has password credentials
    async fn has_password(&self, user_id: Uuid) -> Result<bool>;
}
```

### Step 2: Add to traits/mod.rs

Edit `backend/src/repositories/traits/mod.rs`:

```rust
pub mod user_repository;
pub mod user_credentials_repository;  // ADD THIS
// ... existing traits
```

**Success Criteria**:
- Trait compiles
- All methods have clear documentation
- async_trait applied

---

## Task 2: PostgresUserCredentialsRepository Implementation (45 minutes)

### Step 1: Create implementation file

Create `backend/src/repositories/postgres/postgres_user_credentials_repository.rs`:

```rust
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::user_credentials::UserCredentials;
use crate::repositories::traits::user_credentials_repository::UserCredentialsRepository;

pub struct PostgresUserCredentialsRepository {
    pool: PgPool,
}

impl PostgresUserCredentialsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserCredentialsRepository for PostgresUserCredentialsRepository {
    async fn create(&self, user_id: Uuid, password_hash: String) -> Result<UserCredentials> {
        let credentials = sqlx::query_as::<_, UserCredentials>(
            r#"
            INSERT INTO user_credentials (user_id, password_hash)
            VALUES ($1, $2)
            RETURNING user_id, password_hash, password_updated_at, created_at
            "#,
        )
        .bind(user_id)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(credentials)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserCredentials>> {
        let credentials = sqlx::query_as::<_, UserCredentials>(
            r#"
            SELECT user_id, password_hash, password_updated_at, created_at
            FROM user_credentials
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(credentials)
    }

    async fn update_password(&self, user_id: Uuid, new_password_hash: String) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE user_credentials
            SET password_hash = $1, password_updated_at = NOW()
            WHERE user_id = $2
            "#,
        )
        .bind(new_password_hash)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_credentials
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn has_password(&self, user_id: Uuid) -> Result<bool> {
        let result: Option<bool> = sqlx::query_scalar(
            r#"
            SELECT EXISTS(SELECT 1 FROM user_credentials WHERE user_id = $1)
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.unwrap_or(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::postgres::postgres_user_repository::PostgresUserRepository;
    use crate::repositories::traits::user_repository::{CreateUserData, UserRepository};

    async fn setup_test_pool() -> PgPool {
        // Use test database connection
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/kennwilliamson".to_string());

        PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    async fn create_test_user(pool: &PgPool) -> Uuid {
        let user_repo = PostgresUserRepository::new(pool.clone());
        let user_data = CreateUserData {
            email: format!("test-{}@example.com", Uuid::new_v4()),
            password_hash: "temp_hash".to_string(),
            display_name: "Test User".to_string(),
            slug: format!("test-{}", Uuid::new_v4()),
        };

        let user = user_repo.create_user(&user_data).await.unwrap();
        user.id
    }

    #[tokio::test]
    async fn test_create_credentials() {
        let pool = setup_test_pool().await;
        let repo = PostgresUserCredentialsRepository::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        let credentials = repo.create(user_id, "$2b$12$test_hash".to_string()).await;
        assert!(credentials.is_ok());

        let creds = credentials.unwrap();
        assert_eq!(creds.user_id, user_id);
        assert_eq!(creds.password_hash, "$2b$12$test_hash");
    }

    #[tokio::test]
    async fn test_find_by_user_id() {
        let pool = setup_test_pool().await;
        let repo = PostgresUserCredentialsRepository::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Create credentials
        repo.create(user_id, "hash123".to_string()).await.unwrap();

        // Find credentials
        let found = repo.find_by_user_id(user_id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().password_hash, "hash123");
    }

    #[tokio::test]
    async fn test_find_by_user_id_not_found() {
        let pool = setup_test_pool().await;
        let repo = PostgresUserCredentialsRepository::new(pool);
        let non_existent_id = Uuid::new_v4();

        let found = repo.find_by_user_id(non_existent_id).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_update_password() {
        let pool = setup_test_pool().await;
        let repo = PostgresUserCredentialsRepository::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Create credentials
        repo.create(user_id, "old_hash".to_string()).await.unwrap();

        // Update password
        repo.update_password(user_id, "new_hash".to_string()).await.unwrap();

        // Verify update
        let updated = repo.find_by_user_id(user_id).await.unwrap().unwrap();
        assert_eq!(updated.password_hash, "new_hash");
    }

    #[tokio::test]
    async fn test_delete_credentials() {
        let pool = setup_test_pool().await;
        let repo = PostgresUserCredentialsRepository::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Create credentials
        repo.create(user_id, "hash".to_string()).await.unwrap();

        // Delete credentials
        repo.delete(user_id).await.unwrap();

        // Verify deletion
        let found = repo.find_by_user_id(user_id).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_has_password() {
        let pool = setup_test_pool().await;
        let repo = PostgresUserCredentialsRepository::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // User without credentials
        let has_password = repo.has_password(user_id).await.unwrap();
        assert_eq!(has_password, false);

        // Create credentials
        repo.create(user_id, "hash".to_string()).await.unwrap();

        // User with credentials
        let has_password = repo.has_password(user_id).await.unwrap();
        assert_eq!(has_password, true);
    }
}
```

### Step 2: Add to postgres/mod.rs

Edit `backend/src/repositories/postgres/mod.rs`:

```rust
pub mod postgres_user_repository;
pub mod postgres_user_credentials_repository;  // ADD THIS
// ... existing modules
```

### Step 3: Run integration tests

```bash
cd backend
cargo test postgres_user_credentials_repository::tests -- --test-threads=4
# Expected: All tests pass
```

**Success Criteria**:
- All repository methods implemented
- Integration tests pass
- CRUD operations work correctly

---

## Task 3: UserExternalLoginRepository Trait (30 minutes)

### Step 1: Create trait file

Create `backend/src/repositories/traits/user_external_login_repository.rs`:

```rust
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::models::db::user_external_login::UserExternalLogin;

/// Data for creating external login
pub struct CreateExternalLogin {
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
}

/// Repository trait for user external logins (OAuth)
#[async_trait]
pub trait UserExternalLoginRepository: Send + Sync {
    /// Create external login (link OAuth provider to user)
    async fn create(&self, data: CreateExternalLogin) -> Result<UserExternalLogin>;

    /// Find external login by provider and provider user ID
    async fn find_by_provider(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Option<UserExternalLogin>>;

    /// Find all external logins for a user
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<UserExternalLogin>>;

    /// Unlink a provider from a user
    async fn unlink_provider(&self, user_id: Uuid, provider: &str) -> Result<()>;

    /// Delete external login by ID
    async fn delete(&self, id: Uuid) -> Result<()>;

    /// Check if provider is linked to user
    async fn is_provider_linked(&self, user_id: Uuid, provider: &str) -> Result<bool>;
}
```

### Step 2: Add to traits/mod.rs

```rust
pub mod user_external_login_repository;  // ADD THIS
```

**Success Criteria**:
- Trait compiles
- CreateExternalLogin struct defined
- All methods documented

---

## Task 4: PostgresUserExternalLoginRepository Implementation (45 minutes)

### Step 1: Create implementation file

Create `backend/src/repositories/postgres/postgres_user_external_login_repository.rs`:

```rust
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::user_external_login::UserExternalLogin;
use crate::repositories::traits::user_external_login_repository::{
    CreateExternalLogin, UserExternalLoginRepository,
};

pub struct PostgresUserExternalLoginRepository {
    pool: PgPool,
}

impl PostgresUserExternalLoginRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserExternalLoginRepository for PostgresUserExternalLoginRepository {
    async fn create(&self, data: CreateExternalLogin) -> Result<UserExternalLogin> {
        let login = sqlx::query_as::<_, UserExternalLogin>(
            r#"
            INSERT INTO user_external_logins (user_id, provider, provider_user_id, linked_at)
            VALUES ($1, $2, $3, NOW())
            RETURNING id, user_id, provider, provider_user_id, linked_at, created_at, updated_at
            "#,
        )
        .bind(data.user_id)
        .bind(data.provider)
        .bind(data.provider_user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(login)
    }

    async fn find_by_provider(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Option<UserExternalLogin>> {
        let login = sqlx::query_as::<_, UserExternalLogin>(
            r#"
            SELECT id, user_id, provider, provider_user_id, linked_at, created_at, updated_at
            FROM user_external_logins
            WHERE provider = $1 AND provider_user_id = $2
            "#,
        )
        .bind(provider)
        .bind(provider_user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(login)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<UserExternalLogin>> {
        let logins = sqlx::query_as::<_, UserExternalLogin>(
            r#"
            SELECT id, user_id, provider, provider_user_id, linked_at, created_at, updated_at
            FROM user_external_logins
            WHERE user_id = $1
            ORDER BY linked_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(logins)
    }

    async fn unlink_provider(&self, user_id: Uuid, provider: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_external_logins
            WHERE user_id = $1 AND provider = $2
            "#,
        )
        .bind(user_id)
        .bind(provider)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_external_logins
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn is_provider_linked(&self, user_id: Uuid, provider: &str) -> Result<bool> {
        let result: Option<bool> = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM user_external_logins
                WHERE user_id = $1 AND provider = $2
            )
            "#,
        )
        .bind(user_id)
        .bind(provider)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.unwrap_or(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::postgres::postgres_user_repository::PostgresUserRepository;
    use crate::repositories::traits::user_repository::{CreateUserData, UserRepository};

    async fn setup_test_pool() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/kennwilliamson".to_string());

        PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    async fn create_test_user(pool: &PgPool) -> Uuid {
        let user_repo = PostgresUserRepository::new(pool.clone());
        let user_data = CreateUserData {
            email: format!("test-{}@example.com", Uuid::new_v4()),
            password_hash: "temp_hash".to_string(),
            display_name: "Test User".to_string(),
            slug: format!("test-{}", Uuid::new_v4()),
        };

        let user = user_repo.create_user(&user_data).await.unwrap();
        user.id
    }

    #[tokio::test]
    async fn test_create_external_login() {
        let pool = setup_test_pool().await;
        let repo = PostgresUserExternalLoginRepository::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        let data = CreateExternalLogin {
            user_id,
            provider: "google".to_string(),
            provider_user_id: "google_123".to_string(),
        };

        let login = repo.create(data).await.unwrap();
        assert_eq!(login.user_id, user_id);
        assert_eq!(login.provider, "google");
        assert_eq!(login.provider_user_id, "google_123");
    }

    #[tokio::test]
    async fn test_find_by_provider() {
        let pool = setup_test_pool().await;
        let repo = PostgresUserExternalLoginRepository::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Create login
        let data = CreateExternalLogin {
            user_id,
            provider: "github".to_string(),
            provider_user_id: "github_456".to_string(),
        };
        repo.create(data).await.unwrap();

        // Find by provider
        let found = repo.find_by_provider("github", "github_456").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().user_id, user_id);
    }

    #[tokio::test]
    async fn test_find_by_user_id_multiple_providers() {
        let pool = setup_test_pool().await;
        let repo = PostgresUserExternalLoginRepository::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Create Google login
        repo.create(CreateExternalLogin {
            user_id,
            provider: "google".to_string(),
            provider_user_id: "google_123".to_string(),
        }).await.unwrap();

        // Create GitHub login
        repo.create(CreateExternalLogin {
            user_id,
            provider: "github".to_string(),
            provider_user_id: "github_456".to_string(),
        }).await.unwrap();

        // Find all logins for user
        let logins = repo.find_by_user_id(user_id).await.unwrap();
        assert_eq!(logins.len(), 2);

        let providers: Vec<String> = logins.iter().map(|l| l.provider.clone()).collect();
        assert!(providers.contains(&"google".to_string()));
        assert!(providers.contains(&"github".to_string()));
    }

    #[tokio::test]
    async fn test_unlink_provider() {
        let pool = setup_test_pool().await;
        let repo = PostgresUserExternalLoginRepository::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Create login
        repo.create(CreateExternalLogin {
            user_id,
            provider: "google".to_string(),
            provider_user_id: "google_123".to_string(),
        }).await.unwrap();

        // Unlink provider
        repo.unlink_provider(user_id, "google").await.unwrap();

        // Verify unlinked
        let found = repo.find_by_provider("google", "google_123").await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_is_provider_linked() {
        let pool = setup_test_pool().await;
        let repo = PostgresUserExternalLoginRepository::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Not linked initially
        let is_linked = repo.is_provider_linked(user_id, "google").await.unwrap();
        assert_eq!(is_linked, false);

        // Create login
        repo.create(CreateExternalLogin {
            user_id,
            provider: "google".to_string(),
            provider_user_id: "google_123".to_string(),
        }).await.unwrap();

        // Now linked
        let is_linked = repo.is_provider_linked(user_id, "google").await.unwrap();
        assert_eq!(is_linked, true);
    }
}
```

### Step 2: Add to postgres/mod.rs

```rust
pub mod postgres_user_external_login_repository;  // ADD THIS
```

### Step 3: Run integration tests

```bash
cargo test postgres_user_external_login_repository::tests -- --test-threads=4
# Expected: All tests pass
```

**Success Criteria**:
- All repository methods implemented
- Integration tests pass
- Multi-provider support verified

---

## Task 5: Update SQLx Cache (15 minutes)

```bash
./scripts/prepare-sqlx.sh --clean
cd backend && cargo check
# Expected: No errors
```

**Success Criteria**:
- SQLx cache updated
- Backend compiles

---

## Task 6: Create Mock Implementations (30 minutes)

These will be created in a later phase, but document the interface requirements now.

Create `backend/src/repositories/mocks/mock_user_credentials_repository.rs` and `mock_user_external_login_repository.rs` placeholders:

```rust
// TODO(Phase 6): Implement mock repositories for unit testing
```

---

## Deliverables

At the end of this phase, you should have:

1. **Repository Traits**
   - `user_credentials_repository.rs`
   - `user_external_login_repository.rs`

2. **Postgres Implementations**
   - `postgres_user_credentials_repository.rs`
   - `postgres_user_external_login_repository.rs`

3. **Integration Tests**
   - All CRUD operations tested
   - Multi-provider scenarios tested
   - Edge cases covered

---

## Success Criteria

**Before proceeding to Phase 3B**, verify:

- [ ] UserCredentialsRepository trait created
- [ ] PostgresUserCredentialsRepository implemented
- [ ] All credentials repository tests pass
- [ ] UserExternalLoginRepository trait created
- [ ] PostgresUserExternalLoginRepository implemented
- [ ] All external login repository tests pass
- [ ] Multi-provider support verified
- [ ] SQLx cache updated
- [ ] Backend compiles without errors
- [ ] Full test suite still passes

**Time Check**: This phase should take 2-3 hours.

---

## Common Issues

### Issue: Foreign key constraint violation
**Cause**: Test user doesn't exist
**Solution**: Use `create_test_user()` helper before creating credentials/logins

### Issue: UNIQUE constraint violation
**Cause**: Duplicate (provider, provider_user_id)
**Solution**: Use unique provider_user_id for each test

### Issue: Tests interfere with each other
**Cause**: Shared test data
**Solution**: Use unique identifiers (Uuid) for each test

---

## Next Steps

1. **Commit the changes**:
   ```bash
   git add backend/src/repositories/traits/user_credentials_repository.rs
   git add backend/src/repositories/traits/user_external_login_repository.rs
   git add backend/src/repositories/postgres/postgres_user_credentials_repository.rs
   git add backend/src/repositories/postgres/postgres_user_external_login_repository.rs
   git commit -m "feat(repos): add credentials and external login repositories (Phase 3A)

   - Add UserCredentialsRepository trait and Postgres implementation
   - Add UserExternalLoginRepository trait and Postgres implementation
   - Add comprehensive integration tests for both repositories
   - Support multi-provider OAuth scenarios
   "
   ```

2. **Proceed to Phase 3B**: [PHASE-03B-PROFILE-REPOS.md](PHASE-03B-PROFILE-REPOS.md)

---

**Phase Status**: ⬜ Not Started → **Continue when ready**

**Estimated Completion Time**: 2-3 hours

**Next Phase**: [Phase 3B: Profile & Preferences Repositories](PHASE-03B-PROFILE-REPOS.md)
