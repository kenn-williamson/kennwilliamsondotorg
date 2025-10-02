import { z } from 'zod'
import { defineEventHandler, readValidatedBody, createError } from 'h3'
import { useRuntimeConfig } from '#imports'
import { getClientInfo } from '../../utils/client-ip'
import { API_ROUTES } from '#shared/config/api-routes'
import { rateLimitMiddleware } from '../../utils/rate-limiter'

const bodySchema = z.object({
  email: z.string().email(),
  display_name: z.string().min(2).max(50).trim(),
  password: z.string().min(8)
})

export default defineEventHandler(async (event: any) => {
  const { email, display_name, password } = await readValidatedBody(event, bodySchema.parse)
  
  // Check rate limit for registration
  const isRateLimited = await rateLimitMiddleware(event, '/auth/register')
  if (isRateLimited) {
    throw createError({
      statusCode: 429,
      statusMessage: 'Too many registration attempts. Please wait 5 minutes before trying again.'
    })
  }
  
  try {
    const config = useRuntimeConfig()
    
    // Extract client information for proper IP forwarding
    const clientInfo = getClientInfo(event)
    
    console.log(`🔍 [Register API] Client IP: ${clientInfo.ip}, User-Agent: ${clientInfo.userAgent}`)
    
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
      }
    }>(`${config.apiBase}${API_ROUTES.PUBLIC.AUTH.REGISTER}`, {
      method: 'POST',
      body: { email, display_name, password },
      headers: {
        // Forward the original client IP headers for proper refresh token tracking
        'X-Real-IP': clientInfo.ip,
        'X-Forwarded-For': clientInfo.ip,
        'X-Forwarded-Proto': clientInfo.protocol,
        'User-Agent': clientInfo.userAgent
      }
    })
    
    await setUserSession(event, {
      user: {
        id: response.user.id,
        email: response.user.email,
        display_name: response.user.display_name,
        slug: response.user.slug,
        roles: response.user.roles,
        created_at: response.user.created_at
      },
      secure: {
        // Store the JWT token and refresh token for backend API calls
        jwtToken: response.token,
        refreshToken: response.refresh_token
      },
      loggedInAt: new Date()
    })
    
    return { success: true }
  } catch (error: any) {
    throw createError({
      statusCode: error.statusCode || 400,
      statusMessage: error.data?.error || 'Registration failed'
    })
  }
})