/**
 * useRequestFetchWithAuth - Simple wrapper around useRequestFetch with JWT injection
 * 
 * Uses useRequestFetch for SSR-safe requests and adds JWT tokens for protected routes
 */

import { requiresAuth } from '#shared/config/api-routes'
import { clientJwtManager } from '#shared/utils/client-jwt-manager'

export function useRequestFetchWithAuth() {
  const config = useRuntimeConfig()
  const requestFetch = useRequestFetch()

  const fetchWithAuth = async <T = any>(url: string, options: any = {}): Promise<T> => {
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
        console.log(`✅ [RequestFetchWithAuth] Added JWT token to protected request`)
      } else {
        console.log(`❌ [RequestFetchWithAuth] No JWT token available for protected request`)
      }
    } else {
      console.log(`ℹ️ [RequestFetchWithAuth] Public route, no JWT token needed`)
    }

    console.log(`🔄 [RequestFetchWithAuth] ${options.method || 'GET'} ${targetUrl}`)
    console.log(`🔍 [RequestFetchWithAuth] Full URL: ${baseURL}${targetUrl}`)

    // Make the request using useRequestFetch for SSR-safe requests
    const response = await requestFetch<T>(targetUrl, {
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

  /**
   * Get JWT token using the appropriate manager for the current environment
   */
  const getJwtToken = async (): Promise<string | null> => {
    if (process.server) {
      // Server-side: Use useRequestFetch to get JWT from /api/auth/jwt
      // This automatically forwards cookies during SSR
      try {
        const response = await requestFetch('/api/auth/jwt')
        return response.token
      } catch (error) {
        console.log('🔍 [RequestFetchWithAuth] Server-side JWT fetch failed:', error)
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

  return fetchWithAuth
}
