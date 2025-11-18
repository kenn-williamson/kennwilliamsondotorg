# Bot Protection for Registration Endpoint - Design Document

## Problem Statement

**Current Issue**: Bot spam is creating pending user accounts with scraped email addresses, resulting in:
- Wasted AWS SES email sends (cost and reputation impact)
- Database bloat with fake user records
- Potential email suppression list pollution if emails bounce

**Not a Security Issue**: Bots cannot access accounts (email verification required, no token issued until verified), but they waste resources and create operational overhead.

**Current Protection**: Only IP-based rate limiting (3 registrations/hour per IP), which can be bypassed with proxy rotation.

---

## Solution Overview

Implement **defense-in-depth bot protection** with three layers:

1. **Primary Defense**: Cloudflare Turnstile (invisible CAPTCHA)
2. **Secondary Defense**: Honeypot field validation
3. **Adjusted Rate Limiting**: Increase limits from 3/hour to 10/hour (CAPTCHA provides primary protection)

**Design Principle**: Keep existing registration flow intact, add validation layers before user creation.

---

## Architecture Decisions

### Decision 1: Cloudflare Turnstile (Invisible Mode)

**Decision**: Use Cloudflare Turnstile as primary bot protection mechanism.

**Why**:
- **Free Tier**: Unlimited requests for non-commercial use
- **Privacy-Friendly**: No user tracking, GDPR/CCPA compliant out-of-the-box
- **Invisible Mode**: Zero user interaction in most cases (only shows challenge for suspicious requests)
- **No Google Dependency**: Independent infrastructure (vs. reCAPTCHA)
- **Performance**: CDN-backed, minimal latency impact
- **Easy Integration**: Simple JavaScript widget + backend verification API

**Alternatives Considered**:
- **Google reCAPTCHA v3**: Rejected due to Google dependency and privacy concerns
- **hCaptcha**: Good alternative, but Turnstile has better performance and simpler pricing
- **Backend-only fingerprinting**: Rejected - too easy to bypass without client-side challenge

**Trade-offs**:
- Adds external dependency (Cloudflare API)
- Requires JavaScript enabled in browser (acceptable for this use case)
- Small latency increase (~50-200ms for token generation + verification)

---

### Decision 2: Honeypot Field (Secondary Defense)

**Decision**: Add hidden form field that legitimate users won't fill, bots will.

**Why**:
- **Zero User Impact**: Invisible to humans, catches dumb bots
- **No External Dependency**: Pure frontend + backend logic
- **Defense in Depth**: Catches bots that bypass CAPTCHA via token reuse
- **Minimal Implementation Cost**: ~10 lines of code frontend + backend

**Alternatives Considered**:
- **Timing Analysis**: Rejected - legitimate users have widely varying form fill times
- **Mouse Movement Tracking**: Rejected - too complex, accessibility concerns
- **Disposable Email Blocking**: Rejected per user preference (too restrictive)

**Trade-offs**:
- Only catches unsophisticated bots (most current spam is from dumb scrapers)
- Can be bypassed by analyzing HTML/CSS (but combined with Turnstile, unlikely)

---

### Decision 3: Frontend + Backend Validation (Defense in Depth)

**Decision**: Generate CAPTCHA token in browser, validate in Rust backend before user creation.

**Why**:
- **Security**: Backend validation prevents token replay attacks
- **Fail-Safe**: If frontend bypassed, backend still validates
- **Auditability**: Backend logging of failed validations for monitoring
- **Consistent with Architecture**: Matches existing 3-layer pattern (API → Service → Repository)

**Alternatives Considered**:
- **Backend-only validation**: Rejected - can't generate CAPTCHA token without frontend
- **Frontend-only validation**: Rejected - trivially bypassable

**Trade-offs**:
- Requires changes in both codebases (frontend + backend)
- Additional backend HTTP request to Cloudflare verification API (~100ms latency)

---

### Decision 4: Relax Rate Limits to 10/hour

**Decision**: Increase registration rate limit from 3/hour to 10/hour per IP.

**Why**:
- **Better UX**: Current 3/hour limit is restrictive for shared IPs (offices, cafes, college dorms)
- **CAPTCHA Protection**: Turnstile provides primary bot defense, rate limiting becomes secondary
- **Reduced Support Burden**: Fewer legitimate users hitting rate limits
- **Competitive Parity**: Most sites with CAPTCHA allow 10-20 registrations/hour per IP

