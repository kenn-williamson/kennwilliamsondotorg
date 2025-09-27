import { API_ROUTES } from '#shared/config/api-routes'
import { requireValidJwtToken } from '../../utils/jwt-handler'

export default defineEventHandler(async (event) => {
  try {
    // Get valid JWT token (with automatic refresh if needed)
    const jwtToken = await requireValidJwtToken(event)

    // Get the backend URL from runtime config
    const config = useRuntimeConfig()

    console.log('🔍 [Phrases API] Fetching random phrase')
    console.log('🔍 [Phrases API] Server config apiBase:', config.apiBase)
    console.log('🔍 [Phrases API] Public config apiBase:', config.public.apiBase)

    const response = await $fetch(`${config.apiBase}${API_ROUTES.PROTECTED.PHRASES.RANDOM}`, {
      headers: {
        'Authorization': `Bearer ${jwtToken}`,
        'Content-Type': 'application/json'
      }
    })

    return response
  } catch (error: any) {
    console.error('Error fetching random phrase:', error)
    
    // If it's a 401, pass it through
    if (error.statusCode === 401) {
      throw createError({
        statusCode: 401,
        statusMessage: 'Unauthorized'
      })
    }
    
    // For other errors, return a generic error
    throw createError({
      statusCode: 500,
      statusMessage: 'Failed to fetch random phrase'
    })
  }
})
