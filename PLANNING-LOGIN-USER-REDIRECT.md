# Planning: Preserve User Redirect Through Google OAuth Flow

## Problem Statement

When a user tries to access a protected page (e.g., `/profile`) without being authenticated, they are redirected to `/login?redirect=/profile`. However, when they sign in with Google OAuth, they are always redirected to `/` instead of their intended destination.

**Current Flow:**
1. User tries to access `/profile` → redirected to `/login?redirect=/profile` ✅
2. User clicks "Sign in with Google" → OAuth flow begins ✅
3. Backend generates OAuth URL with state parameter (CSRF token)
4. Google redirects back to `/auth/google/callback` with code and state
5. Callback page completes auth → redirects to `/` hardcoded ❌
6. User loses their intended destination

**Root Cause:** The redirect parameter from the login page is not preserved through the Google OAuth flow.

## Solution Design

### Approach: Encode Redirect in OAuth State Parameter

The OAuth `state` parameter is designed for passing application state through the OAuth flow. We'll use it to encode both the CSRF token and the redirect URL.

**State Format:**
```
{csrf_token}|{base64_encoded_redirect}
```

**Why This Approach:**
- Follows OAuth 2.0 best practices
- Secure (state is validated by backend via Redis PKCE storage)
- Reliable (no dependency on browser storage or cookies)
- No new storage mechanisms needed

### Security Considerations

1. **Validate Redirect URLs:**
   - Must start with `/` (internal redirect only)
   - Must not start with `//` (prevents protocol-relative URLs)
   - Default to `/` if validation fails

2. **State Validation:**
   - Existing CSRF protection via Redis PKCE storage remains intact
   - State is validated during callback (checks Redis for PKCE verifier)

3. **Base64 Encoding:**
   - Prevents special characters from breaking URL encoding
   - Makes state parameter URL-safe

## Implementation Plan

### Phase 1: Backend Changes (Rust)

#### 1.1 Update OAuth URL Route (`backend/src/routes/auth.rs`)

**File:** `backend/src/routes/auth.rs`

**Changes:**
- Add query parameter extraction for optional `redirect` parameter
- Pass redirect to `auth_service.google_oauth_url()`

**Current:**
```rust
pub async fn google_oauth_url(
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.google_oauth_url().await {
        Ok((url, _csrf_token)) => {
            let response = crate::models::api::user::GoogleOAuthUrlResponse { url };
            Ok(HttpResponse::Ok().json(response))
        }
        // ...
    }
}
```

**Updated:**
```rust
use actix_web::web::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GoogleOAuthUrlQuery {
    redirect: Option<String>,
}

pub async fn google_oauth_url(
    query: Query<GoogleOAuthUrlQuery>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.google_oauth_url(query.redirect.clone()).await {
        Ok((url, _csrf_token)) => {
            let response = crate::models::api::user::GoogleOAuthUrlResponse { url };
            Ok(HttpResponse::Ok().json(response))
        }
        // ...
    }
}
```

#### 1.2 Update OAuth URL Service Method (`backend/src/services/auth/auth_service/oauth.rs`)

**File:** `backend/src/services/auth/auth_service/oauth.rs`

**Changes:**
- Modify `google_oauth_url()` signature to accept `Option<String>` redirect
- Encode redirect into state parameter before storing in Redis
- Use format: `{csrf_token}|{base64_redirect}`

**Current:**
```rust
pub async fn google_oauth_url(&self) -> Result<(String, CsrfToken)> {
    // ...
    let (auth_url, csrf_token, pkce_verifier) = oauth_service.get_authorization_url().await?;

    pkce_storage
        .store_pkce(csrf_token.secret(), pkce_verifier.secret(), 300)
        .await?;

    Ok((auth_url, csrf_token))
}
```

**Updated:**
```rust
pub async fn google_oauth_url(&self, redirect: Option<String>) -> Result<(String, CsrfToken)> {
    // ...
    let (auth_url, csrf_token, pkce_verifier) = oauth_service.get_authorization_url().await?;

    // Encode redirect into state parameter
    let state_with_redirect = if let Some(redirect_url) = redirect {
        let encoded_redirect = base64::encode(redirect_url.as_bytes());
        format!("{}|{}", csrf_token.secret(), encoded_redirect)
    } else {
        csrf_token.secret().to_string()
    };

    // Store enhanced state with PKCE verifier
    pkce_storage
        .store_pkce(&state_with_redirect, pkce_verifier.secret(), 300)
        .await?;

    // Note: The auth_url will need to use the enhanced state
    // This requires updating the OAuth service to accept custom state

    Ok((auth_url, csrf_token))
}
```

**Note:** This may require updating `GoogleOAuthService::get_authorization_url()` to accept a custom state parameter instead of generating its own. Alternative: Store the redirect separately in Redis with the csrf_token as key.

#### 1.3 Update OAuth Callback Service Method

**File:** `backend/src/services/auth/auth_service/oauth.rs`

