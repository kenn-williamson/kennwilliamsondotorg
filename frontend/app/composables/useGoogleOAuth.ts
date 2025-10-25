/**
 * Google OAuth Composable
 * Handles Google OAuth flow from initiation to callback completion
 *
 * Flow:
 * 1. initiateOAuth() - Get OAuth URL, redirect to Google
 * 2. Google redirects to /auth/google/callback page
 * 3. handleOAuthCallback() - Complete authentication, set session
 */

import { useSmartFetch } from '~/composables/useSmartFetch'
import { useBaseService } from '~/composables/useBaseService'
import { API_ROUTES } from '#shared/config/api-routes'
import type { GoogleOAuthUrlResponse } from '#shared/types'

interface OAuthCallbackResult {
  success: boolean
  user: any
  redirect_url?: string
}

export const useGoogleOAuth = () => {
  // Create dependencies
  const smartFetch = useSmartFetch()

  // Use base service for request execution
  const { executeRequest, isLoading, error, hasError } = useBaseService()

  /**
   * Initiate Google OAuth flow by redirecting to Google
   *
   * Flow:
   * 1. Get OAuth URL from server (includes PKCE challenge stored in Redis)
   * 2. Redirect browser to Google
   * 3. User authorizes
   * 4. Google redirects to /auth/google/callback page
   */
  const initiateOAuth = async (): Promise<void> => {
    return executeRequest(
      async () => {
        // Get redirect from current route query
        const route = useRoute()
        const redirect = route.query.redirect as string | undefined

        // Build URL with redirect parameter if present
        const url = redirect
          ? `${API_ROUTES.API.AUTH.GOOGLE_URL}?redirect=${encodeURIComponent(redirect)}`
          : API_ROUTES.API.AUTH.GOOGLE_URL

        // Get Google OAuth URL from backend (PKCE verifier stored in Redis)
        const response = await smartFetch<GoogleOAuthUrlResponse>(
          url,
          { method: 'GET' }
        )

        // Redirect to Google OAuth consent screen
        window.location.href = response.url
      },
      'initiateOAuth'
    )
  }

  /**
   * Handle OAuth callback after Google redirects back
   *
   * Flow:
   * 1. Send code and state to Nuxt API route
   * 2. Nuxt API calls backend with code and state
   * 3. Backend retrieves PKCE verifier from Redis using state
   * 4. Backend exchanges code for tokens and returns JWT
   * 5. Nuxt API sets session
   */
  const handleOAuthCallback = async (code: string, state: string): Promise<OAuthCallbackResult> => {
    return executeRequest(
      async () => {
        // Call Nuxt API route which will complete OAuth flow
        const result = await smartFetch<OAuthCallbackResult>(
          API_ROUTES.API.AUTH.GOOGLE_CALLBACK,
          {
            method: 'POST',
            body: { code, state }
          }
        )

        // Session is set by the API route
        // Return result with optional redirect_url for caller
        return result
      },
      'handleOAuthCallback'
    )
  }

  return {
    initiateOAuth,
    handleOAuthCallback,
    isLoading,
    error,
    hasError,
  }
}
