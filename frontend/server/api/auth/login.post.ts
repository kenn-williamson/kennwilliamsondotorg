import { z } from 'zod'
import { defineEventHandler, readValidatedBody, createError } from 'h3'
import { useRuntimeConfig } from '#imports'
import { getClientInfo } from '../../utils/client-ip'
import { API_ROUTES } from '#shared/config/api-routes'
import { rateLimitMiddleware } from '../../utils/rate-limiter'
import type { AuthResponse } from '#shared/types'

const bodySchema = z.object({
  email: z.string().email(),
  password: z.string().min(8)
})

export default defineEventHandler(async (event: any) => {
  const { email, password } = await readValidatedBody(event, bodySchema.parse)

  // Check rate limit for login
  const isRateLimited = await rateLimitMiddleware(event, '/auth/login')
  if (isRateLimited) {
    throw createError({
      statusCode: 429,
      statusMessage: 'Too many login attempts. Please wait 5 minutes before trying again.'
    })
  }
  
  try {
    const config = useRuntimeConfig()
    
    // Extract client information for proper IP forwarding
    const clientInfo = getClientInfo(event)
    
    console.log(`🔍 [Login API] Client IP: ${clientInfo.ip}, User-Agent: ${clientInfo.userAgent}`)
    
    const response = await $fetch<AuthResponse>(`${config.apiBase}${API_ROUTES.PUBLIC.AUTH.LOGIN}`, {
      method: 'POST',
      body: { email, password },
      headers: {
        // Forward the original client IP headers for proper refresh token tracking
        'X-Real-IP': clientInfo.ip,
        'X-Forwarded-For': clientInfo.ip,
        'X-Forwarded-Proto': clientInfo.protocol,
        'User-Agent': clientInfo.userAgent
      }
    })

    await setUserSession(event, {
      user: response.user,
      secure: {
        // Store the JWT token and refresh token for backend API calls
        jwtToken: response.token,
        refreshToken: response.refresh_token
      },
      loggedInAt: new Date()
    })
    
    return { success: true }
  } catch (error: any) {
    console.error('Login error:', error)
    throw createError({
      statusCode: error.statusCode || 401,
      statusMessage: error.data?.error || 'Invalid email or password'
    })
  }
})