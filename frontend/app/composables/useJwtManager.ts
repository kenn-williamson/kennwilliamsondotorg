/**
 * useJwtManager - Client-side JWT token management
 * 
 * Handles getting JWT tokens from the server and managing their lifecycle.
 * Automatically refreshes tokens when they expire.
 */

interface JwtToken {
  token: string
  expiresAt: string
}

export function useJwtManager() {
  const jwtToken = ref<string | null>(null)
  const expiresAt = ref<Date | null>(null)
  const isLoading = ref(false)

  // Check if the current token is expired
  const isExpired = computed(() => {
    if (!expiresAt.value) return true
    return new Date() >= expiresAt.value
  })

  // Check if we have a valid (non-expired) token
  const hasValidToken = computed(() => {
    return jwtToken.value && !isExpired.value
  })

  // Get a fresh JWT token from the server
  const getJwtToken = async (): Promise<string | null> => {
    // If we have a valid token, return it
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
      const response = await $fetch<JwtToken>('/api/auth/jwt')
      
      jwtToken.value = response.token
      expiresAt.value = new Date(response.expiresAt)
      
      console.log('✅ [JWT Manager] Got fresh JWT token, expires at:', expiresAt.value)
      return response.token
    } catch (error: any) {
      console.error('❌ [JWT Manager] Failed to get JWT token:', error.message)
      
      // If it's a 401, the session might be invalid
      if (error.statusCode === 401) {
        const { clear } = useUserSession()
        await clear()
        navigateTo('/login')
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
    hasValidToken,
    getToken,
    clearToken
  }
}
