/**
 * Test endpoint to check current session state
 * 
 * This shows what tokens are available in the session without exposing sensitive data.
 */

export default defineEventHandler(async (event) => {
  try {
    // Get the current user session
    const session = await getUserSession(event)

    if (!session?.user) {
      return {
        hasUser: false,
        hasJwtToken: false,
        hasRefreshToken: false,
        message: 'No user session found'
      }
    }

    const hasJwtToken = !!session.secure?.jwtToken
    const hasRefreshToken = !!session.secure?.refreshToken

    console.log('🧪 [Test] Session state for:', session.user.email)
    console.log('🧪 [Test] Has JWT token:', hasJwtToken)
    console.log('🧪 [Test] Has refresh token:', hasRefreshToken)

    return {
      hasUser: true,
      hasJwtToken,
      hasRefreshToken,
      user: {
        email: session.user.email,
        display_name: session.user.display_name
      },
      message: 'Session state retrieved'
    }
  } catch (error: any) {
    console.error('❌ [Test] Failed to get session state:', error.message)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.message || 'Failed to get session state'
    })
  }
})