**Alternatives Considered**:
- **Remove rate limiting entirely**: Rejected - defense in depth requires multiple layers
- **Keep 3/hour**: Rejected - too restrictive with CAPTCHA in place
- **Increase to 20/hour**: Rejected - still want some protection against distributed bot attacks

**Trade-offs**:
- Slightly higher risk of distributed bot attacks (mitigated by CAPTCHA)
- Minimal impact on AWS t3.small resource usage (Redis counters are lightweight)

---

## Implementation Design

### Frontend Changes

#### File: `/frontend/app/pages/register.vue`

**Changes**:
1. Add Cloudflare Turnstile widget component (invisible, auto-render)
2. Add hidden honeypot field with CSS to hide from users
3. Generate CAPTCHA token before form submission
4. Pass `captcha_token` and `honeypot` in request body
5. Handle CAPTCHA-specific errors (token expired, verification failed)

**Implementation Pattern**:
```vue
<template>
  <div>
    <!-- Existing form fields -->
    <input v-model="email" type="email" />
    <input v-model="displayName" type="text" />
    <input v-model="password" type="password" />

    <!-- NEW: Honeypot field (hidden with CSS) -->
    <input
      v-model="honeypot"
      type="text"
      name="website"
      autocomplete="off"
      tabindex="-1"
      aria-hidden="true"
      class="honeypot-field"
    />

    <!-- NEW: Turnstile widget (invisible) -->
    <div id="turnstile-widget"></div>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'

const honeypot = ref('')
let turnstile = null

onMounted(() => {
  // Load Cloudflare Turnstile SDK
  turnstile = window.turnstile.render('#turnstile-widget', {
    sitekey: 'YOUR_SITE_KEY',
    theme: 'light',
    size: 'invisible',
  })
})

async function handleSubmit(values) {
  // Generate CAPTCHA token
  const captchaToken = await turnstile.getResponse()

  // Submit registration
  await register({
    email: values.email,
    display_name: values.displayName,
    password: values.password,
    captcha_token: captchaToken,
    honeypot: honeypot.value,
  })
}
</script>

<style scoped>
.honeypot-field {
  position: absolute;
  left: -9999px;
  width: 1px;
  height: 1px;
  opacity: 0;
  pointer-events: none;
}
</style>
```

**Integration Points**:
- Line 226-256 in `register.vue`: Add token generation before `register()` call
- Add Turnstile SDK script tag in `nuxt.config.ts` or layout
- Add honeypot field validation schema (optional - backend validates)

---

#### File: `/frontend/shared/schemas/auth.ts`

**Changes** (Optional):
1. Add `captcha_token` to `registerSchema` (optional field)
2. Add `honeypot` to schema (must be empty string)

**Rationale**: Frontend validation is optional since backend performs actual verification.

---

#### File: `/frontend/nuxt.config.ts`

**Changes**:
1. Add Cloudflare Turnstile SDK script to head

```typescript
export default defineNuxtConfig({
  app: {
    head: {
      script: [
        {
          src: 'https://challenges.cloudflare.com/turnstile/v0/api.js',
          async: true,
          defer: true,
        },
      ],
    },
  },
})
```

---

### Backend Changes

#### File: `/backend/src/models/api/user.rs`

