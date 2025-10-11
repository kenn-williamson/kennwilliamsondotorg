# Phase 3B: Profile & Preferences Repositories

**Estimated Time**: 2-3 hours
**Prerequisites**: [Phase 3A: Credentials & External Login Repositories](PHASE-03A-CREDENTIALS-REPOS.md) complete
**Next Phase**: [Phase 4A: Registration Service Updates](PHASE-04A-REGISTRATION-SERVICE.md)

## Objective

Create repository traits and implementations for `user_profiles` and `user_preferences` tables. These handle non-authentication user data.

**Key Principle**: Profile and preferences are optional/configurable user data, separate from authentication.

---

## Summary

This phase follows the same TDD pattern as Phase 3A:

1. **UserProfileRepository**: CRUD operations for user profile data (real_name, bio, avatar_url, location, website)
2. **UserPreferencesRepository**: CRUD operations for user preferences (timer settings, future feature flags)

Both repositories need:
- Trait definitions
- Postgres implementations with integration tests
- Mock placeholders for unit testing

---

## Task 1: UserProfileRepository (60 minutes)

### Trait Definition

`backend/src/repositories/traits/user_profile_repository.rs`:

```rust
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;
use crate::models::db::user_profile::UserProfile;

#[derive(Debug, Clone)]
pub struct UpdateProfile {
    pub real_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

#[async_trait]
pub trait UserProfileRepository: Send + Sync {
    async fn create(&self, user_id: Uuid) -> Result<UserProfile>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>>;
    async fn update(&self, user_id: Uuid, data: UpdateProfile) -> Result<UserProfile>;
    async fn delete(&self, user_id: Uuid) -> Result<()>;
}
```

### Postgres Implementation

`backend/src/repositories/postgres/postgres_user_profile_repository.rs`:

Key queries:
- CREATE: `INSERT INTO user_profiles (user_id) VALUES ($1) RETURNING *`
- FIND: `SELECT * FROM user_profiles WHERE user_id = $1`
- UPDATE: `UPDATE user_profiles SET real_name = $1, bio = $2, ... WHERE user_id = $N`
- DELETE: `DELETE FROM user_profiles WHERE user_id = $1`

### Integration Tests

Test scenarios:
- Create empty profile
- Update profile fields individually
- Update multiple fields at once
- Find non-existent profile returns None
- Delete profile

---

## Task 2: UserPreferencesRepository (60 minutes)

### Trait Definition

`backend/src/repositories/traits/user_preferences_repository.rs`:

```rust
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;
use crate::models::db::user_preferences::UserPreferences;

#[async_trait]
pub trait UserPreferencesRepository: Send + Sync {
    async fn create(&self, user_id: Uuid) -> Result<UserPreferences>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserPreferences>>;
    async fn update_timer_settings(&self, user_id: Uuid, is_public: bool, show_in_list: bool) -> Result<()>;
    async fn delete(&self, user_id: Uuid) -> Result<()>;
}
```

### Postgres Implementation

`backend/src/repositories/postgres/postgres_user_preferences_repository.rs`:

Key queries:
- CREATE: `INSERT INTO user_preferences (user_id) VALUES ($1) RETURNING *` (uses defaults)
- FIND: `SELECT * FROM user_preferences WHERE user_id = $1`
- UPDATE_TIMER: `UPDATE user_preferences SET timer_is_public = $1, timer_show_in_list = $2 WHERE user_id = $3`
- DELETE: `DELETE FROM user_preferences WHERE user_id = $1`

### Integration Tests

Test scenarios:
- Create preferences with defaults
- Update timer_is_public
- Update timer_show_in_list
- Business rule validation (show_in_list requires is_public) - test at service layer
- Delete preferences

---

## Task 3: Integration Test Pattern (30 minutes)

Both repositories should follow this test pattern:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_pool() -> PgPool { /* ... */ }
    async fn create_test_user(pool: &PgPool) -> Uuid { /* ... */ }

    #[tokio::test]
    async fn test_create() { /* ... */ }

    #[tokio::test]
    async fn test_find_by_user_id() { /* ... */ }

    #[tokio::test]
    async fn test_update() { /* ... */ }

    #[tokio::test]
    async fn test_delete() { /* ... */ }

    #[tokio::test]
    async fn test_find_non_existent_returns_none() { /* ... */ }
}
```

Run tests:
```bash
cargo test postgres_user_profile_repository::tests -- --test-threads=4
cargo test postgres_user_preferences_repository::tests -- --test-threads=4
```

---

## Task 4: Update Module Exports (15 minutes)

### traits/mod.rs
```rust
pub mod user_profile_repository;
pub mod user_preferences_repository;
```

### postgres/mod.rs
```rust
pub mod postgres_user_profile_repository;
pub mod postgres_user_preferences_repository;
```

---

## Task 5: Update SQLx Cache & Verify (15 minutes)

```bash
./scripts/prepare-sqlx.sh --clean
cd backend && cargo check
cargo test -- --test-threads=4
# Expected: All 227 tests still pass
```

---

## Deliverables

1. **UserProfileRepository**: Trait + Postgres implementation + tests
2. **UserPreferencesRepository**: Trait + Postgres implementation + tests
3. **Module exports updated**
4. **All tests passing**

---

## Success Criteria

- [ ] UserProfileRepository trait and implementation complete
- [ ] UserPreferencesRepository trait and implementation complete
- [ ] All repository integration tests pass
- [ ] SQLx cache updated
- [ ] Backend compiles without errors
- [ ] Full test suite still passes (227 tests)

**Time Check**: 2-3 hours

---

## Next Steps

```bash
git add backend/src/repositories/traits/user_profile_repository.rs
git add backend/src/repositories/traits/user_preferences_repository.rs
git add backend/src/repositories/postgres/postgres_user_profile_repository.rs
git add backend/src/repositories/postgres/postgres_user_preferences_repository.rs
git commit -m "feat(repos): add profile and preferences repositories (Phase 3B)"
```

**Next Phase**: [Phase 4A: Registration Service Updates](PHASE-04A-REGISTRATION-SERVICE.md)
