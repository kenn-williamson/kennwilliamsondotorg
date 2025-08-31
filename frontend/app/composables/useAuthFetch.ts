/**
 * useAuthFetch - HTTP client composable with authentication interceptors
 * 
 * Provides automatic authentication header injection and error handling.
 * Built on top of Nuxt's $fetch with ofetch interceptors.
 */

interface ApiError {
  message: string
  status?: number
}

export function useAuthFetch() {
  const authStore = useAuthStore()

  // Create custom fetch instance with interceptors
  const authFetch = $fetch.create({
    // Request interceptor - automatically inject auth headers
    onRequest({ options }) {
      // Get auth headers from store if user is authenticated
      const authHeaders = authStore.getAuthHeaders()
      
      // Merge with existing headers
      options.headers = {
        ...options.headers,
        ...authHeaders,
      }

      // Add default Content-Type for POST/PUT requests with body
      if (options.body && !(options.headers as any)['Content-Type']) {
        (options.headers as any)['Content-Type'] = 'application/json'
      }

      // Development logging
      if (process.dev) {
        const hasAuth = Object.keys(authHeaders).length > 0
        console.log(`🌐 ${options.method || 'GET'}`, hasAuth ? '🔒' : '🔓')
      }
    },

    // Response interceptor - handle errors and token refresh
    onResponseError({ response, request }) {
      if (process.dev) {
        console.error(`❌ ${response.status} ${request}`)
      }

      // Handle authentication errors
      if (response.status === 401) {
        console.warn('🚫 Unauthorized - clearing auth state')
        
        // Clear auth state (token expired or invalid)
        authStore.clearAuth()
        
        // Redirect to login page if not already there
        const router = useRouter()
        const route = useRoute()
        
        if (route.path !== '/login' && route.path !== '/register') {
          router.push('/login')
        }
      }

      // Transform error into consistent format
      const apiError: ApiError = {
        message: response._data?.error || response._data?.message || 'Request failed',
        status: response.status,
      }

      // Re-throw the error with consistent format
      throw apiError
    },

    // Success response interceptor (optional logging)
    onResponse({ response, request }) {
      if (process.dev) {
        console.log(`✅ ${response.status} ${request}`)
      }
    },
  })

  return authFetch
}