/**
 * Pure Admin Service - No Vue context, accepts fetcher as parameter
 * Easy to test with mock fetchers
 */

import { API_ROUTES } from '#shared/config/api-routes'
import type {
  AdminStats,
  UsersResponse,
  SuggestionsResponse,
  AdminResetPasswordResponse,
  AdminActionResponse,
  AccessRequestsResponse,
  Fetcher
} from '#shared/types'

export const adminService = (fetcher: Fetcher) => ({
  getStats: async (): Promise<AdminStats> => {
    return fetcher<AdminStats>(API_ROUTES.PROTECTED.ADMIN.STATS)
  },

  getUsers: async (searchQuery?: string): Promise<UsersResponse> => {
    const query: Record<string, string | undefined> = {}
    if (searchQuery?.trim()) {
      query.search = searchQuery.trim()
    }
    // Set reasonable limit to prevent performance issues
    query.limit = '100'
    query.offset = '0'
    
    return fetcher<UsersResponse>(API_ROUTES.PROTECTED.ADMIN.USERS, {
      query
    })
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

  resetUserPassword: async (userId: string): Promise<AdminResetPasswordResponse> => {
    return fetcher<AdminResetPasswordResponse>(API_ROUTES.PROTECTED.ADMIN.USER_RESET_PASSWORD(userId), {
      method: 'POST'
    })
  },

  promoteUser: async (userId: string): Promise<AdminActionResponse> => {
    return fetcher<AdminActionResponse>(API_ROUTES.PROTECTED.ADMIN.USER_PROMOTE(userId), {
      method: 'POST'
    })
  },

  addUserRole: async (userId: string, roleName: string): Promise<AdminActionResponse> => {
    return fetcher<AdminActionResponse>(API_ROUTES.PROTECTED.ADMIN.USER_ADD_ROLE(userId, roleName), {
      method: 'POST'
    })
  },

  removeUserRole: async (userId: string, roleName: string): Promise<AdminActionResponse> => {
    return fetcher<AdminActionResponse>(API_ROUTES.PROTECTED.ADMIN.USER_REMOVE_ROLE(userId, roleName), {
      method: 'DELETE'
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
  },

  getAccessRequests: async (): Promise<AccessRequestsResponse> => {
    return fetcher<AccessRequestsResponse>(API_ROUTES.PROTECTED.ADMIN.ACCESS_REQUESTS.LIST)
  },

  approveAccessRequest: async (requestId: string, adminReason: string): Promise<AdminActionResponse> => {
    return fetcher<AdminActionResponse>(API_ROUTES.PROTECTED.ADMIN.ACCESS_REQUESTS.APPROVE(requestId), {
      method: 'POST',
      body: { admin_reason: adminReason }
    })
  },

  rejectAccessRequest: async (requestId: string, adminReason: string): Promise<AdminActionResponse> => {
    return fetcher<AdminActionResponse>(API_ROUTES.PROTECTED.ADMIN.ACCESS_REQUESTS.REJECT(requestId), {
      method: 'POST',
      body: { admin_reason: adminReason }
    })
  }
})
