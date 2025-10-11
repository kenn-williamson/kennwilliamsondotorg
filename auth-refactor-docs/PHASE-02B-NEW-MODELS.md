# Phase 2B: New Table Models

**Estimated Time**: 1-2 hours
**Prerequisites**: [Phase 2A: Core User Models](PHASE-02A-CORE-MODELS.md) complete
**Next Phase**: [Phase 3A: Credentials & External Login Repositories](PHASE-03A-CREDENTIALS-REPOS.md)

## Objective

Create Rust model structs for the 4 new database tables created in Phase 1. Each model will have comprehensive unit tests following TDD principles.

**Key Principle**: Models are pure data structures with no business logic. They map directly to database tables.

---

## TDD Approach

For each model:
1. **Red**: Write tests for serialization, deserialization, and FromRow
2. **Green**: Create the model to make tests pass
3. **Refactor**: Clean up, add documentation

---

## Task 1: Create UserCredentials Model (20 minutes)

### Step 1: Create new file

Create `backend/src/models/db/user_credentials.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// Local password authentication credentials
/// Optional table - OAuth-only users won't have a row here
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserCredentials {
    pub user_id: Uuid,
    pub password_hash: String,
    pub password_updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_credentials_serialization() {
        let creds = UserCredentials {
            user_id: Uuid::new_v4(),
            password_hash: "$2b$12$test_hash".to_string(),
            password_updated_at: Utc::now(),
            created_at: Utc::now(),
        };

        // Verify serialization works
        let json = serde_json::to_string(&creds).unwrap();
        assert!(json.contains("password_hash"));
        assert!(json.contains("$2b$12$test_hash"));
    }

    #[test]
    fn test_user_credentials_structure() {
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        let creds = UserCredentials {
            user_id,
            password_hash: "hashed_password".to_string(),
            password_updated_at: now,
            created_at: now,
        };

        assert_eq!(creds.user_id, user_id);
        assert_eq!(creds.password_hash, "hashed_password");
        assert_eq!(creds.password_updated_at, now);
    }

    #[test]
    fn test_user_credentials_debug_trait() {
        let creds = UserCredentials {
            user_id: Uuid::new_v4(),
            password_hash: "secret".to_string(),
            password_updated_at: Utc::now(),
            created_at: Utc::now(),
        };

        // Verify Debug trait works
        let debug_str = format!("{:?}", creds);
        assert!(debug_str.contains("UserCredentials"));
    }
}
```

### Step 2: Run tests

```bash
cd backend
cargo test user_credentials::tests --lib
# Expected: All tests pass
```

**Success Criteria**:
- Model compiles
- FromRow, Serialize, Debug traits work
- All tests pass

---

## Task 2: Create UserExternalLogin Model (20 minutes)

### Step 1: Create new file

Create `backend/src/models/db/user_external_login.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// OAuth external login (supports multiple providers per user)
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserExternalLogin {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub linked_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_external_login_serialization() {
        let login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            provider: "google".to_string(),
            provider_user_id: "google_12345".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&login).unwrap();
        assert!(json.contains("google"));
        assert!(json.contains("google_12345"));
    }

    #[test]
    fn test_user_external_login_google_provider() {
        let login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            provider: "google".to_string(),
            provider_user_id: "google_123".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(login.provider, "google");
        assert_eq!(login.provider_user_id, "google_123");
    }

    #[test]
    fn test_user_external_login_github_provider() {
        let login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            provider: "github".to_string(),
            provider_user_id: "github_456".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(login.provider, "github");
        assert_eq!(login.provider_user_id, "github_456");
    }

    #[test]
    fn test_user_external_login_multiple_providers() {
        // User can have multiple external logins
        let user_id = Uuid::new_v4();

        let google_login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id,
            provider: "google".to_string(),
            provider_user_id: "google_123".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let github_login = UserExternalLogin {
            id: Uuid::new_v4(),
            user_id,
            provider: "github".to_string(),
            provider_user_id: "github_456".to_string(),
            linked_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Both logins for same user but different providers
        assert_eq!(google_login.user_id, github_login.user_id);
        assert_ne!(google_login.provider, github_login.provider);
        assert_ne!(google_login.id, github_login.id);
    }
}
```

### Step 2: Run tests

```bash
cargo test user_external_login::tests --lib
# Expected: All tests pass
```

**Success Criteria**:
- Model compiles
- Supports multiple providers (google, github, etc.)
- All tests pass

---

