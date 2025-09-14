/**
 * useBackendFetch - Simplified composable for backend API calls with automatic token refresh
 *
 * Architecture:
 * - SSR (page load): GET requests go through Nuxt server API (/api) for immediate hydration
 * - CSR (client loaded): GET requests go directly to Rust backend (/backend)
 * - Mutations: All POST/PUT/DELETE go directly to Rust backend (/backend)
 * - Token Management: JWT manager handles all refresh logic automatically
 */
export function useBackendFetch() {
  const config = useRuntimeConfig()
  const jwtManager = useJwtManager()

  const backendFetch = async <T = any>(url: string, options: any = {}): Promise<T> => {
    const isServer = process.server
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

    // Get fresh token (JWT manager handles expiration check and refresh automatically)
    const token = await jwtManager.getToken()

    // Prepare request options
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

    // Make the request (no retry logic needed - JWT manager handles refresh)
    const response = await $fetch<T>(targetUrl, {
      ...requestOptions,
      baseURL
    })

    return response as T
  }

  return backendFetch
}