**Changes**:
1. Add `captcha_token` field to `CreateUserRequest` struct
2. Add `honeypot` field to `CreateUserRequest` struct

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(length(min = 2, max = 50))]
    pub display_name: String,

    // NEW: Bot protection fields
    pub captcha_token: Option<String>,  // Optional for backward compatibility during rollout
    pub honeypot: Option<String>,       // Must be empty/None for legitimate users
}
```

---

#### File: `/backend/src/routes/auth.rs`

**Changes**:
1. Validate honeypot field (must be empty)
2. Validate Turnstile token via Cloudflare API
3. Return `400 Bad Request` if validation fails
4. Log failed attempts for monitoring

```rust
pub async fn register(
    data: web::Json<CreateUserRequest>,
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
    turnstile_service: web::Data<TurnstileVerificationService>, // NEW
) -> ActixResult<HttpResponse> {
    // NEW: Honeypot validation (catches dumb bots)
    if let Some(honeypot) = &data.honeypot {
        if !honeypot.is_empty() {
            warn!("Honeypot triggered for email: {}", data.email);
            return Ok(HttpResponse::BadRequest().json(json!({
                "error": "Invalid request"
            })));
        }
    }

    // NEW: Turnstile token validation (catches smart bots)
    if let Some(token) = &data.captcha_token {
        let device_info = extract_device_info(&req);
        let verification_result = turnstile_service
            .verify_token(token, &device_info.ip_address)
            .await;

        match verification_result {
            Ok(false) => {
                warn!("Turnstile verification failed for IP: {}", device_info.ip_address);
                return Ok(HttpResponse::BadRequest().json(json!({
                    "error": "CAPTCHA verification failed. Please try again."
                })));
            }
            Err(e) => {
                error!("Turnstile API error: {}", e);
                // Fail open: allow registration if Cloudflare API is down
                // Alternative: fail closed (return error) for higher security
            }
            Ok(true) => {
                // Token valid, continue
            }
        }
    } else {
        // No token provided - reject during enforcement period
        // During rollout: log warning but allow (for backward compatibility)
        warn!("No CAPTCHA token provided for registration: {}", data.email);
    }

    // Existing flow continues...
    let device_info = extract_device_info(&req);
    let result = auth_service.register(&data.into_inner(), device_info).await;

    // ... rest of handler
}
```

---

#### File: `/backend/src/services/turnstile_verification_service.rs` (NEW)

**Purpose**: Encapsulate Cloudflare Turnstile API verification logic.

**Interface**:
```rust
pub struct TurnstileVerificationService {
    secret_key: String,
    http_client: reqwest::Client,
}

impl TurnstileVerificationService {
    pub fn new(secret_key: String) -> Self {
        Self {
            secret_key,
            http_client: reqwest::Client::new(),
        }
    }

    /// Verifies a Turnstile token with Cloudflare API
    ///
    /// Returns:
    /// - Ok(true): Token valid
    /// - Ok(false): Token invalid or expired
    /// - Err: Network/API error (fail open or closed based on policy)
    pub async fn verify_token(
        &self,
        token: &str,
        ip_address: &str,
    ) -> Result<bool, TurnstileError> {
        let response = self.http_client
            .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
            .json(&json!({
                "secret": self.secret_key,
                "response": token,
                "remoteip": ip_address,
            }))
            .send()
            .await?;

        let verification: TurnstileResponse = response.json().await?;

        Ok(verification.success)
    }
}

#[derive(Debug, Deserialize)]
struct TurnstileResponse {
    success: bool,
    #[serde(rename = "error-codes")]
    error_codes: Option<Vec<String>>,
    challenge_ts: Option<String>,
    hostname: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum TurnstileError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Invalid API response")]
    InvalidResponse,
}
```

**Testing Strategy**:
- Unit tests with mocked HTTP responses
- Integration tests with Cloudflare test secret keys
- Mock service for local development (always returns `true`)

---

#### File: `/backend/src/middleware/rate_limiter/config.rs`

**Changes**:
1. Update registration rate limit from 3/hour to 10/hour

```rust
// Before:
RateLimitConfig {
    requests_per_hour: 3,
    burst_limit: 1,
    burst_window: 300,
}

// After:
RateLimitConfig {
    requests_per_hour: 10,    // Increased from 3
    burst_limit: 2,           // Increased from 1
    burst_window: 300,        // Unchanged (5 minutes)
}
```

---

#### File: `/backend/Cargo.toml`

**Changes**:
1. Add `reqwest` dependency for HTTP client (if not already present)
2. Add `thiserror` for error handling (if not already present)

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
thiserror = "1.0"
```

---

### Configuration & Environment Variables

#### File: `/backend/.env.development`

**New Variables**:
```bash
# Cloudflare Turnstile Configuration
TURNSTILE_SECRET_KEY=1x0000000000000000000000000000000AA  # Test secret (always passes)
TURNSTILE_SITE_KEY=1x00000000000000000000AA              # Test site key
```

**Production** (`.env.production`):
```bash
# Get from Cloudflare dashboard after creating Turnstile site
TURNSTILE_SECRET_KEY=<production_secret_from_cloudflare>
TURNSTILE_SITE_KEY=<production_site_key_from_cloudflare>
```

