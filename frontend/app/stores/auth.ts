/**
 * Auth Store - Centralized authentication state management with direct backend calls
 * Supports both SSR and client-side data fetching
 */

import type { User } from '#shared/types'
import { authService } from '~/services/authService'
import { useBackendFetch } from '~/composables/useBackendFetch'

export const useAuthStore = defineStore('auth', () => {
  // State
  const user = ref<User | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Computed
  const isAuthenticated = computed(() => !!user.value)
  const userSlug = computed(() => user.value?.slug || null)
  const hasError = computed(() => !!error.value)

  // Private action handler
  const _handleAction = async <T>(
    action: () => Promise<T>,
    context?: string
  ): Promise<T | undefined> => {
    isLoading.value = true
    error.value = null
    
    try {
      const result = await action()
      return result
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      error.value = errorMessage
      console.error(`[AuthStore] Error${context ? ` in ${context}` : ''}:`, errorMessage)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Get backend URL based on environment
  const getBackendUrl = () => {
    // During SSR, use internal Docker network
    if (process.server) {
      return 'http://backend:8080/backend'
    }
    // On client, use public URL
    return 'https://localhost/backend'
  }

  // Actions
  const fetchCurrentUser = async (): Promise<User | null> => {
    const result = await _handleAction(async () => {
      // Use existing service method with useBackendFetch
      const authServiceInstance = authService(useBackendFetch())
      const response = await authServiceInstance.getCurrentUser()
      user.value = response
      return response
    }, 'fetchCurrentUser')
    
    return result || null
  }

  const getUserSlug = async (): Promise<string | null> => {
    // If we already have user data, return the slug
    if (user.value?.slug) {
      return user.value.slug
    }

    // Otherwise fetch user data
    const userData = await fetchCurrentUser()
    return userData?.slug || null
  }

  const refreshUser = async (): Promise<User | null> => {
    return fetchCurrentUser()
  }

  // Pure state management functions
  const setUser = (userData: User | null) => {
    user.value = userData
  }

  const clearUser = () => {
    user.value = null
    error.value = null
  }

  const clearError = () => {
    error.value = null
  }

  return {
    // State
    user: readonly(user),
    isLoading: readonly(isLoading),
    error: readonly(error),
    
    // Computed
    isAuthenticated,
    userSlug,
    hasError,
    
    // Actions
    fetchCurrentUser,
    getUserSlug,
    refreshUser,
    
    // Pure state management
    setUser,
    clearUser,
    clearError
  }
})
