/**
 * useSmartFetch - Smart fetch composable for SSR-safe requests
 * 
 * Architecture:
 * - Server-side: Uses useRequestFetch for SSR-safe requests that forward cookies
 * - Client-side: Uses clientJwtManager for cached token management
 * - Smart routing: Auth routes (/api/*), Protected routes (/backend/*), Public routes (/backend/*)
 * - Replaces: useAuthFetch, useBackendFetch, useJwtManager
 */

import { requiresAuth } from '#shared/config/api-routes'
import { clientJwtManager } from '#shared/utils/client-jwt-manager'

export function useSmartFetch() {
  const config = useRuntimeConfig()

  /**
   * Get JWT token using the appropriate manager for the current environment
   */
  const getJwtToken = async (): Promise<string | null> => {
    if (process.server) {
      // Server-side: Use useRequestFetch to get JWT from /api/auth/jwt
      // This automatically forwards cookies during SSR
      try {
        const requestFetch = useRequestFetch()
        const response = await requestFetch('/api/auth/jwt')
        return response.token
      } catch (error) {
        console.log('🔍 [SmartFetch] Server-side JWT fetch failed:', error)
        return null
      }
    } else {
      // Client-side: Use cached token (no API calls during hydration)
      return await clientJwtManager.getToken()
    }
  }

  /**
   * Determine the correct base URL for the request
   */
  const getBaseURL = (url: string): string => {
    // Auth routes go to Nuxt server API
    if (url.startsWith('/api/')) {
      return '' // Relative URL for internal Nuxt API
    }
    
    // All other routes go to Rust backend
    // SSR: internal Docker network, Client: nginx proxy
    return process.server ? config.apiBase : config.public.apiBase
  }

  /**
   * Smart fetch function that handles authentication and routing automatically
   */
  const smartFetch = async <T = any>(url: string, options: any = {}): Promise<T> => {
    // Prepare request options
    const requestOptions = { ...options }

    // Determine base URL
    const baseURL = getBaseURL(url)
    const targetUrl = url

    // Handle authentication for protected routes
    if (requiresAuth(url)) {
      const token = await getJwtToken()
      
      if (token) {
        // Initialize headers if not present
        if (!requestOptions.headers) {
          requestOptions.headers = {}
        }

        requestOptions.headers.Authorization = `Bearer ${token}`
        console.log(`✅ [SmartFetch] Added JWT token to protected request`)
        console.log(`🔍 [SmartFetch] Token preview: ${token.substring(0, 20)}...`)
      } else {
        console.log(`❌ [SmartFetch] No JWT token available for protected request`)
      }
    } else {
      console.log(`ℹ️ [SmartFetch] Public route, no JWT token needed`)
    }

    console.log(`🔄 [SmartFetch] ${options.method || 'GET'} ${targetUrl}`)
    console.log(`🔍 [SmartFetch] Full URL: ${baseURL}${targetUrl}`)
    console.log(`🔍 [SmartFetch] Environment: ${process.server ? 'SSR' : 'Client'}`)

    // Make the request
    const response = await $fetch<T>(targetUrl, {
      ...requestOptions,
      baseURL
    })

    // Cache JWT token on successful response (client-side only)
    if (!process.server && requiresAuth(url) && requestOptions.headers?.Authorization) {
      const token = requestOptions.headers.Authorization.replace('Bearer ', '')
      clientJwtManager.setToken(token)
    }

    return response as T
  }

  return smartFetch
}
