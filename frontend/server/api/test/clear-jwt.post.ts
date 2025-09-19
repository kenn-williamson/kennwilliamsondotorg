/**
 * Test endpoint to clear JWT token from session
 * 
 * This simulates the scenario where the JWT token is missing from the session
 * but the refresh token is still valid, to test the automatic refresh logic.
 */

export default defineEventHandler(async (event) => {
  try {
    // Get the current user session
    const session = await getUserSession(event)

    if (!session?.user) {
      throw createError({
        statusCode: 401,
        statusMessage: 'No user session found'
      })
    }

    console.log('🧪 [Test] Clearing JWT token from session for:', session.user.email)
    console.log('🧪 [Test] Before clear - JWT exists:', !!session.secure?.jwtToken)
    console.log('🧪 [Test] Before clear - Refresh exists:', !!session.secure?.refreshToken)

    // Clear only the JWT token, keep the refresh token
    const { jwtToken, ...secureWithoutJwt } = session.secure || {}
    console.log('🧪 [Test] secureWithoutJwt keys:', Object.keys(secureWithoutJwt))
    
    // Use replaceUserSession to completely overwrite the session data
    await replaceUserSession(event, {
      ...session,
      secure: secureWithoutJwt
    })

    // Verify the session was actually updated
    const updatedSession = await getUserSession(event)
    console.log('🧪 [Test] After clear - JWT exists:', !!updatedSession.secure?.jwtToken)
    console.log('🧪 [Test] After clear - Refresh exists:', !!updatedSession.secure?.refreshToken)
    console.log('✅ [Test] JWT token cleared from session')

    return {
      success: true,
      message: 'JWT token cleared from session. Refresh token preserved.',
      user: session.user.email
    }
  } catch (error: any) {
    console.error('❌ [Test] Failed to clear JWT token:', error.message)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.message || 'Failed to clear JWT token'
    })
  }
})
