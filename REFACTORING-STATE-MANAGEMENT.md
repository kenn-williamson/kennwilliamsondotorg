# Complete State Management Refactoring Guide

## Overview

This document details the complete refactoring from our current "Action Composable" pattern to a centralized "Store with Actions" pattern across ALL stores and composables in the frontend.

## Current Architecture Analysis

### Current Pattern Issues
1. **Fragmented State**: `isLoading` and `error` live in `useBaseService`, while business data lives in stores
2. **Extra Abstraction Layer**: Action composables act as middlemen between components and stores
3. **Complex Dependencies**: Components depend on both store and action composable
4. **Testing Complexity**: Need to mock both service and store setters
5. **SSR/Hydration Issues**: Multiple sources of state cause hydration mismatches

### Current Flow
```
Component → Action Composable → useBaseService + Store → Service
```

## New Architecture

### New Pattern Benefits
1. **Centralized State**: All state (`data`, `isLoading`, `error`) lives in one store
2. **Simplified Flow**: Direct component ↔ store communication
3. **Easier Testing**: Test store actions directly with mocked services
4. **Better SSR**: Single source of truth for all state
5. **Reduced Complexity**: Eliminate middleman composables

### New Flow
```
Component → Store (with actions) → Service
```

## Complete Refactoring Scope

### Stores to Refactor (3 Total)
1. **`stores/admin.ts`** - Admin panel state management
2. **`stores/incident-timers.ts`** - Timer CRUD operations
3. **`stores/phrases.ts`** - Phrase management and suggestions

### Action Composables to Remove (5 Total)
1. **`useAdminActions.ts`** - Admin operations orchestration
2. **`useAuthActions.ts`** - Authentication operations orchestration
3. **`useAuthProfileActions.ts`** - Profile management operations orchestration
4. **`useIncidentTimerActions.ts`** - Timer operations orchestration
5. **`usePhrasesActions.ts`** - Phrase operations orchestration

### Supporting Composables to Keep (4 Total)
1. **`useBaseService.ts`** - **REMOVE** (logic moves to stores)
2. **`useBackendFetch.ts`** - **KEEP** (HTTP client)
3. **`useAuthFetch.ts`** - **KEEP** (SSR auth client)
4. **`useJwtManager.ts`** - **KEEP** (JWT token management)

## Detailed Refactoring Plan

### Phase 1: Admin Store Refactoring

#### Current Files
- **Store**: `stores/admin.ts` (pure state management)
- **Actions**: `useAdminActions.ts` (orchestration layer)
- **Components**: All admin components use `useAdminActions()`

#### Refactoring Steps

