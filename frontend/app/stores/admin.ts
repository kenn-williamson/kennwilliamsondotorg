/**
 * Centralized Admin Store - State management with actions
 */

import type { User, AdminStats, AdminResetPasswordResponse, AccessRequestWithUser } from '#shared/types'
import type { PhraseSuggestion } from '#shared/types/phrases'
import { adminService } from '~/services/adminService'
import { useSmartFetch } from '~/composables/useSmartFetch'
import { useSessionWatcher } from '~/composables/useSessionWatcher'
import { useDebounceFn } from '@vueuse/core'

export const useAdminStore = defineStore('admin', () => {
  const users = ref<User[]>([])
  const suggestions = ref<PhraseSuggestion[]>([])
  const accessRequests = ref<AccessRequestWithUser[]>([])
  const stats = ref<AdminStats | null>(null)
  const searchQuery = ref('')
  const selectedUser = ref<User | null>(null)
  const newPassword = ref<string | null>(null)

  const isLoading = ref(false)
  const error = ref<string | null>(null)
  
  // Remove client-side filtering - use server-side search instead
  const hasError = computed(() => !!error.value)
  
  const pendingSuggestions = computed((): PhraseSuggestion[] => {
    return suggestions.value
  })

  const pendingAccessRequests = computed((): AccessRequestWithUser[] => {
    return accessRequests.value.filter(req => req.status === 'pending')
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

  const fetchAccessRequests = async () => {
    const data = await _handleAction(() => adminServiceInstance.getAccessRequests(), 'fetchAccessRequests')
    if (data) {
      accessRequests.value = data.requests
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

  const resetUserPassword = async (userId: string): Promise<AdminResetPasswordResponse | undefined> => {
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

  const addUserRole = async (userId: string, roleName: string) => {
    await _handleAction(() => adminServiceInstance.addUserRole(userId, roleName), 'addUserRole')
    _handleSuccess(`Role '${roleName}' added successfully`)

    // Update local state
    const user = users.value.find(u => u.id === userId)
    if (user && !user.roles.includes(roleName)) {
      user.roles = [...user.roles, roleName]
    }

    // Refresh users to get latest state
    await fetchUsers()
  }

  const removeUserRole = async (userId: string, roleName: string) => {
    await _handleAction(() => adminServiceInstance.removeUserRole(userId, roleName), 'removeUserRole')
    _handleSuccess(`Role '${roleName}' removed successfully`)

    // Update local state
    const user = users.value.find(u => u.id === userId)
    if (user) {
      user.roles = user.roles.filter(r => r !== roleName)
    }

    // Refresh users to get latest state
    await fetchUsers()
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

  const approveAccessRequest = async (requestId: string, adminReason: string) => {
    await _handleAction(() => adminServiceInstance.approveAccessRequest(requestId, adminReason), 'approveAccessRequest')
    _handleSuccess('Access request approved successfully')

    // Remove from local state
    accessRequests.value = accessRequests.value.filter(r => r.id !== requestId)
  }

  const rejectAccessRequest = async (requestId: string, adminReason: string) => {
    await _handleAction(() => adminServiceInstance.rejectAccessRequest(requestId, adminReason), 'rejectAccessRequest')
    _handleSuccess('Access request rejected successfully')

    // Remove from local state
    accessRequests.value = accessRequests.value.filter(r => r.id !== requestId)
  }

  // Pure state management functions
  const setUsers = (usersList: User[]) => {
    users.value = usersList
  }

  const setSuggestions = (suggestionsList: PhraseSuggestion[]) => {
    suggestions.value = suggestionsList
  }

  const setAccessRequests = (requestsList: AccessRequestWithUser[]) => {
    accessRequests.value = requestsList
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

  const removeAccessRequest = (requestId: string) => {
    accessRequests.value = accessRequests.value.filter(r => r.id !== requestId)
  }

  const clearNewPassword = () => {
    newPassword.value = null
  }

  const clearState = () => {
    users.value = []
    suggestions.value = []
    accessRequests.value = []
    stats.value = null
    searchQuery.value = ''
    selectedUser.value = null
    newPassword.value = null
    error.value = null
  }

  const clearAllData = () => {
    users.value = []
    suggestions.value = []
    accessRequests.value = []
    stats.value = null
    searchQuery.value = ''
    selectedUser.value = null
    newPassword.value = null
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
    accessRequests: readonly(accessRequests),
    stats: readonly(stats),
    searchQuery: readonly(searchQuery),
    selectedUser: readonly(selectedUser),
    newPassword: readonly(newPassword),
    isLoading: readonly(isLoading),
    error: readonly(error),

    pendingSuggestions,
    pendingAccessRequests,
    hasError,

    fetchStats,
    fetchUsers,
    fetchSuggestions,
    fetchAccessRequests,
    searchUsers,
    deactivateUser,
    activateUser,
    resetUserPassword,
    promoteUser,
    addUserRole,
    removeUserRole,
    approveSuggestion,
    rejectSuggestion,
    approveAccessRequest,
    rejectAccessRequest,

    setUsers,
    setSuggestions,
    setAccessRequests,
    setStats,
    setSearchQuery,
    setSelectedUser,
    setNewPassword,
    updateUserActiveStatus,
    updateUserRoles,
    removeSuggestion,
    removeAccessRequest,
    clearNewPassword,
    clearState,
    clearAllData,
    setError
  }
})
