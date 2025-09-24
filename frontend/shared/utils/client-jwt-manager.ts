/**
 * Client JWT Manager
 * 
 * Client-side JWT token management with caching
 * Avoids API calls during SSR hydration by using cached tokens
 */

import { isJwtExpired } from './jwt'

// Token cache
let cachedToken: string | null = null
let tokenExpiresAt: number | null = null

export const clientJwtManager = {
  /**
   * Get cached JWT token if valid
   * @returns JWT token string or null if no valid cached token
   */
  getToken: async (): Promise<string | null> => {
    // Check if we have a valid cached token
    if (cachedToken && tokenExpiresAt && Date.now() < tokenExpiresAt) {
      console.log('✅ [Client JWT Manager] Using cached token')
      return cachedToken
    }

    // Check if cached token is expired
    if (cachedToken && isJwtExpired(cachedToken)) {
      console.log('🔄 [Client JWT Manager] Cached token expired, clearing cache')
      cachedToken = null
      tokenExpiresAt = null
    }

    // During SSR hydration, don't make API calls - just return null
    // This prevents the hydration failure we were experiencing
    if (process.server) {
      console.log('🔍 [Client JWT Manager] SSR context - no API calls during hydration')
      return null
    }

    console.log('🔍 [Client JWT Manager] No valid cached token available')
    return null
  },

  /**
   * Set JWT token in cache
   * @param token - JWT token string
   * @param expiresAt - Token expiration timestamp
   */
  setToken: (token: string, expiresAt?: number): void => {
    cachedToken = token
    tokenExpiresAt = expiresAt || (Date.now() + 60 * 60 * 1000) // Default 1 hour
    console.log('✅ [Client JWT Manager] Token cached until:', new Date(tokenExpiresAt).toISOString())
  },

  /**
   * Clear cached token
   */
  clearToken: (): void => {
    cachedToken = null
    tokenExpiresAt = null
    console.log('🧹 [Client JWT Manager] Token cache cleared')
  },

  /**
   * Check if we have a valid cached token
   * @returns true if valid cached token exists
   */
  hasValidToken: (): boolean => {
    return !!(cachedToken && tokenExpiresAt && Date.now() < tokenExpiresAt && !isJwtExpired(cachedToken))
  }
}
