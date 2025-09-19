import { API_ROUTES } from '#shared/config/api-routes'

export default defineEventHandler(async (event) => {
  const userSlug = getRouterParam(event, 'user_slug')
  
  console.log(`🔍 [API] Phrase request for user: ${userSlug}`)
  console.log(`🔍 [API] Request headers:`, getHeaders(event))
  console.log(`🔍 [API] Request URL:`, getRequestURL(event).toString())
  
  if (!userSlug) {
    console.log(`❌ [API] No user slug provided`)
    throw createError({
      statusCode: 400,
      statusMessage: 'User slug is required'
    })
  }
  
  try {
    // Public endpoint - no authentication required
    const config = useRuntimeConfig()
    const backendUrl = `${config.apiBase}${API_ROUTES.PUBLIC.PHRASES.BY_USER_SLUG(userSlug)}`
    
    console.log(`🔍 [API] Calling backend at: ${backendUrl}`)
    console.log(`🔍 [API] Server config apiBase: ${config.apiBase}`)
    console.log(`🔍 [API] Public config apiBase: ${config.public.apiBase}`)
    
    const phraseResponse = await $fetch(backendUrl)
    
    console.log(`✅ [API] Backend response received:`, phraseResponse)
    return phraseResponse
  } catch (error: any) {
    console.log(`❌ [API] Backend call failed:`, error.message)
    console.log(`❌ [API] Error details:`, error)
    throw createError({
      statusCode: error.statusCode || 404,
      statusMessage: error.data?.error || 'Phrase not found'
    })
  }
})
