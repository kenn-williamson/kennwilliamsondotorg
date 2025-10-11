# Phase 4C: OAuth Service Updates

**Estimated Time**: 2-3 hours
**Prerequisites**: [Phase 4B: Login & Authentication Service](PHASE-04B-LOGIN-SERVICE.md) complete
**Next Phase**: [Phase 4D: Profile Management Service](PHASE-04D-PROFILE-SERVICE.md)

## Objective

Update OAuth flow to use `user_external_logins` table instead of `google_user_id` column in users table. Enable multi-provider OAuth support.

**Key Principle**: One user can have multiple OAuth providers linked (Google, GitHub, Microsoft, etc.).

---

## TDD Approach

1. **Red**: Write tests for OAuth scenarios (new user, existing user, linking)
2. **Green**: Update OAuth callback to use external_login_repository
3. **Refactor**: Support account linking strategy

---

## Current OAuth Flow

From `backend/src/services/auth/auth_service/oauth.rs`:

```rust
// CURRENT: Find by google_user_id in users table
pub async fn google_oauth_callback(&self, code: String, state: String) -> Result<AuthResponse> {
    let google_user_info = /* fetch from Google */;

    // Find existing user by google_user_id
    let user = if let Some(existing_user) = self.user_repository
        .find_by_google_user_id(&google_user_info.sub)
        .await? {
        existing_user
    } else {
        // Create new OAuth user
        self.user_repository.create_oauth_user(&CreateOAuthUserData {
            email: google_user_info.email,
            display_name: google_user_info.name,
            slug: generate_slug(&google_user_info.email),
            real_name: Some(google_user_info.name),
            google_user_id: Some(google_user_info.sub),
        }).await?
    };

    // Generate tokens...
}
```

---

## New OAuth Flow

```rust
// NEW: Find by provider + provider_user_id in user_external_logins table
pub async fn google_oauth_callback(&self, code: String, state: String) -> Result<AuthResponse> {
    let google_user_info = /* fetch from Google */;

    // 1. Check if this OAuth account is already linked
    let existing_login = self.external_login_repository
        .find_by_provider("google", &google_user_info.sub)
        .await?;

    let user = if let Some(login) = existing_login {
        // Existing OAuth user - load their account
        self.user_repository
            .find_by_id(login.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?
    } else {
        // New OAuth login - check if email exists
        if let Some(existing_user) = self.user_repository
            .find_by_email(&google_user_info.email)
            .await? {

            // Email exists - link OAuth to existing account
            // (only if email is verified for security)
            let roles = self.user_repository.get_user_roles(existing_user.id).await?;
            if roles.contains(&"email-verified".to_string()) {
                // Link OAuth provider to existing verified account
                self.external_login_repository.create(CreateExternalLogin {
                    user_id: existing_user.id,
                    provider: "google".to_string(),
                    provider_user_id: google_user_info.sub,
                }).await?;

                // Update profile with OAuth data
                if let Some(profile_repo) = &self.profile_repository {
                    profile_repo.update(existing_user.id, UpdateProfile {
                        real_name: Some(google_user_info.name),
                        avatar_url: Some(google_user_info.picture),
                        ..Default::default()
                    }).await.ok(); // Ignore errors for optional profile update
                }

                existing_user
            } else {
                return Err(anyhow::anyhow!(
                    "An account with this email exists but is not verified. Please verify your email first."
                ));
            }
        } else {
            // No existing user - create new OAuth user
            let user = self.user_repository.create_user(&CreateUserData {
                email: google_user_info.email.clone(),
                password_hash: String::new(), // Temp: still required by old schema
                display_name: google_user_info.name.clone(),
                slug: generate_slug(&google_user_info.email),
            }).await?;

            // Create external login
            self.external_login_repository.create(CreateExternalLogin {
                user_id: user.id,
                provider: "google".to_string(),
                provider_user_id: google_user_info.sub,
            }).await?;

            // Create profile
            if let Some(profile_repo) = &self.profile_repository {
                profile_repo.create(user.id).await?;
                profile_repo.update(user.id, UpdateProfile {
                    real_name: Some(google_user_info.name),
                    avatar_url: Some(google_user_info.picture),
                    ..Default::default()
                }).await.ok();
            }

            // Create preferences
            if let Some(prefs_repo) = &self.preferences_repository {
                prefs_repo.create(user.id).await?;
            }

            // Assign email-verified role (OAuth emails are pre-verified)
            self.user_repository.add_role_to_user(user.id, "email-verified").await?;

            user
        }
    };

    // Generate tokens...
}
```

