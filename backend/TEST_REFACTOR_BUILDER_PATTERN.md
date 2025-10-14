# Test Refactor: Builder Pattern for Test Fixtures

## ⚠️  Running Tests: Parallelism Issue

**Problem:** This project has **167 integration tests** that use TestContainers (Docker). Running with default parallelism (`--test-threads=4`) causes:
- Resource exhaustion (Docker API limits, connection pool limits)
- Test hangs (some tests wait 60+ seconds)
- Dangling containers (use `scripts/cleanup-test-containers.sh` to clean)

**Solution:** Run tests with reduced parallelism:
```bash
cargo test -- --test-threads=2   # Recommended
cargo test -- --test-threads=1   # Safest (slower)
```

**Why?** Each test spins up a PostgreSQL container (`ghcr.io/fboulnois/pg_uuidv7:1.6.0`). With 4 threads, we hit Docker resource limits.

---

## Problem Statement

**Before:** Tests manually construct structs, leading to massive fragility:
```rust
// This breaks when User model changes (e.g., adding audit fields)
User {
    id: Uuid::new_v4(),
    email: "test@example.com".to_string(),
    password_hash: Some("hash".to_string()),  // ← Moved to separate table
    display_name: "Test".to_string(),
    slug: "test".to_string(),
    real_name: Some("Real".to_string()),      // ← Moved to separate table
    google_user_id: Some("google_123".to_string()), // ← Moved to separate table
    timer_is_public: false,                   // ← Moved to separate table
    timer_show_in_list: false,                // ← Moved to separate table
    active: true,
    created_at: Utc::now(),
    updated_at: Utc::now(),
}
```

**Impact:** When User model changed (multi-table auth refactor), we had to update 50+ manual constructions across tests.

## Solution: Idiomatic Rust Builder Pattern

**After:** Use test-specific builders with sensible defaults:
```rust
// Resilient to User model changes - only update builder once
UserBuilder::new()
    .with_email("test@example.com")
    .build()  // Everything else has sensible defaults
```

**When User changes (e.g., adding `audit_user_id`):** Update builder once, all tests continue working.

## Pattern Details

### Idiomatic Rust Approach (Research-Based)

Based on Rust community standards (Cargo project, official patterns):

1. **Production Builders:** Enforce required fields, no defaults
2. **Test Builders:** Sensible defaults, override what matters

### Our Implementation

**Location:** `src/test_utils/` (available to both unit and integration tests)

**Structure:**
```
src/
  test_utils/
    mod.rs              # Re-exports all builders
    user_builder.rs     # ✅ DONE
    incident_timer_builder.rs  # ⏳ TODO
    phrase_builder.rs   # ⏳ TODO
    refresh_token_builder.rs   # ⏳ TODO
```

**Usage:**
```rust
// Unit tests (in src/)
use crate::test_utils::UserBuilder;

// Integration tests (in tests/)
use backend::test_utils::UserBuilder;
```

## Current Status

### Phase 1: User Model ✅ COMPLETED

- [x] Created `src/test_utils/user_builder.rs`
- [x] Updated `src/lib.rs` to export test_utils
- [x] Updated fixture helpers in `tests/fixtures/database.rs`
- [x] Updated fixture helpers in `tests/fixtures/test_context.rs`
- [x] Updated repository test fixtures (4 files)
- [x] Updated OAuth test fixtures
- [x] Updated admin test fixtures

**Files Updated:**
- `tests/fixtures/database.rs` - create_verified_user, create_unverified_user, create_oauth_user
- `tests/fixtures/test_context.rs` - TestContext helper methods
- `tests/repositories/testcontainers_user_credentials_repository_tests.rs`
- `tests/repositories/testcontainers_user_profile_repository_tests.rs`
- `tests/repositories/testcontainers_user_external_login_repository_tests.rs`
- `tests/repositories/testcontainers_user_preferences_repository_tests.rs`
- `tests/api/testcontainers_oauth_tests.rs`
- `tests/admin_role_management_tests.rs`

### Phase 2: Additional Builders ⏳ IN PROGRESS

#### 2.1 IncidentTimerBuilder ⏳ TODO
**Model:** `src/models/db/incident_timer.rs`
```rust
pub struct IncidentTimer {
    pub id: Uuid,
    pub user_id: Uuid,
    pub reset_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Builder Location:** `src/test_utils/incident_timer_builder.rs`

**Builder API:**
```rust
IncidentTimerBuilder::new()
    .with_user_id(user_id)
    .with_reset_timestamp(timestamp)
    .with_notes("Test notes")
    .persist(pool)  // Or .build() for in-memory
    .await
