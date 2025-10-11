# User Authentication Schema Refactor Design Document

## Executive Summary

### Purpose
This document proposes a comprehensive refactoring of the user authentication and profile system from a monolithic `users` table to a normalized multi-table architecture. This refactor addresses test brittleness, enables multi-provider OAuth, and creates a scalable foundation for future feature development.

### Strategic Context
**Upcoming features that make this refactor essential:**
- Messaging system with extensive preferences (notifications, privacy controls, message settings)
- QR code invite system with user settings
- Additional OAuth providers (GitHub, Microsoft, LinkedIn)
- User preference expansion for any new feature

**Current pain point:** Adding any preference field to the `users` table breaks ALL tests that create users (as demonstrated by the recent `timer_is_public` field addition).

### Effort Estimate
**Total: 10-14 days** of focused development across 6 phases

### Decision Timeline
- **Do Now**: If planning to build messaging, multi-OAuth, or invite systems in next 3-6 months
- **Defer**: If shipping features fast and okay with ongoing test brittleness

---

## Current State Analysis

### Current Schema

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255),                    -- Local auth
    display_name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    active BOOLEAN NOT NULL DEFAULT true,
    real_name VARCHAR(255),                        -- Profile data
    google_user_id VARCHAR(255) UNIQUE,            -- OAuth data
    timer_is_public BOOLEAN NOT NULL DEFAULT false,-- Preference
    timer_show_in_list BOOLEAN NOT NULL DEFAULT false, -- Preference
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Problems with Current Design

**1. Test Brittleness**
- Every test helper that creates users must select ALL columns
- Adding a preference column breaks ~50+ test files
- Recent example: Adding `timer_is_public` broke 4 admin API tests
- Location: `backend/tests/test_helpers.rs:318` - SELECT query missing new columns

**2. Mixed Concerns in Single Table**
- **Authentication** data (password_hash)
- **Profile** data (real_name)
- **OAuth** data (google_user_id)
- **Preferences** (timer_is_public, timer_show_in_list)
- Violates Single Responsibility Principle

**3. OAuth Scalability Limitations**
- Only supports one OAuth provider (Google)
- Cannot link multiple providers to same account
- No storage for provider-specific tokens/refresh tokens
- Hard to add GitHub, Microsoft, LinkedIn, etc.

**4. Performance Implications**
- Wide table for every auth query
- Poor cache utilization (auth needs narrow table)
- Mixed access patterns (auth vs preferences)

**5. Security Concerns**
- Password changes and preference changes treated same
- No separate access control patterns
- Replication inconsistency (password needs immediate, preferences eventual)

### Impact Scope

**Backend:**
- **Files affected**: ~107 Rust files
- **Code references**: ~190 occurrences of user fields
- **Models**: 8 files with User struct references
- **Repositories**: ~15 files
- **Services**: ~20 files
- **Routes**: ~10 files

**Frontend:**
- **TypeScript files**: 6 interface files
- **Component impact**: Minimal (mostly type changes)

**Tests:**
- **Backend tests**: 227 tests (many create users)
- **Frontend tests**: 175 tests (minimal impact)
- **Test helpers**: Multiple files need updates

**Data:**
- **GDPR/CCPA compliance**: Data export must include all tables
- **Migration complexity**: Move existing data to new tables
- **Backup requirements**: Full backup before migration

---

## Research Findings

### Industry Best Practices

Based on web research of OAuth multi-provider patterns and authentication schema design:

#### 1. Separation of Concerns (Software Engineering Stack Exchange, Vertabelo)

**Key Finding:** Separate tables for authentication, profile, and preferences is industry standard.

**Rationale:**
- **Security**: Authentication data needs strict access control, preferences don't
- **Performance**: Narrow auth table = better caching and query performance
- **Replication**: Password changes need immediate consistency, preferences only need eventual consistency
- **Maintainability**: Adding preferences doesn't affect authentication code

#### 2. Multi-Provider OAuth Schema (Spring Security OAuth2, Vertabelo)

**Spring Security Reference Implementation:**
```sql
oauth2_authorized_client (
    client_registration_id VARCHAR(100),  -- 'google', 'github', etc.
    principal_name VARCHAR(200),
    access_token_type VARCHAR(100),
    access_token_value TEXT,
    access_token_issued_at TIMESTAMP,
    access_token_expires_at TIMESTAMP,
    access_token_scopes VARCHAR(1000),
    refresh_token_value TEXT,
    ...
)
```

**Key Pattern:** Separate table for each OAuth provider with provider-specific fields.

**Vertabelo Best Practice:**
```sql
user_external_login (
    id,
    user_id FK,
    provider ENUM('google', 'github', 'microsoft'),
    provider_user_id,  -- Their unique ID
    access_token,
    refresh_token,
    expires_at
)
```

Supports multiple providers per user via separate rows.

#### 3. User Preferences Storage Approaches

**Option A: Columns** (current approach)
- ✅ Simple queries
- ✅ Type safety
- ❌ Schema changes for new preferences
- ❌ Breaks tests when adding columns

**Option B: JSONB** (PostgreSQL specialty)
```sql
user_preferences (
    user_id FK,
    preferences JSONB  -- {"timer_public": true, "theme": "dark"}
)
```
- ✅ No schema changes for new preferences
- ✅ Can index JSONB fields
- ❌ Less type safety
- ❌ More complex queries

**Option C: Key-Value/EAV**
```sql
user_settings (
    user_id FK,
    setting_name VARCHAR,
    setting_value TEXT
)
```
- ✅ Maximum flexibility
- ❌ "Notoriously messy" (multiple sources)
- ❌ Complex queries
- ❌ Not recommended unless absolutely necessary

**Recommendation:** Option A (columns) for structured preferences with Option B (JSONB) for future unstructured preferences.

#### 4. Authentication vs Profile Separation

**DoneDone Blog, Vertabelo:**
- **Identity data**: user name, password hash, email, last login → Keep together
- **User profile**: preferences, bio, avatar, latest activity → Separate table
- **Rationale**: Identity rarely changes, profile changes frequently

**Performance benefit:** Thin authentication table fits better in memory and cache.