## Task 3: Create UserProfile Model (20 minutes)

### Step 1: Create new file

Create `backend/src/models/db/user_profile.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// User profile data (bio, avatar, etc.)
/// Optional data - users may not have profile information
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub real_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_profile_serialization() {
        let profile = UserProfile {
            user_id: Uuid::new_v4(),
            real_name: Some("John Doe".to_string()),
            bio: Some("Software developer".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            location: Some("San Francisco".to_string()),
            website: Some("https://johndoe.com".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains("John Doe"));
        assert!(json.contains("Software developer"));
    }

    #[test]
    fn test_user_profile_minimal() {
        // Profile with all optional fields as None
        let profile = UserProfile {
            user_id: Uuid::new_v4(),
            real_name: None,
            bio: None,
            avatar_url: None,
            location: None,
            website: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(profile.real_name.is_none());
        assert!(profile.bio.is_none());
        assert!(profile.avatar_url.is_none());
    }

    #[test]
    fn test_user_profile_partial() {
        // Profile with some fields filled
        let profile = UserProfile {
            user_id: Uuid::new_v4(),
            real_name: Some("Jane Smith".to_string()),
            bio: None,
            avatar_url: Some("https://example.com/jane.jpg".to_string()),
            location: None,
            website: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(profile.real_name, Some("Jane Smith".to_string()));
        assert!(profile.bio.is_none());
        assert!(profile.avatar_url.is_some());
    }

    #[test]
    fn test_user_profile_from_oauth() {
        // Profile created from OAuth data
        let profile = UserProfile {
            user_id: Uuid::new_v4(),
            real_name: Some("OAuth User".to_string()),
            bio: None,
            avatar_url: Some("https://lh3.googleusercontent.com/...".to_string()),
            location: None,
            website: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(profile.real_name.is_some());
        assert!(profile.avatar_url.is_some());
    }
}
```

### Step 2: Run tests

```bash
cargo test user_profile::tests --lib
# Expected: All tests pass
```

**Success Criteria**:
- Model compiles
- All fields optional (except user_id, timestamps)
- All tests pass

---

## Task 4: Create UserPreferences Model (20 minutes)

### Step 1: Create new file

Create `backend/src/models/db/user_preferences.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// User application preferences
/// Every user has preferences (created with defaults)
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserPreferences {
    pub user_id: Uuid,
    pub timer_is_public: bool,
    pub timer_show_in_list: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserPreferences {
    /// Create default preferences for a new user
    pub fn default_for_user(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            timer_is_public: false,
            timer_show_in_list: false,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_preferences_serialization() {
        let prefs = UserPreferences {
            user_id: Uuid::new_v4(),
            timer_is_public: true,
            timer_show_in_list: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&prefs).unwrap();
        assert!(json.contains("timer_is_public"));
        assert!(json.contains("timer_show_in_list"));
    }

    #[test]
    fn test_user_preferences_defaults() {
        let user_id = Uuid::new_v4();
        let prefs = UserPreferences::default_for_user(user_id);

        assert_eq!(prefs.user_id, user_id);
        assert_eq!(prefs.timer_is_public, false);
        assert_eq!(prefs.timer_show_in_list, false);
    }

    #[test]
    fn test_user_preferences_public_timer() {
        let prefs = UserPreferences {
            user_id: Uuid::new_v4(),
            timer_is_public: true,
            timer_show_in_list: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(prefs.timer_is_public, true);
        assert_eq!(prefs.timer_show_in_list, true);
    }

    #[test]
    fn test_user_preferences_validation_ready() {
        // Future: Business rule - show_in_list requires is_public
        // This test documents the expected constraint
        let prefs = UserPreferences {
            user_id: Uuid::new_v4(),
            timer_is_public: false,
            timer_show_in_list: true, // This should be validated at service layer
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Model allows it (validation happens in service layer)
        assert_eq!(prefs.timer_is_public, false);
        assert_eq!(prefs.timer_show_in_list, true);
        // Note: This invalid state is allowed at model level
        // but should be caught by service layer validation
    }
}
```

### Step 2: Run tests

```bash
cargo test user_preferences::tests --lib
# Expected: All tests pass
```

**Success Criteria**:
- Model compiles
- Default preferences helper
- All tests pass
- Documentation for future business rules

---

## Task 5: Update mod.rs Exports (15 minutes)

Edit `backend/src/models/db/mod.rs`:

