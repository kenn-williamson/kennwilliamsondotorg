# Phase 5: Data Export (CRITICAL - GDPR/CCPA)

**Estimated Time**: 2-3 hours
**Prerequisites**: [Phase 4D: Profile Management Service](PHASE-04D-PROFILE-SERVICE.md) complete
**Next Phase**: [Phase 6: Test Helpers & Builder Pattern](PHASE-06-TEST-HELPERS.md)

## ⚠️ CRITICAL IMPORTANCE

This phase MUST be completed successfully before proceeding. GDPR/CCPA violations carry severe legal penalties:
- **GDPR fines**: Up to €20 million or 4% of global annual revenue
- **CCPA fines**: $7,500 per intentional violation
- **Response time**: Must respond within 30 days

**BLOCKER**: Cannot proceed to Phase 6 until data export is verified complete.

---

## Objective

Update data export functionality to include ALL new tables in the user data export. Increment export version to "2.0" to indicate schema change.

**Key Principle**: Every new table with user data MUST be included in the export.

---

## TDD Approach

1. **Red**: Write comprehensive export tests verifying ALL tables
2. **Green**: Update data_export.rs to query all new tables
3. **Refactor**: Verify completeness with manual test

---

## Current Data Export

From `backend/src/services/auth/auth_service/data_export.rs`:

```rust
pub async fn export_user_data(&self, user_id: Uuid) -> Result<UserDataExport> {
    let user = self.user_repository.find_by_id(user_id).await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    let roles = self.user_repository.get_user_roles(user_id).await?;

    let user_export = UserExportData {
        id: user.id,
        email: user.email,
        display_name: user.display_name,
        slug: user.slug,
        real_name: user.real_name,              // Currently in users table
        google_user_id: user.google_user_id,    // Currently in users table
        active: user.active,
        email_verified: roles.contains(&"email-verified".to_string()),
        created_at: user.created_at,
        updated_at: user.updated_at,
        roles,
    };

    // ... export timers, phrases, sessions, etc.

    Ok(UserDataExport {
        export_version: "1.0",  // OLD VERSION
        export_date: Utc::now(),
        user: user_export,
        incident_timers,
        phrase_suggestions,
        phrase_exclusions,
        active_sessions,
        verification_history,
    })
}
```

---

## New Data Export (v2.0)

