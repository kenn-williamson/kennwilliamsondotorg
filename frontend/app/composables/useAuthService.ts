/**
 * useAuthService - Authentication operations using Nuxt Auth Utils
 * 
 * Handles login/register operations via Nuxt API routes that manage sessions.
 * For reactive state, use useUserSession() directly in components.
 */

import { useBackendFetch } from './useBackendFetch'

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

  return {
    async login(credentials: LoginRequest): Promise<{ success: boolean }> {
      try {
        // Call Nuxt API route that handles session creation
        await $fetch('/api/auth/login', {
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
        await $fetch('/api/auth/register', {
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

    async logout(): Promise<void> {
      // Clear the JWT token first
      const jwtManager = useJwtManager()
      jwtManager.clearToken()
      
      // Clear the session
      await clear()
    },
  }
}