---

## Proposed Architecture

### New Schema Design

```sql
-- ============================================================================
-- Core Identity (Rarely Changes)
-- ============================================================================
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    email VARCHAR(255) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- Narrow table for fast auth lookups
-- Only essential identity fields
-- Rarely modified after creation

-- ============================================================================
-- Local Password Authentication (Optional - NULL for OAuth-only users)
-- ============================================================================
CREATE TABLE user_credentials (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    password_hash VARCHAR(255) NOT NULL,
    password_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- Separate table allows:
-- - OAuth-only users (no row in this table)
-- - Strict access control on password changes
-- - Future: password history, reset tokens

-- ============================================================================
-- OAuth External Logins (Supports Multiple Providers per User)
-- ============================================================================
CREATE TABLE user_external_logins (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,  -- 'google', 'github', 'microsoft', etc.
    provider_user_id VARCHAR(255) NOT NULL,  -- Their unique ID for us
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_user_id)
);
-- Allows:
-- - Multiple providers per user (Google + GitHub)
-- - Provider-specific token storage
-- - Easy addition of new providers
-- - Account linking strategy

CREATE INDEX idx_user_external_logins_user_id ON user_external_logins(user_id);
CREATE INDEX idx_user_external_logins_provider ON user_external_logins(provider, provider_user_id);

-- ============================================================================
-- User Profile (Optional Profile Data)
-- ============================================================================
CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    real_name VARCHAR(255),
    bio TEXT,
    avatar_url VARCHAR(500),
    location VARCHAR(255),
    website VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- Future-ready for:
-- - Rich profile data
-- - Social features
-- - Portfolio information

-- ============================================================================
-- User Preferences (Settings - Can Grow Without Breaking Auth)
-- ============================================================================
CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    -- Current preferences
    timer_is_public BOOLEAN NOT NULL DEFAULT false,
    timer_show_in_list BOOLEAN NOT NULL DEFAULT false,

    -- Future messaging preferences (planned)
    -- notification_email BOOLEAN DEFAULT true,
    -- notification_push BOOLEAN DEFAULT false,
    -- message_privacy VARCHAR(20) DEFAULT 'friends',

    -- Future UI preferences (planned)
    -- theme VARCHAR(20) DEFAULT 'steampunk',
    -- timezone VARCHAR(50),
    -- language VARCHAR(10) DEFAULT 'en',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- Adding preferences = no auth test breakage
-- Clear separation from authentication
-- Scalable for many future features
```

### Field Mapping: Old → New

| Old Location | Field | New Location |
|--------------|-------|--------------|
| users | id, email, display_name, slug, active, created_at, updated_at | users (unchanged) |
| users | password_hash | user_credentials.password_hash |
| users | google_user_id | user_external_logins (provider='google', provider_user_id) |
| users | real_name | user_profiles.real_name |
| users | timer_is_public, timer_show_in_list | user_preferences |

### Rationale for Each Table

**users**: Core immutable identity
- Used in JWTs, foreign keys, audit logs
- Rarely changes after creation
- Narrow table = better performance

**user_credentials**: Local authentication only
- Allows OAuth-only users (no row here)
- Strict access control (require current password)
- Future: password history, complexity rules

**user_external_logins**: Multi-provider OAuth
- One row per provider per user
- Stores provider-specific tokens
- Easy to add new providers
- Supports account linking

**user_profiles**: Optional rich profile data
- Bio, avatar, location, etc.
- Public-facing information
- Future social features

**user_preferences**: Application settings
- Can grow indefinitely
- Doesn't affect auth performance
- Clear ownership by user

---

## Impact Analysis

### Database Layer

**Migrations Required:**
1. Create `user_credentials` table
2. Create `user_external_logins` table
3. Create `user_profiles` table
4. Create `user_preferences` table
5. Migrate data from `users` to new tables
6. Remove old columns from `users` (after verification period)

**Data Migration Strategy:**
```sql
-- Phase 1: Create new tables (empty)
-- Phase 2: Dual-write (old + new tables)
-- Phase 3: Backfill existing data
INSERT INTO user_credentials (user_id, password_hash)
SELECT id, password_hash FROM users WHERE password_hash IS NOT NULL;

INSERT INTO user_external_logins (user_id, provider, provider_user_id)
SELECT id, 'google', google_user_id FROM users WHERE google_user_id IS NOT NULL;

INSERT INTO user_profiles (user_id, real_name)
SELECT id, real_name FROM users WHERE real_name IS NOT NULL;

INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list)
SELECT id, timer_is_public, timer_show_in_list FROM users;
-- Phase 4: Verify data integrity
-- Phase 5: Switch to read from new tables
-- Phase 6: Drop old columns
```

**Indexes Required:**
```sql
-- user_external_logins
CREATE INDEX idx_user_external_logins_user_id ON user_external_logins(user_id);
CREATE INDEX idx_user_external_logins_provider ON user_external_logins(provider, provider_user_id);

-- All foreign keys automatically indexed by PostgreSQL
```

### Backend Layer

#### Models (`backend/src/models/db/`)

**New Models to Create:**

```rust
// user_credentials.rs
#[derive(Debug, Clone, FromRow)]
pub struct UserCredentials {
    pub user_id: Uuid,
    pub password_hash: String,
    pub password_updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// user_external_login.rs
#[derive(Debug, Clone, FromRow)]
pub struct UserExternalLogin {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,  // Or enum: OAuthProvider
    pub provider_user_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// user_profile.rs
#[derive(Debug, Clone, FromRow)]
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

// user_preferences.rs
#[derive(Debug, Clone, FromRow)]
pub struct UserPreferences {
    pub user_id: Uuid,
    pub timer_is_public: bool,
    pub timer_show_in_list: bool,
    // Future fields added here don't break tests!
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Modified User Model:**

```rust
// user.rs - AFTER refactor
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub slug: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// New composite struct for full user data
#[derive(Debug, Clone)]
pub struct UserWithDetails {
    pub user: User,
    pub credentials: Option<UserCredentials>,
    pub external_logins: Vec<UserExternalLogin>,
    pub profile: Option<UserProfile>,
    pub preferences: UserPreferences,
}
```

#### Repositories (`backend/src/repositories/`)

**New Repository Traits:**

```rust
// traits/user_credentials_repository.rs
#[async_trait]
pub trait UserCredentialsRepository: Send + Sync {
    async fn create(&self, user_id: Uuid, password_hash: String) -> Result<UserCredentials>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserCredentials>>;
    async fn update_password(&self, user_id: Uuid, new_password_hash: String) -> Result<()>;
    async fn delete(&self, user_id: Uuid) -> Result<()>;
}

