# User Updates Refactor - Working Document

## Problem Statement

**Current Issues:**
1. **Session Goes Stale on Page Reload**: nuxt-auth-utils stores session data in encrypted cookie. On page reload, cookie is read but not refreshed from backend. User preferences in DB != preferences in session.
2. **Incomplete Pass-Through Route**: Existing `/server/api/auth/profile.put.ts` only saves 6 fields to session (id, email, display_name, slug, roles, created_at), losing preferences, profile, external_accounts data.

**Example:**
- User changes preference `timer_show_in_list` from `true` to `false`
- Backend updates database successfully ✓
- Session not updated immediately ✗
- Page reload reads stale cookie showing `true` ✗
- User sees incorrect preference state until session expires

## Root Causes

1. **nuxt-auth-utils behavior**: Session persists in encrypted cookie, doesn't auto-fetch from backend on reload
2. **Broken pass-through**: `/server/api/auth/profile.put.ts` lines 46-57 overwrites user object with partial data
3. **No preferences pass-through**: Preferences updates call backend directly, don't update session

## Architecture Solution

### Pass-Through Routes Pattern
Server routes that:
1. Receive request from client
2. Call backend API with JWT
3. Get full UserResponse from backend
4. Update session with `setUserSession(event, { user: response })`
5. Return success to client

**Benefits:**
- Single source of truth (backend DB)
- Immediate session updates
- Standard nuxt-auth-utils pattern (used in OAuth callback)

### Session Refresh Plugin
Plugin that runs on app initialization:
1. Check if user is logged in
2. Fetch current user data from `/backend/protected/auth/me`
3. Update session with fresh data
4. Ensures session always starts fresh on page load

**Benefits:**
- Catches admin changes (role updates, account status)
- Catches email verification changes
- Catches updates from other tabs/devices
- Safety net for any missed session updates

## Implementation Checklist

### Phase 1: Fix Existing Profile Pass-Through

**File:** `/home/kenn/projects/kennwilliamsondotorg/frontend/server/api/auth/profile.put.ts`

**Current (Lines 46-57):**
```typescript
await setUserSession(event, {
  ...session,
  user: {
    id: response.id,
    email: response.email,
    display_name: response.display_name,
    slug: response.slug,
    roles: response.roles,
    created_at: response.created_at
    // ❌ MISSING: email_verified, has_credentials, profile, external_accounts, preferences
  }
})
```

**Fix:**
- [ ] Verify backend returns full UserResponse (check type definition)
- [ ] Update `setUserSession()` to save complete user object
- [ ] Test profile update immediately reflects in session

**Backend Endpoint:** `PUT /backend/protected/auth/profile`
**Backend File:** `/home/kenn/projects/kennwilliamsondotorg/backend/src/routes/auth.rs:195-225`

### Phase 2: Create Preferences Pass-Through

**New File:** `/home/kenn/projects/kennwilliamsondotorg/frontend/server/api/user/preferences.put.ts`

**Implementation:**
```typescript
import { defineEventHandler, createError, readBody } from 'h3'
import { useRuntimeConfig } from '#imports'
import { requireValidJwtToken } from '../../utils/jwt-handler'
import { API_ROUTES } from '#shared/config/api-routes'
import type { User, UpdatePreferencesRequest } from '#shared/types'

export default defineEventHandler(async (event) => {
  try {
    const jwtToken = await requireValidJwtToken(event)
    const body = await readBody<UpdatePreferencesRequest>(event)
    const config = useRuntimeConfig()

    // Call backend preferences endpoint
    const user = await $fetch<User>(
      `${config.apiBase}${API_ROUTES.PROTECTED.AUTH.PREFERENCES}`,
      {
        method: 'PUT',
        headers: { 'Authorization': `Bearer ${jwtToken}` },
        body
      }
    )

    // Update session with full user object
    const session = await getUserSession(event)
    await setUserSession(event, {
      ...session,
      user
    })

    return { message: 'Preferences updated successfully' }
  } catch (error: any) {
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.data?.error || 'Failed to update preferences'
    })
  }
})
```

**Backend Endpoint:** `PUT /backend/protected/auth/preferences`
**Backend File:** `/home/kenn/projects/kennwilliamsondotorg/backend/src/routes/auth.rs:510-538`

