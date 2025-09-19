import { defineEventHandler, createError } from 'h3'
import { useRuntimeConfig } from '#imports'
import { getClientInfo } from '../../utils/client-ip'

export default defineEventHandler(async (event) => {
  try {
    // Get the current user session to access the JWT token
    const session = await getUserSession(event)

    if (!session?.user) {
      throw createError({
        statusCode: 401,
        statusMessage: 'Authentication required'
      })
    }

    const jwtToken = session.secure?.jwtToken
    if (!jwtToken) {
      throw createError({
        statusCode: 401,
        statusMessage: 'No JWT token available'
      })
    }

    console.log('🔍 [Me API] Fetching current user for:', session.user.email)

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
    }>(`${config.apiBase}/auth/me`, {
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

    return response
  } catch (error: any) {
    console.error('❌ [Me API] Failed to fetch user data:', error.message)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.data?.error || 'Failed to fetch user data'
    })
  }
})
