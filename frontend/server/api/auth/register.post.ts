import { z } from 'zod'
import { defineEventHandler, readValidatedBody, createError } from 'h3'
import { useRuntimeConfig } from '#imports'

const bodySchema = z.object({
  email: z.string().email(),
  display_name: z.string().min(2).max(50).trim(),
  password: z.string().min(8)
})

export default defineEventHandler(async (event: any) => {
  const { email, display_name, password } = await readValidatedBody(event, bodySchema.parse)
  
  try {
    // Call the Rust backend for registration
    const config = useRuntimeConfig()
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
    }>(`${config.apiBase}/auth/register`, {
      method: 'POST',
      body: { email, display_name, password }
    })
    
    // Set the user session using Nuxt Auth Utils
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