**Changes:**
- Parse state parameter to extract redirect URL
- Return redirect in `AuthResponse`

**Current:**
```rust
pub async fn google_oauth_callback(
    &self,
    code: String,
    state: String,
) -> Result<AuthResponse> {
    // Retrieve PKCE verifier from storage using state parameter
    let verifier_secret = pkce_storage
        .retrieve_and_delete_pkce(&state)
        .await?
        .ok_or_else(|| anyhow!("Invalid or expired OAuth state"))?;

    // ... exchange code, fetch user info, create/login user ...

    Ok(AuthResponse {
        token,
        refresh_token: refresh_token_string,
        user: user_response,
    })
}
```

**Updated:**
```rust
pub async fn google_oauth_callback(
    &self,
    code: String,
    state: String,
) -> Result<AuthResponse> {
    // Parse state to extract redirect
    let (csrf_token, redirect_url) = parse_state_parameter(&state);

    // Retrieve PKCE verifier using original csrf_token
    let verifier_secret = pkce_storage
        .retrieve_and_delete_pkce(&csrf_token)
        .await?
        .ok_or_else(|| anyhow!("Invalid or expired OAuth state"))?;

    // ... exchange code, fetch user info, create/login user ...

    Ok(AuthResponse {
        token,
        refresh_token: refresh_token_string,
        user: user_response,
        redirect_url, // New field
    })
}

// Helper function
fn parse_state_parameter(state: &str) -> (String, Option<String>) {
    if let Some((csrf, encoded_redirect)) = state.split_once('|') {
        if let Ok(decoded) = base64::decode(encoded_redirect) {
            if let Ok(redirect) = String::from_utf8(decoded) {
                return (csrf.to_string(), Some(redirect));
            }
        }
    }
    (state.to_string(), None)
}
```

#### 1.4 Update AuthResponse Model

**File:** `backend/src/models/api/user.rs`

**Changes:**
- Add optional `redirect_url` field to `AuthResponse`

**Current:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}
```

**Updated:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: UserResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}
```

### Phase 2: Frontend Changes (TypeScript/Vue)

#### 2.1 Update Google OAuth Composable

**File:** `frontend/app/composables/useGoogleOAuth.ts`

**Changes:**
- Extract redirect from route query when initiating OAuth
- Pass redirect to backend

**Current:**
```typescript
const initiateOAuth = async (): Promise<void> => {
  return executeRequest(
    async () => {
      const response = await smartFetch<GoogleOAuthUrlResponse>(
        API_ROUTES.API.AUTH.GOOGLE_URL,
        { method: 'GET' }
      )
      window.location.href = response.url
    },
    'initiateOAuth'
  )
}
```

**Updated:**
```typescript
const initiateOAuth = async (): Promise<void> => {
  return executeRequest(
    async () => {
      // Get redirect from current route
      const route = useRoute()
      const redirect = route.query.redirect as string | undefined

      // Build URL with redirect parameter
      const url = redirect
        ? `${API_ROUTES.API.AUTH.GOOGLE_URL}?redirect=${encodeURIComponent(redirect)}`
        : API_ROUTES.API.AUTH.GOOGLE_URL

      const response = await smartFetch<GoogleOAuthUrlResponse>(
        url,
        { method: 'GET' }
      )

      window.location.href = response.url
    },
    'initiateOAuth'
  )
}
```

#### 2.2 Update Nuxt Server API Route for OAuth URL

**File:** `frontend/server/api/auth/google/url.get.ts`

**Changes:**
- Extract redirect from query parameters
- Forward to backend

**Current:**
```typescript
export default defineEventHandler(async (event) => {
  try {
    const config = useRuntimeConfig()

    const response = await $fetch<{ url: string }>(
      `${config.apiBase}${API_ROUTES.PUBLIC.AUTH.GOOGLE_URL}`,
      { method: 'GET' }
    )

    return response
  } catch (error: any) {
    // ...
  }
})
```

**Updated:**
```typescript
export default defineEventHandler(async (event) => {
  try {
    const config = useRuntimeConfig()
    const query = getQuery(event)
    const redirect = query.redirect as string | undefined

    // Build URL with redirect parameter
    const url = redirect
      ? `${config.apiBase}${API_ROUTES.PUBLIC.AUTH.GOOGLE_URL}?redirect=${encodeURIComponent(redirect)}`
      : `${config.apiBase}${API_ROUTES.PUBLIC.AUTH.GOOGLE_URL}`

    const response = await $fetch<{ url: string }>(url, { method: 'GET' })

    return response
  } catch (error: any) {
    // ...
  }
})
```

#### 2.3 Update Nuxt Server API Route for OAuth Callback

**File:** `frontend/server/api/auth/google/callback.post.ts`

**Changes:**
- Extract redirect from backend response
- Return it to frontend

**Current:**
```typescript
return {
  success: true,
  user: response.user
}
```

**Updated:**
```typescript
return {
  success: true,
  user: response.user,
  redirect_url: response.redirect_url
}
```