// traits/user_external_login_repository.rs
#[async_trait]
pub trait UserExternalLoginRepository: Send + Sync {
    async fn create(&self, data: CreateExternalLogin) -> Result<UserExternalLogin>;
    async fn find_by_provider(&self, provider: &str, provider_user_id: &str) -> Result<Option<UserExternalLogin>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<UserExternalLogin>>;
    async fn update_tokens(&self, id: Uuid, access_token: String, refresh_token: Option<String>, expires_at: Option<DateTime<Utc>>) -> Result<()>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

// traits/user_profile_repository.rs
#[async_trait]
pub trait UserProfileRepository: Send + Sync {
    async fn create(&self, user_id: Uuid) -> Result<UserProfile>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>>;
    async fn update_real_name(&self, user_id: Uuid, real_name: Option<String>) -> Result<()>;
    async fn update(&self, user_id: Uuid, data: UpdateProfile) -> Result<UserProfile>;
}

// traits/user_preferences_repository.rs
#[async_trait]
pub trait UserPreferencesRepository: Send + Sync {
    async fn create(&self, user_id: Uuid) -> Result<UserPreferences>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserPreferences>>;
    async fn update(&self, user_id: Uuid, data: UpdatePreferences) -> Result<UserPreferences>;
    async fn update_timer_settings(&self, user_id: Uuid, is_public: bool, show_in_list: bool) -> Result<()>;
}
```

**Modified UserRepository:**

```rust
// traits/user_repository.rs - Updated queries
impl UserRepository {
    // Now needs JOINs for complete data
    async fn find_by_id_with_details(&self, user_id: Uuid) -> Result<Option<UserWithDetails>> {
        // JOIN users + user_credentials + user_external_logins + user_profiles + user_preferences
    }

    // Lightweight version for auth
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        // Query ONLY users table (fast auth lookup)
    }

    // Remove old methods
    // - find_by_google_user_id() → Move to UserExternalLoginRepository
    // - update_real_name() → Move to UserProfileRepository
}
```

#### Services (`backend/src/services/`)

**AuthService Updates:**

Major changes in:
- `auth_service/register.rs`: Create entries in multiple tables
- `auth_service/login.rs`: Check user_credentials table
- `auth_service/oauth.rs`: Use user_external_logins table
- `auth_service/password.rs`: Update user_credentials table
- `auth_service/data_export.rs`: **CRITICAL** - JOIN all tables for GDPR export

Example OAuth flow change:
```rust
// BEFORE
let existing_user = self.user_repository
    .find_by_google_user_id(&google_user_info.sub)
    .await?;

// AFTER
let existing_login = self.external_login_repository
    .find_by_provider("google", &google_user_info.sub)
    .await?;

if let Some(login) = existing_login {
    let user = self.user_repository.find_by_id(login.user_id).await?;
    // ...
}
```

**UserManagementService Updates:**

- Admin operations need to handle multiple tables
- Account deletion cascades properly (ON DELETE CASCADE)

#### Routes (`backend/src/routes/`)

Minimal changes - mostly just pass data to services.

**Updated routes:**
- `auth.rs`: Registration creates multiple table entries
- `profile.rs`: Updates go to appropriate table

### Frontend Layer

**TypeScript Interface Updates:**

```typescript
// shared/types/auth.ts - BEFORE
export interface AuthenticatedUser {
  id: string
  email: string
  display_name: string
  slug: string
  roles: string[]
  created_at: string
  email_verified?: boolean
  real_name?: string           // REMOVED from base user
  google_user_id?: string      // REMOVED from base user
}

// shared/types/auth.ts - AFTER
export interface AuthenticatedUser {
  id: string
  email: string
  display_name: string
  slug: string
  roles: string[]
  created_at: string
  email_verified?: boolean
}

// NEW: Profile interface
export interface UserProfile {
  real_name?: string
  bio?: string
  avatar_url?: string
  location?: string
  website?: string
}

// NEW: Preferences interface
export interface UserPreferences {
  timer_is_public: boolean
  timer_show_in_list: boolean
  // Future: notification_email, theme, etc.
}

// NEW: Complete user data
export interface UserWithDetails {
  user: AuthenticatedUser
  profile?: UserProfile
  preferences: UserPreferences
  has_password: boolean
  external_logins: { provider: string }[]
}
```

**Minimal component changes** - most components use high-level types.

### Test Layer

**CRITICAL: All Test Helpers Need Updates**

**Test Helper Updates Required:**

```rust
// backend/tests/test_helpers.rs

// BEFORE (brittle - breaks when adding columns)
pub async fn create_test_user_in_db(...) -> Result<User> {
    sqlx::query("INSERT INTO users (email, password_hash, ...) VALUES ...")
        .execute(pool)
        .await?;

    // SELECT must list ALL columns (breaks when adding preferences)
    sqlx::query_as::<_, User>("SELECT id, email, password_hash, display_name, slug, active, real_name, google_user_id, timer_is_public, timer_show_in_list, created_at, updated_at FROM users WHERE id = $1")
        .fetch_one(pool)
        .await
}

