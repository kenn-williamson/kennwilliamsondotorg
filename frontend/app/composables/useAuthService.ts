/**
 * useAuthService - Authentication operations using Nuxt Auth Utils
 * 
 * Handles login/register operations via Nuxt API routes that manage sessions.
 * For reactive state, use useUserSession() directly in components.
 */

import { useBackendFetch } from './useBackendFetch'
import { useAuthFetch } from './useAuthFetch'
import { API_ROUTES } from '#shared/config/api-routes'

interface LoginRequest {
  email: string
  password: string
}

interface RegisterRequest {
  email: string
  password: string
  display_name: string
}

interface SlugPreviewResponse {
  slug: string
  available: boolean
  final_slug: string
}

export function useAuthService() {
  const { clear, fetch: refreshSession } = useUserSession()
  const { executeRequest, executeRequestWithSuccess, backendFetch, authFetch } = useBaseService()

  return {
    async login(credentials: LoginRequest): Promise<{ success: boolean }> {
      return executeRequestWithSuccess(
        async () => {
          // Call Nuxt API route that handles session creation
          await authFetch(API_ROUTES.API.AUTH.LOGIN, {
            method: 'POST',
            body: credentials,
          })
          
          // Refresh session to get updated user data
          await refreshSession()
          
          return { success: true }
        },
        'Login successful',
        'login'
      )
    },

    async register(userData: RegisterRequest): Promise<{ success: boolean }> {
      return executeRequestWithSuccess(
        async () => {
          // Call Nuxt API route that handles session creation  
          await authFetch(API_ROUTES.API.AUTH.REGISTER, {
            method: 'POST',
            body: userData,
          })
          
          // Refresh session to get updated user data
          await refreshSession()
          return { success: true }
        },
        'Registration successful',
        'register'
      )
    },

    async previewSlug(displayName: string): Promise<SlugPreviewResponse> {
      return executeRequest(
        () => backendFetch(API_ROUTES.PUBLIC.AUTH.PREVIEW_SLUG, {
          method: 'POST',
          body: { display_name: displayName },
        }),
        'previewSlug'
      )
    },


    async revokeAllSessions(): Promise<{ success: boolean }> {
      return executeRequestWithSuccess(
        async () => {
          await backendFetch(API_ROUTES.PROTECTED.AUTH.REVOKE_ALL, { method: 'POST' })

          // Clear local session after revoking all sessions
          await clear()
          return { success: true }
        },
        'All sessions revoked successfully',
        'revokeAllSessions'
      )
    },

    async logout(): Promise<void> {
      return executeRequest(
        async () => {
          console.log('🔍 [Auth Service] Starting logout process...')
          
          // Call the Nuxt server logout endpoint which handles refresh token revocation
          try {
            console.log('🔄 [Auth Service] Calling Nuxt server /api/auth/logout...')
            await authFetch(API_ROUTES.API.AUTH.LOGOUT, { method: 'POST' })
            console.log('✅ [Auth Service] Logout completed on server')
          } catch (error) {
            console.error('❌ [Auth Service] Failed to logout on server:', error)
            // Continue with client-side cleanup even if server logout fails
          }

          // Clear authentication state
          await clear()
        },
        'logout'
      )
    },
  }
}