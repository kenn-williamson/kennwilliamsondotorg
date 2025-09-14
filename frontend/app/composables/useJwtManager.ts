/**
 * useJwtManager - Client-side JWT token management with refresh token support
 *
 * Handles JWT token lifecycle with 1-hour expiration and automatic refresh.
 * Tokens are refreshed proactively and on-demand when expired.
 */

import { parseJwtToken, isJwtExpired, isJwtExpiringSoon } from '#shared/utils/jwt'

interface JwtToken {
  token: string
  expiresAt: string
}

export function useJwtManager() {
  const jwtToken = ref<string | null>(null)
  const expiresAt = ref<Date | null>(null)
  const isLoading = ref(false)
  const refreshTimeout = ref<NodeJS.Timeout | null>(null)

  // Check if the current token is expired or expiring soon
  const isExpired = computed(() => {
    if (!jwtToken.value) return true
    return isJwtExpired(jwtToken.value)
  })

  // Check if token is expiring within 5 minutes
  const isExpiringSoon = computed(() => {
    if (!jwtToken.value) return true
    return isJwtExpiringSoon(jwtToken.value, 5)
  })

  // Check if we have a valid token that's not expiring soon
  const hasValidToken = computed(() => {
    return jwtToken.value && !isExpired.value && !isExpiringSoon.value
  })

  // Parse JWT token to extract expiration using shared utility
  const parseJwtExpiration = (token: string): Date | null => {
    const result = parseJwtToken(token)
    if (!result.isValid) {
      console.error('❌ [JWT Manager] Failed to parse JWT token:', result.error)
      return null
    }
    return result.expiration
  }

  // Schedule proactive token refresh
  const scheduleTokenRefresh = (token: string) => {
    // Clear existing timeout
    if (refreshTimeout.value) {
      clearTimeout(refreshTimeout.value)
    }

    const expirationTime = parseJwtExpiration(token)
    if (!expirationTime) return

    // Schedule refresh 5 minutes before expiration
    const refreshTime = expirationTime.getTime() - 5 * 60 * 1000
    const timeUntilRefresh = refreshTime - Date.now()

    if (timeUntilRefresh > 0) {
      console.log(`⏰ [JWT Manager] Scheduling proactive refresh in ${Math.round(timeUntilRefresh / 1000 / 60)} minutes`)

      refreshTimeout.value = setTimeout(() => {
        console.log('🔄 [JWT Manager] Proactive token refresh triggered')
        $fetch('/api/auth/refresh', { method: 'POST' }).catch((error) => {
          console.warn('⚠️ [JWT Manager] Proactive refresh failed:', error.message)
        })
      }, timeUntilRefresh)
    }
  }

  // Get a fresh JWT token from the server
  const getJwtToken = async (): Promise<string | null> => {
    // If we have a valid token that's not expiring soon, return it
    if (hasValidToken.value) {
      console.log('✅ [JWT Manager] Using cached valid token')
      return jwtToken.value
    }

    // If we're already loading, wait for it
    if (isLoading.value) {
      console.log('⏳ [JWT Manager] Waiting for token refresh...')
      // Wait for loading to complete
      while (isLoading.value) {
        await new Promise(resolve => setTimeout(resolve, 100))
      }
      return jwtToken.value
    }

    isLoading.value = true
    console.log('🔄 [JWT Manager] Fetching fresh JWT token...')

    try {
      // Try to refresh the token first if we have a session
      const { user } = useUserSession()
      if (user.value && (isExpired.value || isExpiringSoon.value)) {
        try {
          await $fetch('/api/auth/refresh', { method: 'POST' })
          console.log('✅ [JWT Manager] Token refreshed via refresh endpoint')
        } catch (refreshError) {
          console.warn('⚠️ [JWT Manager] Refresh failed, falling back to JWT endpoint')
        }
      }

      // Get the current token from the session
      const response = await $fetch<JwtToken>('/api/auth/jwt')

      jwtToken.value = response.token
      expiresAt.value = new Date(response.expiresAt)

      // Schedule proactive refresh for this token
      scheduleTokenRefresh(response.token)

      console.log('✅ [JWT Manager] Got fresh JWT token, expires at:', expiresAt.value)
      return response.token
    } catch (error: any) {
      console.error('❌ [JWT Manager] Failed to get JWT token:', error.message)

      // If it's a 401, the session might be invalid
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

      return null
    } finally {
      isLoading.value = false
    }
  }

  // Clear the stored token (on logout)
  const clearToken = () => {
    console.log('🧹 [JWT Manager] Clearing JWT token')
    jwtToken.value = null
    expiresAt.value = null

    // Clear any scheduled refresh
    if (refreshTimeout.value) {
      clearTimeout(refreshTimeout.value)
      refreshTimeout.value = null
    }
  }

  // Get token with automatic refresh
  const getToken = async (): Promise<string | null> => {
    return await getJwtToken()
  }

  return {
    jwtToken: readonly(jwtToken),
    expiresAt: readonly(expiresAt),
    isLoading: readonly(isLoading),
    isExpired,
    isExpiringSoon,
    hasValidToken,
    getToken,
    clearToken
  }
}
