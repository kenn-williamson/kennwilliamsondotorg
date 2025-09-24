import { defineEventHandler, createError } from 'h3'
import { useRuntimeConfig } from '#imports'
import { API_ROUTES } from '#shared/config/api-routes'

export default defineEventHandler(async (event: any) => {
  try {
    // Get the user session to access the refresh token
    const session = await getUserSession(event)
    
    // If no user data, just clear the session (handles stale sessions)
    if (!session?.user) {
      console.log('🔍 [Logout API] No user data found, clearing stale session')
      await clearUserSession(event)
      return { success: true, message: 'Stale session cleared' }
    }
    
    const refreshToken = session.secure?.refreshToken
    console.log('🔍 [Logout API] Starting logout process for:', session.user.email)
    console.log('🔍 [Logout API] Refresh token exists:', !!refreshToken)

    // Revoke the refresh token on the backend if we have one
    if (refreshToken) {
      try {
        const config = useRuntimeConfig()
        console.log('🔄 [Logout API] Calling backend /auth/revoke...')
        
        await $fetch(`${config.apiBase}${API_ROUTES.PROTECTED.AUTH.REVOKE}`, {
          method: 'POST',
          body: { refresh_token: refreshToken }
        })
        
        console.log('✅ [Logout API] Refresh token revoked on backend')
      } catch (error: any) {
        console.error('❌ [Logout API] Failed to revoke refresh token:', error.message)
        // Continue with logout even if revoke fails
      }
    } else {
      console.log('⚠️ [Logout API] No refresh token found, skipping revocation')
    }

    // Clear the session (this will remove both JWT and refresh token)
    await clearUserSession(event)
    
    console.log('✅ [Logout API] Session cleared successfully')
    
    return { success: true }
  } catch (error: any) {
    console.error('❌ [Logout API] Error during logout:', error.message)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.data?.error || 'Logout failed'
    })
  }
})
