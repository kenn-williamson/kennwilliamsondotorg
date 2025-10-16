/**
 * Pure Auth Profile Service - No Vue context, accepts fetcher as parameter
 * Easy to test with mock fetchers
 */

import { API_ROUTES } from '#shared/config/api-routes'
import type { ProfileUpdateRequest, PasswordChangeRequest, SetPasswordRequest, SlugPreviewResponse, Fetcher } from '#shared/types'

export const authProfileService = (fetcher: Fetcher) => ({
  updateProfile: async (data: ProfileUpdateRequest): Promise<{ message: string }> => {
    return fetcher<{ message: string }>(API_ROUTES.API.AUTH.PROFILE, {
      method: 'PUT',
      body: data
    })
  },

  changePassword: async (data: PasswordChangeRequest): Promise<{ message: string }> => {
    return fetcher<{ message: string }>(API_ROUTES.PROTECTED.AUTH.CHANGE_PASSWORD, {
      method: 'PUT',
      body: data
    })
  },

  setPassword: async (data: SetPasswordRequest): Promise<{ message: string }> => {
    return fetcher<{ message: string }>(API_ROUTES.PROTECTED.AUTH.SET_PASSWORD, {
      method: 'PUT',
      body: data
    })
  },

  previewSlug: async (displayName: string): Promise<SlugPreviewResponse> => {
    return fetcher<SlugPreviewResponse>(API_ROUTES.PUBLIC.AUTH.PREVIEW_SLUG, {
      method: 'POST',
      body: { display_name: displayName }
    })
  },

  validateSlug: async (slug: string): Promise<{ available: boolean }> => {
    return fetcher<{ available: boolean }>(API_ROUTES.PROTECTED.AUTH.VALIDATE_SLUG, {
      method: 'GET',
      query: { slug }
    })
  },

  deleteAccount: async (): Promise<{ message: string }> => {
    return fetcher<{ message: string }>(API_ROUTES.PROTECTED.AUTH.DELETE_ACCOUNT, {
      method: 'DELETE'
    })
  },

  exportUserData: async (): Promise<Blob> => {
    // Note: Blob responses require special handling
    const response = await fetcher<Blob>(API_ROUTES.PROTECTED.AUTH.EXPORT_DATA, {
      method: 'GET',
      responseType: 'blob' as any
    })
    return response
  }
})
