/**
 * Server JWT Manager
 * 
 * Server-side JWT token management using existing jwt-handler.ts
 * Extracts JWT tokens directly from secure session cookies during SSR
 */

import { getValidJwtToken, requireValidJwtToken } from './jwt-handler'

export const serverJwtManager = {
  /**
   * Get JWT token from secure session cookies
   * @param event - Nuxt event object
   * @returns JWT token string or null if no valid token available
   */
  getToken: async (event: any): Promise<string | null> => {
    try {
      return await getValidJwtToken(event)
    } catch (error) {
      console.log('🔍 [Server JWT Manager] No valid token available:', error)
      return null
    }
  },

  /**
   * Require JWT token, throwing error if not available
   * @param event - Nuxt event object
   * @returns JWT token string
   * @throws 401 error if no valid token available
   */
  requireToken: async (event: any): Promise<string> => {
    return await requireValidJwtToken(event)
  }
}
