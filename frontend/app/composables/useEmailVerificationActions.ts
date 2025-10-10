/**
 * Email Verification Actions Composable
 * Orchestrates email verification flow with useBaseService for consistent error handling
 */

import { useSmartFetch } from '~/composables/useSmartFetch'
import { useBaseService } from '~/composables/useBaseService'
import { API_ROUTES } from '#shared/config/api-routes'
import type { SendVerificationEmailResponse, VerifyEmailResponse } from '#shared/types'

export const useEmailVerificationActions = () => {
  // Create dependencies
  const smartFetch = useSmartFetch()
  const { clearToken } = useJwtManager()

  // Use base service for request execution
  const { executeRequest, isLoading, error, hasError } = useBaseService()

  /**
   * Send verification email to the current user
   */
  const sendVerificationEmail = async (): Promise<SendVerificationEmailResponse> => {
    return executeRequest(
      () => smartFetch<SendVerificationEmailResponse>(
        API_ROUTES.API.AUTH.SEND_VERIFICATION,
        { method: 'POST' }
      ),
      'sendVerificationEmail'
    )
  }

  /**
   * Verify email with token
   * @param token - Verification token from email link
   */
  const verifyEmail = async (token: string): Promise<VerifyEmailResponse> => {
    return executeRequest(
      async () => {
        // Verify the email with the backend (direct route - useSmartFetch adds /backend prefix)
        const result = await smartFetch<VerifyEmailResponse>(
          API_ROUTES.PUBLIC.AUTH.VERIFY_EMAIL,
          {
            method: 'GET',
            query: { token },
          }
        )

        // Fetch fresh user data with updated roles and update session
        // /api/auth/me forces a token refresh and returns updated user data
        const freshUserData = await $fetch(API_ROUTES.API.AUTH.ME)
        console.log('✅ [Email Verification] Got fresh user data with roles:', freshUserData.roles)

        // Clear JWT cache to force regeneration with updated roles on next request
        clearToken()

        // Refresh the client-side session to get the updated user data
        const { fetch: refreshSession } = useUserSession()
        await refreshSession()

        return result
      },
      'verifyEmail'
    )
  }

  return {
    sendVerificationEmail,
    verifyEmail,
    isLoading,
    error,
    hasError,
  }
}
