import { API_ROUTES } from '#shared/config/api-routes'
import { requireValidJwtToken } from '../utils/jwt-handler'

export default defineEventHandler(async (event) => {
  try {
    // Get valid JWT token (with automatic refresh if needed)
    const jwtToken = await requireValidJwtToken(event)
    
    console.log('🔍 [SSR API] Getting user timers')
    
    // Call the backend with the JWT token
    const config = useRuntimeConfig()
    const timers = await $fetch(`${config.apiBase}${API_ROUTES.PROTECTED.TIMERS.LIST}`, {
      headers: {
        'Authorization': `Bearer ${jwtToken}`
      }
    })
    
    console.log('✅ [SSR API] Retrieved timers for SSR:', timers)
    return timers
  } catch (error: any) {
    console.log('❌ [SSR API] Error getting timers:', error.message)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.data?.error || 'Failed to get timers'
    })
  }
})
