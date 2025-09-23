/**
 * Admin Action Composable - Orchestrates services + stores
 * Handles context-aware operations and bridges between services and stores
 */

import { useAdminStore } from '~/stores/admin'
import { adminService } from '~/services/adminService'
import { useBaseService } from '~/composables/useBaseService'
import { useBackendFetch } from '~/composables/useBackendFetch'
import { useAuthFetch } from '~/composables/useAuthFetch'

export const useAdminActions = () => {
  // Create dependencies at the top level
  const backendFetch = useBackendFetch()
  
  // Use base service for request execution
  const { executeRequest, executeRequestWithSuccess, isLoading, error, hasError } = useBaseService()
  
  // Create service instance
  const adminServiceBackend = adminService(backendFetch)
  
  // Destructure service methods
  const { 
    getStats, 
    getUsers, 
    getSuggestions, 
    deactivateUser: deactivateUserService, 
    activateUser: activateUserService, 
    resetUserPassword: resetUserPasswordService, 
    promoteUser: promoteUserService, 
    approveSuggestion: approveSuggestionService, 
    rejectSuggestion: rejectSuggestionService 
  } = adminServiceBackend

  // Destructure store methods
  const { 
    setUsers, 
    setSuggestions, 
    setStats, 
    setSearchQuery, 
    setSelectedUser, 
    setNewPassword, 
    updateUserActiveStatus, 
    updateUserRoles, 
    removeSuggestion, 
    clearNewPassword, 
    clearState,
    users,
    searchQuery
  } = useAdminStore()

  const fetchStats = async () => {
    const data = await executeRequest(() => getStats(), 'fetchStats')
    setStats(data)
    return data
  }

  const fetchUsers = async (searchQueryParam?: string) => {
    const data = await executeRequest(
      () => getUsers(searchQueryParam || searchQuery),
      'fetchUsers'
    )
    setUsers(data.users)
    return data
  }

  const fetchSuggestions = async () => {
    const data = await executeRequest(() => getSuggestions(), 'fetchSuggestions')
    setSuggestions(data.suggestions)
    return data
  }

  const deactivateUser = async (userId: string) => {
    await executeRequestWithSuccess(
      () => deactivateUserService(userId),
      'User deactivated successfully',
      'deactivateUser'
    )
    updateUserActiveStatus(userId, false)
  }

  const activateUser = async (userId: string) => {
    await executeRequestWithSuccess(
      () => activateUserService(userId),
      'User activated successfully',
      'activateUser'
    )
    updateUserActiveStatus(userId, true)
  }

  const resetUserPassword = async (userId: string) => {
    const data = await executeRequestWithSuccess(
      () => resetUserPasswordService(userId),
      'Password reset successfully',
      'resetUserPassword'
    )
    setNewPassword(data.new_password)
    return data
  }

  const promoteUser = async (userId: string) => {
    await executeRequestWithSuccess(
      () => promoteUserService(userId),
      'User promoted to admin successfully',
      'promoteUser'
    )
    const user = users.find(u => u.id === userId)
    if (user) {
      updateUserRoles(userId, [...user.roles, 'admin'])
    }
  }

  const approveSuggestion = async (suggestionId: string, adminReason: string) => {
    await executeRequestWithSuccess(
      () => approveSuggestionService(suggestionId, adminReason),
      'Suggestion approved successfully',
      'approveSuggestion'
    )
    removeSuggestion(suggestionId)
  }

  const rejectSuggestion = async (suggestionId: string, adminReason: string) => {
    await executeRequestWithSuccess(
      () => rejectSuggestionService(suggestionId, adminReason),
      'Suggestion rejected successfully',
      'rejectSuggestion'
    )
    removeSuggestion(suggestionId)
  }

  // Aliases for backward compatibility
  const fetchPhraseSuggestions = fetchSuggestions

  return {
    fetchStats,
    fetchUsers,
    fetchSuggestions,
    deactivateUser,
    activateUser,
    resetUserPassword,
    promoteUser,
    approveSuggestion,
    rejectSuggestion,
    // Aliases for backward compatibility
    fetchPhraseSuggestions,
    isLoading,
    error,
    hasError
  }
}
