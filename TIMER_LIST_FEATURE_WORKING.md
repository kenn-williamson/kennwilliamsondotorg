# Public Timer List Feature - Design Document

**Status:** Design Phase
**Created:** 2025-10-10
**Author:** Architecture Team

## Overview

This document outlines the design for a public timer list feature that allows users to share their incident timers publicly and appear in a community list page. The feature includes two-level privacy controls and supports both authenticated and non-authenticated viewing experiences.

## User Requirements

### Primary Features
1. **Public Timer List Page**
   - Displays all users who have opted to show their timers publicly
   - Shows to non-logged-in users with call-to-action (CTA) to sign in
   - Shows to logged-in users as a tab on incidents page (without CTA)

2. **Privacy Controls (Account Page)**
   - **Toggle 1:** "Make Timer Public" - Master switch for timer visibility
   - **Toggle 2:** "Show in List" - Controls appearance in public list
   - Dependency: Cannot enable "Show in List" unless "Make Timer Public" is enabled

3. **Privacy Logic**
   - If timer is not public → Return 404 on `/{user_slug}/incident-timer` page
   - Only users with BOTH flags enabled appear in public list
   - Privacy-first defaults (both flags false for new users)

---

## Architecture Design

### Database Schema Changes

#### Users Table Modifications
```sql
-- Migration: 20251010_add_timer_privacy_flags.up.sql
ALTER TABLE users
  ADD COLUMN timer_is_public BOOLEAN NOT NULL DEFAULT false,
  ADD COLUMN timer_show_in_list BOOLEAN NOT NULL DEFAULT false;

-- Composite index for efficient list queries
CREATE INDEX idx_users_timer_visibility
  ON users(timer_is_public, timer_show_in_list)
  WHERE timer_is_public = true AND timer_show_in_list = true;

-- Down migration: 20251010_add_timer_privacy_flags.down.sql
DROP INDEX IF EXISTS idx_users_timer_visibility;
ALTER TABLE users
  DROP COLUMN IF EXISTS timer_show_in_list,
  DROP COLUMN IF EXISTS timer_is_public;
```

#### Database Model Updates

**File:** `backend/src/models/db/user.rs`

Add to `User` struct:
```rust
pub timer_is_public: bool,
pub timer_show_in_list: bool,
```

New type for list queries:
```rust
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct UserWithTimer {
    pub id: Uuid,
    pub display_name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub reset_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}
```

---

## Backend Implementation

### Repository Layer

**File:** `backend/src/repositories/traits/user_repository.rs`

```rust
#[async_trait]
pub trait UserRepository: Send + Sync {
    // ... existing methods

    /// Update timer privacy settings for a user
    /// Returns error if show_in_list=true but is_public=false
    async fn update_timer_privacy(
        &self,
        user_id: Uuid,
        is_public: bool,
        show_in_list: bool
    ) -> Result<User>;

    /// Get users with public timers for the list page
    /// Only returns users where both flags are true
    async fn get_users_with_public_timers(
        &self,
        limit: i64,
        offset: i64
    ) -> Result<Vec<UserWithTimer>>;
}
```

**File:** `backend/src/repositories/postgres/postgres_user_repository.rs`

```rust
async fn update_timer_privacy(
    &self,
    user_id: Uuid,
    is_public: bool,
    show_in_list: bool
) -> Result<User> {
    // Validation: Cannot show in list if not public
    if show_in_list && !is_public {
        return Err(anyhow::anyhow!(
            "Cannot enable 'Show in List' when timer is not public"
        ));
    }

    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET timer_is_public = $1,
            timer_show_in_list = $2,
            updated_at = NOW()
        WHERE id = $3
        RETURNING *
        "#,
        is_public,
        show_in_list,
        user_id
    )
    .fetch_one(&self.pool)
    .await?;

    Ok(user)
}

async fn get_users_with_public_timers(
    &self,
    limit: i64,
    offset: i64
) -> Result<Vec<UserWithTimer>> {
    let users = sqlx::query_as!(
        UserWithTimer,
        r#"
        SELECT
            u.id,
            u.display_name,
            u.slug,
            u.created_at,
            it.reset_timestamp,
            it.notes
        FROM users u
        INNER JOIN incident_timers it ON u.id = it.user_id
        WHERE u.timer_is_public = true
          AND u.timer_show_in_list = true
        ORDER BY it.reset_timestamp DESC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(&self.pool)
    .await?;

    Ok(users)
}
```

