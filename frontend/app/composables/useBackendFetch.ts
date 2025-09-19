/**
 * useBackendFetch - Direct backend API calls with JWT authentication
 *
 * Architecture:
 * - All requests go directly to Rust backend (/backend)
 * - JWT tokens are added automatically for protected requests only
 * - Used for client-side operations and mutations
 * - For SSR prefetching, use $fetch('/api/*') directly in pages
 */

import { requiresAuth } from '#shared/config/api-routes'
import { useJwtManager } from './useJwtManager'

export function useBackendFetch() {
  const config = useRuntimeConfig()
  const jwtManager = useJwtManager()

  const backendFetch = async <T = any>(url: string, options: any = {}): Promise<T> => {
    // All backend calls use the same base URL
    const baseURL = config.public.apiBase
    const targetUrl = url

    // Prepare request options
    const requestOptions = { ...options }

    // Only add JWT token for protected routes
    if (requiresAuth(url)) {
      const token = await jwtManager.getToken()
      
      if (token) {
        // Initialize headers if not present
        if (!requestOptions.headers) {
          requestOptions.headers = {}
        }

        requestOptions.headers.Authorization = `Bearer ${token}`
        console.log(`✅ [useBackendFetch] Added JWT token to protected request`)
      } else {
        console.log(`❌ [useBackendFetch] No JWT token available for protected request`)
      }
    } else {
      console.log(`ℹ️ [useBackendFetch] Public route, no JWT token needed`)
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