**Step 1: Enhance Admin Store**
```typescript
// File: stores/admin.ts
import { defineStore } from 'pinia'
import { adminService } from '~/services/adminService'
import { useBackendFetch } from '~/composables/useBackendFetch'

export const useAdminStore = defineStore('admin', () => {
  // State - add transient state
  const users = ref<User[]>([])
  const suggestions = ref<PhraseSuggestion[]>([])
  const stats = ref<AdminStats | null>(null)
  const searchQuery = ref('')
  const selectedUser = ref<User | null>(null)
  const newPassword = ref<string | null>(null)
  
  // NEW: Transient state (moved from useBaseService)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Computed
  const filteredUsers = computed((): User[] => {
    if (!searchQuery.value.trim()) return users.value
    const query = searchQuery.value.toLowerCase()
    return users.value.filter(user => 
      user.display_name.toLowerCase().includes(query) ||
      user.email.toLowerCase().includes(query)
    )
  })

  const pendingSuggestions = computed((): PhraseSuggestion[] => {
    return suggestions.value
  })

  const hasError = computed(() => !!error.value)

  // Service instance
  const backendFetch = useBackendFetch()
  const adminServiceInstance = adminService(backendFetch)

  // NEW: Private action handler (replaces useBaseService logic)
  const _handleAction = async <T>(
    action: () => Promise<T>,
    context?: string
  ): Promise<T | undefined> => {
    isLoading.value = true
    error.value = null
    
    try {
      const result = await action()
      return result
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      error.value = errorMessage
      console.error(`[AdminStore] Error${context ? ` in ${context}` : ''}:`, errorMessage)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // NEW: Private success handler
  const _handleSuccess = (message: string): void => {
    console.log(`[AdminStore] Success: ${message}`)
    // TODO: Add toast notifications here
  }

  // NEW: Actions (migrated from useAdminActions)
  const fetchStats = async () => {
    const data = await _handleAction(() => adminServiceInstance.getStats(), 'fetchStats')
    if (data) {
      stats.value = data
    }
    return data
  }

  const fetchUsers = async (searchQueryParam?: string) => {
    const data = await _handleAction(
      () => adminServiceInstance.getUsers(searchQueryParam || searchQuery.value),
      'fetchUsers'
    )
    if (data) {
      users.value = data.users
    }
    return data
  }

  const fetchSuggestions = async () => {
    const data = await _handleAction(() => adminServiceInstance.getSuggestions(), 'fetchSuggestions')
    if (data) {
      suggestions.value = data.suggestions
    }
    return data
  }

  const deactivateUser = async (userId: string) => {
    await _handleAction(() => adminServiceInstance.deactivateUser(userId), 'deactivateUser')
    _handleSuccess('User deactivated successfully')
    
    // Update local state
    const user = users.value.find(u => u.id === userId)
    if (user) {
      user.active = false
    }
  }

  const activateUser = async (userId: string) => {
    await _handleAction(() => adminServiceInstance.activateUser(userId), 'activateUser')
    _handleSuccess('User activated successfully')
    
    // Update local state
    const user = users.value.find(u => u.id === userId)
    if (user) {
      user.active = true
    }
  }

  const resetUserPassword = async (userId: string) => {
    const data = await _handleAction(() => adminServiceInstance.resetUserPassword(userId), 'resetUserPassword')
    _handleSuccess('Password reset successfully')
    
    if (data) {
      newPassword.value = data.new_password
    }
    return data
  }

  const promoteUser = async (userId: string) => {
    await _handleAction(() => adminServiceInstance.promoteUser(userId), 'promoteUser')
    _handleSuccess('User promoted to admin successfully')
    
    // Update local state
    const user = users.value.find(u => u.id === userId)
    if (user) {
      user.roles = [...user.roles, 'admin']
    }
  }

  const approveSuggestion = async (suggestionId: string, adminReason: string) => {
    await _handleAction(() => adminServiceInstance.approveSuggestion(suggestionId, adminReason), 'approveSuggestion')
    _handleSuccess('Suggestion approved successfully')
    
    // Remove from local state
    suggestions.value = suggestions.value.filter(s => s.id !== suggestionId)
  }

  const rejectSuggestion = async (suggestionId: string, adminReason: string) => {
    await _handleAction(() => adminServiceInstance.rejectSuggestion(suggestionId, adminReason), 'rejectSuggestion')
    _handleSuccess('Suggestion rejected successfully')
    
    // Remove from local state
    suggestions.value = suggestions.value.filter(s => s.id !== suggestionId)
  }

  // Utility actions
  const setSearchQuery = (query: string) => {
    searchQuery.value = query
  }

  const setSelectedUser = (user: User | null) => {
    selectedUser.value = user
  }

  const clearNewPassword = () => {
    newPassword.value = null
  }

  const clearState = () => {
    users.value = []
    suggestions.value = []
    stats.value = null
    searchQuery.value = ''
    selectedUser.value = null
    newPassword.value = null
    error.value = null
  }

  return {
    // State
    users: readonly(users),
    suggestions: readonly(suggestions),
    stats: readonly(stats),
    searchQuery: readonly(searchQuery),
    selectedUser: readonly(selectedUser),
    newPassword: readonly(newPassword),
    isLoading: readonly(isLoading),
    error: readonly(error),
    
    // Computed
    filteredUsers,
    pendingSuggestions,
    hasError,
    
    // Actions
    fetchStats,
    fetchUsers,
    fetchSuggestions,
    deactivateUser,
    activateUser,
    resetUserPassword,
    promoteUser,
    approveSuggestion,
    rejectSuggestion,
    setSearchQuery,
    setSelectedUser,
    clearNewPassword,
    clearState
  }
})
```

