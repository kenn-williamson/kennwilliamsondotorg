import { defineEventHandler, createError } from 'h3'
import { useRuntimeConfig } from '#imports'
import { getClientInfo } from '../../utils/client-ip'
import { API_ROUTES } from '#shared/config/api-routes'
import { requireValidJwtToken } from '../../utils/jwt-handler'

export default defineEventHandler(async (event) => {
  try {
    // Get valid JWT token (with automatic refresh if needed)
    const jwtToken = await requireValidJwtToken(event)

    console.log('🔍 [Profile API] Updating user profile')

    const body = await readBody(event)

    const config = useRuntimeConfig()
    
    // Extract client information for proper IP forwarding
    const clientInfo = getClientInfo(event)
    
    console.log(`🔍 [Profile API] Client IP: ${clientInfo.ip}, User-Agent: ${clientInfo.userAgent}`)
    
    const response = await $fetch<{
      id: string
      email: string
      display_name: string
      slug: string
      roles: string[]
      created_at: string
    }>(`${config.apiBase}${API_ROUTES.PROTECTED.AUTH.PROFILE}`, {
      method: 'PUT',
      headers: {
        'Authorization': `Bearer ${jwtToken}`,
        'Content-Type': 'application/json',
        // Forward the original client IP headers
        'X-Real-IP': clientInfo.ip,
        'X-Forwarded-For': clientInfo.ip,
        'X-Forwarded-Proto': clientInfo.protocol,
        'User-Agent': clientInfo.userAgent
      },
      body
    })

    console.log('✅ [Profile API] Profile updated successfully')

    const session = await getUserSession(event)
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

    console.log('✅ [Profile API] Updated session with fresh user data')

    return response
  } catch (error: any) {
    console.error('❌ [Profile API] Failed to update profile:', error.message)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.data?.error || 'Failed to update profile'
    })
  }
})
