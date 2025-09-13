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
    
    console.log('🔍 [SSR API] Getting user timers for:', session.user.email)
    
    // Call the backend with the JWT token from the session
    const config = useRuntimeConfig()
    const timers = await $fetch(`${config.apiBase}/incident-timers`, {
      headers: {
        'Authorization': `Bearer ${session.secure?.jwtToken}`
      }
    })
    
    console.log('✅ [SSR API] Retrieved timers for SSR:', timers)
    return timers
  } catch (error: any) {
    console.log('❌ [SSR API] Error getting timers:', error.message)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.data?.error || 'Failed to get timers'
    })
  }
})
