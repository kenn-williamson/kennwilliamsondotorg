import { parseJwtToken } from '#shared/utils/jwt'

export default defineEventHandler(async (event) => {
  try {
    // Get the user session to access the JWT token
    const session = await getUserSession(event)
    
    if (!session?.user) {
      throw createError({
        statusCode: 401,
        statusMessage: 'Authentication required'
      })
    }
    
    if (!session.secure?.jwtToken) {
      throw createError({
        statusCode: 401,
        statusMessage: 'No JWT token available in session'
      })
    }
    
    console.log('🔍 [JWT API] Providing JWT token for:', session.user.email)

    // Extract expiration from JWT token using shared utility
    const result = parseJwtToken(session.secure.jwtToken)
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
      token: session.secure.jwtToken,
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