**Step 2: Update Admin Components**

**File: `components/Admin/OverviewTab.vue`**
```vue
<script setup lang="ts">
// OLD: Import action composable
// import { useAdminActions } from '~/composables/useAdminActions'

// NEW: Import store directly
import { useAdminStore } from '~/stores/admin'

// OLD: Use action composable
// const { fetchStats, isLoading, error } = useAdminActions()

// NEW: Use store directly
const adminStore = useAdminStore()

// Load stats on mount
onMounted(async () => {
  if (!adminStore.stats) {
    await adminStore.fetchStats()
  }
})

// Refresh stats function
const refreshStats = async () => {
  await adminStore.fetchStats()
}
</script>

<template>
  <!-- OLD: Use composable state -->
  <!-- <div v-if="isLoading"> -->
  
  <!-- NEW: Use store state -->
  <div v-if="adminStore.isLoading">
    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
  </div>

  <!-- OLD: Use composable error -->
  <!-- <div v-else-if="error"> -->
  
  <!-- NEW: Use store error -->
  <div v-else-if="adminStore.error">
    <p class="text-red-800 text-sm">{{ adminStore.error }}</p>
    <button @click="refreshStats">Try again</button>
  </div>

  <!-- Rest of template remains the same -->
</template>
```

**Step 3: Update All Admin Components**
- `components/Admin/AdminPanel.vue`
- `components/Admin/OverviewTab.vue`
- `components/Admin/UsersTab.vue`
- `components/Admin/PhraseSuggestionApprovalTab.vue`
- `components/Admin/UserSearchBox.vue`
- `components/Admin/AdminTabNavigation.vue`

**Step 4: Delete Action Composable**
```bash
rm frontend/app/composables/useAdminActions.ts
```

### Phase 2: Auth Store Refactoring

#### Current Files
- **Store**: No dedicated auth store (auth state in session)
- **Actions**: `useAuthActions.ts` (authentication operations)
- **Components**: Login, register, header components

#### Refactoring Steps

**Step 1: Create Auth Store**
```typescript
// File: stores/auth.ts (NEW FILE)
import { defineStore } from 'pinia'
import { authService } from '~/services/authService'
import { useBackendFetch } from '~/composables/useBackendFetch'

export const useAuthStore = defineStore('auth', () => {
  // State
  const user = ref<User | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Computed
  const isAuthenticated = computed(() => !!user.value)
  const hasError = computed(() => !!error.value)

  // Service instance
  const backendFetch = useBackendFetch()
  const authServiceInstance = authService(backendFetch)

  // Private action handler
  const _handleAction = async <T>(
    action: () => Promise<T>,
    context?: string
  ): Promise<T | undefined> => {
    isLoading.value = true
    error.value = null
    
    try {
      const result = await action()
      return result
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      error.value = errorMessage
      console.error(`[AuthStore] Error${context ? ` in ${context}` : ''}:`, errorMessage)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Actions (migrated from useAuthActions)
  const login = async (email: string, password: string) => {
    const data = await _handleAction(() => authServiceInstance.login(email, password), 'login')
    if (data) {
      user.value = data.user
      // Handle JWT tokens (existing logic)
    }
    return data
  }

  const register = async (email: string, password: string, displayName: string) => {
    const data = await _handleAction(() => authServiceInstance.register(email, password, displayName), 'register')
    if (data) {
      user.value = data.user
      // Handle JWT tokens (existing logic)
    }
    return data
  }

  const logout = async () => {
    await _handleAction(() => authServiceInstance.logout(), 'logout')
    user.value = null
    // Clear JWT tokens (existing logic)
  }

  const refreshToken = async () => {
    const data = await _handleAction(() => authServiceInstance.refreshToken(), 'refreshToken')
    if (data) {
      user.value = data.user
      // Handle JWT tokens (existing logic)
    }
    return data
  }

  const getCurrentUser = async () => {
    const data = await _handleAction(() => authServiceInstance.getCurrentUser(), 'getCurrentUser')
    if (data) {
      user.value = data
    }
    return data
  }

  return {
    // State
    user: readonly(user),
    isLoading: readonly(isLoading),
    error: readonly(error),
    
    // Computed
    isAuthenticated,
    hasError,
    
    // Actions
    login,
    register,
    logout,
    refreshToken,
    getCurrentUser
  }
})
```

