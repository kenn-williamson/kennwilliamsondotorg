import { defineEventHandler, createError } from 'h3'
import { useRuntimeConfig } from '#imports'
import { getClientInfo } from '../../utils/client-ip'
import { API_ROUTES } from '#shared/config/api-routes'
import { requireValidJwtToken } from '../../utils/jwt-handler'

export default defineEventHandler(async (event) => {
  try {
    // Get valid JWT token (with automatic refresh if needed)
    const jwtToken = await requireValidJwtToken(event)

    console.log('🔍 [Me API] Fetching current user')

    // Call the backend /auth/me endpoint
    const config = useRuntimeConfig()
    
    // Extract client information for proper IP forwarding
    const clientInfo = getClientInfo(event)
    
    console.log(`🔍 [Me API] Client IP: ${clientInfo.ip}, User-Agent: ${clientInfo.userAgent}`)
    
    const response = await $fetch<{
      id: string
      email: string
      display_name: string
      slug: string
      roles: string[]
      created_at: string
    }>(`${config.apiBase}${API_ROUTES.PROTECTED.AUTH.ME}`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${jwtToken}`,
        // Forward the original client IP headers
        'X-Real-IP': clientInfo.ip,
        'X-Forwarded-For': clientInfo.ip,
        'X-Forwarded-Proto': clientInfo.protocol,
        'User-Agent': clientInfo.userAgent
      }
    })

    console.log('✅ [Me API] Got fresh user data from backend')

    // Get current session to update it with fresh user data
    const session = await getUserSession(event)
    
    // Update the session with fresh user data to keep it in sync
    await setUserSession(event, {
      ...session,
      user: {
        id: response.id,
        email: response.email,
        display_name: response.display_name,
        slug: response.slug,
        roles: response.roles,
        created_at: response.created_at
      }
    })

    console.log('✅ [Me API] Updated session with fresh user data')

    return response
  } catch (error: any) {
    console.error('❌ [Me API] Failed to fetch user data:', error.message)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.data?.error || 'Failed to fetch user data'
    })
  }
})