```rust
pub mod user;
pub mod user_credentials;
pub mod user_external_login;
pub mod user_profile;
pub mod user_preferences;
pub mod refresh_token;
pub mod incident_timer;
pub mod phrase;
pub mod email_suppression;

// Re-export all types for convenience
pub use user::{
    User,
    SlimUser,
    UserWithDetails,
    ExternalLoginInfo,
    ProfileInfo,
    PreferencesInfo,
    UserWithRoles,
    UserWithTimer,
    VerificationToken,
    PasswordResetToken,
};

pub use user_credentials::UserCredentials;
pub use user_external_login::UserExternalLogin;
pub use user_profile::UserProfile;
pub use user_preferences::UserPreferences;

pub use refresh_token::RefreshToken;
pub use incident_timer::IncidentTimer;
pub use phrase::{Phrase, PhraseSuggestion, UserExcludedPhrase};
pub use email_suppression::EmailSuppression;
```

**Success Criteria**:
- All new models exported
- Consistent export pattern
- No breaking changes

---

## Task 6: Run All Tests (15 minutes)

### Step 1: Test new models

```bash
cd backend
cargo test user_credentials::tests --lib
cargo test user_external_login::tests --lib
cargo test user_profile::tests --lib
cargo test user_preferences::tests --lib
# Expected: All pass
```

### Step 2: Test full suite

```bash
cargo test -- --test-threads=4
# Expected: All 227 tests still pass
```

### Step 3: Verify compilation

```bash
cargo check
# Expected: No errors, no warnings
```

**Success Criteria**:
- All new model tests pass
- Full test suite still passes
- No compilation errors

---

## Deliverables

At the end of this phase, you should have:

1. **New Model Files**
   - `backend/src/models/db/user_credentials.rs`
   - `backend/src/models/db/user_external_login.rs`
   - `backend/src/models/db/user_profile.rs`
   - `backend/src/models/db/user_preferences.rs`

2. **Updated `backend/src/models/db/mod.rs`**
   - All new models exported

3. **Comprehensive Tests**
   - Serialization tests
   - Structure tests
   - Edge case tests (OAuth-only, minimal profiles, etc.)
   - All tests passing

---

## Success Criteria

**Before proceeding to Phase 3A**, verify:

- [ ] `UserCredentials` model created with tests
- [ ] `UserExternalLogin` model created with tests
- [ ] `UserProfile` model created with tests
- [ ] `UserPreferences` model created with tests
- [ ] All models have FromRow, Serialize, Debug derives
- [ ] `UserPreferences::default_for_user()` helper implemented
- [ ] All new model tests pass
- [ ] Full test suite still passes (227 tests)
- [ ] No compilation errors or warnings
- [ ] All new models exported from mod.rs

**Time Check**: This phase should take 1-2 hours.

---

## Common Issues

### Issue: FromRow trait not working
**Cause**: Field names don't match database columns
**Solution**: Ensure struct field names match database column names exactly

### Issue: Serialization fails
**Cause**: Missing Serialize derive
**Solution**: Add `#[derive(Serialize)]` to all structs

### Issue: Tests can't find models
**Cause**: Not exported from mod.rs
**Solution**: Add `pub use model_name::ModelName;` to mod.rs

### Issue: DateTime serialization format issues
**Cause**: Chrono serialization
**Solution**: Already handled by Serialize derive, no action needed

---

## Next Steps

Once all success criteria are met:

1. **Commit the changes**:
   ```bash
   git add backend/src/models/db/user_credentials.rs
   git add backend/src/models/db/user_external_login.rs
   git add backend/src/models/db/user_profile.rs
   git add backend/src/models/db/user_preferences.rs
   git add backend/src/models/db/mod.rs
   git commit -m "feat(models): add models for new auth schema tables (Phase 2B)

   - Add UserCredentials model for local authentication
   - Add UserExternalLogin model for multi-provider OAuth
   - Add UserProfile model for optional profile data
   - Add UserPreferences model for user settings
   - Add comprehensive unit tests for all models
   - Export all new models from mod.rs
   "
   ```

2. **Proceed to Phase 3A**:
   - Read [PHASE-03A-CREDENTIALS-REPOS.md](PHASE-03A-CREDENTIALS-REPOS.md)
   - Create repositories for credentials and external logins
   - Continue TDD cycle

---

**Phase Status**: ⬜ Not Started → **Continue when ready**

**Estimated Completion Time**: 1-2 hours

**Next Phase**: [Phase 3A: Credentials & External Login Repositories](PHASE-03A-CREDENTIALS-REPOS.md)