```rust
pub async fn export_user_data(&self, user_id: Uuid) -> Result<UserDataExport> {
    // 1. Core user identity
    let user = self.user_repository.find_by_id(user_id).await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    let roles = self.user_repository.get_user_roles(user_id).await?;
    let email_verified = roles.contains(&"email-verified".to_string());

    // 2. NEW: Authentication details (from user_credentials)
    let has_password = if let Some(creds_repo) = &self.credentials_repository {
        creds_repo.has_password(user_id).await.unwrap_or(false)
    } else {
        false
    };

    let password_last_changed = if let Some(creds_repo) = &self.credentials_repository {
        creds_repo.find_by_user_id(user_id).await.ok()
            .flatten()
            .map(|c| c.password_updated_at)
    } else {
        None
    };

    // 3. NEW: External logins (from user_external_logins)
    let external_logins = if let Some(ext_repo) = &self.external_login_repository {
        ext_repo.find_by_user_id(user_id).await.unwrap_or_default()
            .into_iter()
            .map(|login| ExternalLoginExport {
                provider: login.provider,
                provider_user_id: login.provider_user_id,
                linked_at: login.linked_at,
            })
            .collect()
    } else {
        vec![]
    };

    // 4. NEW: Profile data (from user_profiles)
    let profile = if let Some(profile_repo) = &self.profile_repository {
        profile_repo.find_by_user_id(user_id).await.ok()
            .flatten()
            .map(|p| ProfileExport {
                real_name: p.real_name,
                bio: p.bio,
                avatar_url: p.avatar_url,
                location: p.location,
                website: p.website,
            })
    } else {
        None
    };

    // 5. NEW: Preferences (from user_preferences)
    let preferences = if let Some(prefs_repo) = &self.preferences_repository {
        prefs_repo.find_by_user_id(user_id).await.ok()
            .flatten()
            .map(|p| PreferencesExport {
                timer_is_public: p.timer_is_public,
                timer_show_in_list: p.timer_show_in_list,
            })
    } else {
        None
    };

    // 6. Existing data (timers, phrases, sessions, etc.)
    let incident_timers = if let Some(timer_repo) = &self.incident_timer_repository {
        timer_repo.find_by_user_id(user_id).await.unwrap_or_default()
            .into_iter()
            .map(|timer| IncidentTimerExportData {
                id: timer.id,
                reset_timestamp: timer.reset_timestamp,
                notes: timer.notes,
                created_at: timer.created_at,
                updated_at: timer.updated_at,
            })
            .collect()
    } else {
        vec![]
    };

    let phrase_suggestions = if let Some(phrase_repo) = &self.phrase_repository {
        phrase_repo.get_user_suggestions(user_id).await.unwrap_or_default()
            .into_iter()
            .map(|s| PhraseSuggestionExportData {
                id: s.id,
                user_id: s.user_id,
                phrase_text: s.phrase_text,
                status: s.status,
                admin_reason: s.admin_reason,
                created_at: s.created_at,
                updated_at: s.updated_at,
            })
            .collect()
    } else {
        vec![]
    };

    let phrase_exclusions = if let Some(phrase_repo) = &self.phrase_repository {
        phrase_repo.get_user_excluded_phrases(user_id).await.unwrap_or_default()
            .into_iter()
            .map(|(id, phrase_text, excluded_at)| PhraseExclusionExportData {
                id,
                phrase_text,
                excluded_at,
            })
            .collect()
    } else {
        vec![]
    };

    let active_sessions = self.refresh_token_repository
        .find_by_user_id(user_id).await.unwrap_or_default()
        .into_iter()
        .map(|token| SessionExportData {
            id: token.id,
            device_info: token.device_info,
            created_at: token.created_at,
            last_used_at: token.last_used_at,
            expires_at: token.expires_at,
        })
        .collect();

    let verification_history = if let Some(verification_repo) = &self.verification_token_repository {
        verification_repo.find_by_user_id(user_id).await.unwrap_or_default()
            .into_iter()
            .map(|token| VerificationTokenExportData {
                id: token.id,
                expires_at: token.expires_at,
                created_at: token.created_at,
            })
            .collect()
    } else {
        vec![]
    };

    // NEW: Password reset history (for transparency)
    let password_reset_history = if let Some(reset_repo) = &self.password_reset_token_repository {
        reset_repo.find_by_user_id(user_id).await.unwrap_or_default()
            .into_iter()
            .map(|token| PasswordResetExportData {
                id: token.id,
                expires_at: token.expires_at,
                used_at: token.used_at,
                created_at: token.created_at,
            })
            .collect()
    } else {
        vec![]
    };

    Ok(UserDataExport {
        export_version: "2.0".to_string(),  // ⚠️ INCREMENT VERSION
        export_date: Utc::now(),
        user: UserExportData {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            slug: user.slug,
            active: user.active,
            email_verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
            roles,
        },
        authentication: AuthenticationExport {  // NEW
            has_password,
            password_last_changed,
        },
        external_logins,  // NEW
        profile,          // NEW
        preferences,      // NEW
        incident_timers,
        phrase_suggestions,
        phrase_exclusions,
        active_sessions,
        verification_history,
        password_reset_history,  // NEW
    })
}
```

---

## Update Export Data Structures

In `backend/src/models/api/data_export.rs`:

```rust
#[derive(Debug, Serialize)]
pub struct AuthenticationExport {
    pub has_password: bool,
    pub password_last_changed: Option<DateTime<Utc>>,
    // NOTE: password_hash is NOT included (security)
}

#[derive(Debug, Serialize)]
pub struct ExternalLoginExport {
    pub provider: String,
    pub provider_user_id: String,  // Their Google/GitHub ID
    pub linked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProfileExport {
    pub real_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PreferencesExport {
    pub timer_is_public: bool,
    pub timer_show_in_list: bool,
    // Future: notification settings, theme, etc.
}

#[derive(Debug, Serialize)]
pub struct PasswordResetExportData {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserDataExport {
    pub export_version: String,  // "2.0"
    pub export_date: DateTime<Utc>,
    pub user: UserExportData,
    pub authentication: AuthenticationExport,      // NEW
    pub external_logins: Vec<ExternalLoginExport>, // NEW
    pub profile: Option<ProfileExport>,            // NEW
    pub preferences: Option<PreferencesExport>,    // NEW
    pub incident_timers: Vec<IncidentTimerExportData>,
    pub phrase_suggestions: Vec<PhraseSuggestionExportData>,
    pub phrase_exclusions: Vec<PhraseExclusionExportData>,
    pub active_sessions: Vec<SessionExportData>,
    pub verification_history: Vec<VerificationTokenExportData>,
    pub password_reset_history: Vec<PasswordResetExportData>,  // NEW
}
```

---

## Comprehensive Export Tests

### Test 1: Export includes all new tables

