/**
 * Auth Profile Action Composable - Orchestrates services + session management
 * Handles context-aware operations and bridges between services and session state
 */

import { authProfileService } from '~/services/authProfileService'
import { useBaseService } from '~/composables/useBaseService'
import { useSmartFetch } from '~/composables/useSmartFetch'
import type { ProfileUpdateRequest, PasswordChangeRequest, SetPasswordRequest, SlugPreviewResponse } from '#shared/types'

export const useAuthProfileActions = () => {
  // Destructure base service utilities
  const { executeRequest, executeRequestWithSuccess, isLoading, error, hasError } = useBaseService()
  const smartFetch = useSmartFetch()
  const { fetch: refreshSession, clear: clearSession } = useUserSession()
  
  // Create service instance
  const authProfileServiceInstance = authProfileService(smartFetch)
  
  // Destructure service methods
  const {
    updateProfile: updateProfileService,
    changePassword: changePasswordService,
    setPassword: setPasswordService,
    previewSlug: previewSlugService,
    validateSlug: validateSlugService,
    deleteAccount: deleteAccountService,
    exportUserData: exportUserDataService
  } = authProfileServiceInstance

  const updateProfile = async (data: ProfileUpdateRequest): Promise<{ message: string }> => {
    return executeRequestWithSuccess(
      async () => {
        // Pass-through updates server-side session cookie
        const result = await updateProfileService(data)

        // Refresh client-side reactive refs from updated cookie
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

  const setPassword = async (data: SetPasswordRequest): Promise<{ message: string }> => {
    return executeRequestWithSuccess(
      () => setPasswordService(data),
      'Password set successfully',
      'setPassword'
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

  const deleteAccount = async (): Promise<{ message: string }> => {
    return executeRequestWithSuccess(
      async () => {
        // Call service
        const result = await deleteAccountService()

        // Clear session after successful deletion
        await clearSession()

        return result
      },
      'Account deleted successfully',
      'deleteAccount'
    )
  }

  const exportUserData = async (): Promise<Blob> => {
    return executeRequest(
      () => exportUserDataService(),
      'exportUserData'
    )
  }

  return {
    updateProfile,
    changePassword,
    setPassword,
    previewSlug,
    validateSlug,
    deleteAccount,
    exportUserData,
    isLoading,
    error,
    hasError
  }
}
