# Phase 2A: Core User Models

**Estimated Time**: 1-2 hours
**Prerequisites**: [Phase 1: Database Schema](PHASE-01-DATABASE-SCHEMA.md) complete
**Next Phase**: [Phase 2B: New Table Models](PHASE-02B-NEW-MODELS.md)

## Objective

Update the core `User` model to prepare for the multi-table architecture. In this phase, we'll maintain backward compatibility by keeping all fields temporarily, but we'll start creating the foundation for the new composite structures.

**Key Principle**: Don't break existing code yet. Add new structures alongside old ones.

---

## TDD Approach

For this phase:
1. **Red**: Write tests for new model structures first
2. **Green**: Create the models to make tests pass
3. **Refactor**: Clean up model definitions

---

## Current User Model

From `backend/src/models/db/user.rs`:

```rust
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,        // Will move to user_credentials
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub real_name: Option<String>,            // Will move to user_profiles
    pub google_user_id: Option<String>,       // Will move to user_external_logins
    pub timer_is_public: bool,                // Will move to user_preferences
    pub timer_show_in_list: bool,             // Will move to user_preferences
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

---

## Task 1: Create Slimmed User Model (30 minutes)

### Step 1: Write tests first (TDD Red)

Create `backend/src/models/db/user.rs` test module at the bottom of the file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_slim_user_serialization() {
        // Test that SlimUser only contains core identity fields
        let user = SlimUser {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Verify serialization works
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("test@example.com"));
        assert!(json.contains("test-user"));

        // Verify it doesn't contain removed fields
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("google_user_id"));
        assert!(!json.contains("timer_is_public"));
    }

    #[test]
    fn test_slim_user_has_no_auth_fields() {
        // Verify SlimUser doesn't expose sensitive auth data
        use serde_json::Value;

        let user = SlimUser {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            slug: "test-user".to_string(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_value(&user).unwrap();

        // These fields should NOT exist in SlimUser
        assert!(json.get("password_hash").is_none());
        assert!(json.get("google_user_id").is_none());
        assert!(json.get("real_name").is_none());
        assert!(json.get("timer_is_public").is_none());
    }
}
```

Run tests (they should fail):
```bash
cd backend
cargo test user::tests --lib
# Expected: Compilation error - SlimUser doesn't exist yet
```

### Step 2: Create SlimUser model (TDD Green)

Add to `backend/src/models/db/user.rs` after the existing `User` struct:

```rust
/// Slim user model with only core identity fields
/// This is the target structure after refactoring
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct SlimUser {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            slug: user.slug,
            active: user.active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
```

Run tests again:
```bash
cargo test user::tests --lib
# Expected: Tests pass
```

**Success Criteria**:
- Tests compile and pass
- SlimUser has no auth/profile/preference fields
- Conversion from User to SlimUser works

---

## Task 2: Create UserWithDetails Composite (30 minutes)

### Step 1: Write tests first (TDD Red)

Add to test module:

```rust
#[test]
fn test_user_with_details_structure() {
    // Test composite structure for full user data
    let user = SlimUser {
        id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        display_name: "Test User".to_string(),
        slug: "test-user".to_string(),
        active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let details = UserWithDetails {
        user: user.clone(),
        has_password: true,
        external_logins: vec![],
        profile: None,
        preferences: None,
    };

    // Verify structure
    assert_eq!(details.user.id, user.id);
    assert_eq!(details.has_password, true);
    assert_eq!(details.external_logins.len(), 0);
    assert!(details.profile.is_none());
}

#[test]
fn test_user_with_details_oauth_user() {
    // Test OAuth-only user (no password)
    let user = SlimUser {
        id: Uuid::new_v4(),
        email: "oauth@example.com".to_string(),
        display_name: "OAuth User".to_string(),
        slug: "oauth-user".to_string(),
        active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let external_login = ExternalLoginInfo {
        provider: "google".to_string(),
        linked_at: Utc::now(),
    };

    let details = UserWithDetails {
        user: user.clone(),
        has_password: false,
        external_logins: vec![external_login],
        profile: None,
        preferences: None,
    };

    // Verify OAuth-only structure
    assert_eq!(details.has_password, false);
    assert_eq!(details.external_logins.len(), 1);
    assert_eq!(details.external_logins[0].provider, "google");
}
```

Run tests (they should fail):
```bash
cargo test user::tests --lib
# Expected: Compilation errors
```

### Step 2: Create composite structures (TDD Green)

Add to `backend/src/models/db/user.rs`:

```rust
/// External login information for user details
#[derive(Debug, Clone, Serialize)]
pub struct ExternalLoginInfo {
    pub provider: String,
    pub linked_at: DateTime<Utc>,
}

/// Profile information summary
#[derive(Debug, Clone, Serialize)]
pub struct ProfileInfo {
    pub real_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

/// Preferences summary
#[derive(Debug, Clone, Serialize)]
pub struct PreferencesInfo {
    pub timer_is_public: bool,
    pub timer_show_in_list: bool,
}

/// Complete user data with all related information
/// Used when we need full user context (e.g., profile pages, admin views)
#[derive(Debug, Clone, Serialize)]
pub struct UserWithDetails {
    pub user: SlimUser,
    pub has_password: bool,
    pub external_logins: Vec<ExternalLoginInfo>,
    pub profile: Option<ProfileInfo>,
    pub preferences: Option<PreferencesInfo>,
}
```