### Service Layer

**File:** `backend/src/services/user_service.rs`

```rust
impl UserService {
    /// Update timer privacy settings with validation
    pub async fn update_timer_privacy(
        &self,
        user_id: Uuid,
        is_public: bool,
        show_in_list: bool,
    ) -> Result<User> {
        self.user_repository
            .update_timer_privacy(user_id, is_public, show_in_list)
            .await
    }

    /// Get paginated list of public timers
    pub async fn get_public_timer_list(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<UserWithTimer>> {
        let offset = (page - 1) * page_size;
        self.user_repository
            .get_users_with_public_timers(page_size, offset)
            .await
    }
}
```

**File:** `backend/src/services/incident_timer.rs` (UPDATE)

Modify existing `get_latest_by_user_slug` to check privacy:

```rust
pub async fn get_latest_by_user_slug(
    &self,
    user_slug: &str,
) -> Result<Option<(IncidentTimer, String)>> {
    // Get user by slug
    let user = self.user_repository
        .get_by_slug(user_slug)
        .await?;

    // ✅ NEW: Check timer privacy
    if !user.timer_is_public {
        return Ok(None);  // Return None to trigger 404
    }

    // Existing logic continues...
    let timer = self.incident_timer_repository
        .get_latest_by_user_id(user.id)
        .await?;

    match timer {
        Some(t) => Ok(Some((t, user.display_name))),
        None => Ok(None),
    }
}
```

### API Layer

**File:** `backend/src/models/api/user.rs`

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTimerPrivacyRequest {
    pub is_public: bool,
    pub show_in_list: bool,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PublicTimerListResponse {
    pub id: Uuid,
    pub display_name: String,
    pub slug: String,
    pub reset_timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}
```

**File:** `backend/src/routes/user.rs` (or create new file)

```rust
/// Protected endpoint - Update timer privacy settings
pub async fn update_timer_privacy(
    req: HttpRequest,
    data: web::Json<UpdateTimerPrivacyRequest>,
    service: web::Data<UserService>,
) -> ActixResult<HttpResponse> {
    let user_id = req
        .extensions()
        .get::<Uuid>()
        .cloned()
        .unwrap();

    match service
        .update_timer_privacy(
            user_id,
            data.is_public,
            data.show_in_list
        )
        .await
    {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            log::error!(
                "Failed to update timer privacy for user {}: {}",
                user_id,
                err
            );
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": err.to_string()
            })))
        }
    }
}

/// Public endpoint - Get list of public timers
pub async fn get_public_timer_list(
    query: web::Query<PaginationQuery>,
    service: web::Data<UserService>,
) -> ActixResult<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100); // Max 100

    match service
        .get_public_timer_list(page, page_size)
        .await
    {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(err) => {
            log::error!("Failed to get public timer list: {}", err);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })))
        }
    }
}
```

**File:** `backend/src/routes/mod.rs`

```rust
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // ... existing routes

        // Public routes
        .route(
            "/public-timers",
            web::get().to(user::get_public_timer_list)
        )

        // Protected routes (requires auth)
        .route(
            "/profile/timer-privacy",
            web::put().to(user::update_timer_privacy)
        );
}
```

---

## Frontend Implementation

### Type Definitions

**File:** `frontend/shared/types/timer-privacy.ts` (NEW)

```typescript
export interface UpdateTimerPrivacyRequest {
  is_public: boolean
  show_in_list: boolean
}

export interface PublicTimerListItem {
  id: string
  display_name: string
  slug: string
  reset_timestamp: string
  notes: string | null
}

