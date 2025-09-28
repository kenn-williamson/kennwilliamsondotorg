/**
 * Centralized Admin Store - State management with actions
 */

import type { User, AdminStats } from '#shared/types'
import type { PhraseSuggestion } from '#shared/types/phrases'
import { adminService } from '~/services/adminService'
import { useSmartFetch } from '~/composables/useSmartFetch'
import { useSessionWatcher } from '~/composables/useSessionWatcher'
import { useDebounceFn } from '@vueuse/core'

export const useAdminStore = defineStore('admin', () => {
  const users = ref<User[]>([])
  const suggestions = ref<PhraseSuggestion[]>([])
  const stats = ref<AdminStats | null>(null)
  const searchQuery = ref('')
  const selectedUser = ref<User | null>(null)
  const newPassword = ref<string | null>(null)
  const activeTab = ref('overview')
  
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  
  // Remove client-side filtering - use server-side search instead
  const hasError = computed(() => !!error.value)
  
  const pendingSuggestions = computed((): PhraseSuggestion[] => {
    return suggestions.value
  })

  // Service instance
  const smartFetch = useSmartFetch()
  const adminServiceInstance = adminService(smartFetch)

  // Private action handler (replaces useBaseService logic)
  const _handleAction = async <T>(
    action: () => Promise<T>,
    context?: string
  ): Promise<T | undefined> => {
    isLoading.value = true
    error.value = null
    
    try {
      const result = await action()
      return result
    } catch (err: any) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      error.value = errorMessage
      console.error(`[AdminStore] Error${context ? ` in ${context}` : ''}:`, errorMessage)
      
      // Handle all errors gracefully - keep them in state for UI to display
      return undefined
    } finally {
      isLoading.value = false
    }
  }

  // Private success handler
  const _handleSuccess = (message: string): void => {
    console.log(`[AdminStore] Success: ${message}`)
    // TODO: Add toast notifications here
  }

  // Get backend URL based on environment
  const getBackendUrl = () => {
    // During SSR, use internal Docker network
    if (process.server) {
      return 'http://backend:8080/backend'
    }
    // On client, use public URL
    return 'https://localhost/backend'
  }

  const fetchStats = async () => {
    const data = await _handleAction(() => adminServiceInstance.getStats(), 'fetchStats')
    if (data) {
      stats.value = data
    }
    return data
  }

  // SSR-compatible stats fetching
  const fetchStatsSSR = async (): Promise<AdminStats | null> => {
    const result = await _handleAction(async () => {
      // Use existing service method with useSmartFetch
      const response = await adminServiceInstance.getStats()
      stats.value = response
      return response
    }, 'fetchStatsSSR')
    
    return result || null
  }

  const fetchUsers = async (searchQueryParam?: string) => {
    const data = await _handleAction(
      () => adminServiceInstance.getUsers(searchQueryParam || searchQuery.value),
      'fetchUsers'
    )
    if (data) {
      users.value = data
    }
    return data
  }

  // Debounced search function
  const debouncedUserSearch = useDebounceFn(async (query: string) => {
    await fetchUsers(query)
  }, 300)

  // Search function that triggers debounced search
  const searchUsers = (query: string) => {
    searchQuery.value = query
    debouncedUserSearch(query)
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

  // Pure state management functions
  const setUsers = (usersList: User[]) => {
    users.value = usersList
  }

  const setSuggestions = (suggestionsList: PhraseSuggestion[]) => {
    suggestions.value = suggestionsList
  }

  const setStats = (statsData: AdminStats) => {
    stats.value = statsData
  }

  const setSearchQuery = (query: string) => {
    searchQuery.value = query
  }

  const setSelectedUser = (user: User | null) => {
    selectedUser.value = user
  }

  const setActiveTab = (tab: string) => {
    activeTab.value = tab
  }

  const setNewPassword = (password: string | null) => {
    newPassword.value = password
  }

  const updateUserActiveStatus = (userId: string, active: boolean) => {
    const user = users.value.find(u => u.id === userId)
    if (user) {
      user.active = active
    }
  }

  const updateUserRoles = (userId: string, roles: string[]) => {
    const user = users.value.find(u => u.id === userId)
    if (user) {
      user.roles = roles
    }
  }

  const removeSuggestion = (suggestionId: string) => {
    suggestions.value = suggestions.value.filter(s => s.id !== suggestionId)
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
    activeTab.value = 'overview'
    error.value = null
  }

  const clearAllData = () => {
    users.value = []
    suggestions.value = []
    stats.value = null
    searchQuery.value = ''
    selectedUser.value = null
    newPassword.value = null
    activeTab.value = 'overview'
    isLoading.value = false
    error.value = null
    
    console.log('🧹 [AdminStore] All data cleared')
  }

  // Test utility method (only for testing)
  const setError = (errorMessage: string | null) => {
    error.value = errorMessage
  }

  // Set up session watcher for automatic cleanup on logout
  useSessionWatcher(clearAllData)

  return {
    users: readonly(users),
    suggestions: readonly(suggestions),
    stats: readonly(stats),
    searchQuery: readonly(searchQuery),
    selectedUser: readonly(selectedUser),
    newPassword: readonly(newPassword),
    activeTab: readonly(activeTab),
    isLoading: readonly(isLoading),
    error: readonly(error),
    
    pendingSuggestions,
    hasError,
    
    fetchStats,
    fetchStatsSSR,
    fetchUsers,
    fetchSuggestions,
    searchUsers,
    deactivateUser,
    activateUser,
    resetUserPassword,
    promoteUser,
    approveSuggestion,
    rejectSuggestion,
    
    setUsers,
    setSuggestions,
    setStats,
    setSearchQuery,
    setSelectedUser,
    setActiveTab,
    setNewPassword,
    updateUserActiveStatus,
    updateUserRoles,
    removeSuggestion,
    clearNewPassword,
    clearState,
    clearAllData,
    setError
  }
})