// AFTER (resilient - uses builder pattern)
pub async fn create_test_user_in_db(...) -> Result<User> {
    // Insert into users table (narrow)
    let user = sqlx::query_as::<_, User>("INSERT INTO users (email, display_name, slug) VALUES ($1, $2, $3) RETURNING *")
        .bind(email)
        .bind(display_name)
        .bind(slug)
        .fetch_one(pool)
        .await?;

    // Optionally create credentials
    if let Some(password_hash) = password_hash {
        sqlx::query("INSERT INTO user_credentials (user_id, password_hash) VALUES ($1, $2)")
            .bind(user.id)
            .bind(password_hash)
            .execute(pool)
            .await?;
    }

    // Create default preferences
    sqlx::query("INSERT INTO user_preferences (user_id) VALUES ($1)")
        .bind(user.id)
        .execute(pool)
        .await?;

    Ok(user)
}
```

**Builder Pattern for Test Users:**

```rust
// NEW: Test user builder pattern
pub struct TestUserBuilder<'a> {
    pool: &'a PgPool,
    email: String,
    display_name: String,
    slug: String,
    password: Option<String>,
    google_id: Option<String>,
    real_name: Option<String>,
    timer_public: bool,
    timer_in_list: bool,
}

impl<'a> TestUserBuilder<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        let rand_id = Uuid::new_v4().to_string();
        Self {
            pool,
            email: format!("test-{}@example.com", rand_id),
            display_name: format!("Test User {}", rand_id),
            slug: format!("test-{}", rand_id),
            password: Some("Test123!@#".to_string()),
            google_id: None,
            real_name: None,
            timer_public: false,
            timer_in_list: false,
        }
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn oauth_only(mut self) -> Self {
        self.password = None;
        self
    }

    pub fn with_google_id(mut self, google_id: impl Into<String>) -> Self {
        self.google_id = Some(google_id.into());
        self
    }

    pub fn timer_public(mut self) -> Self {
        self.timer_public = true;
        self
    }

    pub async fn build(self) -> Result<User> {
        // Create user
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, display_name, slug) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&self.email)
        .bind(&self.display_name)
        .bind(&self.slug)
        .fetch_one(self.pool)
        .await?;

        // Create credentials if password provided
        if let Some(password) = self.password {
            let password_hash = bcrypt::hash(password, 4)?;
            sqlx::query("INSERT INTO user_credentials (user_id, password_hash) VALUES ($1, $2)")
                .bind(user.id)
                .bind(password_hash)
                .execute(self.pool)
                .await?;
        }

        // Create external login if Google ID provided
        if let Some(google_id) = self.google_id {
            sqlx::query("INSERT INTO user_external_logins (user_id, provider, provider_user_id) VALUES ($1, 'google', $2)")
                .bind(user.id)
                .bind(google_id)
                .execute(self.pool)
                .await?;
        }

        // Create profile if real_name provided
        if let Some(real_name) = self.real_name {
            sqlx::query("INSERT INTO user_profiles (user_id, real_name) VALUES ($1, $2)")
                .bind(user.id)
                .bind(real_name)
                .execute(self.pool)
                .await?;
        }

        // Create preferences (always)
        sqlx::query("INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list) VALUES ($1, $2, $3)")
            .bind(user.id)
            .bind(self.timer_public)
            .bind(self.timer_in_list)
            .execute(self.pool)
            .await?;

        Ok(user)
    }
}

// Usage in tests - MUCH CLEANER
let user = TestUserBuilder::new(&pool)
    .with_email("admin@test.com")
    .timer_public()
    .build()
    .await?;

// OAuth user
let oauth_user = TestUserBuilder::new(&pool)
    .with_google_id("google_12345")
    .oauth_only()  // No password
    .build()
    .await?;
```

**Benefits of Builder Pattern in Tests:**
- Adding preferences? Just update builder defaults
- Tests don't break when schema changes
- Clear, readable test setup
- Follows CODING-RULES.md standards

**Test Impact Summary:**
- **227 backend tests**: Need test helper updates
- **Mock repositories**: 4 new mock repositories needed
- **Integration tests**: Update user creation patterns
- **Data export tests**: Verify all tables included

---

## Migration Strategy

### Phase 1: Database Schema (1-2 days)

**Tasks:**
1. Create UP migration for 4 new tables
2. Create DOWN migration for rollback
3. Run migration on development database
4. Verify constraints and indexes

**Acceptance Criteria:**
- ✅ All 4 tables created with correct constraints
- ✅ Foreign keys properly set with ON DELETE CASCADE
- ✅ Indexes created on foreign keys and lookup columns
- ✅ Migration reversible (DOWN migration works)

**Dual-Write Phase Begins:**
- Application writes to BOTH old and new tables
- Reads still from old table
- Safety period for data verification

### Phase 2: Data Migration (1 day)

**Tasks:**
1. Write data migration script
2. Backfill existing data to new tables
3. Verify data integrity (row counts, checksums)
4. Test rollback procedure

**Migration Script:**
```sql
-- Backfill user_credentials
INSERT INTO user_credentials (user_id, password_hash)
SELECT id, password_hash
FROM users
WHERE password_hash IS NOT NULL;

-- Backfill user_external_logins
INSERT INTO user_external_logins (user_id, provider, provider_user_id)
SELECT id, 'google', google_user_id
FROM users
WHERE google_user_id IS NOT NULL;

-- Backfill user_profiles
INSERT INTO user_profiles (user_id, real_name)
SELECT id, real_name
FROM users
WHERE real_name IS NOT NULL;

-- Backfill user_preferences (ALL users get preferences)
INSERT INTO user_preferences (user_id, timer_is_public, timer_show_in_list)
SELECT id, timer_is_public, timer_show_in_list
FROM users;

-- Verify counts
SELECT
    (SELECT COUNT(*) FROM users) as total_users,
    (SELECT COUNT(*) FROM user_credentials) as users_with_password,
    (SELECT COUNT(*) FROM user_external_logins) as users_with_oauth,
    (SELECT COUNT(*) FROM user_profiles) as users_with_profile,
    (SELECT COUNT(*) FROM user_preferences) as users_with_prefs;