**Tasks:**
- [ ] Create `/frontend/server/api/user/preferences.put.ts`
- [ ] Update `userPreferencesService` to call `/api/user/preferences` instead of `/backend/protected/auth/preferences`
- [ ] Remove `refreshSession()` call from `PreferencesForm.vue`
- [ ] Test preference toggle immediately updates session

### Phase 3: Verify Backend Returns Full UserResponse

**Files to Check:**
- `backend/src/routes/auth.rs` - `update_profile()` function (lines 195-225)
- `backend/src/routes/auth.rs` - `update_preferences()` function (lines 510-538)

**Verify:**
- [ ] `update_profile()` returns complete UserResponse (not just partial fields)
- [ ] `update_preferences()` returns complete UserResponse with updated preferences
- [ ] Both responses include: profile, preferences, external_accounts, email_verified, has_credentials

**Expected Response Structure:**
```rust
UserResponse {
    id, email, display_name, slug, roles, created_at,
    email_verified, has_credentials,
    profile: Option<ProfileData>,
    external_accounts: Vec<ExternalAccount>,
    preferences: Option<PreferencesData>
}
```

### Phase 4: Create Session Refresh Plugin

**New File:** `/home/kenn/projects/kennwilliamsondotorg/frontend/app/plugins/session-refresh.client.ts`

**Implementation:**
```typescript
export default defineNuxtPlugin(async () => {
  const { loggedIn, fetch: refreshSession } = useUserSession()

  // Only refresh if user is logged in
  if (loggedIn.value) {
    try {
      await refreshSession()
      console.log('✅ Session refreshed on app load')
    } catch (error) {
      console.error('❌ Failed to refresh session on load:', error)
      // Don't throw - let app continue with stale session
      // User will be prompted to log in again if session is invalid
    }
  }
})
```

**Tasks:**
- [ ] Create plugin file with `.client.ts` suffix (client-only)
- [ ] Test session refreshes on page reload
- [ ] Test works with SSR (plugin should only run client-side)
- [ ] Verify doesn't break initial auth flow (login/register/OAuth)

### Phase 5: Update Frontend Services

**File:** `/home/kenn/projects/kennwilliamsondotorg/frontend/app/services/userPreferencesService.ts`

**Current:**
```typescript
updatePreferences: async (data: UpdatePreferencesRequest): Promise<User> => {
  return fetcher<User>(API_ROUTES.PROTECTED.AUTH.PREFERENCES, {
    method: 'PUT',
    body: data
  })
}
```

**Update to:**
```typescript
updatePreferences: async (data: UpdatePreferencesRequest): Promise<{ message: string }> => {
  return fetcher<{ message: string }>('/api/user/preferences', {
    method: 'PUT',
    body: data
  })
}
```

**File:** `/home/kenn/projects/kennwilliamsondotorg/frontend/app/composables/useAuthProfileActions.ts`

**Current (Lines 31-45):**
```typescript
const updateProfile = async (data: ProfileUpdateRequest): Promise<{ message: string }> => {
  return executeRequestWithSuccess(
    async () => {
      const result = await updateProfileService(data)
      await refreshSession() // ← Remove this
      return result
    },
    'Profile updated successfully',
    'updateProfile'
  )
}
```

**Update to:**
```typescript
const updateProfile = async (data: ProfileUpdateRequest): Promise<{ message: string }> => {
  return executeRequestWithSuccess(
    () => updateProfileService(data), // Pass-through handles session update
    'Profile updated successfully',
    'updateProfile'
  )
}
```

**Tasks:**
- [ ] Update userPreferencesService to call `/api/user/preferences`
- [ ] Update service return type to `{ message: string }`
- [ ] Remove `refreshSession()` call from useAuthProfileActions
- [ ] Update incident-timers store `updateUserPreferences` action
- [ ] Update PreferencesForm to not call `refreshSession()`

### Phase 6: Testing

**Manual Tests:**
- [ ] Update profile (display_name/slug) - verify session updates immediately
- [ ] Toggle preference (timer_is_public) - verify session updates immediately
- [ ] Reload page after preference change - verify fresh data loaded from backend
- [ ] Open two tabs, change preference in one - verify other tab sees change after reload
- [ ] Login → verify session loads correctly
- [ ] Register → verify session loads correctly
- [ ] OAuth callback → verify session loads correctly