---

## Account Linking Strategy

### Decision Tree

```
OAuth Login Attempt
    │
    ├─ OAuth ID exists in external_logins?
    │   └─ YES → Login as that user
    │
    └─ NO → Email exists in users?
        ├─ YES → Email verified?
        │   ├─ YES → Link OAuth to existing account
        │   └─ NO → Reject (security: prevent account takeover)
        │
        └─ NO → Create new user + external login
```

### Security Considerations

**Why require email verification before linking?**
- Prevents account takeover
- Example attack: Attacker creates unverified account with victim's email, then links OAuth
- Solution: Only link to verified accounts

---

## Test Scenarios

### Test 1: New Google user (no existing account)

```rust
#[tokio::test]
async fn test_oauth_new_user() {
    let mut user_repo = MockUserRepository::new();
    let mut external_login_repo = MockUserExternalLoginRepository::new();
    let mut profile_repo = MockUserProfileRepository::new();
    let mut prefs_repo = MockUserPreferencesRepository::new();

    // No existing external login
    external_login_repo.expect_find_by_provider()
        .returning(|_, _| Ok(None));

    // No existing user with email
    user_repo.expect_find_by_email()
        .returning(|_| Ok(None));

    // User creation
    user_repo.expect_create_user()
        .returning(|_| Ok(test_user()));

    // External login creation
    external_login_repo.expect_create()
        .returning(|_| Ok(test_external_login()));

    // Profile and preferences creation
    profile_repo.expect_create().returning(|_| Ok(test_profile()));
    prefs_repo.expect_create().returning(|_| Ok(test_preferences()));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .external_login_repository(Box::new(external_login_repo))
        .profile_repository(Box::new(profile_repo))
        .preferences_repository(Box::new(prefs_repo))
        .build();

    let result = auth_service.google_oauth_callback(test_code(), test_state()).await;
    assert!(result.is_ok());
}
```

### Test 2: Existing Google user logs in again

```rust
#[tokio::test]
async fn test_oauth_existing_user_login() {
    let mut user_repo = MockUserRepository::new();
    let mut external_login_repo = MockUserExternalLoginRepository::new();

    let user_id = Uuid::new_v4();

    // Existing external login found
    external_login_repo.expect_find_by_provider()
        .returning(move |_, _| Ok(Some(test_external_login(user_id))));

    // Load user
    user_repo.expect_find_by_id()
        .returning(move |_| Ok(Some(test_user(user_id))));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .external_login_repository(Box::new(external_login_repo))
        .build();

    let result = auth_service.google_oauth_callback(test_code(), test_state()).await;
    assert!(result.is_ok());
}
```

### Test 3: Link Google to existing verified account

```rust
#[tokio::test]
async fn test_oauth_link_to_verified_account() {
    let mut user_repo = MockUserRepository::new();
    let mut external_login_repo = MockUserExternalLoginRepository::new();

    let user_id = Uuid::new_v4();

    // No existing external login
    external_login_repo.expect_find_by_provider()
        .returning(|_, _| Ok(None));

    // Existing user with same email (verified)
    user_repo.expect_find_by_email()
        .returning(move |_| Ok(Some(test_user(user_id))));

    user_repo.expect_get_user_roles()
        .returning(|_| Ok(vec!["user".to_string(), "email-verified".to_string()]));

    // Link OAuth to existing account
    external_login_repo.expect_create()
        .returning(|_| Ok(test_external_login()));

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .external_login_repository(Box::new(external_login_repo))
        .build();

    let result = auth_service.google_oauth_callback(test_code(), test_state()).await;
    assert!(result.is_ok());
}
```

### Test 4: Reject linking to unverified account