#### File: `/frontend/.env`

**New Variables**:
```bash
# Cloudflare Turnstile Site Key (public, safe to expose)
NUXT_PUBLIC_TURNSTILE_SITE_KEY=1x00000000000000000000AA
```

---

## Request Flow (After Implementation)

```
┌─────────────────────────────────────────────────────────────┐
│ 1. USER SUBMITS REGISTRATION FORM                          │
│    - Cloudflare Turnstile generates token (invisible)      │
│    - Honeypot field remains empty (hidden from user)       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. CLIENT-SIDE VALIDATION (VeeValidate)                    │
│    - Email format, password strength, display name         │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. API REQUEST                                              │
│    POST /api/auth/register                                  │
│    {                                                        │
│      email, display_name, password,                        │
│      captcha_token: "turnstile_token_here",                │
│      honeypot: ""                                          │
│    }                                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. BACKEND: RATE LIMITER MIDDLEWARE                        │
│    - Check: < 10 registrations/hour per IP? ✓              │
│    - Check: < 2 registrations per 5-min burst? ✓           │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│ 5. BACKEND: BOT PROTECTION VALIDATION (NEW)                │
│    routes/auth.rs::register()                              │
│                                                             │
│    A. Honeypot Check:                                      │
│       if honeypot.is_some() && !honeypot.is_empty() {      │
│         return 400 Bad Request  // Bot detected            │
│       }                                                     │
│                                                             │
│    B. Turnstile Verification:                              │
│       POST https://challenges.cloudflare.com/turnstile/... │
│       {                                                     │
│         secret: TURNSTILE_SECRET_KEY,                      │
│         response: captcha_token,                           │
│         remoteip: user_ip                                  │
│       }                                                     │
│                                                             │
│       if !verification.success {                           │
│         return 400 Bad Request  // CAPTCHA failed          │
│       }                                                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│ 6. EXISTING REGISTRATION FLOW                              │
│    - Slug generation                                        │
│    - Password hashing                                       │
│    - Atomic user creation (5 tables)                       │
│    - Event publishing (email verification)                 │
│    - Token generation (JWT + refresh)                      │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│ 7. RESPONSE                                                 │
│    HTTP 201 Created                                         │
│    { token, refresh_token, user }                          │
└─────────────────────────────────────────────────────────────┘
```

---

## Testing Strategy

### Unit Tests

**File**: `/backend/src/services/turnstile_verification_service_test.rs`

**Tests**:
1. `test_verify_token_success()`: Valid token returns `Ok(true)`
2. `test_verify_token_failure()`: Invalid token returns `Ok(false)`
3. `test_verify_token_network_error()`: Network failure returns `Err`
4. `test_verify_token_malformed_response()`: Invalid JSON returns `Err`

**Mocking**: Use `mockito` or `wiremock` to mock Cloudflare API responses.

---

### Integration Tests

**File**: `/backend/tests/integration/auth_registration_bot_protection_test.rs`

**Tests**:
1. `test_registration_with_valid_captcha()`: Registration succeeds with valid token
2. `test_registration_with_invalid_captcha()`: Returns 400 with invalid token
3. `test_registration_with_honeypot_filled()`: Returns 400 when honeypot not empty
4. `test_registration_without_captcha()`: Returns 400 (after enforcement rollout)
5. `test_rate_limit_increased()`: 10 registrations succeed within 1 hour

**Test Environment**:
- Use Cloudflare test secret keys (`1x0000000000000000000000000000000AA`)
- Test keys always pass verification
- Honeypot tests don't require external API

---

### Frontend Tests

**File**: `/frontend/tests/pages/register.spec.ts` (if using Vitest/Playwright)

**Tests**:
1. `test_turnstile_widget_renders()`: Widget div present in DOM
2. `test_honeypot_field_hidden()`: Honeypot field has correct CSS classes
3. `test_captcha_token_included_in_request()`: Request payload includes token
4. `test_registration_error_handling()`: Shows user-friendly error on CAPTCHA failure

---

## Deployment Checklist

### Pre-Deployment

