export default defineEventHandler(async (event) => {
  try {
    // Get the user session to access the JWT token
    const session = await getUserSession(event)
    
    if (!session?.user) {
      throw createError({
        statusCode: 401,
        statusMessage: 'Authentication required'
      })
    }
    
    if (!session.secure?.jwtToken) {
      throw createError({
        statusCode: 401,
        statusMessage: 'No JWT token available in session'
      })
    }

    // Get the backend URL from runtime config
    const config = useRuntimeConfig()
    const backendUrl = config.apiBase

    console.log('🔍 [Phrases API] Fetching random phrase for user:', session.user.email)
    console.log('🔍 [Phrases API] Server config apiBase:', config.apiBase)
    console.log('🔍 [Phrases API] Public config apiBase:', config.public.apiBase)

    // Forward the request to the backend with the JWT token from session
    const response = await $fetch(`${backendUrl}/phrases/random`, {
      headers: {
        'Authorization': `Bearer ${session.secure.jwtToken}`,
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