```rust
#[tokio::test]
async fn test_export_includes_all_tables() {
    let user_id = Uuid::new_v4();

    // Setup all mock repositories
    let mut user_repo = MockUserRepository::new();
    let mut creds_repo = MockUserCredentialsRepository::new();
    let mut ext_repo = MockUserExternalLoginRepository::new();
    let mut profile_repo = MockUserProfileRepository::new();
    let mut prefs_repo = MockUserPreferencesRepository::new();
    let mut refresh_repo = MockRefreshTokenRepository::new();

    // User data
    user_repo.expect_find_by_id()
        .returning(move |_| Ok(Some(test_user(user_id))));
    user_repo.expect_get_user_roles()
        .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

    // Credentials
    creds_repo.expect_has_password().returning(|_| Ok(true));
    creds_repo.expect_find_by_user_id()
        .returning(|_| Ok(Some(test_credentials())));

    // External logins
    ext_repo.expect_find_by_user_id()
        .returning(|_| Ok(vec![test_external_login()]));

    // Profile
    profile_repo.expect_find_by_user_id()
        .returning(|_| Ok(Some(test_profile())));

    // Preferences
    prefs_repo.expect_find_by_user_id()
        .returning(|_| Ok(Some(test_preferences())));

    // Sessions
    refresh_repo.expect_find_by_user_id()
        .returning(|_| Ok(vec![]));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .credentials_repository(Box::new(creds_repo))
        .external_login_repository(Box::new(ext_repo))
        .profile_repository(Box::new(profile_repo))
        .preferences_repository(Box::new(prefs_repo))
        .refresh_token_repository(Box::new(refresh_repo))
        .build();

    let export = auth_service.export_user_data(user_id).await.unwrap();

    // ⚠️ CRITICAL ASSERTIONS
    assert_eq!(export.export_version, "2.0");
    assert!(export.authentication.has_password);
    assert!(export.authentication.password_last_changed.is_some());
    assert_eq!(export.external_logins.len(), 1);
    assert!(export.profile.is_some());
    assert!(export.preferences.is_some());
}
```

### Test 2: OAuth-only user export

```rust
#[tokio::test]
async fn test_export_oauth_only_user() {
    let user_id = Uuid::new_v4();

    let mut user_repo = MockUserRepository::new();
    let mut creds_repo = MockUserCredentialsRepository::new();
    let mut ext_repo = MockUserExternalLoginRepository::new();
    let mut refresh_repo = MockRefreshTokenRepository::new();

    user_repo.expect_find_by_id()
        .returning(move |_| Ok(Some(test_user(user_id))));
    user_repo.expect_get_user_roles()
        .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

    // NO credentials (OAuth-only)
    creds_repo.expect_has_password().returning(|_| Ok(false));
    creds_repo.expect_find_by_user_id().returning(|_| Ok(None));

    // External login exists
    ext_repo.expect_find_by_user_id()
        .returning(|_| Ok(vec![test_external_login()]));

    refresh_repo.expect_find_by_user_id()
        .returning(|_| Ok(vec![]));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .credentials_repository(Box::new(creds_repo))
        .external_login_repository(Box::new(ext_repo))
        .refresh_token_repository(Box::new(refresh_repo))
        .build();

    let export = auth_service.export_user_data(user_id).await.unwrap();

    assert_eq!(export.authentication.has_password, false);
    assert!(export.authentication.password_last_changed.is_none());
    assert_eq!(export.external_logins.len(), 1);
    assert_eq!(export.external_logins[0].provider, "google");
}
```

### Test 3: User with multiple OAuth providers

```rust
#[tokio::test]
async fn test_export_multiple_oauth_providers() {
    let user_id = Uuid::new_v4();

    let mut user_repo = MockUserRepository::new();
    let mut ext_repo = MockUserExternalLoginRepository::new();
    let mut refresh_repo = MockRefreshTokenRepository::new();

    user_repo.expect_find_by_id()
        .returning(move |_| Ok(Some(test_user(user_id))));
    user_repo.expect_get_user_roles()
        .returning(|_| Ok(vec!["user".to_string()]));

    // Multiple external logins
    ext_repo.expect_find_by_user_id()
        .returning(|_| Ok(vec![
            test_external_login_google(),
            test_external_login_github(),
        ]));

    refresh_repo.expect_find_by_user_id()
        .returning(|_| Ok(vec![]));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .external_login_repository(Box::new(ext_repo))
        .refresh_token_repository(Box::new(refresh_repo))
        .build();

    let export = auth_service.export_user_data(user_id).await.unwrap();

    assert_eq!(export.external_logins.len(), 2);

    let providers: Vec<String> = export.external_logins
        .iter()
        .map(|l| l.provider.clone())
        .collect();
    assert!(providers.contains(&"google".to_string()));
    assert!(providers.contains(&"github".to_string()));
}
```