```

**Acceptance Criteria:**
- ✅ All existing data migrated to new tables
- ✅ No data loss (verified by checksums)
- ✅ Referential integrity maintained
- ✅ Rollback tested and working

### Phase 3: Backend Models & Repositories (2-3 days)

**Tasks:**
1. Create 4 new model structs
2. Implement 4 new repository traits
3. Implement Postgres repositories
4. Implement mock repositories for testing
5. Update UserRepository to use JOINs
6. Update test helpers to use builder pattern

**Acceptance Criteria:**
- ✅ All new models compile
- ✅ All new repositories implement traits
- ✅ Mock repositories created for unit tests
- ✅ TestUserBuilder implemented and documented
- ✅ At least one test using new builder pattern

### Phase 4: Backend Services & Data Export (2-3 days)

**Tasks:**
1. Update AuthService (registration, login, OAuth, password)
2. Update UserManagementService (admin operations)
3. **CRITICAL**: Update data export to include all tables
4. Update profile management services
5. Dual-write implementation (write to both old and new)

**Data Export Changes (CRITICAL for GDPR/CCPA):**
```rust
// BEFORE
pub async fn export_user_data(&self, user_id: Uuid) -> Result<UserDataExport> {
    let user = self.user_repository.find_by_id(user_id).await?;
    // ... exports single user record
}

// AFTER
pub async fn export_user_data(&self, user_id: Uuid) -> Result<UserDataExport> {
    // Get user from all tables
    let user = self.user_repository.find_by_id(user_id).await?;
    let credentials = self.credentials_repository.find_by_user_id(user_id).await?;
    let external_logins = self.external_login_repository.find_by_user_id(user_id).await?;
    let profile = self.profile_repository.find_by_user_id(user_id).await?;
    let preferences = self.preferences_repository.find_by_user_id(user_id).await?;

    // Build comprehensive export
    UserDataExport {
        export_version: "2.0",  // Increment version
        user: user_export,
        authentication: AuthenticationExport {
            has_password: credentials.is_some(),
            password_last_changed: credentials.map(|c| c.password_updated_at),
        },
        external_logins: external_logins.into_iter().map(|l| ExternalLoginExport {
            provider: l.provider,
            linked_at: l.created_at,
        }).collect(),
        profile: profile.map(|p| ProfileExport {
            real_name: p.real_name,
            bio: p.bio,
            // ... other fields
        }),
        preferences: PreferencesExport {
            timer_is_public: preferences.timer_is_public,
            timer_show_in_list: preferences.timer_show_in_list,
        },
        // ... rest of export (timers, phrases, etc.)
    }
}
```

**Acceptance Criteria:**
- ✅ Registration creates entries in all relevant tables
- ✅ Login checks user_credentials table
- ✅ OAuth flow uses user_external_logins
- ✅ Password changes update user_credentials
- ✅ **Data export includes ALL tables (verified manually)**
- ✅ Dual-write working (writes to old + new)

### Phase 5: Testing & Validation (2-3 days)

**Tasks:**
1. Update all test helpers to use builder pattern
2. Fix all broken backend tests (expected: ~100 failures initially)
3. Add tests for new repositories
4. Integration test OAuth flows
5. **Test data export completeness** (critical)
6. Performance testing (query benchmarks)

**Test Verification Checklist:**
- ✅ All 227 backend tests passing
- ✅ OAuth flow tested (Google login)
- ✅ Registration creates all table entries
- ✅ Password change updates correct table
- ✅ Account deletion cascades properly
- ✅ Data export includes all user data
- ✅ Builder pattern used in all new tests

### Phase 6: Cutover & Cleanup (1-2 days)

**Tasks:**
1. Switch reads from old table to new tables
2. Stop dual-write
3. Monitor for issues (24-48 hour observation period)
4. Create migration to drop old columns
5. Run final migration (remove password_hash, google_user_id, real_name, timer_* from users)
6. Update documentation

**Final Migration:**
```sql
-- After successful observation period
ALTER TABLE users DROP COLUMN password_hash;
ALTER TABLE users DROP COLUMN google_user_id;
ALTER TABLE users DROP COLUMN real_name;
ALTER TABLE users DROP COLUMN timer_is_public;
ALTER TABLE users DROP COLUMN timer_show_in_list;
```

**Acceptance Criteria:**
- ✅ Application reading from new tables only
- ✅ No dual-write overhead
- ✅ Old columns removed
- ✅ All tests passing
- ✅ Production monitoring shows no errors
- ✅ Data export verified post-cleanup

---

## Risk Analysis & Mitigation

### Risk 1: Data Loss During Migration

**Severity:** CRITICAL
**Probability:** LOW (with proper procedures)

**Mitigation:**
- ✅ Full database backup before migration
- ✅ Dual-write safety period (write to both old and new)
- ✅ Data verification checksums
- ✅ Tested rollback procedure
- ✅ Observation period before dropping old columns

**Rollback Plan:**
1. Stop application
2. Run DOWN migration (drop new tables)
3. Verify old data intact
4. Restart application

### Risk 2: Breaking GDPR Data Export

**Severity:** CRITICAL (legal compliance)
**Probability:** MEDIUM

**Mitigation:**
- ✅ Update data export in Phase 4 (BEFORE cutover)
- ✅ Manual verification of export completeness
- ✅ Automated tests for export data
- ✅ Increment export version to 2.0
- ✅ Test export with real user data (dev environment)

**Verification Checklist:**
- [ ] Export includes user credentials (has_password flag, not hash)
- [ ] Export includes all external logins (provider list)
- [ ] Export includes profile data (real_name, bio, etc.)
- [ ] Export includes all preferences
- [ ] Export includes all associated data (timers, phrases, etc.)
- [ ] Export format is valid JSON
- [ ] Export is downloadable by user

### Risk 3: OAuth Flow Breaks

**Severity:** HIGH
**Probability:** MEDIUM

**Mitigation:**
- ✅ Test Google OAuth flow thoroughly in Phase 5
- ✅ Keep OAuth integration tests running
- ✅ Test account linking scenarios
- ✅ Test new user registration via OAuth
- ✅ Test existing user login via OAuth

**Test Scenarios:**
1. New user registers via Google → Creates user + external_login
2. Existing email-verified user links Google → Adds external_login
3. Existing Google user logs in → Finds via external_login
4. User with password + Google → Both credentials + external_login exist
5. OAuth-only user (no password) → No credentials row

### Risk 4: Test Failures Overwhelming

**Severity:** MEDIUM
**Probability:** HIGH (expected ~100 initial failures)

**Mitigation:**
- ✅ Update test helpers FIRST (Phase 3)
- ✅ Implement builder pattern early
- ✅ Fix tests incrementally (not all at once)
- ✅ Categorize failures (schema, helper, logic)
- ✅ Use builder pattern to prevent future brittleness

**Expected Failure Categories:**
1. **Schema failures**: Column not found (quick fix: update SELECTs)
2. **Helper failures**: create_test_user needs new pattern (fix helper once)
3. **Logic failures**: Services query wrong table (update service logic)

**Strategy:**
- Fix all test helpers first → 50% of failures auto-resolve
- Fix schema issues next → 30% more resolve
- Fix service logic last → Remaining 20%

### Risk 5: Performance Regression

**Severity:** MEDIUM
**Probability:** LOW

**Mitigation:**
- ✅ Benchmark queries before and after
- ✅ Ensure proper indexes on foreign keys
- ✅ Use lightweight User model for auth (no JOINs)
- ✅ Only JOIN when full data needed
- ✅ Monitor slow query log

**Performance Expectations:**
- **Auth queries (find_by_id)**: FASTER (narrow users table)
- **Full user queries (with details)**: SAME (JOINs replace wide row)
- **Profile updates**: SAME (separate table, fewer rows)
- **Preference updates**: FASTER (separate table, no lock contention)

---

## Alternatives Considered

### Alternative 1: JSONB Preferences Column

**Approach:**
```sql
CREATE TABLE users (
    -- ... existing fields
    preferences JSONB DEFAULT '{}'::jsonb
);

