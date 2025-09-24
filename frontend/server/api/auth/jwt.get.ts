import { parseJwtToken } from '#shared/utils/jwt'
import { getValidJwtToken } from '../../utils/jwt-handler'

export default defineEventHandler(async (event) => {
  try {
    // Get valid JWT token (with automatic refresh if needed)
    const jwtToken = await getValidJwtToken(event)
    
    if (!jwtToken) {
      // Clear the session and return 401 error (don't redirect during SSR)
      console.log('🔄 [JWT API] No valid authentication, clearing session')
      await clearUserSession(event)
      throw createError({
        statusCode: 401,
        statusMessage: 'Authentication required'
      })
    }
    
    console.log('🔍 [JWT API] Providing JWT token')

    // Extract expiration from JWT token using shared utility
    const result = parseJwtToken(jwtToken)
    let expiresAt: string
    
    if (result.isValid) {
      expiresAt = result.expiration.toISOString()
      console.log('🔍 [JWT API] Token expires at:', expiresAt)
    } else {
      console.warn('⚠️ [JWT API] Failed to parse JWT expiration:', result.error)
      expiresAt = new Date(Date.now() + 60 * 60 * 1000).toISOString() // 1 hour from now
    }

    // Return the JWT token to the client
    return {
      token: jwtToken,
      expiresAt
    }
  } catch (error: any) {
    console.log('❌ [JWT API] Error providing JWT token:', error.message)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.data?.error || 'Failed to get JWT token'
    })
  }
})