### Test 4: Export format is valid JSON

```rust
#[tokio::test]
async fn test_export_valid_json() {
    let user_id = Uuid::new_v4();

    // ... setup mocks ...

    let export = auth_service.export_user_data(user_id).await.unwrap();

    // Verify serialization to JSON works
    let json = serde_json::to_string_pretty(&export).unwrap();
    assert!(json.contains("export_version"));
    assert!(json.contains("2.0"));
    assert!(json.contains("authentication"));
    assert!(json.contains("external_logins"));
    assert!(json.contains("profile"));
    assert!(json.contains("preferences"));

    // Verify can be deserialized
    let _parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
}
```

---

## Manual Verification Checklist

**CRITICAL**: Before marking this phase complete, manually verify export:

1. **Create test user with complete data**:
   ```sql
   -- In development database
   INSERT INTO users (email, display_name, slug) VALUES ('export-test@example.com', 'Export Test', 'export-test') RETURNING id;
   -- Use returned ID in following commands

   INSERT INTO user_credentials (user_id, password_hash) VALUES ('USER_ID', '$2b$12$test');
   INSERT INTO user_external_logins (user_id, provider, provider_user_id) VALUES ('USER_ID', 'google', 'google_123');
   INSERT INTO user_profiles (user_id, real_name, bio) VALUES ('USER_ID', 'Test User', 'Test bio');
   INSERT INTO user_preferences (user_id, timer_is_public) VALUES ('USER_ID', true);
   ```

2. **Request data export** via API or service call

3. **Verify JSON includes**:
   - ✅ export_version: "2.0"
   - ✅ authentication.has_password
   - ✅ authentication.password_last_changed
   - ✅ external_logins array with Google entry
   - ✅ profile object with real_name and bio
   - ✅ preferences object with timer_is_public
   - ✅ All existing tables (timers, phrases, sessions)

4. **Verify password_hash is NOT included** (security)

5. **Test OAuth-only user** (no credentials) exports correctly

6. **Test user with multiple providers** exports all providers

---

## Deliverables

1. **Updated data_export.rs**: Queries all new tables
2. **Updated export models**: New export structures
3. **Export version**: Incremented to "2.0"
4. **Comprehensive tests**: All export scenarios
5. **Manual verification**: Confirmed with real data

---

## Success Criteria

**⚠️ BLOCKER - All must be checked before Phase 6**:

- [ ] export_user_data() queries user_credentials table
- [ ] export_user_data() queries user_external_logins table
- [ ] export_user_data() queries user_profiles table
- [ ] export_user_data() queries user_preferences table
- [ ] export_user_data() queries password_reset_tokens table
- [ ] Export version incremented to "2.0"
- [ ] AuthenticationExport includes has_password and password_last_changed
- [ ] ExternalLoginExport includes all linked providers
- [ ] ProfileExport includes all profile fields
- [ ] PreferencesExport includes all preference fields
- [ ] Password hash is NOT included in export (security)
- [ ] All export tests pass
- [ ] Manual verification with real user completed
- [ ] OAuth-only user export verified
- [ ] Multi-provider user export verified
- [ ] Full test suite still passes (227 tests)

**Time Check**: 2-3 hours

---

## Common Issues

### Issue: Export missing new tables
**Cause**: Forgot to query a repository
**Solution**: Check all new repositories are queried

### Issue: Export includes password_hash
**Cause**: Accidentally exported sensitive data
**Solution**: NEVER export password_hash, only has_password flag

### Issue: Export version not updated
**Cause**: Forgot to increment
**Solution**: Change "1.0" to "2.0"

### Issue: Optional repositories cause errors
**Cause**: Using .unwrap() on None
**Solution**: Use if let Some(repo) = &self.repo pattern

---

## Next Steps

**ONLY proceed after manual verification**:

```bash
git add backend/src/services/auth/auth_service/data_export.rs
git add backend/src/models/api/data_export.rs
git commit -m "feat(services): update data export for multi-table schema (Phase 5 - CRITICAL)

GDPR/CCPA COMPLIANCE UPDATE:
- Export version 2.0
- Include user_credentials (has_password flag, password_last_changed)
- Include user_external_logins (all linked OAuth providers)
- Include user_profiles (complete profile data)
- Include user_preferences (all user settings)
- Include password_reset_tokens (for transparency)
- Add comprehensive export tests
- Manual verification completed
"
```

**Next Phase**: [Phase 6: Test Helpers & Builder Pattern](PHASE-06-TEST-HELPERS.md)
