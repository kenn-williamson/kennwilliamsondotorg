import { defineEventHandler, createError } from 'h3'
import { performRefresh } from '../../utils/jwt-handler'

export default defineEventHandler(async (event) => {
  try {
    // Force refresh to get new JWT with latest user data and roles from database
    const session = await getUserSession(event)
    const refreshToken = session?.secure?.refreshToken

    if (!refreshToken) {
      throw createError({
        statusCode: 401,
        statusMessage: 'Authentication required'
      })
    }

    console.log('🔄 [Me API] Forcing token refresh to get latest user data and roles')

    // performRefresh calls the refresh endpoint which returns full AuthResponse
    // and automatically updates the session with fresh user data and tokens
    await performRefresh(event, session, refreshToken)

    // Get the updated session with fresh user data
    const updatedSession = await getUserSession(event)

    if (!updatedSession?.user) {
      throw createError({
        statusCode: 401,
        statusMessage: 'Failed to refresh session'
      })
    }

    console.log('✅ [Me API] Session refreshed with latest user data and roles:', updatedSession.user.roles)

    return updatedSession.user
  } catch (error: any) {
    console.error('❌ [Me API] Failed to fetch user data:', error.message)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.data?.error || 'Failed to fetch user data'
    })
  }
})