**Step 2: Update Auth Components**
- `pages/login.vue`
- `pages/register.vue`
- `components/Layout/AppHeader.vue`

**Step 3: Delete Action Composable**
```bash
rm frontend/app/composables/useAuthActions.ts
```

### Phase 3: Auth Profile Store Refactoring

#### Current Files
- **Store**: No dedicated profile store
- **Actions**: `useAuthProfileActions.ts` (profile management)
- **Components**: Profile page components

#### Refactoring Steps

**Step 1: Add Profile Actions to Auth Store**
```typescript
// File: stores/auth.ts (ADD TO EXISTING)
// Add these actions to the existing auth store:

const updateProfile = async (displayName: string, slug: string) => {
  const data = await _handleAction(() => authServiceInstance.updateProfile(displayName, slug), 'updateProfile')
  if (data && user.value) {
    user.value.display_name = data.display_name
    user.value.slug = data.slug
  }
  return data
}

const changePassword = async (currentPassword: string, newPassword: string) => {
  await _handleAction(() => authServiceInstance.changePassword(currentPassword, newPassword), 'changePassword')
}

const previewSlug = async (displayName: string) => {
  const data = await _handleAction(() => authServiceInstance.previewSlug(displayName), 'previewSlug')
  return data
}

// Add to return object:
return {
  // ... existing returns
  updateProfile,
  changePassword,
  previewSlug
}
```

**Step 2: Update Profile Components**
- `pages/profile.vue`
- `components/Profile/AccountInformationForm.vue`
- `components/Profile/SecurityForm.vue`

**Step 3: Delete Action Composable**
```bash
rm frontend/app/composables/useAuthProfileActions.ts
```

### Phase 4: Incident Timer Store Refactoring

#### Current Files
- **Store**: `stores/incident-timers.ts` (pure state management)
- **Actions**: `useIncidentTimerActions.ts` (timer operations)
- **Components**: Timer components

#### Refactoring Steps

