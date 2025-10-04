/**
 * Email Verification Composable
 * Handles email verification flow including sending verification emails and verifying tokens
 */

import { useSmartFetch } from '~/composables/useSmartFetch'
import { useBaseService } from '~/composables/useBaseService'
import { API_ROUTES } from '#shared/config/api-routes'
import type { SendVerificationEmailResponse, VerifyEmailResponse } from '#shared/types'

export const useEmailVerification = () => {
  // Create dependencies
  const smartFetch = useSmartFetch()
  const { fetch: refreshSession } = useUserSession()

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
          API_ROUTES.PUBLIC.VERIFY_EMAIL,
          {
            method: 'GET',
            query: { token },
          }
        )

        // Refresh session to get updated user data with email_verified flag
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