```rust
#[tokio::test]
async fn test_oauth_reject_link_to_unverified_account() {
    let mut user_repo = MockUserRepository::new();
    let mut external_login_repo = MockUserExternalLoginRepository::new();

    let user_id = Uuid::new_v4();

    // No existing external login
    external_login_repo.expect_find_by_provider()
        .returning(|_, _| Ok(None));

    // Existing user with same email (NOT verified)
    user_repo.expect_find_by_email()
        .returning(move |_| Ok(Some(test_user(user_id))));

    user_repo.expect_get_user_roles()
        .returning(|_| Ok(vec!["user".to_string()])); // No email-verified role

    let auth_service = AuthService::builder()
        .user_repository(Box::new(user_repo))
        .external_login_repository(Box::new(external_login_repo))
        .build();

    let result = auth_service.google_oauth_callback(test_code(), test_state()).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not verified"));
}
```

### Test 5: User with multiple providers (Google + GitHub)

```rust
#[tokio::test]
async fn test_oauth_user_with_multiple_providers() {
    // User already has Google linked, now linking GitHub
    let mut external_login_repo = MockUserExternalLoginRepository::new();

    let user_id = Uuid::new_v4();

    // Find all external logins for user
    external_login_repo.expect_find_by_user_id()
        .returning(move |_| Ok(vec![
            test_external_login_google(user_id),
            test_external_login_github(user_id),
        ]));

    // Both logins exist for same user
    let logins = external_login_repo.find_by_user_id(user_id).await.unwrap();
    assert_eq!(logins.len(), 2);

    let providers: Vec<String> = logins.iter().map(|l| l.provider.clone()).collect();
    assert!(providers.contains(&"google".to_string()));
    assert!(providers.contains(&"github".to_string()));
}
```

---

## Multi-Provider Support (Future)

With this architecture, adding new OAuth providers is straightforward:

```rust
// Future: GitHub OAuth
pub async fn github_oauth_callback(&self, code: String) -> Result<AuthResponse> {
    let github_user_info = /* fetch from GitHub */;

    let existing_login = self.external_login_repository
        .find_by_provider("github", &github_user_info.id)
        .await?;

    // Same logic as Google, different provider
}

// Future: Microsoft OAuth
pub async fn microsoft_oauth_callback(&self, code: String) -> Result<AuthResponse> {
    // Same pattern
}
```

---

## Deliverables

1. **Updated oauth.rs**: Uses external_logins table
2. **Account linking logic**: Links to verified accounts
3. **Multi-provider support**: Architecture supports any OAuth provider
4. **Profile updates**: OAuth data updates user profile
5. **Comprehensive tests**: All OAuth scenarios covered

---

## Success Criteria

**Before proceeding to Phase 4D**, verify:

- [ ] OAuth uses external_logins table
- [ ] New OAuth user creates user + external_login + profile + preferences
- [ ] Existing OAuth user login works
- [ ] Account linking works for verified accounts
- [ ] Account linking rejected for unverified accounts
- [ ] Profile updated with OAuth data (real_name, avatar)
- [ ] Multi-provider support verified
- [ ] All OAuth tests pass
- [ ] Full test suite still passes (227 tests)

**Time Check**: 2-3 hours

---

## Common Issues

### Issue: Account takeover via unverified email
**Cause**: Linking OAuth to unverified account
**Solution**: Check for email-verified role before linking

### Issue: Multiple external_logins for same provider
**Cause**: No UNIQUE constraint check
**Solution**: Database UNIQUE(provider, provider_user_id) prevents this

### Issue: Profile update fails and breaks OAuth flow
**Cause**: Profile update errors not handled
**Solution**: Use `.ok()` to ignore optional profile update errors

---

## Next Steps

```bash
git add backend/src/services/auth/auth_service/oauth.rs
git commit -m "feat(services): update OAuth to use external_logins table (Phase 4C)

- Use user_external_logins table instead of google_user_id column
- Implement account linking strategy (verified emails only)
- Support multi-provider OAuth architecture
- Update profile with OAuth data
- Add comprehensive OAuth scenario tests
"
```

**Next Phase**: [Phase 4D: Profile Management Service](PHASE-04D-PROFILE-SERVICE.md)
