/**
 * Auth Profile Action Composable - Orchestrates services + session management
 * Handles context-aware operations and bridges between services and session state
 */

import { authProfileService } from '~/services/authProfileService'
import { useBaseService } from '~/composables/useBaseService'
import { useBackendFetch } from '~/composables/useBackendFetch'
import { useAuthFetch } from '~/composables/useAuthFetch'
import type { ProfileUpdateRequest, PasswordChangeRequest, SlugPreviewResponse } from '#shared/types'

export const useAuthProfileActions = () => {
  // Destructure base service utilities
  const { executeRequest, executeRequestWithSuccess, isLoading, error, hasError } = useBaseService()
  const backendFetch = useBackendFetch()
  const authFetch = useAuthFetch()
  const { fetch: refreshSession } = useUserSession()
  
  // Create service instances
  const authProfileServiceBackend = authProfileService(backendFetch)
  const authProfileServiceAuth = authProfileService(authFetch)
  
  // Destructure service methods (use backend for protected routes, auth for session management)
  const { 
    updateProfile: updateProfileBackend, 
    changePassword: changePasswordBackend, 
    previewSlug: previewSlugBackend 
  } = authProfileServiceBackend

  const updateProfile = async (data: ProfileUpdateRequest): Promise<{ message: string }> => {
    return executeRequestWithSuccess(
      async () => {
        // Call service
        const result = await updateProfileBackend(data)
        
        // Refresh session to get updated user data
        await refreshSession()
        
        return result
      },
      'Profile updated successfully',
      'updateProfile'
    )
  }

  const changePassword = async (data: PasswordChangeRequest): Promise<{ message: string }> => {
    return executeRequestWithSuccess(
      () => changePasswordBackend(data),
      'Password changed successfully',
      'changePassword'
    )
  }

  const previewSlug = async (displayName: string): Promise<SlugPreviewResponse> => {
    return executeRequest(
      () => previewSlugBackend(displayName),
      'previewSlug'
    )
  }

  return {
    updateProfile,
    changePassword,
    previewSlug,
    isLoading,
    error,
    hasError
  }
}
