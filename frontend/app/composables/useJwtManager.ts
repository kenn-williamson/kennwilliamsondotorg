/**
 * useJwtManager - Simplified JWT token management
 *
 * Client-side JWT manager that delegates all complexity to server-side JWT handler.
 * Server handles token validation, refresh logic, and session management.
 */

import type { JwtToken } from '#shared/types'

export function useJwtManager() {
  // Cache for JWT token
  let cachedToken: string | null = null
  let tokenExpiresAt: number | null = null

  /**
   * Get JWT token from server with caching
   * Server handles all validation and refresh logic automatically
   */
  const getToken = async (): Promise<string | null> => {
    try {
      // Check if we have a valid cached token
      if (cachedToken && tokenExpiresAt && Date.now() < tokenExpiresAt) {
        console.log('✅ [JWT Manager] Using cached token')
        return cachedToken
      }

      // Use API endpoint for both SSR and client-side
      // The /api/auth/jwt endpoint handles JWT extraction properly
      console.log('🔄 [JWT Manager] Getting fresh token from /api/auth/jwt')
      const response = await $fetch<JwtToken>('/api/auth/jwt')
      console.log('✅ [JWT Manager] Got fresh token from /api/auth/jwt')
      
      // Cache the token
      cachedToken = response.token
      tokenExpiresAt = new Date(response.expiresAt).getTime()
      
      return response.token
  } catch (error: any) {
    const errorMessage = error instanceof Error ? error.message : String(error)
    console.log('❌ [JWT Manager] No token available:', errorMessage)
    
    // Clear cache on error
    cachedToken = null
    tokenExpiresAt = null
    
    // If this is a 401 (session expired), clear the session and redirect
    if (error.statusCode === 401 && process.client) {
      console.log('🔄 [JWT Manager] Session expired, clearing session and redirecting to login')
      const { clear } = useUserSession()
      await clear()
      await navigateTo('/login')
    }
    
    // Re-throw the error so stores can handle it
    throw error
  }
  }

  /**
   * Clear cached token (useful for logout)
   */
  const clearToken = () => {
    cachedToken = null
    tokenExpiresAt = null
    console.log('🧹 [JWT Manager] Cleared cached token')
  }

  return {
    getToken,
    clearToken
  }
}
