/**
 * useSmartFetch - Smart routing fetcher with automatic route detection
 *
 * Architecture:
 * - Server-side: Uses useRequestFetch for SSR-safe requests with cookie forwarding
 * - Client-side: Uses $fetch for standard client-side requests
 * - JWT tokens are added automatically for protected requests only
 * - URL routing: SSR uses internal Docker network, Client uses nginx proxy
 * - Smart routing: Automatically chooses passthrough vs direct based on route config
 */

import { requiresAuth } from '#shared/config/api-routes'
import type { SmartFetchOptions } from '#shared/types'
import { useJwtManager } from './useJwtManager'

export function useSmartFetch() {
  const config = useRuntimeConfig()
  const jwtManager = useJwtManager()
  
  // Get requestFetch at composable level (in proper Nuxt context)
  const requestFetch = import.meta.server ? useRequestFetch() : null
  
  return async <T = any>(route: string, options: SmartFetchOptions = {}): Promise<T> => {
    const { query, ...requestOptions } = options
    
    // Determine routing type from route prefix
    const routing = route.startsWith('/api/') ? 'passthrough' : 'direct'
    
    // Use correct base URL based on environment and routing type
    let baseURL: string
    if (routing === 'passthrough') {
      // Passthrough routes use /api/ prefix
      baseURL = import.meta.server ? config.apiBase : ''
    } else {
      // Direct backend routes use /backend/ prefix
      baseURL = import.meta.server ? config.apiBase : config.public.apiBase
    }
    
    // Build URL with query parameters and handle prefix removal
    let targetUrl = route

    if (query) {
      const params = new URLSearchParams()
      Object.entries(query).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          params.append(key, String(value))
        }
      })
      if (params.toString()) {
        targetUrl = `${targetUrl}?${params.toString()}`
      }
    }

    // Prepare request options
    const finalRequestOptions = { ...requestOptions }

    // Choose fetcher based on routing type
    const fetcher = import.meta.server && requestFetch ? requestFetch : $fetch

    // Only add JWT token for protected routes
    if (requiresAuth(route)) {
      const token = await jwtManager.getToken()
      
      // Initialize headers if not present
      if (!finalRequestOptions.headers) {
        finalRequestOptions.headers = {}
      }

      finalRequestOptions.headers.Authorization = `Bearer ${token}`
      console.log(`✅ [useSmartFetch] Added JWT token to protected request`)
      console.log(`🔍 [useSmartFetch] Token preview: ${token.substring(0, 20)}...`)
      console.log(`🔍 [useSmartFetch] Authorization header: ${finalRequestOptions.headers.Authorization.substring(0, 30)}...`)
    } else {
      console.log(`ℹ️ [useSmartFetch] Public route, no JWT token needed`)
    }

    // Make the request
    try {
      const result = await fetcher(`${baseURL}${targetUrl}`, finalRequestOptions)
      console.log(`✅ [useSmartFetch] Request successful: ${targetUrl}`)
      return result as T
    } catch (error) {
      console.error(`❌ [useSmartFetch] Request failed: ${targetUrl}`, error)
      throw error
    }
  }
}