Run tests:
```bash
cargo test user::tests --lib
# Expected: All tests pass
```

**Success Criteria**:
- Tests compile and pass
- `UserWithDetails` aggregates all user data
- Support for OAuth-only users (no password)
- Support for users with multiple external logins

---

## Task 3: Keep Existing User Struct Intact (15 minutes)

**Important**: We're NOT removing fields from `User` yet. This happens in Phase 7 (Data Migration).

For now, add a comment to the existing `User` struct:

```rust
/// Current monolithic user model
/// TODO(Phase 7): Remove password_hash, google_user_id, real_name, timer_* fields
/// after data migration is complete
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,        // TODO: Move to user_credentials
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub real_name: Option<String>,            // TODO: Move to user_profiles
    pub google_user_id: Option<String>,       // TODO: Move to user_external_logins
    pub timer_is_public: bool,                // TODO: Move to user_preferences
    pub timer_show_in_list: bool,             // TODO: Move to user_preferences
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Why Keep It?** All existing code still uses `User`. We'll migrate service by service in later phases.

---

## Task 4: Update mod.rs Exports (15 minutes)

Edit `backend/src/models/db/mod.rs`:

```rust
pub mod user;
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

pub use refresh_token::RefreshToken;
pub use incident_timer::IncidentTimer;
pub use phrase::{Phrase, PhraseSuggestion, UserExcludedPhrase};
pub use email_suppression::EmailSuppression;
```

**Success Criteria**:
- All new types exported
- Existing exports unchanged
- No breaking changes to consuming code

---

## Task 5: Run Tests (15 minutes)

### Step 1: Run unit tests

```bash
cd backend
cargo test models::db::user --lib
# Expected: All tests pass
```

### Step 2: Run full test suite

```bash
cargo test -- --test-threads=4
# Expected: All 227 tests still pass
# (We haven't broken anything yet)
```

### Step 3: Verify compilation

```bash
cargo check
# Expected: No errors, no warnings
```

**Success Criteria**:
- All model tests pass
- Full test suite still passes (227 tests)
- No compilation errors
- No warnings

---

## Deliverables

At the end of this phase, you should have:

1. **Updated `backend/src/models/db/user.rs`**
   - `SlimUser` struct (target structure)
   - `UserWithDetails` composite
   - `ExternalLoginInfo`, `ProfileInfo`, `PreferencesInfo` helpers
   - Conversion from `User` to `SlimUser`
   - Comprehensive unit tests
   - TODO comments on existing `User` struct

2. **Updated `backend/src/models/db/mod.rs`**
   - All new types exported
   - No breaking changes

3. **Passing Tests**
   - All new model tests pass
   - All existing tests still pass

---

## Success Criteria

**Before proceeding to Phase 2B**, verify:

- [ ] `SlimUser` struct created with core fields only
- [ ] `UserWithDetails` composite struct created
- [ ] `ExternalLoginInfo`, `ProfileInfo`, `PreferencesInfo` helper structs created
- [ ] Conversion from `User` to `SlimUser` implemented
- [ ] Unit tests for all new structures
- [ ] All model tests pass
- [ ] Full test suite still passes (227 tests)
- [ ] No compilation errors or warnings
- [ ] `mod.rs` exports all new types
- [ ] Existing `User` struct unchanged (backward compatibility)

**Time Check**: This phase should take 1-2 hours.

---

## Common Issues

### Issue: Serialization tests fail
**Cause**: Missing `Serialize` derive
**Solution**: Add `#[derive(Serialize)]` to all structs

### Issue: FromRow trait not found
**Cause**: Missing sqlx import
**Solution**: Ensure `use sqlx::FromRow;` at top of file

### Issue: Tests can't find new structs
**Cause**: Not exported from mod.rs
**Solution**: Add to `pub use user::{...}` in mod.rs

### Issue: Existing tests fail
**Cause**: Breaking change introduced
**Solution**: Revert - we shouldn't break anything in this phase

---

## Next Steps

Once all success criteria are met:

1. **Commit the changes**:
   ```bash
   git add backend/src/models/db/user.rs
   git add backend/src/models/db/mod.rs
   git commit -m "feat(models): add SlimUser and UserWithDetails composite (Phase 2A)

   - Add SlimUser with core identity fields only
   - Add UserWithDetails composite for full user data
   - Add helper structs for external logins, profile, preferences
   - Add conversion from User to SlimUser
   - Add comprehensive unit tests
   - Maintain backward compatibility with existing User struct
   "
   ```

2. **Proceed to Phase 2B**:
   - Read [PHASE-02B-NEW-MODELS.md](PHASE-02B-NEW-MODELS.md)
   - Create models for new tables
   - Continue TDD cycle

---

**Phase Status**: ⬜ Not Started → **Continue when ready**

**Estimated Completion Time**: 1-2 hours

**Next Phase**: [Phase 2B: New Table Models](PHASE-02B-NEW-MODELS.md)
