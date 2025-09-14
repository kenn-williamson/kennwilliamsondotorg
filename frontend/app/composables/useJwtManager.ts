/**
 * useJwtManager - Simplified JWT token management with pre-request refresh
 *
 * Simple approach: Check token expiration before each API call.
 * If token expires within 1 minute, refresh it automatically.
 */

import { parseJwtToken, isJwtExpired, isJwtExpiringSoon } from '#shared/utils/jwt'

interface JwtToken {
  token: string
  expiresAt: string
}

export function useJwtManager() {
  const jwtToken = ref<string | null>(null)
  const isLoading = ref(false)

  // Check if the current token is expired
  const isExpired = computed(() => {
    if (!jwtToken.value) return true
    return isJwtExpired(jwtToken.value)
  })

  // Check if we have a valid token (not expired)
  const hasValidToken = computed(() => {
    return jwtToken.value && !isExpired.value
  })

  // Check if token expires within specified minutes
  const isExpiringSoon = (minutes: number = 1): boolean => {
    if (!jwtToken.value) return true
    return isJwtExpiringSoon(jwtToken.value, minutes)
  }

  // Refresh token via the refresh endpoint
  const refreshToken = async (): Promise<boolean> => {
    try {
      console.log('🔄 [JWT Manager] Refreshing token via refresh endpoint')
      await $fetch('/api/auth/refresh', { method: 'POST' })

      // Get the new token from session
      const response = await $fetch<JwtToken>('/api/auth/jwt')
      jwtToken.value = response.token

      console.log('✅ [JWT Manager] Token refreshed successfully')
      return true
    } catch (error: any) {
      console.error('❌ [JWT Manager] Token refresh failed:', error.message)

      // If refresh fails with 401, session is invalid
      if (error.statusCode === 401) {
        const { clear } = useUserSession()
        await clear()

        // Don't redirect if we're already on an auth page
        const currentPath = useRoute().path
        const isAuthPage = currentPath === '/login' || currentPath === '/register'

        if (!isAuthPage) {
          await navigateTo('/login')
        }
      }

      return false
    }
  }

  // Get token from session (initial load)
  const fetchTokenFromSession = async (): Promise<boolean> => {
    try {
      console.log('🔄 [JWT Manager] Fetching token from session')
      const response = await $fetch<JwtToken>('/api/auth/jwt')
      jwtToken.value = response.token
      console.log('✅ [JWT Manager] Got token from session')
      return true
    } catch (error: any) {
      console.warn('⚠️ [JWT Manager] No token available in session:', error.message)
      return false
    }
  }

  // Main token getter with pre-request refresh logic
  const getToken = async (): Promise<string | null> => {
    // If we're already loading, wait for it
    if (isLoading.value) {
      console.log('⏳ [JWT Manager] Waiting for token operation...')
      while (isLoading.value) {
        await new Promise(resolve => setTimeout(resolve, 100))
      }
      return jwtToken.value
    }

    isLoading.value = true

    try {
      // If no token, try to get from session first
      if (!jwtToken.value) {
        await fetchTokenFromSession()
      }

      // If still no token, user needs to login
      if (!jwtToken.value) {
        console.log('❌ [JWT Manager] No token available')
        return null
      }

      // Check if token is expired or expires within 1 minute
      if (isExpired.value) {
        console.log('🔄 [JWT Manager] Token is expired, refreshing...')
        const refreshed = await refreshToken()
        if (!refreshed) return null
      } else if (isExpiringSoon(1)) {
        console.log('🔄 [JWT Manager] Token expires within 1 minute, refreshing...')
        const refreshed = await refreshToken()
        if (!refreshed) {
          // If refresh failed but token isn't fully expired yet, use it
          console.warn('⚠️ [JWT Manager] Refresh failed, using existing token')
        }
      } else {
        console.log('✅ [JWT Manager] Using valid cached token')
      }

      return jwtToken.value
    } finally {
      isLoading.value = false
    }
  }

  // Clear the stored token (on logout)
  const clearToken = () => {
    console.log('🧹 [JWT Manager] Clearing JWT token')
    jwtToken.value = null
  }

  return {
    jwtToken: readonly(jwtToken),
    isLoading: readonly(isLoading),
    isExpired,
    hasValidToken,
    getToken,
    clearToken
  }
}
