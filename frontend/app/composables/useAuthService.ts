/**
 * useAuthService - Authentication operations using Nuxt Auth Utils
 * 
 * Handles login/register operations via Nuxt API routes that manage sessions.
 * For reactive state, use useUserSession() directly in components.
 */

import { useBackendFetch } from './useBackendFetch'
import { useAuthFetch } from './useAuthFetch'

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
  const backendFetch = useBackendFetch()
  const authFetch = useAuthFetch()

  return {
    async login(credentials: LoginRequest): Promise<{ success: boolean }> {
      try {
        // Call Nuxt API route that handles session creation
        await authFetch('/auth/login', {
          method: 'POST',
          body: credentials,
        })
        
        // Refresh session state on client-side
        await refreshSession()
        
        return { success: true }
      } catch (error: any) {
        throw new Error(error.data?.statusMessage || 'Login failed')
      }
    },

    async register(userData: RegisterRequest): Promise<{ success: boolean }> {
      try {
        // Call Nuxt API route that handles session creation  
        await authFetch('/auth/register', {
          method: 'POST',
          body: userData,
        })
        
        // Refresh session state on client-side
        await refreshSession()
        return { success: true }
      } catch (error: any) {
        throw new Error(error.data?.statusMessage || 'Registration failed')
      }
    },

    async previewSlug(displayName: string): Promise<SlugPreviewResponse> {
      // Direct call to backend (no session needed)
      return backendFetch('/auth/preview-slug', {
        method: 'POST',
        body: { display_name: displayName },
      })
    },

    async refreshToken(): Promise<{ success: boolean }> {
      try {
        await authFetch('/auth/refresh', { method: 'POST' })
        return { success: true }
      } catch (error: any) {
        throw new Error(error.data?.statusMessage || 'Token refresh failed')
      }
    },

    async revokeAllSessions(): Promise<{ success: boolean }> {
      try {
        const backendFetch = useBackendFetch()
        await backendFetch('/auth/revoke-all', { method: 'POST' })

        // Clear local session after revoking all sessions
        await this.logout()
        return { success: true }
      } catch (error: any) {
        throw new Error('Failed to revoke all sessions')
      }
    },

    async logout(): Promise<void> {
      try {
        console.log('🔍 [Auth Service] Starting logout process...')
        
        // Call the Nuxt server logout endpoint which handles refresh token revocation
        try {
          console.log('🔄 [Auth Service] Calling Nuxt server /api/auth/logout...')
          await $fetch('/api/auth/logout', { method: 'POST' })
          console.log('✅ [Auth Service] Logout completed on server')
        } catch (error) {
          console.error('❌ [Auth Service] Failed to logout on server:', error)
          // Continue with client-side cleanup even if server logout fails
        }
      } catch (error) {
        console.error('❌ [Auth Service] Error during logout:', error)
        // Continue with logout even if cleanup fails
      }

      // Clear the JWT token first
      const jwtManager = useJwtManager()
      jwtManager.clearToken()

      // Clear the session
      await clear()
    },
  }
}