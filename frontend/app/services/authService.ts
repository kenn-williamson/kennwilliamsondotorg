/**
 * Pure Auth Service - No Vue context, accepts fetcher as parameter
 * Easy to test with mock fetchers
 */

import { API_ROUTES } from '#shared/config/api-routes'
import type { LoginRequest, RegisterRequest, SlugPreviewResponse, SlugValidationResponse, Fetcher, AuthenticatedUser } from '#shared/types'

export const authService = (fetcher: Fetcher) => ({
  login: async (credentials: LoginRequest): Promise<{ success: boolean }> => {
    return fetcher<{ success: boolean }>(API_ROUTES.API.AUTH.LOGIN, {
      method: 'POST',
      body: credentials,
    })
  },

  register: async (userData: RegisterRequest): Promise<{ success: boolean }> => {
    return fetcher<{ success: boolean }>(API_ROUTES.API.AUTH.REGISTER, {
      method: 'POST',
      body: userData,
    })
  },

  previewSlug: async (displayName: string): Promise<SlugPreviewResponse> => {
    return fetcher<SlugPreviewResponse>(API_ROUTES.PUBLIC.AUTH.PREVIEW_SLUG, {
      method: 'POST',
      body: { display_name: displayName },
    })
  },

  validateSlug: async (slug: string): Promise<SlugValidationResponse> => {
    return fetcher<SlugValidationResponse>(API_ROUTES.PROTECTED.AUTH.VALIDATE_SLUG, {
      method: 'GET',
      query: { slug }
    })
  },

  revokeAllSessions: async (): Promise<{ success: boolean }> => {
    return fetcher<{ success: boolean }>(API_ROUTES.PROTECTED.AUTH.REVOKE_ALL, {
      method: 'POST'
    })
  },

  logout: async (): Promise<void> => {
    return fetcher<void>(API_ROUTES.API.AUTH.LOGOUT, {
      method: 'POST'
    })
  },

  getCurrentUser: async (): Promise<AuthenticatedUser> => {
    return fetcher<AuthenticatedUser>(API_ROUTES.API.AUTH.ME)
  }
})
