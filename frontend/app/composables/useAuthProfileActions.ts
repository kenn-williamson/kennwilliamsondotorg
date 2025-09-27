/**
 * Auth Profile Action Composable - Orchestrates services + session management
 * Handles context-aware operations and bridges between services and session state
 */

import { authProfileService } from '~/services/authProfileService'
import { useBaseService } from '~/composables/useBaseService'
import { useSmartFetch } from '~/composables/useSmartFetch'
import type { ProfileUpdateRequest, PasswordChangeRequest, SlugPreviewResponse } from '#shared/types'

export const useAuthProfileActions = () => {
  // Destructure base service utilities
  const { executeRequest, executeRequestWithSuccess, isLoading, error, hasError } = useBaseService()
  const smartFetch = useSmartFetch()
  const { fetch: refreshSession } = useUserSession()
  
  // Create service instance
  const authProfileServiceInstance = authProfileService(smartFetch)
  
  // Destructure service methods
  const { 
    updateProfile: updateProfileService, 
    changePassword: changePasswordService, 
    previewSlug: previewSlugService,
    validateSlug: validateSlugService 
  } = authProfileServiceInstance

  const updateProfile = async (data: ProfileUpdateRequest): Promise<{ message: string }> => {
    return executeRequestWithSuccess(
      async () => {
        // Call service
        const result = await updateProfileService(data)
        
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
      () => changePasswordService(data),
      'Password changed successfully',
      'changePassword'
    )
  }

  const previewSlug = async (displayName: string): Promise<SlugPreviewResponse> => {
    return executeRequest(
      () => previewSlugService(displayName),
      'previewSlug'
    )
  }

  const validateSlug = async (slug: string): Promise<{ available: boolean }> => {
    return executeRequest(
      () => validateSlugService(slug),
      'validateSlug'
    )
  }

  return {
    updateProfile,
    changePassword,
    previewSlug,
    validateSlug,
    isLoading,
    error,
    hasError
  }
}
