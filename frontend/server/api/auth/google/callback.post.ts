import { defineEventHandler, createError, readBody } from 'h3'
import { useRuntimeConfig } from '#imports'
import { getClientInfo } from '../../../utils/client-ip'
import { API_ROUTES } from '#shared/config/api-routes'

export default defineEventHandler(async (event: any) => {
  // Get code and state from POST body
  const body = await readBody(event)
  const { code, state } = body

  if (!code) {
    console.error('❌ [Google OAuth Callback] No authorization code provided')
    throw createError({
      statusCode: 400,
      message: 'No authorization code provided'
    })
  }

  if (!state) {
    console.error('❌ [Google OAuth Callback] No state parameter provided')
    throw createError({
      statusCode: 400,
      message: 'No state parameter provided'
    })
  }

  try {
    const config = useRuntimeConfig()

    // Extract client information for proper IP forwarding
    const clientInfo = getClientInfo(event)

    console.log(`🔍 [Google OAuth Callback] Processing callback for state: ${state}`)

    const response = await $fetch<{
      token: string
      refresh_token: string
      user: {
        id: string
        email: string
        display_name: string
        slug: string
        roles: string[]
        created_at: string
        email_verified?: boolean
        real_name?: string
        google_user_id?: string
      }
    }>(`${config.apiBase}${API_ROUTES.PUBLIC.GOOGLE_CALLBACK}`, {
      method: 'POST',
      body: { code, state },
      headers: {
        // Forward the original client IP headers for proper refresh token tracking
        'X-Real-IP': clientInfo.ip,
        'X-Forwarded-For': clientInfo.ip,
        'X-Forwarded-Proto': clientInfo.protocol,
        'User-Agent': clientInfo.userAgent
      }
    })

    // Set user session with JWT tokens
    await setUserSession(event, {
      user: {
        id: response.user.id,
        email: response.user.email,
        display_name: response.user.display_name,
        slug: response.user.slug,
        roles: response.user.roles,
        created_at: response.user.created_at,
        email_verified: response.user.email_verified,
        real_name: response.user.real_name,
        google_user_id: response.user.google_user_id,
      },
      secure: {
        // Store the JWT token and refresh token for backend API calls
        jwtToken: response.token,
        refreshToken: response.refresh_token
      },
      loggedInAt: new Date()
    })

    console.log('✅ [Google OAuth Callback] Session set successfully')

    return {
      success: true,
      user: response.user
    }
  } catch (error: any) {
    console.error('❌ [Google OAuth Callback] Error:', error)

    // Check for specific error messages from backend
    const errorMessage = error.data?.error || error.message || 'Authentication failed'

    throw createError({
      statusCode: error.statusCode || 400,
      message: errorMessage
    })
  }
})
