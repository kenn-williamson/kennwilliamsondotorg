/**
 * useAuthProfileService - Profile management operations
 * 
 * Handles profile updates and password changes using direct backend calls.
 */

import { useBaseService } from './useBaseService'
import { API_ROUTES } from '#shared/config/api-routes'

interface ProfileUpdateRequest {
  display_name: string
  slug: string
}

interface PasswordChangeRequest {
  current_password: string
  new_password: string
}

interface SlugPreviewResponse {
  slug: string
  available: boolean
  final_slug: string
}

export function useAuthProfileService() {
  const { executeRequest, executeRequestWithSuccess, backendFetch, isLoading, error, hasError } = useBaseService()

  return {
    // Update user profile
    async updateProfile(data: ProfileUpdateRequest) {
      return executeRequestWithSuccess(
        () => backendFetch(API_ROUTES.PROTECTED.AUTH.PROFILE, {
          method: 'PUT',
          body: data
        }),
        'Profile updated successfully',
        'updateProfile'
      )
    },

    // Change user password
    async changePassword(data: PasswordChangeRequest) {
      return executeRequestWithSuccess(
        () => backendFetch(API_ROUTES.PROTECTED.AUTH.CHANGE_PASSWORD, {
          method: 'PUT',
          body: data
        }),
        'Password changed successfully',
        'changePassword'
      )
    },

    // Preview slug availability
    async previewSlug(displayName: string) {
      return executeRequest(
        () => backendFetch<SlugPreviewResponse>(API_ROUTES.PUBLIC.AUTH.PREVIEW_SLUG, {
          method: 'POST',
          body: { display_name: displayName }
        }),
        'previewSlug'
      )
    },

    // Expose base service state for components
    isLoading,
    error,
    hasError,
  }
}