**Edge Cases:**
- [ ] Backend returns error - verify session not corrupted
- [ ] Network failure during preference update - verify session rollback/consistency
- [ ] Rapid preference toggles - verify no race conditions

## Backend Verification Requirements

### Profile Update Endpoint

**File:** `backend/src/routes/auth.rs:195-225`

**Current Return Type:** Need to verify

**Expected:**
```rust
pub async fn update_profile(
    req: HttpRequest,
    data: web::Json<ProfileUpdateRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    match auth_service.update_user_profile(user_id, data.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)), // ← Should return UserResponse
        Err(err) => { /* error handling */ }
    }
}
```

**Verify:**
- Returns full `UserResponse` (not partial)
- Includes all fields: profile, preferences, external_accounts

### Preferences Update Endpoint

**File:** `backend/src/routes/auth.rs:510-538`

**Current Implementation:** ✓ Already returns full UserResponse

```rust
pub async fn update_preferences(
    req: HttpRequest,
    data: web::Json<UpdatePreferencesRequest>,
    auth_service: web::Data<AuthService>,
) -> ActixResult<HttpResponse> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match auth_service
        .update_timer_privacy(user_id, data.timer_is_public, data.timer_show_in_list)
        .await
    {
        Ok(user) => Ok(HttpResponse::Ok().json(user)), // ✓ Returns full UserResponse
        Err(err) => { /* error handling */ }
    }
}
```

**Status:** ✅ Verified - already returns complete UserResponse

## Architecture Benefits

### Immediate Session Updates
- User actions immediately reflected in session
- No stale data between updates and reloads
- Consistent UX across all user-initiated changes

### Safety Net
- Session refresh plugin catches changes from:
  - Admin role updates
  - Email verification
  - Other browser tabs
  - Other devices
- Always starts with fresh data on app load

### Single Source of Truth
- Backend database is authoritative
- Session is always derived from backend state
- No client-side state management complexity

### Standard Pattern
- Follows nuxt-auth-utils best practices
- Same pattern as OAuth callback
- Easy to extend for future user update endpoints

## Future Considerations

### Set Password Endpoint
**Current:** Returns `{ message: string }`, changes `has_credentials` flag

**Options:**
1. Keep as-is, rely on session refresh plugin
2. Update to return UserResponse + create pass-through
3. Force re-login after setting password (creates new session)

**Recommendation:** Option 1 - session refresh plugin handles it on next reload

### Password Change Endpoint
**Current:** Returns `{ message: string }`, may revoke tokens

**Options:**
1. Keep as-is (no session-relevant data changes)
2. If revoking tokens on password change, force re-login

**Recommendation:** Option 1 - password changes don't affect session data

### Email Verification
**Current:** Separate verification flow

**Consideration:** Email verification changes `email_verified` flag and adds `email-verified` role. Session refresh plugin will catch this on next app load.

## Performance Considerations

### Session Refresh on Load
- **Cost:** One extra API call on app initialization
- **Benefit:** Always accurate session data
- **Optimization:** Could add timestamp check (only refresh if session older than X minutes)

**Recommendation:** Start simple (always refresh), optimize later if needed

### Pass-Through Routes
- **No additional cost:** Same number of API calls as before
- **Benefit:** Session updates happen server-side (more reliable)
- **Trade-off:** None - strictly better than current approach

## Status

**Completed:**
- ✅ Problem identification
- ✅ Architecture design
- ✅ Implementation plan

**Not Started:**
- ⏳ Backend verification (Phase 3)
- ⏳ Fix profile pass-through (Phase 1)
- ⏳ Create preferences pass-through (Phase 2)
- ⏳ Create session refresh plugin (Phase 4)
- ⏳ Update frontend services (Phase 5)
- ⏳ Testing (Phase 6)

**Blocked By:**
- None - ready to implement

## Notes

- This refactor aligns with backend's multi-table architecture (users, user_preferences, user_profiles, user_credentials, user_external_logins)
- Pass-through pattern already partially implemented in profile route - just needs completion
- Session refresh plugin is lightweight and follows Nuxt plugin best practices
- All changes are backwards compatible with existing API contracts
