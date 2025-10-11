# Phase 4D: Profile Management Service

**Estimated Time**: 1-2 hours
**Prerequisites**: [Phase 4C: OAuth Service Updates](PHASE-04C-OAUTH-SERVICE.md) complete
**Next Phase**: [Phase 5: Data Export (CRITICAL)](PHASE-05-DATA-EXPORT.md)

## Objective

Update profile management operations to use the appropriate repositories: `user_repository` for core identity fields, `profile_repository` for profile data, and `preferences_repository` for settings.

**Key Principle**: Route updates to the correct table based on data type (identity, profile, or preferences).

---

## TDD Approach

1. **Red**: Write tests for profile update scenarios
2. **Green**: Update profile.rs to use multiple repositories
3. **Refactor**: Validate business rules (e.g., show_in_list requires is_public)

---

## Current Profile Management

From `backend/src/services/auth/auth_service/profile.rs`:

```rust
// CURRENT: All updates go to users table
pub async fn update_profile(&self, user_id: Uuid, updates: ProfileUpdateRequest) -> Result<User> {
    // Update display_name, slug, real_name all in users table
    self.user_repository.update_user(user_id, &UserUpdates {
        display_name: updates.display_name,
        slug: updates.slug,
    }).await?;

    if let Some(real_name) = updates.real_name {
        self.user_repository.update_real_name(user_id, Some(real_name)).await?;
    }

    self.user_repository.find_by_id(user_id).await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))
}

pub async fn update_timer_privacy(&self, user_id: Uuid, is_public: bool, show_in_list: bool) -> Result<User> {
    // Validate: show_in_list requires is_public
    if show_in_list && !is_public {
        return Err(anyhow::anyhow!("Timer must be public to show in list"));
    }

    self.user_repository.update_timer_privacy(user_id, is_public, show_in_list).await
}
```

---

## New Profile Management

```rust
// NEW: Route updates to appropriate repositories

pub async fn update_profile(&self, user_id: Uuid, updates: ProfileUpdateRequest) -> Result<()> {
    // 1. Update core identity fields (display_name, slug) → users table
    if updates.display_name.is_some() || updates.slug.is_some() {
        self.user_repository.update_user(user_id, &UserUpdates {
            display_name: updates.display_name.unwrap_or_else(|| {
                // Fetch current if not updating
                /* ... */
            }),
            slug: updates.slug.unwrap_or_else(|| {
                // Fetch current if not updating
                /* ... */
            }),
        }).await?;
    }

    // 2. Update profile fields (real_name, bio, etc.) → user_profiles table
    if updates.real_name.is_some() || updates.bio.is_some() || updates.avatar_url.is_some() {
        if let Some(profile_repo) = &self.profile_repository {
            // Ensure profile exists
            if profile_repo.find_by_user_id(user_id).await?.is_none() {
                profile_repo.create(user_id).await?;
            }

            profile_repo.update(user_id, UpdateProfile {
                real_name: updates.real_name,
                bio: updates.bio,
                avatar_url: updates.avatar_url,
                location: updates.location,
                website: updates.website,
            }).await?;
        }
    }

    Ok(())
}

pub async fn update_timer_privacy(&self, user_id: Uuid, is_public: bool, show_in_list: bool) -> Result<()> {
    // Validate business rule
    if show_in_list && !is_public {
        return Err(anyhow::anyhow!("Timer must be public to show in list"));
    }

    // Update preferences → user_preferences table
    if let Some(prefs_repo) = &self.preferences_repository {
        prefs_repo.update_timer_settings(user_id, is_public, show_in_list).await?;
    }

    Ok(())
}

pub async fn get_profile(&self, user_id: Uuid) -> Result<UserWithDetails> {
    // Gather data from multiple tables
    let user = self.user_repository.find_by_id(user_id).await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    let has_password = if let Some(creds_repo) = &self.credentials_repository {
        creds_repo.has_password(user_id).await.unwrap_or(false)
    } else {
        false
    };

    let external_logins = if let Some(ext_repo) = &self.external_login_repository {
        ext_repo.find_by_user_id(user_id).await.unwrap_or_default()
            .into_iter()
            .map(|login| ExternalLoginInfo {
                provider: login.provider,
                linked_at: login.linked_at,
            })
            .collect()
    } else {
        vec![]
    };

    let profile = if let Some(profile_repo) = &self.profile_repository {
        profile_repo.find_by_user_id(user_id).await?
            .map(|p| ProfileInfo {
                real_name: p.real_name,
                bio: p.bio,
                avatar_url: p.avatar_url,
                location: p.location,
                website: p.website,
            })
    } else {
        None
    };

    let preferences = if let Some(prefs_repo) = &self.preferences_repository {
        prefs_repo.find_by_user_id(user_id).await?
            .map(|p| PreferencesInfo {
                timer_is_public: p.timer_is_public,
                timer_show_in_list: p.timer_show_in_list,
            })
    } else {
        None
    };

    Ok(UserWithDetails {
        user: SlimUser::from(user),
        has_password,
        external_logins,
        profile,
        preferences,
    })
}
```

---

## Test Scenarios

### Test 1: Update display_name only (core identity)