export interface PaginationQuery {
  page?: number
  page_size?: number
}
```

**File:** `frontend/shared/types/index.ts` (UPDATE)

```typescript
export * from './timer-privacy'
```

### API Routes Configuration

**File:** `frontend/shared/config/api-routes.ts`

```typescript
export const API_ROUTES = {
  PUBLIC: {
    // ... existing routes
    TIMERS: {
      PUBLIC_LIST: '/public/public-timers'
    }
  },

  PROTECTED: {
    AUTH: {
      // ... existing routes
      UPDATE_TIMER_PRIVACY: '/protected/profile/timer-privacy'
    }
  }
}
```

### Service Layer

**File:** `frontend/app/services/userService.ts` (NEW or extend)

```typescript
import { API_ROUTES } from '#shared/config/api-routes'
import type {
  Fetcher,
  UpdateTimerPrivacyRequest,
  PublicTimerListItem
} from '#shared/types'

export const userService = (fetcher: Fetcher) => ({
  updateTimerPrivacy: async (
    data: UpdateTimerPrivacyRequest
  ): Promise<void> => {
    return fetcher<void>(
      API_ROUTES.PROTECTED.AUTH.UPDATE_TIMER_PRIVACY,
      {
        method: 'PUT',
        body: data,
      }
    )
  },

  getPublicTimerList: async (
    page = 1,
    pageSize = 20
  ): Promise<PublicTimerListItem[]> => {
    return fetcher<PublicTimerListItem[]>(
      API_ROUTES.PUBLIC.TIMERS.PUBLIC_LIST,
      {
        method: 'GET',
        query: { page, page_size: pageSize }
      }
    )
  }
})
```

### Composable

**File:** `frontend/app/composables/useTimerPrivacy.ts` (NEW)

```typescript
import { userService } from '~/services/userService'
import { useBaseService } from '~/composables/useBaseService'
import { useSmartFetch } from '~/composables/useSmartFetch'
import type { UpdateTimerPrivacyRequest } from '#shared/types'

export const useTimerPrivacy = () => {
  const smartFetch = useSmartFetch()
  const { user, refresh: refreshSession } = useUserSession()
  const {
    executeRequest,
    executeRequestWithSuccess,
    isLoading,
    error,
    hasError
  } = useBaseService()

  const userServiceInstance = userService(smartFetch)
  const { updateTimerPrivacy: updatePrivacyService } = userServiceInstance

  const updateTimerPrivacy = async (
    isPublic: boolean,
    showInList: boolean
  ): Promise<void> => {
    return executeRequestWithSuccess(
      async () => {
        await updatePrivacyService({
          is_public: isPublic,
          show_in_list: showInList
        })

        // Refresh session to get updated user data
        await refreshSession()
      },
      'Timer privacy settings updated successfully',
      'updateTimerPrivacy'
    )
  }

  return {
    updateTimerPrivacy,
    isLoading,
    error,
    hasError,
    currentSettings: computed(() => ({
      isPublic: user.value?.timer_is_public ?? false,
      showInList: user.value?.timer_show_in_list ?? false
    }))
  }
}
```

### Privacy Toggles Component

**File:** `frontend/app/components/TimerPrivacyToggles.vue` (NEW)

```vue
<template>
  <div class="space-y-4">
    <!-- Toggle 1: Make Timer Public -->
    <div class="flex items-center justify-between">
      <div>
        <label class="text-sm font-medium text-gray-900">
          Make Timer Public
        </label>
        <p class="text-sm text-gray-500">
          Allow others to view your timer at /{{ user.slug }}/incident-timer
        </p>
      </div>
      <button
        @click="togglePublic"
        :disabled="isLoading"
        :class="toggleButtonClass(isPublic)"
        aria-label="Toggle timer public visibility"
      >
        <span :class="toggleThumbClass(isPublic)" />
      </button>
    </div>

    <!-- Toggle 2: Show in List (disabled if not public) -->
    <div class="flex items-center justify-between">
      <div>
        <label
          :class="[
            'text-sm font-medium',
            !isPublic ? 'text-gray-400' : 'text-gray-900'
          ]"
        >
          Show in Public List
        </label>
        <p
          :class="[
            'text-sm',
            !isPublic ? 'text-gray-400' : 'text-gray-500'
          ]"
        >
          Display your timer in the public timers list
        </p>
      </div>
      <button
        @click="toggleShowInList"
        :disabled="isLoading || !isPublic"
        :class="toggleButtonClass(showInList && isPublic)"
        aria-label="Toggle show in public list"
      >
        <span :class="toggleThumbClass(showInList && isPublic)" />
      </button>
    </div>

    <!-- Info message when not public -->
    <div
      v-if="!isPublic"
      class="text-sm text-gray-500 bg-gray-50 p-3 rounded-md"
    >
      ℹ️ Enable "Make Timer Public" first to show in the public list
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import { useTimerPrivacy } from '~/composables/useTimerPrivacy'

