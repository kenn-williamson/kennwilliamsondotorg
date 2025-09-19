/**
 * useAdminService - Admin operations for user management and phrase moderation
 * 
 * Handles admin API calls for user management, phrase suggestions, and system statistics.
 * Requires admin role authentication.
 */

import { useBaseService } from './useBaseService'
import { API_ROUTES } from '#shared/config/api-routes'

interface AdminStats {
  total_users: number
  active_users: number
  pending_suggestions: number
  total_phrases: number
}

interface User {
  id: string
  email: string
  display_name: string
  slug: string
  active: boolean
  roles: string[]
  created_at: string
}

interface UsersResponse {
  users: User[]
  total: number
}

interface PhraseSuggestion {
  id: string
  phrase_text: string
  user_display_name: string
  created_at: string
}

interface SuggestionsResponse {
  suggestions: PhraseSuggestion[]
  total: number
}

interface ResetPasswordResponse {
  new_password: string
}

export function useAdminService() {
  const { executeRequest, executeRequestWithSuccess, backendFetch, isLoading, error, hasError } = useBaseService()

  return {
    // Get system statistics
    async getStats(): Promise<AdminStats> {
      return executeRequest(
        () => backendFetch<AdminStats>(API_ROUTES.PROTECTED.ADMIN.STATS),
        'getStats'
      )
    },

    // Get users with optional search
    async getUsers(searchQuery?: string): Promise<UsersResponse> {
      return executeRequest(
        async () => {
          const params = new URLSearchParams()
          if (searchQuery?.trim()) {
            params.append('search', searchQuery.trim())
          }

          const url = `${API_ROUTES.PROTECTED.ADMIN.USERS}${params.toString() ? `?${params.toString()}` : ''}`
          return backendFetch<UsersResponse>(url)
        },
        'getUsers'
      )
    },

    // Get pending phrase suggestions
    async getSuggestions(): Promise<SuggestionsResponse> {
      return executeRequest(
        () => backendFetch<SuggestionsResponse>(API_ROUTES.PROTECTED.ADMIN.SUGGESTIONS.LIST),
        'getSuggestions'
      )
    },

    // Deactivate user
    async deactivateUser(userId: string): Promise<{ message: string }> {
      return executeRequestWithSuccess(
        () => backendFetch<{ message: string }>(API_ROUTES.PROTECTED.ADMIN.USER_DEACTIVATE(userId), {
          method: 'POST',
        }),
        'User deactivated successfully',
        'deactivateUser'
      )
    },

    // Activate user
    async activateUser(userId: string): Promise<{ message: string }> {
      return executeRequestWithSuccess(
        () => backendFetch<{ message: string }>(API_ROUTES.PROTECTED.ADMIN.USER_ACTIVATE(userId), {
          method: 'POST',
        }),
        'User activated successfully',
        'activateUser'
      )
    },

    // Reset user password
    async resetUserPassword(userId: string): Promise<ResetPasswordResponse> {
      return executeRequestWithSuccess(
        () => backendFetch<ResetPasswordResponse>(API_ROUTES.PROTECTED.ADMIN.USER_RESET_PASSWORD(userId), {
          method: 'POST',
        }),
        'Password reset successfully',
        'resetUserPassword'
      )
    },

    // Promote user to admin
    async promoteUser(userId: string): Promise<{ message: string }> {
      return executeRequestWithSuccess(
        () => backendFetch<{ message: string }>(API_ROUTES.PROTECTED.ADMIN.USER_PROMOTE(userId), {
          method: 'POST',
        }),
        'User promoted to admin successfully',
        'promoteUser'
      )
    },

    // Approve phrase suggestion
    async approveSuggestion(suggestionId: string, adminReason: string): Promise<{ message: string }> {
      return executeRequestWithSuccess(
        () => backendFetch<{ message: string }>(API_ROUTES.PROTECTED.ADMIN.SUGGESTIONS.APPROVE(suggestionId), {
          method: 'POST',
          body: { admin_reason: adminReason },
        }),
        'Suggestion approved successfully',
        'approveSuggestion'
      )
    },

    // Reject phrase suggestion
    async rejectSuggestion(suggestionId: string, adminReason: string): Promise<{ message: string }> {
      return executeRequestWithSuccess(
        () => backendFetch<{ message: string }>(API_ROUTES.PROTECTED.ADMIN.SUGGESTIONS.REJECT(suggestionId), {
          method: 'POST',
          body: { admin_reason: adminReason },
        }),
        'Suggestion rejected successfully',
        'rejectSuggestion'
      )
    },

    // Expose base service state for components
    isLoading,
    error,
    hasError,
  }
}
