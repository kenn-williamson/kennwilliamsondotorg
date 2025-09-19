/**
 * JWT Handler Utility
 * 
 * Centralized JWT token management for all Nuxt API routes.
 * Handles JWT retrieval, validation, and automatic refresh.
 */

import { isJwtExpired } from '#shared/utils/jwt'
import { API_ROUTES } from '#shared/config/api-routes'

// Refresh lock to prevent multiple simultaneous refresh operations
const refreshLocks = new Map<string, Promise<string | null>>()

/**
 * Get a valid JWT token, attempting refresh if needed
 * @param event - Nuxt event object
 * @returns JWT token string or null if no valid token available
 */
export async function getValidJwtToken(event: any): Promise<string | null> {
  try {
    // Log which endpoint is calling the JWT handler
    console.log(`🔍 [JWT Handler] Called from: ${event.node.req.method} ${event.node.req.url}`)
    
    // 1. Get JWT token from session
    const session = await getUserSession(event)
    const jwtToken = session?.secure?.jwtToken
    
    // 2. Check if JWT exists and is valid
    if (jwtToken && !isJwtExpired(jwtToken)) {
      return jwtToken
    }
    
    // 3. If no valid JWT but refresh token exists → attempt refresh
    const refreshToken = session?.secure?.refreshToken
    if (refreshToken) {
      // Create a unique key for this user's refresh operation
      const userId = session?.user?.id || 'anonymous'
      const lockKey = `refresh_${userId}`
      
      // Check if there's already a refresh operation in progress
      if (refreshLocks.has(lockKey)) {
        console.log('⏳ [JWT Handler] Refresh already in progress, waiting...')
        return await refreshLocks.get(lockKey)!
      }
      
      console.log('🔄 [JWT Handler] Attempting refresh with token:', refreshToken.substring(0, 20) + '...')
      
      // Create the refresh promise and store it in the lock
      const refreshPromise = performRefresh(event, session, refreshToken)
      refreshLocks.set(lockKey, refreshPromise)
      
      try {
        const result = await refreshPromise
        return result
      } finally {
        // Clean up the lock when done
        refreshLocks.delete(lockKey)
      }
    }
    
    // 5. If refresh fails → return null
    return null
  } catch (error) {
    console.log('❌ [JWT Handler] Error getting valid token:', error)
    return null
  }
}

/**
 * Perform the actual refresh operation
 */
async function performRefresh(event: any, session: any, refreshToken: string): Promise<string | null> {
  try {
    const config = useRuntimeConfig()
    const refreshResponse = await $fetch<{
      token: string
      refresh_token: string
    }>(`${config.apiBase}${API_ROUTES.PUBLIC.AUTH.REFRESH}`, {
      method: 'POST',
      body: { refresh_token: refreshToken }
    })
    
    console.log('✅ [JWT Handler] Refresh successful, got new tokens')
    console.log('🔄 [JWT Handler] New JWT:', refreshResponse.token.substring(0, 20) + '...')
    console.log('🔄 [JWT Handler] New refresh token:', refreshResponse.refresh_token.substring(0, 20) + '...')
    
    // 4. If refresh succeeds → update session and return new JWT
    if (refreshResponse.token) {
      console.log('🔄 [JWT Handler] Updating session with new tokens...')
      await setUserSession(event, {
        secure: {
          jwtToken: refreshResponse.token,
          refreshToken: refreshResponse.refresh_token
        }
      })
      console.log('✅ [JWT Handler] Session updated successfully')
      
      // Re-read the session to ensure we have the latest data
      const updatedSession = await getUserSession(event)
      console.log('🔄 [JWT Handler] Re-read session, new refresh token:', updatedSession?.secure?.refreshToken?.substring(0, 20) + '...')
      
      return refreshResponse.token
    }
    
    return null
  } catch (refreshError) {
    console.log('❌ [JWT Handler] Refresh failed:', refreshError)
    // Clear the entire user session on refresh failure
    await clearUserSession(event)
    return null
  }
}

/**
 * Get a valid JWT token, throwing 401 error if not available
 * @param event - Nuxt event object
 * @returns JWT token string
 * @throws 401 error if no valid token available
 */
export async function requireValidJwtToken(event: any): Promise<string> {
  const token = await getValidJwtToken(event)
  if (!token) {
    throw createError({
      statusCode: 401,
      statusMessage: 'Authentication required'
    })
  }
  return token
}
