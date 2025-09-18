/**
 * useBackendFetch - Direct backend API calls with JWT authentication
 *
 * Architecture:
 * - All requests go directly to Rust backend (/backend)
 * - JWT tokens are added automatically for authenticated requests
 * - Used for client-side operations and mutations
 * - For SSR prefetching, use $fetch('/api/*') directly in pages
 */
export function useBackendFetch() {
  const config = useRuntimeConfig()
  const jwtManager = useJwtManager()

  const backendFetch = async <T = any>(url: string, options: any = {}): Promise<T> => {
    // Always go directly to Rust backend
    const baseURL = config.public.apiBase
    const targetUrl = url

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
      console.log(`✅ [useBackendFetch] Added JWT token to request`)
    } else {
      console.log(`❌ [useBackendFetch] No JWT token available for request`)
    }

    console.log(`🔄 [useBackendFetch] ${options.method || 'GET'} ${targetUrl}`)
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