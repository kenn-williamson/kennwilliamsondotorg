import { defineEventHandler, createError } from 'h3'
import { useRuntimeConfig } from '#imports'
import { getClientInfo } from '../../utils/client-ip'

export default defineEventHandler(async (event) => {
  try {
    // Get the current user session to access the refresh token
    const session = await getUserSession(event)

    if (!session?.user) {
      throw createError({
        statusCode: 401,
        statusMessage: 'Authentication required'
      })
    }

    const refreshToken = session.secure?.refreshToken
    if (!refreshToken) {
      throw createError({
        statusCode: 401,
        statusMessage: 'No refresh token available'
      })
    }

    console.log('🔄 [Refresh API] Attempting token refresh for:', session.user.email)

    // Call the backend refresh endpoint
    const config = useRuntimeConfig()
    
    // Extract client information for proper IP forwarding
    const clientInfo = getClientInfo(event)
    
    console.log(`🔍 [Refresh API] Client IP: ${clientInfo.ip}, User-Agent: ${clientInfo.userAgent}`)
    
    const response = await $fetch<{
      token: string
      refresh_token: string
    }>(`${config.apiBase}/auth/refresh`, {
      method: 'POST',
      body: { refresh_token: refreshToken },
      headers: {
        // Forward the original client IP headers for proper refresh token tracking
        'X-Real-IP': clientInfo.ip,
        'X-Forwarded-For': clientInfo.ip,
        'X-Forwarded-Proto': clientInfo.protocol,
        'User-Agent': clientInfo.userAgent
      }
    })

    console.log('✅ [Refresh API] Got new tokens, updating session')

    // Update the session with new tokens
    await setUserSession(event, {
      ...session,
      secure: {
        jwtToken: response.token,
        refreshToken: response.refresh_token
      }
    })

    return { success: true }
  } catch (error: any) {
    console.error('❌ [Refresh API] Token refresh failed:', error.message)

    // Clear the session on refresh failure
    await clearUserSession(event)

    throw createError({
      statusCode: 401,
      statusMessage: 'Token refresh failed'
    })
  }
})