1. **Create Cloudflare Turnstile Site**:
   - Visit https://dash.cloudflare.com/?to=/:account/turnstile
   - Click "Add Site"
   - Configure:
     - Site name: "KennWilliamson.org Registration"
     - Domain: `kennwilliamson.org`
     - Widget mode: Invisible
   - Copy Site Key and Secret Key

2. **Update Environment Variables**:
   - Backend `.env.production`: Add `TURNSTILE_SECRET_KEY`
   - Frontend `.env.production`: Add `NUXT_PUBLIC_TURNSTILE_SITE_KEY`
   - AWS Parameter Store (if using): Store secrets securely

3. **Test in Staging**:
   - Deploy to staging environment
   - Test registration flow end-to-end
   - Verify Cloudflare dashboard shows verification requests
   - Test error handling (invalid token, honeypot filled)

4. **Update Rate Limit Config**:
   - Verify rate limit changes deployed (10/hour)
   - Monitor Redis for rate limit counters

---

### Deployment

1. **Deploy Backend**:
   - Deploy Rust backend with new validation logic
   - Verify `TurnstileVerificationService` initialized with secret key
   - Check logs for successful Turnstile API connections

2. **Deploy Frontend**:
   - Deploy Nuxt.js frontend with Turnstile widget
   - Verify Turnstile SDK loads in browser console
   - Check widget renders (inspect element for `#turnstile-widget`)

3. **Gradual Rollout** (Optional):
   - Phase 1: Log warnings for missing CAPTCHA tokens, but allow registration
   - Phase 2: After 1 week, enforce CAPTCHA requirement (reject if missing)
   - Monitor error rates and adjust

---

### Post-Deployment

1. **Monitor Cloudflare Dashboard**:
   - Track verification requests per day
   - Check success/failure rates
   - Identify suspicious patterns (high failure rates from specific IPs)

2. **Monitor Application Logs**:
   - Track honeypot trigger events (`warn!("Honeypot triggered...")`)
   - Track CAPTCHA failure events (`warn!("Turnstile verification failed...")`)
   - Set up alerts for anomalous bot activity

3. **Monitor Registration Metrics**:
   - Compare registration rates before/after deployment
   - Track legitimate user registration success rates
   - Monitor AWS SES email send volume (should decrease)

4. **Database Cleanup** (Optional):
   - Identify and purge existing bot-created accounts (unverified accounts older than 30 days)
   - Script: `DELETE FROM users WHERE email_verified = false AND created_at < NOW() - INTERVAL '30 days'`

---

## Rollback Plan

If bot protection causes issues:

1. **Immediate Rollback** (Backend):
   - Revert `routes/auth.rs` changes
   - Remove CAPTCHA token validation
   - Keep honeypot validation (minimal risk)

2. **Gradual Rollback** (Frontend):
   - Remove Turnstile widget rendering
   - Remove `captcha_token` from request payload
   - Keep honeypot field (no user impact)

3. **Revert Rate Limits**:
   - Change rate limit back to 3/hour if abuse continues

**Rollback Trigger**: Registration success rate drops below 95% (legitimate users blocked).

---

## Monitoring & Metrics

### Key Metrics to Track

1. **Bot Detection Rate**:
   - Honeypot triggers per day
   - CAPTCHA verification failures per day
   - Percentage of registration attempts blocked

2. **Legitimate User Impact**:
   - Registration success rate (target: >98%)
   - Time-to-register (CAPTCHA should add <500ms)
   - Rate limit rejections (should decrease with higher limits)

3. **Resource Impact**:
   - AWS SES email sends per day (should decrease)
   - Database growth rate (new users per day)
   - Cloudflare API latency (P95, P99)

4. **Security Events**:
   - Failed CAPTCHA attempts from single IP (distributed attack indicator)
   - Honeypot triggers with valid CAPTCHA tokens (sophisticated bot)

### Logging Examples

```rust
// Success
info!("Registration successful with CAPTCHA validation: email={}", data.email);

// Honeypot trigger
warn!("Honeypot triggered: email={}, ip={}", data.email, ip_address);

// CAPTCHA failure
warn!("Turnstile verification failed: ip={}, error_codes={:?}", ip_address, error_codes);

// Cloudflare API error
error!("Turnstile API request failed: {}, failing open", error);
```

---

## Cost Analysis

### Cloudflare Turnstile Pricing

