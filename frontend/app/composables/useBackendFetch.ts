/**
 * useBackendFetch - Custom composable for direct backend API calls
 * 
 * This composable creates a $fetch instance that automatically includes
 * JWT authentication using the JWT manager for authenticated requests.
 * Used for mutations (POST/PUT/DELETE) that don't need SSR proxying.
 */
export function useBackendFetch() {
  const config = useRuntimeConfig()
  const jwtManager = useJwtManager()
  
  return $fetch.create({
    baseURL: config.public.apiBase,
    async onRequest({ options }) {
      // Get JWT token (will auto-refresh if needed)
      const token = await jwtManager.getToken()
      
      if (token) {
        // Initialize headers if not present
        if (!options.headers) {
          options.headers = new Headers()
        }
        
        // Use the proper Headers API method
        options.headers.append('Authorization', `Bearer ${token}`)
        console.log('✅ [useBackendFetch] Added JWT token to request')
      } else {
        console.log('❌ [useBackendFetch] No JWT token available')
      }
    },
    onResponseError({ response, request }) {
      // Handle common error cases
      if (response.status === 401) {
        // Don't redirect to login if we're already on a login/register page
        // or if this is a login/register API call
        const currentPath = useRoute().path
        const isAuthPage = currentPath === '/login' || currentPath === '/register'
        const isAuthCall = request.toString().includes('/auth/login') || request.toString().includes('/auth/register')
        
        if (!isAuthPage && !isAuthCall) {
          const { clear } = useUserSession()
          clear()
          navigateTo('/login')
        }
      }
    }
  })
}