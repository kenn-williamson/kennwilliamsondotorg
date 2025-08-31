# HTTP Client Architecture Refactor - Vue 3 Composables & Interceptors

## Overview
Refactor from inheritance-based BaseService pattern to modern Vue 3 composable-based HTTP client with interceptors. This architecture eliminates inheritance, provides automatic authentication handling, and prepares for future refresh token implementation.

## Research Summary & Reasoning

### Vue 3 Best Practices (2024-2025)
- **Composables over inheritance**: Vue 3 explicitly moved away from mixins/inheritance toward composition patterns
- **HTTP interceptors are standard**: Custom fetch composables with interceptors are the recommended Nuxt 3 pattern
- **Repository pattern**: Clean separation of API layer from components using composables
- **Horizontal composition**: Modern practices favor composable functions over vertical inheritance hierarchies
- **Future-proof architecture**: Interceptors handle cross-cutting concerns like token refresh transparently

### Problems with Current BaseService Inheritance
- Creates tight coupling between services and base class
- Requires passing authStore to every protected method call
- Not idiomatic Vue 3 - inheritance pattern is discouraged
- Difficult to extend with cross-cutting concerns (logging, caching, etc.)
- Makes testing more complex due to inheritance dependencies

### Benefits of New Architecture
- **Auto-authentication**: Headers injected automatically via interceptors
- **Refresh token ready**: Response interceptors can handle 401s transparently
- **Composable pattern**: Services are composable functions, not classes
- **Dependency injection**: Clean injection of HTTP client and auth state
- **Maintainable**: No inheritance hierarchy, easier to reason about
- **Testable**: Easy to mock individual composables

## New Architecture Overview

### Core Composables
1. **useAuthFetch**: Custom fetch client with auth interceptors
2. **useAuthService**: Authentication operations (login, register, me)
3. **useIncidentTimerService**: Timer CRUD operations
4. **useServices**: Aggregator composable for all services

### HTTP Interceptor Pattern
- **Request interceptor**: Auto-inject auth headers from store
- **Response interceptor**: Handle 401 errors, trigger token refresh
- **Error handling**: Consistent error transformation across all requests
- **Logging**: Optional request/response logging for development

### Service Pattern
- Services are composable functions returning methods
- No classes or inheritance
- Each service uses the auth-enabled fetch client
- Clean, functional approach with dependency injection

## Files Requiring Refactoring

### 1. Create New Files
- `frontend/app/composables/useAuthFetch.ts` - Core HTTP client with interceptors
- `frontend/app/composables/useAuthService.ts` - Auth service as composable
- `frontend/app/composables/useIncidentTimerService.ts` - Timer service as composable

### 2. Remove Files
- `frontend/app/services/base.service.ts` - Delete inheritance base class
- `frontend/app/services/auth.service.ts` - Replace with composable
- `frontend/app/services/incident-timer.service.ts` - Replace with composable

### 3. Update Existing Files

#### `frontend/app/composables/useServices.ts` (Lines 1-16)
- **Current**: Creates service class instances with apiBase parameter
- **Refactor**: Return composable functions instead of class instances
- **Pattern**: Aggregate all service composables in one place

#### `frontend/app/stores/auth.ts` (Lines 49-81, 83-114, 138-161)
- **Line 53**: Remove `const { authService } = useServices()` call from login method
- **Line 87**: Remove authService parameter from register method call
- **Line 151**: Update checkAuth method to use new useAuthService composable
- **Line 174**: Keep getAuthHeaders method (used by interceptor)
- **Pattern**: Store methods call service composables directly

#### `frontend/app/stores/incident-timers.ts` (Lines 179, 227, 254, 288)
- **Line 179**: `incidentTimerService.getUserTimers(authStore)` → remove authStore param
- **Line 227**: `incidentTimerService.createTimer(timerData, authStore)` → remove authStore param  
- **Line 254**: `incidentTimerService.updateTimer(id, updates, authStore)` → remove authStore param
- **Line 288**: `incidentTimerService.deleteTimer(id, authStore)` → remove authStore param
- **Pattern**: All service calls become parameter-free (auth handled by interceptor)

