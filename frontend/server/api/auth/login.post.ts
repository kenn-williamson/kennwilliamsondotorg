import { z } from 'zod'
import { defineEventHandler, readValidatedBody, createError } from 'h3'
import { useRuntimeConfig } from '#imports'

const bodySchema = z.object({
  email: z.string().email(),
  password: z.string().min(8)
})

export default defineEventHandler(async (event: any) => {
  const { email, password } = await readValidatedBody(event, bodySchema.parse)
  
  try {
    // Call the Rust backend for authentication
    const config = useRuntimeConfig()
    const response = await $fetch<{
      token: string
      user: {
        id: string
        email: string
        display_name: string
        slug: string
        roles: string[]
        created_at: string
      }
    }>(`${config.apiBase}/auth/login`, {
      method: 'POST',
      body: { email, password }
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
        // Store the JWT token for backend API calls
        jwtToken: response.token
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