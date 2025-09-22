/**
 * Pure Admin Service - No Vue context, accepts fetcher as parameter
 * Easy to test with mock fetchers
 */

import { API_ROUTES } from '#shared/config/api-routes'
import type { 
  AdminStats, 
  UsersResponse, 
  SuggestionsResponse, 
  ResetPasswordResponse,
  AdminActionResponse,
  Fetcher
} from '#shared/types'

export const adminService = (fetcher: Fetcher) => ({
  getStats: async (): Promise<AdminStats> => {
    return fetcher<AdminStats>(API_ROUTES.PROTECTED.ADMIN.STATS)
  },

  getUsers: async (searchQuery?: string): Promise<UsersResponse> => {
    const params = new URLSearchParams()
    if (searchQuery?.trim()) {
      params.append('search', searchQuery.trim())
    }
    const url = `${API_ROUTES.PROTECTED.ADMIN.USERS}${params.toString() ? `?${params.toString()}` : ''}`
    return fetcher<UsersResponse>(url)
  },

  getSuggestions: async (): Promise<SuggestionsResponse> => {
    return fetcher<SuggestionsResponse>(API_ROUTES.PROTECTED.ADMIN.SUGGESTIONS.LIST)
  },

  deactivateUser: async (userId: string): Promise<AdminActionResponse> => {
    return fetcher<AdminActionResponse>(API_ROUTES.PROTECTED.ADMIN.USER_DEACTIVATE(userId), {
      method: 'POST'
    })
  },

  activateUser: async (userId: string): Promise<AdminActionResponse> => {
    return fetcher<AdminActionResponse>(API_ROUTES.PROTECTED.ADMIN.USER_ACTIVATE(userId), {
      method: 'POST'
    })
  },

  resetUserPassword: async (userId: string): Promise<ResetPasswordResponse> => {
    return fetcher<ResetPasswordResponse>(API_ROUTES.PROTECTED.ADMIN.USER_RESET_PASSWORD(userId), {
      method: 'POST'
    })
  },

  promoteUser: async (userId: string): Promise<AdminActionResponse> => {
    return fetcher<AdminActionResponse>(API_ROUTES.PROTECTED.ADMIN.USER_PROMOTE(userId), {
      method: 'POST'
    })
  },

  approveSuggestion: async (suggestionId: string, adminReason: string): Promise<AdminActionResponse> => {
    return fetcher<AdminActionResponse>(API_ROUTES.PROTECTED.ADMIN.SUGGESTIONS.APPROVE(suggestionId), {
      method: 'POST',
      body: { admin_reason: adminReason }
    })
  },

  rejectSuggestion: async (suggestionId: string, adminReason: string): Promise<AdminActionResponse> => {
    return fetcher<AdminActionResponse>(API_ROUTES.PROTECTED.ADMIN.SUGGESTIONS.REJECT(suggestionId), {
      method: 'POST',
      body: { admin_reason: adminReason }
    })
  }
})
