/**
 * Password Reset Actions Composable - State management for password reset flow
 * Orchestrates service calls with useBaseService for consistent error handling
 */

import { authService } from '~/services/authService'
import { useSmartFetch } from '~/composables/useSmartFetch'
import { useBaseService } from '~/composables/useBaseService'
import type { ForgotPasswordResponse, ResetPasswordResponse } from '#shared/types'

export const usePasswordResetActions = () => {
  const smartFetch = useSmartFetch()
  const { executeRequest, executeRequestWithSuccess, isLoading, error, hasError } = useBaseService()

  const authServiceInstance = authService(smartFetch)
  const { forgotPassword: forgotPasswordService, resetPassword: resetPasswordService } = authServiceInstance

  /**
   * Send password reset email
   * Returns the same success message regardless of whether user exists (security)
   */
  const sendResetEmail = async (email: string): Promise<ForgotPasswordResponse> => {
    return executeRequestWithSuccess(
      () => forgotPasswordService(email),
      'If an account exists with that email, you will receive a password reset link shortly.',
      'sendResetEmail'
    )
  }

  /**
   * Reset password with token
   * Validates token and updates password
   */
  const resetPassword = async (token: string, newPassword: string): Promise<ResetPasswordResponse> => {
    return executeRequestWithSuccess(
      () => resetPasswordService(token, newPassword),
      'Password reset successfully! You can now log in with your new password.',
      'resetPassword'
    )
  }

  return {
    sendResetEmail,
    resetPassword,
    isLoading,
    error,
    hasError
  }
}
