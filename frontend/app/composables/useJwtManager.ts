/**
 * useJwtManager - Simplified JWT token management
 *
 * Client-side JWT manager that delegates all complexity to server-side JWT handler.
 * Server handles token validation, refresh logic, and session management.
 */

interface JwtToken {
  token: string
  expiresAt: string
}

export function useJwtManager() {
  /**
   * Get JWT token from server
   * Server handles all validation and refresh logic automatically
   */
  const getToken = async (): Promise<string | null> => {
    try {
      console.log('🔄 [JWT Manager] Getting token from server')
      const response = await $fetch<JwtToken>('/api/auth/jwt')
      console.log('✅ [JWT Manager] Got token from server')
      return response.token
    } catch (error: any) {
      console.log('❌ [JWT Manager] No token available:', error.message)
      return null
    }
  }

  return {
    getToken
  }
}