```

**Tests That Need This:**
- [ ] `tests/api/testcontainers_incident_timer_api_tests.rs`
- [ ] `tests/api/testcontainers_account_deletion_tests.rs` (creates test timers)
- [ ] Any service tests that create timers

#### 2.2 PhraseBuilder ⏳ TODO
**Models:**
- `src/models/db/phrase.rs` - `Phrase`
- `src/models/db/phrase.rs` - `PhraseSuggestion`

**Structs:**
```rust
pub struct Phrase {
    pub id: Uuid,
    pub phrase_text: String,
    pub active: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct PhraseSuggestion {
    pub id: Uuid,
    pub user_id: Uuid,
    pub phrase_text: String,
    pub status: String, // "pending", "approved", "rejected"
    pub admin_id: Option<Uuid>,
    pub admin_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Builder Location:** `src/test_utils/phrase_builder.rs`

**Builder API:**
```rust
PhraseBuilder::new()
    .with_text("Test phrase")
    .with_created_by(user_id)
    .active()  // or .inactive()
    .persist(pool)
    .await

PhraseSuggestionBuilder::new()
    .with_user_id(user_id)
    .with_text("Suggested phrase")
    .pending()  // or .approved() / .rejected()
    .persist(pool)
    .await
```

**Tests That Need This:**
- [ ] `tests/api/testcontainers_phrase_api_tests.rs`
- [ ] `tests/api/testcontainers_account_deletion_tests.rs` (creates test phrases)
- [ ] Any service tests that create phrases

#### 2.3 RefreshTokenBuilder ⏳ TODO
**Model:** `src/models/db/refresh_token.rs`
```rust
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub device_info: Option<serde_json::Value>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}
```

**Builder Location:** `src/test_utils/refresh_token_builder.rs`

**Builder API:**
```rust
RefreshTokenBuilder::new()
    .with_user_id(user_id)
    .with_token_hash("hash")
    .expires_in_days(30)
    .persist(pool)
    .await
```

**Current Manual Construction:**
- [ ] `tests/fixtures/database.rs` - create_test_refresh_token_in_db

**Tests That Need This:**
- [ ] `tests/token_cleanup_tests.rs`
- [ ] Any tests creating refresh tokens

### Phase 3: Documentation & Enforcement ⏳ TODO

- [ ] Update `CODING-RULES.md` with builder pattern requirement
- [ ] Add examples to `DEVELOPMENT-WORKFLOW.md`
- [ ] Add to `IMPLEMENTATION-TESTING.md`

## Unit Tests in `src/` (48 files)

### Currently Using UserBuilder ✅
- [x] `src/routes/auth.rs` - Fixed to use simple User construction (unit test appropriate)

### Files With Unit Tests (Audit Needed)
These files contain `#[cfg(test)]` blocks - audit for manual struct construction:

**Models:**
- [ ] `src/models/db/user.rs`
- [ ] `src/models/db/user_credentials.rs`
- [ ] `src/models/db/user_external_login.rs`
- [ ] `src/models/db/user_preferences.rs`
- [ ] `src/models/db/user_profile.rs`
- [ ] `src/models/api/data_export.rs`

**Repositories (Mocks):**
- [ ] `src/repositories/mocks/mock_admin_repository.rs`
- [ ] `src/repositories/mocks/mock_email_suppression_repository.rs`
- [ ] `src/repositories/mocks/mock_incident_timer_repository.rs`
- [ ] `src/repositories/mocks/mock_phrase_repository.rs`
- [ ] `src/repositories/mocks/mock_pkce_storage.rs`
- [ ] `src/repositories/mocks/mock_refresh_token_repository.rs`
- [ ] `src/repositories/mocks/mock_user_repository.rs`
- [ ] `src/repositories/redis/redis_pkce_storage.rs`

**Services:**
- [ ] `src/services/admin/phrase_moderation/mod.rs`
- [ ] `src/services/admin/stats/mod.rs`
- [ ] `src/services/admin/user_management/mod.rs`
- [ ] `src/services/auth/auth_service/account_deletion.rs`
- [ ] `src/services/auth/auth_service/builder.rs`
- [ ] `src/services/auth/auth_service/data_export.rs`
- [ ] `src/services/auth/auth_service/email_verification.rs`
- [ ] `src/services/auth/auth_service/login.rs`
- [ ] `src/services/auth/auth_service/oauth.rs`
- [ ] `src/services/auth/auth_service/password.rs`
- [ ] `src/services/auth/auth_service/password_reset.rs`
- [ ] `src/services/auth/auth_service/profile.rs`
- [ ] `src/services/auth/auth_service/refresh_token.rs`
- [ ] `src/services/auth/auth_service/register.rs`
- [ ] `src/services/auth/auth_service/slug.rs`
- [ ] `src/services/auth/jwt.rs`
- [ ] `src/services/auth/oauth/config.rs`
- [ ] `src/services/auth/oauth/google_oauth_service.rs`
- [ ] `src/services/auth/oauth/mock_google_oauth_service.rs`
- [ ] `src/services/cleanup/mod.rs`
- [ ] `src/services/email/mock_email_service.rs`
- [ ] `src/services/email/ses_email_service.rs`
- [ ] `src/services/incident_timer/create.rs`
- [ ] `src/services/incident_timer/delete.rs`
- [ ] `src/services/incident_timer/read.rs`
- [ ] `src/services/incident_timer/update.rs`
- [ ] `src/services/phrase/admin_management.rs`
- [ ] `src/services/phrase/exclusions.rs`
- [ ] `src/services/phrase/public_access.rs`
- [ ] `src/services/phrase/suggestions.rs`
- [ ] `src/services/phrase/user_management.rs`

**Middleware:**
- [ ] `src/middleware/rate_limiter/config.rs`
- [ ] `src/middleware/rate_limiter/middleware.rs`

## Integration Tests in `tests/` (35 files)

### Using UserBuilder ✅
- [x] `tests/fixtures/database.rs`
- [x] `tests/fixtures/test_context.rs`
- [x] `tests/repositories/testcontainers_user_credentials_repository_tests.rs`
- [x] `tests/repositories/testcontainers_user_profile_repository_tests.rs`
- [x] `tests/repositories/testcontainers_user_external_login_repository_tests.rs`
- [x] `tests/repositories/testcontainers_user_preferences_repository_tests.rs`
- [x] `tests/api/testcontainers_oauth_tests.rs`
- [x] `tests/admin_role_management_tests.rs`

### Need Other Builders ⏳
- [ ] `tests/api/testcontainers_health_api_tests.rs` - May need builders
- [ ] `tests/api/testcontainers_auth_api_tests.rs` - Likely uses UserBuilder already
- [ ] `tests/api/testcontainers_sns_webhook_api_tests.rs` - May need builders
- [ ] `tests/api/testcontainers_rbac_feature_gating_tests.rs` - May need builders
- [ ] `tests/api/testcontainers_phrase_api_tests.rs` - NEEDS PhraseBuilder
- [ ] `tests/api/testcontainers_incident_timer_api_tests.rs` - NEEDS IncidentTimerBuilder
- [ ] `tests/api/testcontainers_admin_api_tests.rs` - May need builders
- [ ] `tests/api/testcontainers_account_deletion_tests.rs` - NEEDS multiple builders
- [ ] `tests/api/testcontainers_multi_table_integration_tests.rs` - May need builders
- [ ] `tests/services/email_suppression_integration_tests.rs` - May need builders
- [ ] `tests/services/data_export_integration_tests.rs` - May need builders
- [ ] `tests/services/sns_webhook_handler_tests.rs` - May need builders
- [ ] `tests/repositories/testcontainers_email_suppression_repository_tests.rs` - May need builders
- [ ] `tests/rate_limiting_integration_tests.rs` - May need builders
- [ ] `tests/redis_integration_tests.rs` - May need builders
- [ ] `tests/redis_pkce_storage_integration_tests.rs` - May need builders
- [ ] `tests/token_cleanup_tests.rs` - NEEDS RefreshTokenBuilder

## Guidelines for Adding New Builders

When you encounter a struct that's frequently constructed in tests:

### 1. Create the Builder
```rust
// src/test_utils/my_struct_builder.rs

#[derive(Clone)]
pub struct MyStructBuilder {
    field1: Option<Type1>,
    field2: Option<Type2>,
    // ... all fields as Option<T>
}

impl MyStructBuilder {
    pub fn new() -> Self {
        Self {
            field1: None,
            field2: None,
        }
    }

    pub fn build(self) -> MyStruct {
        MyStruct {
            field1: self.field1.unwrap_or_else(|| /* sensible default */),
            field2: self.field2.unwrap_or_else(|| /* sensible default */),
        }
    }

    // For integration tests with database
    pub async fn persist(self, pool: &PgPool) -> Result<MyStruct> {
        // INSERT into database
        // Return the created struct
    }

    // Chainable setters
    pub fn with_field1(mut self, value: Type1) -> Self {
        self.field1 = Some(value);
        self
    }
}
```

### 2. Export from test_utils
```rust
// src/test_utils/mod.rs
pub mod my_struct_builder;
pub use my_struct_builder::MyStructBuilder;
```

### 3. Update This Document
Add to the appropriate section above.

### 4. Use in Tests
Replace all manual constructions with builder.

## Benefits

1. **Single Source of Truth:** Update struct → update builder once → all tests work
2. **Readable Tests:** Focus on what matters, hide boilerplate
3. **Discoverable:** IDE autocomplete shows all available configuration
4. **Future-Proof:** Adding fields doesn't break existing tests
5. **Idiomatic Rust:** Follows patterns used by Cargo and other major projects

## References

- [Rust Builder Pattern (Official Patterns)](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
- [Builder Pattern for Tests - Julio Merino](https://jmmv.dev/2020/12/builder-pattern-for-tests.html)
- [Testing Rust with Builder Pattern - Dan Munckton](https://dan.munckton.co.uk/blog/2018/03/01/testing-rust-using-the-builder-pattern-for-complex-fixtures/)