- **Free Tier**: 1,000,000 verifications/month
- **Expected Usage**: ~100-500 registrations/month (well within free tier)
- **Cost**: $0/month

### AWS SES Savings

- **Current**: ~1,000 bot registrations/month × $0.10/1000 emails = **$0.10/month waste**
- **After**: ~50 bot registrations/month × $0.10/1000 emails = **$0.005/month**
- **Savings**: Minimal direct cost, but prevents reputation damage

### Developer Time

- **Implementation**: 6-8 hours (frontend + backend + testing)
- **Ongoing Maintenance**: <1 hour/month (monitoring, secret rotation)

---

## Future Enhancements

### Potential Improvements (Not in Scope)

1. **Advanced Bot Detection**:
   - Behavioral analysis (mouse movements, typing patterns)
   - Device fingerprinting (canvas fingerprinting, WebGL)
   - ML-based anomaly detection

2. **Adaptive Rate Limiting**:
   - Lower limits for IPs with failed CAPTCHA attempts
   - Higher limits for verified users (OAuth login history)
   - Geographic-based limits (higher risk regions)

3. **Disposable Email Detection** (Optional):
   - Maintain blocklist of known disposable domains
   - Warn users but allow registration (soft block)
   - Require additional verification for disposable emails

4. **CAPTCHA Analytics Dashboard**:
   - Real-time bot activity visualization
   - Geographic distribution of bot attempts
   - Integration with existing admin dashboard

---

## References

- **Cloudflare Turnstile Docs**: https://developers.cloudflare.com/turnstile/
- **Turnstile API Reference**: https://developers.cloudflare.com/turnstile/get-started/server-side-validation/
- **Honeypot Best Practices**: https://owasp.org/www-community/controls/Blocking_Brute_Force_Attacks
- **Rate Limiting Strategies**: https://www.cloudflare.com/learning/bots/what-is-rate-limiting/

---

## Appendix: File Changes Summary

### Frontend Changes
- `/frontend/app/pages/register.vue` - Add Turnstile widget + honeypot field
- `/frontend/nuxt.config.ts` - Add Turnstile SDK script
- `/frontend/.env` - Add `NUXT_PUBLIC_TURNSTILE_SITE_KEY`
- `/frontend/shared/schemas/auth.ts` - (Optional) Add captcha_token to schema

### Backend Changes
- `/backend/src/models/api/user.rs` - Add `captcha_token` and `honeypot` fields
- `/backend/src/routes/auth.rs` - Add validation logic
- `/backend/src/services/turnstile_verification_service.rs` - (NEW) Verification service
- `/backend/src/middleware/rate_limiter/config.rs` - Increase rate limits
- `/backend/Cargo.toml` - Add `reqwest` dependency
- `/backend/.env.development` - Add `TURNSTILE_SECRET_KEY`
- `/backend/.env.production` - Add production Turnstile secrets

### Test Files (NEW)
- `/backend/src/services/turnstile_verification_service_test.rs`
- `/backend/tests/integration/auth_registration_bot_protection_test.rs`
- `/frontend/tests/pages/register.spec.ts`

---

## Implementation Timeline

**Estimated Total Time**: 6-8 hours

1. **Backend Service Layer** (2 hours):
   - Create `TurnstileVerificationService`
   - Write unit tests with mocked HTTP responses
   - Add to dependency injection

2. **Backend Route Validation** (1 hour):
   - Update `CreateUserRequest` struct
   - Add honeypot + CAPTCHA validation in `register()` handler
   - Add error logging

3. **Frontend Integration** (2 hours):
   - Add Turnstile widget to registration page
   - Add honeypot field with CSS hiding
   - Update form submission to include tokens
   - Test token generation flow

4. **Integration Testing** (1.5 hours):
   - Write integration tests for bot protection flow
   - Test with Cloudflare test keys
   - Verify error handling

5. **Configuration & Deployment** (1.5 hours):
   - Create Cloudflare Turnstile site
   - Update environment variables
   - Deploy to staging and test end-to-end
   - Deploy to production

6. **Monitoring Setup** (1 hour):
   - Add logging statements
   - Set up CloudWatch alerts (if using AWS)
   - Document metrics to track

---

**Document Version**: 1.0
**Last Updated**: 2025-01-16
**Status**: Design Complete, Ready for Implementation