**Step 1: Enhance Incident Timer Store**
```typescript
// File: stores/incident-timers.ts
import { defineStore } from 'pinia'
import { incidentTimerService } from '~/services/incidentTimerService'
import { useBackendFetch } from '~/composables/useBackendFetch'

export const useIncidentTimerStore = defineStore('incident-timers', () => {
  // State - add transient state
  const timers = ref<IncidentTimer[]>([])
  const currentTimer = ref<IncidentTimer | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Computed
  const hasError = computed(() => !!error.value)

  // Service instance
  const backendFetch = useBackendFetch()
  const incidentTimerServiceInstance = incidentTimerService(backendFetch)

  // Private action handler
  const _handleAction = async <T>(
    action: () => Promise<T>,
    context?: string
  ): Promise<T | undefined> => {
    isLoading.value = true
    error.value = null
    
    try {
      const result = await action()
      return result
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      error.value = errorMessage
      console.error(`[IncidentTimerStore] Error${context ? ` in ${context}` : ''}:`, errorMessage)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Actions (migrated from useIncidentTimerActions)
  const fetchTimers = async () => {
    const data = await _handleAction(() => incidentTimerServiceInstance.getTimers(), 'fetchTimers')
    if (data) {
      timers.value = data
    }
    return data
  }

  const createTimer = async (timerData: CreateTimerRequest) => {
    const data = await _handleAction(() => incidentTimerServiceInstance.createTimer(timerData), 'createTimer')
    if (data) {
      timers.value.push(data)
    }
    return data
  }

  const updateTimer = async (timerId: string, timerData: UpdateTimerRequest) => {
    const data = await _handleAction(() => incidentTimerServiceInstance.updateTimer(timerId, timerData), 'updateTimer')
    if (data) {
      const index = timers.value.findIndex(t => t.id === timerId)
      if (index !== -1) {
        timers.value[index] = data
      }
    }
    return data
  }

  const deleteTimer = async (timerId: string) => {
    await _handleAction(() => incidentTimerServiceInstance.deleteTimer(timerId), 'deleteTimer')
    timers.value = timers.value.filter(t => t.id !== timerId)
  }

  const getPublicTimer = async (userSlug: string) => {
    const data = await _handleAction(() => incidentTimerServiceInstance.getPublicTimer(userSlug), 'getPublicTimer')
    if (data) {
      currentTimer.value = data
    }
    return data
  }

  return {
    // State
    timers: readonly(timers),
    currentTimer: readonly(currentTimer),
    isLoading: readonly(isLoading),
    error: readonly(error),
    
    // Computed
    hasError,
    
    // Actions
    fetchTimers,
    createTimer,
    updateTimer,
    deleteTimer,
    getPublicTimer
  }
})
```

**Step 2: Update Timer Components**
- `pages/incidents.vue`
- `components/Timer/TimerStats.vue`
- `components/Timer/TimerListItem.vue`
- `components/Timer/TimerEditModal.vue`
- `components/Timer/TimerResetModal.vue`
- `components/Timer/TimerDisplayTab.vue`
- `components/Timer/TimerControlsTab.vue`
- `pages/[user_slug]/incident-timer.vue`

**Step 3: Delete Action Composable**
```bash
rm frontend/app/composables/useIncidentTimerActions.ts
```

### Phase 5: Phrases Store Refactoring

#### Current Files
- **Store**: `stores/phrases.ts` (pure state management)
- **Actions**: `usePhrasesActions.ts` (phrase operations)
- **Components**: Phrase components

#### Refactoring Steps