```rust
#[tokio::test]
async fn test_update_display_name() {
    let mut user_repo = MockUserRepository::new();

    user_repo.expect_update_user()
        .withf(|_, updates| updates.display_name == "New Name")
        .returning(|_, _| Ok(test_user()));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .build();

    let result = auth_service.update_profile(test_user_id(), ProfileUpdateRequest {
        display_name: Some("New Name".to_string()),
        ..Default::default()
    }).await;

    assert!(result.is_ok());
}
```

### Test 2: Update real_name (profile data)

```rust
#[tokio::test]
async fn test_update_real_name() {
    let user_repo = MockUserRepository::new();
    let mut profile_repo = MockUserProfileRepository::new();

    profile_repo.expect_find_by_user_id()
        .returning(|_| Ok(Some(test_profile())));

    profile_repo.expect_update()
        .withf(|_, data| data.real_name == Some("John Doe".to_string()))
        .returning(|_, _| Ok(test_profile()));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .profile_repository(Box::new(profile_repo))
        .build();

    let result = auth_service.update_profile(test_user_id(), ProfileUpdateRequest {
        real_name: Some("John Doe".to_string()),
        ..Default::default()
    }).await;

    assert!(result.is_ok());
}
```

### Test 3: Update timer preferences

```rust
#[tokio::test]
async fn test_update_timer_preferences() {
    let user_repo = MockUserRepository::new();
    let mut prefs_repo = MockUserPreferencesRepository::new();

    prefs_repo.expect_update_timer_settings()
        .with(eq(test_user_id()), eq(true), eq(true))
        .returning(|_, _, _| Ok(()));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .preferences_repository(Box::new(prefs_repo))
        .build();

    let result = auth_service.update_timer_privacy(test_user_id(), true, true).await;
    assert!(result.is_ok());
}
```

### Test 4: Validate show_in_list requires is_public

```rust
#[tokio::test]
async fn test_timer_privacy_validation() {
    let user_repo = MockUserRepository::new();
    let prefs_repo = MockUserPreferencesRepository::new();

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .preferences_repository(Box::new(prefs_repo))
        .build();

    // Try to set show_in_list=true but is_public=false
    let result = auth_service.update_timer_privacy(test_user_id(), false, true).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("must be public"));
}
```

### Test 5: Get complete profile (multi-table)

```rust
#[tokio::test]
async fn test_get_profile_with_all_data() {
    let mut user_repo = MockUserRepository::new();
    let mut creds_repo = MockUserCredentialsRepository::new();
    let mut ext_repo = MockUserExternalLoginRepository::new();
    let mut profile_repo = MockUserProfileRepository::new();
    let mut prefs_repo = MockUserPreferencesRepository::new();

    let user_id = test_user_id();

    user_repo.expect_find_by_id()
        .returning(move |_| Ok(Some(test_user(user_id))));

    creds_repo.expect_has_password()
        .returning(|_| Ok(true));

    ext_repo.expect_find_by_user_id()
        .returning(|_| Ok(vec![test_external_login()]));

    profile_repo.expect_find_by_user_id()
        .returning(|_| Ok(Some(test_profile())));

    prefs_repo.expect_find_by_user_id()
        .returning(|_| Ok(Some(test_preferences())));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .credentials_repository(Box::new(creds_repo))
        .external_login_repository(Box::new(ext_repo))
        .profile_repository(Box::new(profile_repo))
        .preferences_repository(Box::new(prefs_repo))
        .build();

    let result = auth_service.get_profile(user_id).await;
    assert!(result.is_ok());

    let profile = result.unwrap();
    assert_eq!(profile.has_password, true);
    assert_eq!(profile.external_logins.len(), 1);
    assert!(profile.profile.is_some());
    assert!(profile.preferences.is_some());
}
```

---

## Deliverables

1. **Updated profile.rs**: Routes updates to correct repositories
2. **get_profile()**: Aggregates data from multiple tables
3. **Business rule validation**: Timer privacy validation
4. **Comprehensive tests**: All profile scenarios covered

---

## Success Criteria

**Before proceeding to Phase 5**, verify:

- [ ] profile.rs routes updates to correct repositories
- [ ] Core identity updates use user_repository
- [ ] Profile data updates use profile_repository
- [ ] Preferences updates use preferences_repository
- [ ] Timer privacy validation works
- [ ] get_profile() aggregates multi-table data
- [ ] All profile tests pass
- [ ] Full test suite still passes (227 tests)

**Time Check**: 1-2 hours

---

## Common Issues

### Issue: Profile doesn't exist when trying to update
**Cause**: User created before profile table existed
**Solution**: Create profile if it doesn't exist before updating

### Issue: Optional repository fields cause panics
**Cause**: Using `.unwrap()` on Option<Box<dyn Repo>>
**Solution**: Use `if let Some(repo) = &self.repo { ... }`

### Issue: Validation happens at wrong layer
**Cause**: Business rules in repository
**Solution**: Keep validation in service layer, repositories are dumb CRUD

---

## Next Steps

```bash
git add backend/src/services/auth/auth_service/profile.rs
git commit -m "feat(services): update profile management for multi-table schema (Phase 4D)

- Route updates to appropriate repositories (user, profile, preferences)
- Implement get_profile() to aggregate multi-table data
- Maintain business rule validation (timer privacy)
- Add comprehensive profile update tests
"
```

**Next Phase**: [Phase 5: Data Export (CRITICAL)](PHASE-05-DATA-EXPORT.md)

⚠️ **IMPORTANT**: Phase 5 is critical for GDPR/CCPA compliance and MUST be complete before proceeding to Phase 6.
