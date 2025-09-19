/**
 * useAuthFetch - Composable for authentication API calls
 *
 * All auth operations (login, register, refresh, logout) go through Nuxt server API (/api)
 * This ensures proper session management and cookie handling.
 */
export function useAuthFetch() {
  const authFetch = async <T = any>(url: string, options: any = {}): Promise<T> => {
    // Auth operations always go through Nuxt server API
    // URL should already be a complete API route (e.g., /api/auth/login)
    
    console.log(`🔐 [useAuthFetch] ${options.method || 'GET'} ${url}`)
    
    const response = await $fetch<T>(url, {
      ...options,
      baseURL: '' // Use relative URL for internal Nuxt API
    })
    
    return response as T
  }

  return authFetch
}
