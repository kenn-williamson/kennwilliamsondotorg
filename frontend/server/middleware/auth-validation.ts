/**
 * Server-Only Authentication Validation Middleware
 * 
 * Validates that sessions contain required JWT and refresh tokens.
 * Clears invalid sessions and redirects to login when needed.
 * Runs on every request (server-side only).
 */

export default defineEventHandler(async (event) => {
  // Log that middleware is running
  console.log('🔍 [Auth Validation Middleware] Running for URL:', event.node.req.url)
  
  // Only run on page requests (not API routes or assets)
  if (event.node.req.url?.startsWith('/api/') || 
      event.node.req.url?.startsWith('/_nuxt/') ||
      event.node.req.url?.startsWith('/__nuxt_error')) {
    console.log('🔍 [Auth Validation Middleware] Skipping API/asset route')
    return
  }

  try {
    // Get current session
    const session = await getUserSession(event)
    
    // If no session exists, allow the request to continue
    if (!session) {
      return
    }

    console.log('🔍 [Auth Validation] Session exists, validating completeness...')
    console.log('🔍 [Auth Validation] Session user:', !!session.user)
    console.log('🔍 [Auth Validation] Session secure:', !!session.secure)
    console.log('🔍 [Auth Validation] JWT token:', !!session.secure?.jwtToken)
    console.log('🔍 [Auth Validation] Refresh token:', !!session.secure?.refreshToken)

    // Check if session has required authentication data
    const hasUser = !!session.user
    const hasJwtToken = !!session.secure?.jwtToken
    const hasRefreshToken = !!session.secure?.refreshToken

    // If session exists but lacks required tokens, clear it
    if (hasUser && (!hasJwtToken || !hasRefreshToken)) {
      console.log('❌ [Auth Validation] Invalid session detected - missing required tokens')
      console.log('🔄 [Auth Validation] Clearing invalid session...')
      
      await clearUserSession(event)
      
      // Get the current URL for redirect
      const currentPath = event.node.req.url || '/'
      
      // Don't redirect if already on login page to avoid loops
      if (!currentPath.startsWith('/login')) {
        console.log('🔄 [Auth Validation] Redirecting to login with redirect=', currentPath)
        await sendRedirect(event, `/login?redirect=${encodeURIComponent(currentPath)}`)
      }
      
      return
    }

    // If session has user but no secure data at all, clear it
    if (hasUser && !session.secure) {
      console.log('❌ [Auth Validation] Invalid session detected - no secure data')
      console.log('🔄 [Auth Validation] Clearing invalid session...')
      
      await clearUserSession(event)
      
      const currentPath = event.node.req.url || '/'
      if (!currentPath.startsWith('/login')) {
        console.log('🔄 [Auth Validation] Redirecting to login with redirect=', currentPath)
        await sendRedirect(event, `/login?redirect=${encodeURIComponent(currentPath)}`)
      }
      
      return
    }

    // Session is valid - allow request to continue
    console.log('✅ [Auth Validation] Session validation passed')
    
  } catch (error) {
    console.error('❌ [Auth Validation] Error during session validation:', error)
    // On error, clear session to be safe
    try {
      await clearUserSession(event)
    } catch (clearError) {
      console.error('❌ [Auth Validation] Error clearing session:', clearError)
    }
  }
})