const props = defineProps({
  user: { type: Object, required: true }
})

const { updateTimerPrivacy, isLoading, currentSettings } = useTimerPrivacy()

const isPublic = ref(currentSettings.value.isPublic)
const showInList = ref(currentSettings.value.showInList)

// Watch for prop changes (when user data updates)
watch(() => props.user, (newUser) => {
  isPublic.value = newUser.timer_is_public ?? false
  showInList.value = newUser.timer_show_in_list ?? false
}, { immediate: true })

const togglePublic = async () => {
  const newPublic = !isPublic.value
  // Force showInList to false if making private
  const newShowInList = newPublic ? showInList.value : false

  await updateTimerPrivacy(newPublic, newShowInList)

  isPublic.value = newPublic
  showInList.value = newShowInList
}

const toggleShowInList = async () => {
  if (!isPublic.value) return

  const newShowInList = !showInList.value
  await updateTimerPrivacy(isPublic.value, newShowInList)

  showInList.value = newShowInList
}

const toggleButtonClass = (active) => [
  'relative inline-flex h-6 w-11 items-center rounded-full transition-colors',
  active ? 'bg-sky-600' : 'bg-gray-200',
  isLoading.value ? 'opacity-50 cursor-not-allowed' : ''
]

const toggleThumbClass = (active) => [
  'inline-block h-4 w-4 transform rounded-full bg-white transition-transform',
  active ? 'translate-x-6' : 'translate-x-1'
]
</script>
```

### Profile Page Integration

**File:** `frontend/app/pages/profile.vue` (UPDATE)

Add new section after Security section:

```vue
<!-- Timer Privacy Settings -->
<div class="bg-white shadow rounded-lg mb-8">
  <div class="px-6 py-4 border-b border-gray-200">
    <h2 class="text-lg font-medium text-gray-900">Timer Privacy</h2>
    <p class="mt-1 text-sm text-gray-500">
      Control who can see your incident timer.
    </p>
  </div>
  <div class="px-6 py-4">
    <TimerPrivacyToggles v-if="user" :user="user" />
  </div>
</div>
```

### Public Timer List Page (Standalone)

**File:** `frontend/app/pages/public-timers.vue` (NEW)

```vue
<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-50 px-4 sm:px-6 lg:px-8 py-8">
    <div class="max-w-6xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <h1 class="text-3xl sm:text-4xl font-bold text-gray-900 mb-2">
          Public
          <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-indigo-600">
            Timers
          </span>
        </h1>
        <p class="text-gray-600 text-lg">
          See who's tracking their incident-free time publicly.
        </p>
      </div>

      <!-- CTA for non-authenticated users -->
      <div
        v-if="!user"
        class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 p-6 mb-8"
      >
        <h2 class="text-xl font-semibold text-gray-900 mb-2">
          Want to track your own timer?
        </h2>
        <p class="text-gray-600 mb-4">
          Sign in or create an account to start tracking your incident-free periods.
        </p>
        <div class="flex gap-3">
          <NuxtLink
            to="/login"
            class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
          >
            Sign In
          </NuxtLink>
          <NuxtLink
            to="/register"
            class="px-4 py-2 border border-blue-600 text-blue-600 rounded-md hover:bg-blue-50 transition-colors"
          >
            Create Account
          </NuxtLink>
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="isLoading" class="text-center py-8">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
      </div>

      <!-- Empty State -->
      <div v-else-if="timers.length === 0" class="text-center py-16">
        <p class="text-gray-600">No public timers available yet.</p>
      </div>

      <!-- Timer Grid -->
      <div v-else class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <NuxtLink
          v-for="timer in timers"
          :key="timer.id"
          :to="`/${timer.slug}/incident-timer`"
          class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 p-6 hover:shadow-xl transition-shadow"
        >
          <h3 class="text-lg font-semibold text-gray-900 mb-2">
            {{ timer.display_name }}
          </h3>
          <p class="text-sm text-gray-600 mb-3">
            Last reset: {{ formatDate(timer.reset_timestamp) }}
          </p>
          <p
            v-if="timer.notes"
            class="text-sm text-gray-500 line-clamp-2"
          >
            {{ timer.notes }}
          </p>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { userService } from '~/services/userService'