**Step 1: Enhance Phrases Store**
```typescript
// File: stores/phrases.ts
import { defineStore } from 'pinia'
import { phraseService } from '~/services/phraseService'
import { useBackendFetch } from '~/composables/useBackendFetch'

export const usePhrasesStore = defineStore('phrases', () => {
  // State - add transient state
  const phrases = ref<Phrase[]>([])
  const userPhrases = ref<UserPhrase[]>([])
  const suggestions = ref<PhraseSuggestion[]>([])
  const currentPhrase = ref<string | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Computed
  const hasError = computed(() => !!error.value)
  const activePhrases = computed(() => phrases.value.filter(p => p.active))
  const pendingSuggestions = computed(() => suggestions.value.filter(s => s.status === 'pending'))

  // Service instance
  const backendFetch = useBackendFetch()
  const phraseServiceInstance = phraseService(backendFetch)

  // Private action handler
  const _handleAction = async <T>(
    action: () => Promise<T>,
    context?: string
  ): Promise<T | undefined> => {
    isLoading.value = true
    error.value = null
    
    try {
      const result = await action()
      return result
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      error.value = errorMessage
      console.error(`[PhrasesStore] Error${context ? ` in ${context}` : ''}:`, errorMessage)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Actions (migrated from usePhrasesActions)
  const fetchRandomPhrase = async () => {
    const data = await _handleAction(() => phraseServiceInstance.getRandomPhrase(), 'fetchRandomPhrase')
    if (data) {
      currentPhrase.value = data
    }
    return data
  }

  const fetchUserPhrases = async () => {
    const data = await _handleAction(() => phraseServiceInstance.getUserPhrases(), 'fetchUserPhrases')
    if (data) {
      userPhrases.value = data.phrases
    }
    return data
  }

  const excludePhrase = async (phraseId: string) => {
    await _handleAction(() => phraseServiceInstance.excludePhrase(phraseId), 'excludePhrase')
    const phrase = userPhrases.value.find(p => p.id === phraseId)
    if (phrase) {
      phrase.is_excluded = true
    }
  }

  const removePhraseExclusion = async (phraseId: string) => {
    await _handleAction(() => phraseServiceInstance.removePhraseExclusion(phraseId), 'removePhraseExclusion')
    const phrase = userPhrases.value.find(p => p.id === phraseId)
    if (phrase) {
      phrase.is_excluded = false
    }
  }

  const submitSuggestion = async (phraseText: string) => {
    const data = await _handleAction(() => phraseServiceInstance.submitSuggestion(phraseText), 'submitSuggestion')
    if (data) {
      suggestions.value.push(data.suggestion)
    }
    return data
  }

  const fetchSuggestions = async () => {
    const data = await _handleAction(() => phraseServiceInstance.getSuggestions(), 'fetchSuggestions')
    if (data) {
      suggestions.value = data.suggestions
    }
    return data
  }

  return {
    // State
    phrases: readonly(phrases),
    userPhrases: readonly(userPhrases),
    suggestions: readonly(suggestions),
    currentPhrase: readonly(currentPhrase),
    isLoading: readonly(isLoading),
    error: readonly(error),
    
    // Computed
    hasError,
    activePhrases,
    pendingSuggestions,
    
    // Actions
    fetchRandomPhrase,
    fetchUserPhrases,
    excludePhrase,
    removePhraseExclusion,
    submitSuggestion,
    fetchSuggestions
  }
})
```

**Step 2: Update Phrase Components**
- `components/Phrases/RandomPhrase.vue`
- `components/Timer/PhraseSuggestionsTab.vue`
- `components/Timer/PhraseFilterTab.vue`
- `components/Timer/SuggestionHistoryTab.vue`

**Step 3: Delete Action Composable**
```bash
rm frontend/app/composables/usePhrasesActions.ts
```

### Phase 6: Cleanup

#### Remove Supporting Files
```bash
rm frontend/app/composables/useBaseService.ts
```

#### Update Component Imports
Search and replace across all components:
- Remove: `import { useXxxActions } from '~/composables/useXxxActions'`
- Add: `import { useXxxStore } from '~/stores/xxx'`
- Update: Replace `const { action, isLoading, error } = useXxxActions()` with `const store = useXxxStore()`
- Update: Replace `action()` with `store.action()`
- Update: Replace `isLoading` with `store.isLoading`
- Update: Replace `error` with `store.error`

## Testing Strategy Update

### New Testing Pattern

**OLD PATTERN (Testing Action Composables)**
```typescript
// Complex mocking of both service and store
vi.mock('~/services/adminService')
vi.mock('~/stores/admin', () => ({
  useAdminStore: () => ({
    setUsers: vi.fn(),
    setSuggestions: vi.fn(),
  }),
}))

it('calls service and then calls the store setter', () => {
  const { fetchUsers } = useAdminActions()
  await fetchUsers()
  expect(adminService.getUsers).toHaveBeenCalled()
  expect(useAdminStore().setUsers).toHaveBeenCalled()
})
```

