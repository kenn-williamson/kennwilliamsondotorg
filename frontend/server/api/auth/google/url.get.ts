import { defineEventHandler, createError, getQuery } from 'h3'
import { useRuntimeConfig } from '#imports'
import { API_ROUTES } from '#shared/config/api-routes'

export default defineEventHandler(async (event) => {
  try {
    const config = useRuntimeConfig()
    const query = getQuery(event)
    const redirect = query.redirect as string | undefined

    console.log('🔍 [Google OAuth URL] Fetching OAuth authorization URL', redirect ? `with redirect: ${redirect}` : '')

    // Build URL with redirect parameter if present
    const url = redirect
      ? `${config.apiBase}${API_ROUTES.PUBLIC.AUTH.GOOGLE_URL}?redirect=${encodeURIComponent(redirect)}`
      : `${config.apiBase}${API_ROUTES.PUBLIC.AUTH.GOOGLE_URL}`

    const response = await $fetch<{
      url: string
    }>(url, {
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