import { useSmartFetch } from '~/composables/useSmartFetch'

const { user } = useUserSession()
const smartFetch = useSmartFetch()
const { getPublicTimerList } = userService(smartFetch)

const timers = ref([])
const isLoading = ref(true)

const formatDate = (timestamp) => {
  return new Date(timestamp).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

onMounted(async () => {
  try {
    timers.value = await getPublicTimerList()
  } catch (error) {
    console.error('Failed to load public timers:', error)
  } finally {
    isLoading.value = false
  }
})

useHead({
  title: 'Public Timers',
  meta: [
    {
      name: 'description',
      content: 'View public incident-free timers from the community.'
    }
  ]
})
</script>
```

### Public Timer List Tab (Authenticated)

**File:** `frontend/app/components/incident-timers/PublicTimersTab.vue` (NEW)

```vue
<template>
  <div class="p-6">
    <h2 class="text-xl font-semibold text-gray-900 mb-4">
      Public Timers
    </h2>

    <!-- Loading State -->
    <div v-if="isLoading" class="text-center py-8">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
    </div>

    <!-- Empty State -->
    <div v-else-if="timers.length === 0" class="text-center py-16">
      <p class="text-gray-600">No public timers available yet.</p>
    </div>

    <!-- Timer Grid (no CTA for authenticated users) -->
    <div v-else class="grid gap-4 sm:grid-cols-2">
      <NuxtLink
        v-for="timer in timers"
        :key="timer.id"
        :to="`/${timer.slug}/incident-timer`"
        class="border border-gray-200 rounded-lg p-4 hover:bg-gray-50 transition-colors"
      >
        <h3 class="font-medium text-gray-900 mb-1">
          {{ timer.display_name }}
        </h3>
        <p class="text-sm text-gray-600 mb-2">
          Last reset: {{ formatDate(timer.reset_timestamp) }}
        </p>
        <p
          v-if="timer.notes"
          class="text-sm text-gray-500 line-clamp-2"
        >
          {{ timer.notes }}
        </p>
      </NuxtLink>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { userService } from '~/services/userService'
import { useSmartFetch } from '~/composables/useSmartFetch'

const smartFetch = useSmartFetch()
const { getPublicTimerList } = userService(smartFetch)

const timers = ref([])
const isLoading = ref(true)

const formatDate = (timestamp) => {
  return new Date(timestamp).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

onMounted(async () => {
  try {
    timers.value = await getPublicTimerList()
  } catch (error) {
    console.error('Failed to load public timers:', error)
  } finally {
    isLoading.value = false
  }
})
</script>
```

### Update Tab Navigation

**File:** `frontend/app/components/incident-timers/TabNavigation.vue` (UPDATE)

Add new tab button:

```vue
<button
  @click="setActiveTab('public-timers')"
  :class="tabClass('public-timers')"
>
  Public Timers
</button>
```

**File:** `frontend/app/pages/incidents.vue` (UPDATE)

Add tab content:

```vue
<PublicTimersTab
  v-else-if="incidentTimerStore.activeTab === 'public-timers'"
/>
```

---

## Testing Strategy

### Backend TDD Test Plan (14 Tests)

#### Repository Tests (6 tests)
1. ✅ **Update privacy - both flags true:** Verify successful update
2. ✅ **Update privacy - public true, list false:** Verify allowed
3. ✅ **Update privacy - both false:** Verify successful privacy disable
4. ✅ **Update privacy - validation error:** FAIL when `show_in_list=true` but `is_public=false`
5. ✅ **Get public list - returns only visible:** Only users with both flags true
6. ✅ **Get public list - excludes private:** Users with flags false are excluded

#### Service Tests (4 tests)
1. ✅ **Update timer privacy - success:** Verify service method delegates correctly
2. ✅ **Update timer privacy - validation error:** Verify error propagation
3. ✅ **Get public timer list - pagination:** Verify pagination logic
4. ✅ **Timer visibility check:** `get_latest_by_user_slug` returns None when not public

#### API Tests (4 tests)
1. ✅ **PUT /profile/timer-privacy - success:** 200 response with updated user
2. ✅ **PUT /profile/timer-privacy - validation error:** 400 response with error message
3. ✅ **GET /public-timers - success:** 200 with paginated results
4. ✅ **GET /:user_slug/incident-timer - 404 when private:** Returns 404 when timer not public

### Frontend Testing

#### Manual Test Cases
1. **Toggle Interaction:**
   - Verify Toggle 2 is disabled when Toggle 1 is off
   - Verify Toggle 2 automatically turns off when Toggle 1 is turned off

2. **List Page (Non-authenticated):**
   - Verify list displays
   - Verify CTA shows
   - Verify links work

3. **List Page (Authenticated):**
   - Verify list displays in tab
   - Verify NO CTA shows
   - Verify links work

4. **Privacy Enforcement:**
   - Set timer to public only (not in list) → Verify appears on public page but not in list
   - Set both flags → Verify appears everywhere
   - Make private → Verify 404 on public page

#### Integration Test Scenarios
1. **Privacy Workflow:**
   - Start with both flags false
   - Enable public only → Access public page (success)
   - Check list page (user not shown)
   - Enable show in list → Check list page (user shown)
   - Disable public → Access public page (404)

2. **Pagination:**
   - Create 25+ users with public timers
   - Verify pagination works
   - Verify page_size limit enforced (max 100)

---

## Implementation Checklist

### Phase 1: Database & Backend Core
- [ ] Create migration file `20251010_add_timer_privacy_flags`
- [ ] Run migration on dev database
- [ ] Update `User` model with new fields
- [ ] Create `UserWithTimer` type
- [ ] Update user repository trait
- [ ] Implement PostgreSQL repository methods
- [ ] Write repository tests (6 tests)
- [ ] Update user service
- [ ] Write service tests (4 tests)
- [ ] Update SQLx cache (`./scripts/prepare-sqlx.sh --clean`)

### Phase 2: Backend API
- [ ] Create API models (`UpdateTimerPrivacyRequest`, etc.)
- [ ] Implement API routes (`update_timer_privacy`, `get_public_timer_list`)
- [ ] Update timer service privacy check
- [ ] Register routes in `mod.rs`
- [ ] Write API tests (4 tests)
- [ ] Verify all 14 tests pass (`cargo test`)

### Phase 3: Frontend Core
- [ ] Create type definitions (`timer-privacy.ts`)
- [ ] Update API routes config
- [ ] Create user service (`userService.ts`)
- [ ] Create composable (`useTimerPrivacy.ts`)

### Phase 4: Frontend UI
- [ ] Create `TimerPrivacyToggles.vue` component
- [ ] Update profile page with privacy section
- [ ] Create standalone list page (`public-timers.vue`)
- [ ] Create authenticated tab (`PublicTimersTab.vue`)
- [ ] Update tab navigation
- [ ] Update incidents page with new tab

### Phase 5: Testing & Validation
- [ ] Manual test toggle interactions
- [ ] Test list page (logged in/out)
- [ ] Verify 404 when timer not public
- [ ] Test pagination
- [ ] Integration test full privacy workflow
- [ ] Performance test list query with large dataset

---

## Performance Considerations

### Database Optimization
- **Composite Index:** `idx_users_timer_visibility` optimizes list queries
- **WHERE clause in index:** Partial index only on relevant rows
- **Pagination:** Enforced max page size (100) prevents abuse

### Query Performance
```sql
-- Optimized query uses composite index
EXPLAIN ANALYZE
SELECT u.id, u.display_name, u.slug, u.created_at, it.reset_timestamp, it.notes
FROM users u
INNER JOIN incident_timers it ON u.id = it.user_id
WHERE u.timer_is_public = true AND u.timer_show_in_list = true
ORDER BY it.reset_timestamp DESC
LIMIT 20 OFFSET 0;

-- Expected: Index Scan using idx_users_timer_visibility
```

### Caching Opportunities (Future)
- Cache public timer list with 5-minute TTL
- Invalidate cache on privacy setting changes
- Use Redis for distributed caching in production

---

## Security Considerations

### Privacy Enforcement
- **Default to Private:** Both flags default to `false`
- **Dependency Validation:** Backend validates `show_in_list` requires `is_public`
- **404 on Private:** No information leakage about timer existence
- **User Ownership:** Only user can change their own privacy settings

### API Security
- **Authentication:** Privacy updates require authentication
- **Rate Limiting:** Consider rate limits on list endpoint (future)
- **Input Validation:** Pagination parameters validated and capped

---

## Edge Cases & Error Handling

### Edge Cases
1. **User with timer but deleted incident_timer:** List query uses INNER JOIN (excludes)
2. **User enables list before public:** Backend validation prevents (returns error)
3. **Very long notes in list:** Frontend uses `line-clamp-2` for truncation
4. **No timers in list:** Empty state with helpful message

### Error Handling
1. **Backend validation error:** 400 response with descriptive message
2. **Database error:** 500 response with generic error (log details)
3. **Frontend network error:** Display error message, allow retry
4. **Session expired during update:** Redirect to login

---

## Future Enhancements

### Phase 2 (Post-MVP)
1. **Search & Filter:** Search timers by username
2. **Sorting Options:** Sort by name, reset date, incident-free duration
3. **Infinite Scroll:** Replace pagination with infinite scroll
4. **Timer Cards:** Rich card UI with duration calculation
5. **Social Features:** Comments, reactions on public timers

### Phase 3 (Advanced)
1. **Analytics:** Track public timer views
2. **Badges:** Achievement badges for incident-free milestones
3. **Leaderboards:** Top incident-free users
4. **Notifications:** Notify when followed user resets timer

---

## Migration Notes

### Breaking Changes
- ✅ None - new columns with defaults, backward compatible

### Rollback Plan
```sql
-- If issues arise, rollback with down migration:
./scripts/rollback-migration.sh 20251010_add_timer_privacy_flags
```

### Data Migration
- No existing data migration needed
- All users start with `timer_is_public = false` (private by default)
- Users must opt-in via profile settings

---

## Success Metrics

### Launch Criteria
- ✅ All 14 backend tests pass
- ✅ Privacy validation works correctly
- ✅ List page loads with <500ms response time
- ✅ Toggle interactions work as expected
- ✅ 404 correctly shown for private timers

### Post-Launch Metrics
- % of users who make timers public
- % of public users who opt into list
- Public list page views
- Click-through rate from list to individual timers

---

## Questions & Decisions

### Open Questions
1. **Pagination UX:** Should we use infinite scroll or traditional pagination?
   - **Decision:** Start with traditional pagination, iterate based on feedback

2. **Timer Visibility Levels:** Do we need granular privacy (friends-only, etc.)?
   - **Decision:** Start with binary public/private, iterate if needed

3. **Email Notifications:** Notify users when they appear in public list?
   - **Decision:** Not in MVP, consider for Phase 2

### Resolved Decisions
- ✅ Use boolean flags instead of enum (simpler, more flexible)
- ✅ Default to private (privacy-first approach)
- ✅ Validate dependency in backend (data integrity)
- ✅ Show list as tab for authenticated, standalone for non-authenticated

---

## Resources & References

### Related Files
- Database schema: `backend/migrations/20250914134643_initial_schema.up.sql`
- User model: `backend/src/models/db/user.rs`
- Timer routes: `backend/src/routes/incident_timers.rs`
- Public timer page: `frontend/app/pages/[user_slug]/incident-timer.vue`
- Incidents page: `frontend/app/pages/incidents.vue`
- Profile page: `frontend/app/pages/profile.vue`

### Documentation
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture
- [CODING-RULES.md](CODING-RULES.md) - Development standards
- [DEVELOPMENT-WORKFLOW.md](DEVELOPMENT-WORKFLOW.md) - Development workflows

---

**Document Version:** 1.0
**Last Updated:** 2025-10-10
**Next Review:** After implementation