-- Add preferences without schema changes
UPDATE users SET preferences = preferences || '{"new_pref": true}' WHERE id = $1;
```

**Pros:**
- ✅ No schema changes for new preferences
- ✅ Can index JSONB fields (`CREATE INDEX idx_prefs_timer ON users USING GIN ((preferences->>'timer_public'))`)
- ✅ Flexible, schema-less

**Cons:**
- ❌ Less type safety (values are strings in JSONB)
- ❌ More complex queries (`preferences->>'key'`)
- ❌ Harder to enforce defaults
- ❌ Still mixed with auth data (same table)
- ❌ Doesn't solve OAuth multi-provider issue

**Decision:** Not recommended as primary approach, but could be used WITHIN `user_preferences` table for truly unstructured settings.

### Alternative 2: Defer Until After Messaging Feature

**Approach:**
- Ship messaging feature with current schema
- Add all messaging preferences as columns to `users` table
- Refactor to multi-table after learning what preferences are needed

**Pros:**
- ✅ Ship messaging feature faster (no refactor delay)
- ✅ Learn actual preference requirements first
- ✅ Avoid speculative design

**Cons:**
- ❌ Every messaging preference added breaks all tests
- ❌ Technical debt compounds with more columns
- ❌ Harder migration with more live user data
- ❌ Messaging likely has 10+ preference fields (notifications, privacy, etc.)
- ❌ Still can't add GitHub/Microsoft OAuth easily

**Decision:** Only choose this if:
- Messaging is exploratory/might be scrapped
- You're okay with significant test brittleness for 3-6 months
- You don't plan to add more OAuth providers soon

### Alternative 3: Keep Current Structure

**Approach:**
- Continue with monolithic `users` table
- Add columns as needed
- Accept test brittleness as cost of simplicity

**Pros:**
- ✅ No migration effort
- ✅ Simple queries (no JOINs)
- ✅ Ship features faster

**Cons:**
- ❌ Every column addition breaks ~50 tests
- ❌ Can't add multi-provider OAuth
- ❌ Violates separation of concerns
- ❌ Poor cache utilization (wide table)
- ❌ Mixed access patterns hurt performance
- ❌ Death by 1000 cuts over 6-12 months

**Decision:** Not recommended for long-term maintainability.

---

## Decision Framework

### When to Do This Refactor NOW

**Do it now if you:**
- ✅ Plan to build messaging system (many preferences)
- ✅ Want to add GitHub/Microsoft/LinkedIn OAuth
- ✅ Plan to build QR code invite system
- ✅ Value long-term maintainability over short-term speed
- ✅ Have 2 weeks available for focused refactoring
- ✅ Want to prevent test brittleness going forward

**Strategic indicators:**
- Roadmap includes 3+ features with user preferences
- Considering additional OAuth providers
- Current test breakage is painful
- Building for 12+ month timeline

### When to DEFER This Refactor

**Defer if you:**
- ❌ Need to ship messaging in next 2-3 weeks
- ❌ Messaging feature is exploratory (might scrap it)
- ❌ Only planning 1-2 more preference fields total
- ❌ No plans for additional OAuth providers
- ❌ Okay with test brittleness short-term
- ❌ Might pivot away from user-centric features

**Strategic indicators:**
- Uncertain about feature roadmap
- Timeline pressure to ship something
- Small team, limited capacity
- Under 6-month project timeline

### Decision Criteria

| Factor | Do Now | Defer |
|--------|--------|-------|
| **Messaging feature scope** | Large (10+ preferences) | Small (2-3 preferences) |
| **OAuth providers** | Want multi-provider | Google-only is fine |
| **Timeline** | 12+ months | < 6 months |
| **Team capacity** | Can dedicate 2 weeks | Need to ship fast |
| **Test pain** | Painful, want fix | Manageable |
| **Architecture preference** | Clean, maintainable | Pragmatic, simple |

### Recommendation

**DO THIS REFACTOR NOW** if:
- You're serious about building messaging (which WILL have many preferences)
- You see a 12+ month development timeline
- Clean architecture is important to you

**DEFER** if:
- You need to ship messaging in 2-3 weeks
- Uncertain about feature roadmap
- Under significant timeline pressure

**The Cost:**
- **Do now**: 2 weeks of refactoring, then smooth sailing
- **Defer**: 1-2 hours fixing tests EVERY time you add a preference field (death by 1000 cuts)

---

## Implementation Checklist

### Phase 1: Database Schema ✅

**Migrations:**
- [ ] Create `user_credentials` table migration
- [ ] Create `user_external_logins` table migration
- [ ] Create `user_profiles` table migration
- [ ] Create `user_preferences` table migration
- [ ] Add indexes on foreign keys
- [ ] Add unique constraints (provider + provider_user_id)
- [ ] Test UP migration
- [ ] Test DOWN migration (rollback)

**Data Migration:**
- [ ] Write data backfill script
- [ ] Run backfill in development
- [ ] Verify row counts match
- [ ] Verify referential integrity
- [ ] Test with sample queries

### Phase 2: Backend Models ✅

**New Models:**
- [ ] Create `UserCredentials` model
- [ ] Create `UserExternalLogin` model
- [ ] Create `UserProfile` model
- [ ] Create `UserPreferences` model
- [ ] Create `UserWithDetails` composite model
- [ ] Update `User` model (remove migrated fields)
- [ ] Add serialization derives
- [ ] Add documentation comments

### Phase 3: Backend Repositories ✅

**New Repositories:**
- [ ] Create `UserCredentialsRepository` trait
- [ ] Create `UserExternalLoginRepository` trait
- [ ] Create `UserProfileRepository` trait
- [ ] Create `UserPreferencesRepository` trait
- [ ] Implement Postgres repositories
- [ ] Implement mock repositories
- [ ] Add repository unit tests

**Updated Repositories:**
- [ ] Update `UserRepository::find_by_id()` (no JOINs)
- [ ] Add `UserRepository::find_by_id_with_details()` (with JOINs)
- [ ] Remove `find_by_google_user_id()` (moved to ExternalLogin repo)
- [ ] Remove `update_real_name()` (moved to Profile repo)

### Phase 4: Backend Services ✅

**AuthService:**
- [ ] Update `register()` - create multiple table entries
- [ ] Update `login()` - check user_credentials
- [ ] Update `google_oauth_callback()` - use external_logins
- [ ] Update `change_password()` - update user_credentials
- [ ] Update profile management

**Data Export (CRITICAL):**
- [ ] Update `export_user_data()` to query all tables
- [ ] Increment export version to "2.0"
- [ ] Add authentication export (has_password flag)
- [ ] Add external_logins export (provider list)
- [ ] Add profile export
- [ ] Add preferences export
- [ ] Manually test export completeness
- [ ] Automated test for export structure

**UserManagementService:**
- [ ] Update admin operations for multi-table
- [ ] Verify cascade delete works (ON DELETE CASCADE)

### Phase 5: Test Infrastructure ✅

**Test Helpers:**
- [ ] Implement `TestUserBuilder` with builder pattern
- [ ] Update `create_test_user_in_db()` to use builder
- [ ] Add builder methods (`.with_password()`, `.oauth_only()`, `.timer_public()`)
- [ ] Document builder pattern in test_helpers.rs
- [ ] Create example usage in doc comments

**Mock Repositories:**
- [ ] Create `MockUserCredentialsRepository`
- [ ] Create `MockUserExternalLoginRepository`
- [ ] Create `MockUserProfileRepository`
- [ ] Create `MockUserPreferencesRepository`

### Phase 6: Backend Tests ✅

**Test Fixes:**
- [ ] Update all test helpers to use builder pattern
- [ ] Fix schema failures (column not found)
- [ ] Fix service logic tests
- [ ] Add tests for new repositories
- [ ] Test OAuth flows (Google login, account linking)
- [ ] Test registration creates all tables
- [ ] Test password change updates correct table
- [ ] Test account deletion cascades
- [ ] Test data export completeness
- [ ] Run full test suite (227 backend tests)

### Phase 7: Frontend Updates ✅

**TypeScript Interfaces:**
- [ ] Remove `real_name`, `google_user_id` from `AuthenticatedUser`
- [ ] Create `UserProfile` interface
- [ ] Create `UserPreferences` interface
- [ ] Create `UserWithDetails` interface
- [ ] Update API response types
- [ ] Run TypeScript type checking

**Components:**
- [ ] Test profile settings page
- [ ] Test authentication flows
- [ ] Run frontend tests (175 tests)

### Phase 8: Integration Testing ✅

**OAuth Testing:**
- [ ] Test new user Google registration
- [ ] Test existing user Google login
- [ ] Test account linking (email match + verified)
- [ ] Test OAuth-only user (no password)
- [ ] Test user with both password + Google

**Data Testing:**
- [ ] Test data export download
- [ ] Verify export includes all tables
- [ ] Test data export JSON structure
- [ ] Test account deletion cascade

**Performance:**
- [ ] Benchmark auth queries (should be faster)
- [ ] Benchmark full user queries (should be same)
- [ ] Check slow query log
- [ ] Verify indexes being used

### Phase 9: Cutover ✅

**Switch Reads:**
- [ ] Update code to read from new tables
- [ ] Stop dual-write
- [ ] Deploy to production
- [ ] Monitor for errors (24-48 hours)

**Cleanup:**
- [ ] Create migration to drop old columns
- [ ] Run cleanup migration
- [ ] Verify old columns gone
- [ ] Update documentation
- [ ] Final test suite run

### Phase 10: Documentation ✅

- [ ] Update IMPLEMENTATION-DATABASE.md with new schema
- [ ] Update IMPLEMENTATION-BACKEND.md with new repositories
- [ ] Update ARCHITECTURE.md with multi-table design
- [ ] Document builder pattern in CODING-RULES.md
- [ ] Add migration guide for future developers
- [ ] Update API documentation

---

## Success Criteria

**The refactor is considered successful when:**

✅ **Data Integrity:**
- All existing user data migrated to new tables
- No data loss (verified by checksums)
- Foreign key constraints enforced
- Cascade deletes working properly

✅ **Functionality:**
- All authentication flows working (login, register, OAuth)
- Profile management working (update display_name, real_name, etc.)
- Password changes updating correct table
- Data export includes all tables (GDPR/CCPA compliant)
- Account deletion cascades properly

✅ **Testing:**
- All 227 backend tests passing
- All 175 frontend tests passing
- OAuth integration tests passing
- Data export tests passing
- Builder pattern used in all new tests

✅ **Performance:**
- Auth queries faster or same speed (narrow table)
- Full user queries same speed (JOINs replace wide row)
- No slow queries in production

✅ **Maintainability:**
- Adding new preferences doesn't break tests
- Builder pattern documented and used
- Code follows CODING-RULES.md standards
- Documentation updated

✅ **Production:**
- Deployed successfully to production
- No errors in monitoring (48-hour observation)
- User-facing features all working
- Data export downloadable by users

---

## Next Steps After This Document

**Immediate Actions:**
1. Review this design document
2. Decide: Do now vs defer (use Decision Framework)
3. If proceeding: Schedule 2-week focused sprint
4. If deferring: Document when to revisit (e.g., "before messaging feature")

**If Proceeding:**
1. Create backup of production database
2. Set up development environment
3. Begin Phase 1 (Database Schema)
4. Use this document as implementation checklist
5. Update checklist as you complete each phase

**If Deferring:**
1. Add note to ROADMAP.md: "Defer schema refactor until [condition]"
2. Accept test brittleness as short-term cost
3. Revisit this document when condition met
4. Be prepared for 1-2 hour test fixes when adding preferences

---

## Appendix: Code Examples

### Example: Registration Flow (Before vs After)

**BEFORE (monolithic users table):**
```rust
pub async fn register(&self, data: RegisterRequest) -> Result<AuthResponse> {
    // Hash password
    let password_hash = bcrypt::hash(data.password, 12)?;

    // Create user - ALL in one table
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, display_name, slug)
         VALUES ($1, $2, $3, $4)
         RETURNING *"
    )
    .bind(&data.email)
    .bind(&password_hash)
    .bind(&data.display_name)
    .bind(&slug)
    .fetch_one(&self.pool)
    .await?;

    // ... generate tokens, etc.
}
```

**AFTER (multi-table):**
```rust
pub async fn register(&self, data: RegisterRequest) -> Result<AuthResponse> {
    // Hash password
    let password_hash = bcrypt::hash(data.password, 12)?;

    // Create user identity (narrow table)
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, display_name, slug)
         VALUES ($1, $2, $3)
         RETURNING *"
    )
    .bind(&data.email)
    .bind(&data.display_name)
    .bind(&slug)
    .fetch_one(&self.pool)
    .await?;

    // Create credentials
    sqlx::query("INSERT INTO user_credentials (user_id, password_hash) VALUES ($1, $2)")
        .bind(user.id)
        .bind(&password_hash)
        .execute(&self.pool)
        .await?;

    // Create default preferences
    sqlx::query("INSERT INTO user_preferences (user_id) VALUES ($1)")
        .bind(user.id)
        .execute(&self.pool)
        .await?;

    // ... generate tokens, etc.
}
```

### Example: OAuth Flow (Before vs After)

**BEFORE:**
```rust
pub async fn google_oauth_callback(&self, code: String, state: String) -> Result<AuthResponse> {
    let google_user_info = /* ... fetch from Google ... */;

    // Find by google_user_id
    let user = if let Some(existing_user) = self.user_repository
        .find_by_google_user_id(&google_user_info.sub)
        .await? {
        existing_user
    } else {
        // Create new user with google_user_id
        self.user_repository.create(CreateUserData {
            email: google_user_info.email,
            google_user_id: Some(google_user_info.sub),
            real_name: Some(google_user_info.name),
            password_hash: None,  // OAuth-only user
            // ...
        }).await?
    };

    // ... generate tokens
}
```

**AFTER:**
```rust
pub async fn google_oauth_callback(&self, code: String, state: String) -> Result<AuthResponse> {
    let google_user_info = /* ... fetch from Google ... */;

    // Find by provider + provider_user_id
    let user = if let Some(external_login) = self.external_login_repository
        .find_by_provider("google", &google_user_info.sub)
        .await? {
        // Existing Google user - load user
        self.user_repository.find_by_id(external_login.user_id).await?
    } else {
        // New Google user - create user + external_login + profile + preferences
        let user = self.user_repository.create(CreateUserData {
            email: google_user_info.email,
            display_name: google_user_info.name,
            slug: generate_slug(&google_user_info.email),
        }).await?;

        // Link Google account
        self.external_login_repository.create(CreateExternalLogin {
            user_id: user.id,
            provider: "google".to_string(),
            provider_user_id: google_user_info.sub,
        }).await?;

        // Create profile
        self.profile_repository.create(user.id).await?;
        self.profile_repository.update_real_name(user.id, Some(google_user_info.name)).await?;

        // Create preferences
        self.preferences_repository.create(user.id).await?;

        user
    };

    // ... generate tokens
}
```

### Example: Test User Creation (Before vs After)

**BEFORE (brittle):**
```rust
pub async fn create_test_user_in_db(pool: &PgPool, email: &str) -> Result<User> {
    // Long INSERT with all columns
    sqlx::query("INSERT INTO users (email, password_hash, display_name, slug, active, real_name, google_user_id, timer_is_public, timer_show_in_list) VALUES (...)")
        .execute(pool)
        .await?;

    // Long SELECT with all columns (BREAKS when adding columns)
    sqlx::query_as::<_, User>("SELECT id, email, password_hash, display_name, slug, active, real_name, google_user_id, timer_is_public, timer_show_in_list, created_at, updated_at FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await
}

// Usage - not flexible
let user = create_test_user_in_db(&pool, "test@example.com").await?;
```

**AFTER (builder pattern - resilient):**
```rust
// Usage - flexible and clear
let user = TestUserBuilder::new(&pool)
    .with_email("test@example.com")
    .with_password("Test123!@#")
    .timer_public()
    .build()
    .await?;

// OAuth user
let oauth_user = TestUserBuilder::new(&pool)
    .with_email("oauth@example.com")
    .with_google_id("google_12345")
    .oauth_only()  // No password
    .build()
    .await?;

// Admin user
let admin = TestUserBuilder::new(&pool)
    .with_email("admin@example.com")
    .build()
    .await?;
// Then add admin role separately

// Adding new preferences in future? Just update builder defaults - tests don't break!
```

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-01-11 | Claude Code | Initial design document created |

---

**End of Design Document**

This document serves as the comprehensive blueprint for refactoring the user authentication schema. Use the Implementation Checklist to track progress across multiple development sessions.