#### `frontend/app/pages/register.vue` (Line 242)
- **Line 242**: `await authStore.register({ display_name, email, password }, authService)`
- **Refactor**: Remove authService parameter → `await authStore.register({ display_name, email, password })`
- **Pattern**: Store method calls become cleaner

#### `frontend/app/pages/login.vue` (Line 145)
- **Line 145**: `const result = await authStore.login({ email, password })`
- **Status**: Already clean, no authService parameter
- **Verification**: Ensure login flow works with new pattern

#### `frontend/app/pages/incidents.vue` (Lines 64, 276, 297)
- **Line 64**: Update user slug link reference (relies on store state)
- **Lines 276, 297**: Auth check watchers (no changes needed)
- **Pattern**: Service calls handled through store, auth automatic

#### `frontend/app/pages/index.vue` (Line 93)
- **Line 93**: `authStore.checkAuth()` call in onMounted
- **Status**: No changes needed, store method updated internally
- **Verification**: Ensure SSR compatibility maintained

#### `frontend/app/middleware/auth.ts` (Lines 5, 7, 10)
- **Lines 5, 7, 10**: Uses `authStore.isAuthenticated` for route protection
- **Status**: No changes needed, store state remains same
- **Verification**: Ensure middleware still functions correctly

#### `frontend/app/components/AppHeader.vue` (Lines 38, 137, 139, 186-187, 206)
- **Lines 38, 137**: Authentication state checks for UI rendering
- **Line 139**: Display user email from store
- **Lines 186-187**: User initial calculation from store
- **Line 206**: Logout method call
- **Status**: No changes needed, all use store state/methods
- **Verification**: Ensure UI updates correctly with new auth flow

## Implementation Strategy

### Phase 1: Create HTTP Client Foundation
1. Create useAuthFetch composable with request/response interceptors
2. Add automatic auth header injection from store
3. Add error handling for 401 responses
4. Test with simple API call

### Phase 2: Refactor Auth Service
1. Convert AuthService class to useAuthService composable
2. Remove authStore parameters from all methods
3. Update auth store to use new composable
4. Test login/register/me functionality

### Phase 3: Refactor Timer Service  
1. Convert IncidentTimerService class to useIncidentTimerService composable
2. Remove authStore parameters from all CRUD methods
3. Update incident-timers store to use new composable
4. Update pages/incidents.vue to use simplified API

### Phase 4: Clean Up & Test
1. Delete old service class files
2. Update useServices aggregator
3. Test all auth and timer functionality end-to-end
4. Verify SSR compatibility

## Interceptor Implementation Details

### Request Interceptor Responsibilities
- Get current auth token from store
- Inject Authorization header automatically
- Add any global headers (Content-Type, etc.)
- Handle development logging if enabled

### Response Interceptor Responsibilities  
- Transform errors into consistent format
- Handle 401 responses (future: trigger token refresh)
- Log responses in development mode
- Handle network errors gracefully

### Future Token Refresh Integration
- Response interceptor detects 401 responses
- Automatically attempt token refresh
- Retry original request with new token
- Logout user if refresh fails
- All handled transparently to calling code

## Testing Strategy

### Unit Testing
- Mock useAuthFetch composable in service tests
- Test interceptor behavior in isolation
- Verify error handling and transformation

### Integration Testing
- Test full auth flow with new architecture
- Verify timer CRUD operations work without auth parameters
- Test 401 handling and error states

### Migration Testing
- Verify all existing functionality works unchanged
- Test SSR compatibility with new composables
- Performance testing to ensure no regressions

## Rollback Strategy
If issues arise during refactoring:
1. Keep old service files until new implementation is verified
2. Git branch for refactor with easy revert capability
3. Feature flags to switch between old/new implementations
4. Incremental rollout - auth first, then timers

## Success Criteria
- ✅ All API calls work without passing authStore parameters
- ✅ Authentication headers injected automatically
- ✅ Clean, inheritance-free service layer
- ✅ Preparation for refresh token implementation
- ✅ No regression in existing functionality
- ✅ Improved developer experience and maintainability

---

**Status**: Design Complete - Ready for Implementation  
**Next**: Begin Phase 1 - Create useAuthFetch composable foundation