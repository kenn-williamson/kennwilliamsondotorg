# Security Implementation

## Overview
Security architecture decisions covering authentication, authorization, data protection, and infrastructure security.

## Authentication Decisions

### JWT + Refresh Token Hybrid
**Decision**: Short-lived JWT with long-lived refresh tokens

**Why:**
- JWT: Stateless, no database lookup per request
- Refresh: Revocable, database-backed
- Best of both: Performance + security

**Configuration:**
- Access token: 1 hour (short window if compromised)
- Refresh token: 1 week rolling, 6 month hard limit

**Alternative rejected:**
- JWT only: Can't revoke without database
- Sessions only: Database lookup every request

### Refresh Token Rotation
**Decision**: Generate new refresh token on every refresh

**Why:**
- Limits attack window
- Detects token theft (old token used)
- Industry best practice

**Trade-offs:**
- More database writes
- Worth it: Better security

### Password Hashing
**Decision**: bcrypt with cost factor 12

**Why:**
- Slow by design (brute force resistant)
- Adaptive cost (can increase over time)
- Battle-tested

**Alternatives considered:**
- **Argon2**: Better, but less mature in Rust ecosystem
- **PBKDF2**: Less resistant to GPU attacks
- **bcrypt**: Good enough, widely supported

**Trade-offs:**
- Slower login (intentional)
- Worth it: Security over speed

### OAuth with PKCE
**Decision**: Authorization Code flow with PKCE

**Why:**
- PKCE prevents authorization code interception
- Required for public clients (SPAs)
- More secure than implicit flow

**Flow:**
1. Generate code verifier + challenge
2. Store state in Redis (10min expiry)
3. User authenticates with provider
4. Validate state, exchange code for token
5. Create/link user account

**Trade-offs:**
- More complex than implicit flow
- Worth it: Industry standard for SPAs

### Email Verification
**Decision**: Token-based verification required for features

**Why:**
- Prevents spam accounts
- Confirms user identity
- Required for trusted operations

**Token strategy:**
- Secure random tokens
- 24-hour expiration
- Single-use

**Trade-offs:**
- Friction in signup
- Worth it: Quality users over quantity

## Authorization Decisions

### Role-Based Access Control (RBAC)
**Decision**: 4-tier role system

**Roles:**
1. `user` - Base role (immutable, auto-assigned)
2. `email-verified` - Verified identity (manageable)
3. `trusted-contact` - Personal content access (manageable)
4. `admin` - Full system access (manageable)

**Why RBAC:**
- Clear permission boundaries
- Easy to add roles
- Standard pattern
- Query-friendly

**Why immutable base role:**
- Can't lock out users accidentally
- Base permissions always present
- Admin can revoke additional roles

**Alternative rejected:**
- Permission-based: Too granular for current needs
- Single admin flag: Not flexible enough

### Protected Resources
**Decision**: Users can only access their own data

**Why:**
- Privacy by default
- Clear ownership model
- Simple to verify

**Pattern:**
- Extract user ID from JWT
- Filter queries by user_id
- Reject if mismatch

## Data Protection Decisions

### Input Validation
**Decision**: Validate on both frontend and backend

**Why:**
- Frontend: User experience (instant feedback)
- Backend: Security (never trust client)
- Defense in depth

**Implementation:**
- Frontend: VeeValidate + Yup
- Backend: Serde validation

### SQL Injection Prevention
**Decision**: Parameterized queries only via SQLx

**Why:**
- Compile-time verification
- Impossible to forget
- No string concatenation

**Alternative rejected:**
- String building: Error-prone
- ORM only: Still need raw SQL sometimes

### XSS Prevention
**Decision**: Framework defaults + sanitization

**Why:**
- Vue auto-escapes by default
- Explicit v-html only where needed
- Backend validates input

## Infrastructure Security Decisions

### SSL/TLS Enforcement
**Decision**: HTTPS required, HTTP redirects

**Why:**
- Encrypt all traffic
- Prevent downgrade attacks
- Required for modern APIs

**Certificates:**
- Development: Self-signed
- Production: Let's Encrypt (free, automated)

### Security Headers
**Decision**: Defense-in-depth headers via nginx

**Headers:**
- HSTS: Force HTTPS
- X-Frame-Options: Prevent clickjacking
- X-Content-Type-Options: Prevent MIME sniffing
- X-XSS-Protection: Browser protection
- Referrer-Policy: Limit referer leakage

**Why:**
- Minimal performance cost
- Defense in depth
- Industry best practices

### Container Security
**Decision**: Non-root users, minimal images

**Why:**
- Limit blast radius
- Smaller attack surface
- Security by default

**Pattern:**
- Alpine Linux base (minimal)
- Non-root user execution
- Internal network only

## Session Security Decisions

### JWT Storage
**Decision**: In-memory only (Nuxt session), never localStorage

**Why:**
- localStorage vulnerable to XSS
- Memory cleared on tab close
- httpOnly cookies for refresh tokens

**Trade-offs:**
- Lost on page refresh (re-fetch from session)
- Worth it: Better security

### CSRF Protection
**Decision**: Origin validation + SameSite cookies

**Why:**
- Prevents cross-site requests
- Standard browser protection
- No custom tokens needed

### Automatic Cleanup
**Decision**: Background job removes expired tokens

**Why:**
- Prevents database bloat
- Automatic maintenance
- No manual intervention

**Pattern:**
- Run every 24 hours
- Delete expired refresh + verification tokens
- Logs cleanup results

## API Security Decisions

### Endpoint Protection Levels
**Decision**: Three-tier access control

**Tiers:**
1. **Public**: No auth (health, login, register, public data)
2. **Protected**: JWT required (user resources)
3. **Admin**: Admin role required (management)

**Why:**
- Clear security boundaries
- Self-documenting
- Middleware enforcement

### Error Handling
**Decision**: Generic error messages to clients

**Why:**
- No information leakage
- Log details server-side
- Consistent format

**Pattern:**
- Client: "Authentication failed"
- Server log: "User not found: email@example.com"

## Security Testing

### What We Test
**Decision**: Test boundaries, not internals

**Areas:**
- Invalid JWT handling
- Expired token rejection
- Permission boundaries
- Input validation
- SQL injection attempts (via parameterized query tests)

**Why:**
- Catch security regressions
- Verify protections work
- Document security contracts

## Security Trade-Offs

### Usability vs Security
**Decisions:**
- Email verification required (security wins)
- 1-hour JWT expiry (security wins)
- Automatic refresh (usability wins)
- Remember me via refresh token (balanced)

**Philosophy:**
- Security by default
- Usability where safe
- Clear trade-offs

### Performance vs Security
**Decisions:**
- bcrypt cost 12 (security wins)
- JWT signature validation every request (security wins)
- Connection pooling (balanced)

**Philosophy:**
- Security over raw performance
- Optimize elsewhere

## Future Security Enhancements

**When to add:**
- **2FA**: When user base grows
- **Rate limiting**: Already implemented
- **Audit logging**: When compliance requires
- **WAF**: When attack patterns emerge

**Current approach sufficient for:**
- Personal project scale
- Known threat model
- Single developer oversight