#### 2.4 Update OAuth Callback Page

**File:** `frontend/app/pages/auth/google/callback.vue`

**Changes:**
- Extract redirect from callback response
- Validate redirect URL for security
- Navigate to redirect or default to `/`

**Current:**
```typescript
try {
  await handleOAuthCallback(code, state)
  const { fetch: refreshSession } = useUserSession()
  await refreshSession()
  await router.push('/')
} catch (err) {
  // ...
}
```

**Updated:**
```typescript
try {
  const result = await handleOAuthCallback(code, state)
  const { fetch: refreshSession } = useUserSession()
  await refreshSession()

  // Get redirect from result and validate
  const redirectUrl = result?.redirect_url || '/'
  const isValidRedirect = redirectUrl.startsWith('/') && !redirectUrl.startsWith('//')
  const targetPath = isValidRedirect ? redirectUrl : '/'

  await router.push(targetPath)
} catch (err) {
  // ...
}
```

#### 2.5 Update useGoogleOAuth Composable Return Type

**File:** `frontend/app/composables/useGoogleOAuth.ts`

**Changes:**
- Update `handleOAuthCallback` to return the response data

**Current:**
```typescript
const handleOAuthCallback = async (code: string, state: string): Promise<void> => {
  return executeRequest(
    async () => {
      await smartFetch(
        API_ROUTES.API.AUTH.GOOGLE_CALLBACK,
        { method: 'POST', body: { code, state } }
      )
    },
    'handleOAuthCallback'
  )
}
```

**Updated:**
```typescript
interface OAuthCallbackResult {
  success: boolean
  user: any
  redirect_url?: string
}

const handleOAuthCallback = async (code: string, state: string): Promise<OAuthCallbackResult> => {
  return executeRequest(
    async () => {
      const result = await smartFetch<OAuthCallbackResult>(
        API_ROUTES.API.AUTH.GOOGLE_CALLBACK,
        { method: 'POST', body: { code, state } }
      )
      return result
    },
    'handleOAuthCallback'
  )
}
```

## Testing Plan

### Manual Testing

1. **Basic Redirect Test:**
   - Navigate to `/profile` (protected)
   - Should redirect to `/login?redirect=/profile`
   - Click "Sign in with Google"
   - Complete OAuth flow
   - Should redirect to `/profile`

2. **No Redirect Test:**
   - Navigate directly to `/login`
   - Click "Sign in with Google"
   - Complete OAuth flow
   - Should redirect to `/` (homepage)

3. **Invalid Redirect Test:**
   - Manually craft URL: `/login?redirect=//evil.com`
   - Click "Sign in with Google"
   - Complete OAuth flow
   - Should redirect to `/` (security validation)

4. **Special Characters Test:**
   - Navigate to `/profile?tab=security`
   - Should preserve query parameters through OAuth flow
   - Should redirect to `/profile?tab=security`

### Automated Testing

Backend tests to add (in `backend/src/services/auth/auth_service/oauth.rs`):

```rust
#[tokio::test]
async fn test_oauth_url_with_redirect() {
    // Test that redirect is encoded into state parameter
}

#[tokio::test]
async fn test_oauth_callback_extracts_redirect() {
    // Test that redirect is extracted from state and returned
}

#[tokio::test]
async fn test_oauth_state_without_redirect() {
    // Test backward compatibility when no redirect is provided
}
```

## Alternative Approaches Considered

### 1. Browser LocalStorage
**Pros:** Simple to implement
**Cons:**
- Not reliable (users may have storage disabled)
- Could be cleared between redirect steps
- Security concerns with persistent storage

### 2. Additional Cookie
**Pros:** Works across domains
**Cons:**
- Requires cookie management
- GDPR considerations
- Could be blocked by privacy settings

### 3. Separate Redis Entry
**Pros:** Clean separation
**Cons:**
- Additional Redis operations
- Complexity in cleanup
- State parameter is designed for this use case

**Selected Approach:** OAuth state parameter (best practice, secure, reliable)

## Rollout Strategy

1. Implement backend changes first
2. Test backend changes with manual API calls
3. Implement frontend changes
4. Test full flow in development
5. Deploy to production (low risk - backward compatible)

## Success Criteria

- [ ] Users redirected to intended destination after Google OAuth login
- [ ] No redirect specified → defaults to `/` (current behavior)
- [ ] Invalid redirects are rejected (security)
- [ ] Special characters in redirect URLs are handled correctly
- [ ] Backward compatibility maintained
- [ ] No breaking changes to existing OAuth flow

## Dependencies

**Backend:**
- `base64` crate (already in dependencies)

**Frontend:**
- No new dependencies required

## Estimated Effort

- Backend changes: 2-3 hours
- Frontend changes: 1-2 hours
- Testing: 1 hour
- Total: 4-6 hours

## Future Enhancements

1. Support for email/password login redirect (apply same pattern)
2. Remember last visited protected page (persistence across sessions)
3. Deep linking support for mobile apps