**NEW PATTERN (Testing Store Directly)**
```typescript
// File: stores/admin.spec.ts
import { setActivePinia, createPinia } from 'pinia'
import { useAdminStore } from './admin'
import { adminService } from '~/services/adminService'

// Mock only the service layer
vi.mock('~/services/adminService', () => ({
  adminService: {
    getUsers: vi.fn(),
    getStats: vi.fn(),
  },
}))

describe('Admin Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('fetchUsers should call service and update state on success', async () => {
    // Arrange
    const store = useAdminStore()
    const mockUsers = [{ id: '1', name: 'John Doe' }]
    vi.mocked(adminService.getUsers).mockResolvedValue({ users: mockUsers })

    // Act
    await store.fetchUsers('john')

    // Assert
    expect(adminService.getUsers).toHaveBeenCalledWith('john')
    expect(store.users).toEqual(mockUsers)
    expect(store.isLoading).toBe(false)
    expect(store.error).toBe(null)
  })

  it('fetchUsers should set error state on failure', async () => {
    // Arrange
    const store = useAdminStore()
    const mockError = new Error('API Failure')
    vi.mocked(adminService.getUsers).mockRejectedValue(mockError)

    // Act
    await store.fetchUsers()

    // Assert
    expect(store.error).toEqual(mockError)
    expect(store.users).toEqual([])
    expect(store.isLoading).toBe(false)
  })
})
```

### Test Files to Create
1. `stores/admin.spec.ts`
2. `stores/auth.spec.ts`
3. `stores/incident-timers.spec.ts`
4. `stores/phrases.spec.ts`

## Migration Checklist

### Pre-Migration
- [ ] Backup current codebase
- [ ] Document current component usage patterns
- [ ] Create test database for testing

### Phase 1: Admin Store
- [ ] Enhance `stores/admin.ts` with actions and transient state
- [ ] Update all admin components to use store directly
- [ ] Test admin functionality
- [ ] Delete `useAdminActions.ts`
- [ ] Create `stores/admin.spec.ts`

### Phase 2: Auth Store
- [ ] Create `stores/auth.ts` with authentication actions
- [ ] Update login/register/header components
- [ ] Test authentication flow
- [ ] Delete `useAuthActions.ts`
- [ ] Create `stores/auth.spec.ts`

### Phase 3: Auth Profile Store
- [ ] Add profile actions to `stores/auth.ts`
- [ ] Update profile components
- [ ] Test profile management
- [ ] Delete `useAuthProfileActions.ts`

### Phase 4: Incident Timer Store
- [ ] Enhance `stores/incident-timers.ts` with actions
- [ ] Update all timer components
- [ ] Test timer functionality
- [ ] Delete `useIncidentTimerActions.ts`
- [ ] Create `stores/incident-timers.spec.ts`

### Phase 5: Phrases Store
- [ ] Enhance `stores/phrases.ts` with actions
- [ ] Update phrase components
- [ ] Test phrase functionality
- [ ] Delete `usePhrasesActions.ts`
- [ ] Create `stores/phrases.spec.ts`

### Phase 6: Cleanup
- [ ] Delete `useBaseService.ts`
- [ ] Update all remaining component imports
- [ ] Run full test suite
- [ ] Update documentation

### Post-Migration
- [ ] Verify all functionality works
- [ ] Performance testing
- [ ] Update development workflow documentation
- [ ] Update architecture documentation

## Benefits After Refactoring

1. **Simplified Architecture**: Direct component ↔ store communication
2. **Centralized State**: All related state in one place
3. **Easier Testing**: Test store actions directly
4. **Better SSR**: Single source of truth prevents hydration issues
5. **Reduced Complexity**: Eliminate middleman composables
6. **Improved Maintainability**: Clear separation of concerns
7. **Better Performance**: Fewer abstraction layers

## Risk Mitigation

1. **Incremental Migration**: Refactor one store at a time
2. **Comprehensive Testing**: Test each phase thoroughly
3. **Rollback Plan**: Keep backup of current implementation
4. **Documentation**: Update all documentation as we go
5. **Team Communication**: Ensure all team members understand the changes

---

*This document serves as the complete roadmap for refactoring our state management architecture. Follow the phases sequentially and test thoroughly at each step.*
