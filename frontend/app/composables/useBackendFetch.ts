/**
 * useBackendFetch - Smart composable for backend API calls with SSR/CSR routing
 *
 * Architecture:
 * - SSR (page load): GET requests go through Nuxt server API (/api) for immediate hydration
 * - CSR (client loaded): GET requests go directly to Rust backend (/backend)
 * - Mutations: All POST/PUT/DELETE go directly to Rust backend (/backend)
 * - Auth: Auth operations go through Nuxt server API (/api)
 */
export function useBackendFetch() {
  const config = useRuntimeConfig()
  const jwtManager = useJwtManager()

  const backendFetch = async <T = any>(url: string, options: any = {}): Promise<T> => {
    const isServer = process.server
    const isClient = process.client
    const method = options.method?.toUpperCase() || 'GET'
    
    // Determine the correct base URL and routing strategy
    let baseURL: string
    let targetUrl: string
    
    if (isServer) {
      // SSR: Use Nuxt server API for GET requests, direct backend for mutations
      if (method === 'GET') {
        baseURL = '' // Use relative URL for internal Nuxt API
        targetUrl = `/api${url}` // Route through Nuxt server API
      } else {
        baseURL = config.public.apiBase // Direct to Rust backend
        targetUrl = url
      }
    } else {
      // CSR: Always go directly to Rust backend
      baseURL = config.public.apiBase
      targetUrl = url
    }

    // Helper function to make the actual request
    const makeRequest = async (token: string | null): Promise<T> => {
      const requestOptions = { ...options }

      if (token) {
        // Initialize headers if not present
        if (!requestOptions.headers) {
          requestOptions.headers = {}
        }

        requestOptions.headers.Authorization = `Bearer ${token}`
        console.log(`✅ [useBackendFetch] Added JWT token to ${isServer ? 'SSR' : 'CSR'} request`)
      } else {
        console.log(`❌ [useBackendFetch] No JWT token available for ${isServer ? 'SSR' : 'CSR'} request`)
      }

      console.log(`🔄 [useBackendFetch] ${method} ${targetUrl} (${isServer ? 'SSR' : 'CSR'})`)
      console.log(`🔍 [useBackendFetch] Full URL: ${baseURL}${targetUrl}`)
      
      const response = await $fetch<T>(targetUrl, {
        ...requestOptions,
        baseURL
      })
      
      return response as T
    }

    try {
      // Get JWT token and make initial request
      const token = await jwtManager.getToken()
      return await makeRequest(token)
    } catch (error: any) {
      // If we get a 401 error, try to refresh the token
      if (error.status === 401) {
        const { user } = useUserSession()

        // Only attempt refresh if we have a user session
        if (user.value) {
          try {
            console.log('🔄 [useBackendFetch] 401 error, attempting token refresh')

            // Call the refresh endpoint
            await $fetch('/api/auth/refresh', { method: 'POST' })

            console.log('✅ [useBackendFetch] Token refreshed, retrying original request')

            // Get the new token and retry the original request
            const newToken = await jwtManager.getToken()
            return await makeRequest(newToken)
          } catch (refreshError: any) {
            console.error('❌ [useBackendFetch] Token refresh failed:', refreshError.message)

            // Refresh failed, clear session and redirect to login
            // Don't redirect if we're already on an auth page
            const currentPath = useRoute().path
            const isAuthPage = currentPath === '/login' || currentPath === '/register'

            if (!isAuthPage) {
              await navigateTo('/login')
            }

            throw refreshError
          }
        } else {
          // No user session, redirect to login unless we're on an auth page
          const currentPath = useRoute().path
          const isAuthPage = currentPath === '/login' || currentPath === '/register'

          if (!isAuthPage) {
            await navigateTo('/login')
          }
        }
      }

      throw error
    }
  }

  return backendFetch
}