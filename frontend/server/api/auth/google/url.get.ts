import { defineEventHandler, createError } from 'h3'
import { useRuntimeConfig } from '#imports'
import { API_ROUTES } from '#shared/config/api-routes'

export default defineEventHandler(async (event) => {
  try {
    const config = useRuntimeConfig()

    console.log('🔍 [Google OAuth URL] Fetching OAuth authorization URL')

    const response = await $fetch<{
      url: string
    }>(`${config.apiBase}${API_ROUTES.PUBLIC.GOOGLE_URL}`, {
      method: 'GET',
    })

    console.log('✅ [Google OAuth URL] Successfully retrieved OAuth URL')

    return response
  } catch (error: any) {
    console.error('❌ [Google OAuth URL] Failed to get OAuth URL:', error)
    throw createError({
      statusCode: error.statusCode || 503,
      statusMessage: error.data?.error || 'Google OAuth is not configured'
    })
  }
})
