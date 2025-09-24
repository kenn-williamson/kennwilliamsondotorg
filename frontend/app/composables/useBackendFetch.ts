/**
 * useBackendFetch - Direct backend API calls with JWT authentication
 *
 * Architecture:
 * - Server-side: Uses useRequestFetch for SSR-safe requests with cookie forwarding
 * - Client-side: Uses $fetch for standard client-side requests
 * - JWT tokens are added automatically for protected requests only
 * - URL routing: SSR uses internal Docker network, Client uses nginx proxy
 */

import { requiresAuth } from '#shared/config/api-routes'
import { useJwtManager } from './useJwtManager'

export function useBackendFetch() {
  const config = useRuntimeConfig()
  const jwtManager = useJwtManager()
  
  // Get requestFetch at composable level (in proper Nuxt context)
  const requestFetch = process.server ? useRequestFetch() : null

  const backendFetch = async <T = any>(url: string, options: any = {}): Promise<T> => {
    // Use correct base URL based on environment
    // SSR: internal Docker network, Client: nginx proxy
    const baseURL = process.server ? config.apiBase : config.public.apiBase
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
        console.log(`🔍 [useBackendFetch] Token preview: ${token.substring(0, 20)}...`)
        console.log(`🔍 [useBackendFetch] Authorization header: ${requestOptions.headers.Authorization.substring(0, 30)}...`)
      } else {
        console.log(`❌ [useBackendFetch] No JWT token available for protected request`)
      }
    } else {
      console.log(`ℹ️ [useBackendFetch] Public route, no JWT token needed`)
    }

    console.log(`🔄 [useBackendFetch] ${options.method || 'GET'} ${targetUrl}`)
    console.log(`🔍 [useBackendFetch] Full URL: ${baseURL}${targetUrl}`)
    console.log(`🔍 [useBackendFetch] Environment: ${process.server ? 'SSR' : 'Client'}`)
    console.log(`🔍 [useBackendFetch] Request options:`, JSON.stringify(requestOptions, null, 2))

    // Use appropriate fetch method based on environment
    if (process.server && requestFetch) {
      // Server-side: Use useRequestFetch for SSR-safe requests with cookie forwarding
      return await requestFetch(targetUrl, {
        ...requestOptions,
        baseURL
      }) as T
    } else {
      // Client-side: Use $fetch for standard client-side requests
      return await $fetch(targetUrl, {
        ...requestOptions,
        baseURL
      }) as T
    }
  }

  return backendFetch
}