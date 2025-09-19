/**
 * Session Check Endpoint
 * 
 * This endpoint checks if there's a valid user session without requiring a JWT token.
 * Used by the auth manager to determine if the user is logged in before attempting refresh.
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
        user: null
      }
    }

    const hasJwtToken = !!session.secure?.jwtToken
    const hasRefreshToken = !!session.secure?.refreshToken

    console.log('🔍 [Session Check] User session found:', session.user.email)
    console.log('🔍 [Session Check] Has JWT token:', hasJwtToken)
    console.log('🔍 [Session Check] Has refresh token:', hasRefreshToken)

    return {
      hasUser: true,
      hasJwtToken,
      hasRefreshToken,
      user: {
        id: session.user.id,
        email: session.user.email,
        display_name: session.user.display_name,
        slug: session.user.slug,
        roles: session.user.roles,
        created_at: session.user.created_at
      }
    }
  } catch (error: any) {
    console.error('❌ [Session Check] Failed to check session:', error.message)
    return {
      hasUser: false,
      hasJwtToken: false,
      hasRefreshToken: false,
      user: null
    }
  }
})
