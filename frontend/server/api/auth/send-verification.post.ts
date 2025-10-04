import { defineEventHandler, createError } from 'h3'
import { useRuntimeConfig } from '#imports'
import { getClientInfo } from '../../utils/client-ip'
import { API_ROUTES } from '#shared/config/api-routes'
import { requireValidJwtToken } from '../../utils/jwt-handler'

export default defineEventHandler(async (event) => {
  try {
    // Get valid JWT token (with automatic refresh if needed)
    const jwtToken = await requireValidJwtToken(event)

    console.log('🔍 [Send Verification] Sending verification email')

    const config = useRuntimeConfig()

    // Extract client information for proper IP forwarding
    const clientInfo = getClientInfo(event)

    console.log(`🔍 [Send Verification] Client IP: ${clientInfo.ip}, User-Agent: ${clientInfo.userAgent}`)

    const response = await $fetch<{
      message: string
    }>(`${config.apiBase}${API_ROUTES.PROTECTED.SEND_VERIFICATION}`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${jwtToken}`,
        // Forward the original client IP headers
        'X-Real-IP': clientInfo.ip,
        'X-Forwarded-For': clientInfo.ip,
        'X-Forwarded-Proto': clientInfo.protocol,
        'User-Agent': clientInfo.userAgent
      }
    })

    console.log('✅ [Send Verification] Verification email sent successfully')

    return response
  } catch (error: any) {
    console.error('❌ [Send Verification] Failed to send verification email:', error)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.data?.error || 'Failed to send verification email'
    })
  }
})
