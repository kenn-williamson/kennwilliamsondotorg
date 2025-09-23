/**
 * Auth Action Composable - Orchestrates services + session management
 * Handles context-aware operations and bridges between services and session state
 */

import { authService } from '~/services/authService'
import { useBaseService } from '~/composables/useBaseService'
import { useBackendFetch } from '~/composables/useBackendFetch'
import { useAuthFetch } from '~/composables/useAuthFetch'
import type { LoginRequest, RegisterRequest, SlugPreviewResponse } from '#shared/types'

export const useAuthActions = () => {
  // Create dependencies at the top level
  const authFetch = useAuthFetch()
  const { clear, fetch: refreshSession } = useUserSession()
  
  // Use base service for request execution
  const { executeRequest, executeRequestWithSuccess, isLoading, error, hasError } = useBaseService()
  
  // Create service instance
  const authServiceBackend = authService(authFetch)
  
  // Destructure service methods
  const { 
    login: loginService, 
    register: registerService, 
    previewSlug: previewSlugService, 
    revokeAllSessions: revokeAllSessionsService, 
    logout: logoutService 
  } = authServiceBackend

  const login = async (credentials: LoginRequest): Promise<{ success: boolean }> => {
    return executeRequestWithSuccess(
      async () => {
        // Call service
        await loginService(credentials)
        
        // Refresh session to get updated user data
        await refreshSession()
        
        return { success: true }
      },
      'Login successful',
      'login'
    )
  }

  const register = async (userData: RegisterRequest): Promise<{ success: boolean }> => {
    return executeRequestWithSuccess(
      async () => {
        // Call service
        await registerService(userData)
        
        // Refresh session to get updated user data
        await refreshSession()
        
        return { success: true }
      },
      'Registration successful',
      'register'
    )
  }

  const previewSlug = async (displayName: string): Promise<SlugPreviewResponse> => {
    return executeRequest(
      () => previewSlugService(displayName),
      'previewSlug'
    )
  }

  const revokeAllSessions = async (): Promise<{ success: boolean }> => {
    return executeRequestWithSuccess(
      async () => {
        // Call service
        await revokeAllSessionsService()
        
        // Clear local session after revoking all sessions
        await clear()
        
        return { success: true }
      },
      'All sessions revoked successfully',
      'revokeAllSessions'
    )
  }

  const logout = async (): Promise<void> => {
    return executeRequest(
      async () => {
        console.log('🔍 [Auth Actions] Starting logout process...')
        
        try {
          console.log('🔄 [Auth Actions] Calling logout service...')
          await logoutService()
          console.log('✅ [Auth Actions] Logout completed on server')
        } catch (error) {
          console.error('❌ [Auth Actions] Failed to logout on server:', error)
          // Continue with client-side cleanup even if server logout fails
        }

        // Clear authentication state
        await clear()
      },
      'logout'
    )
  }

  return {
    login,
    register,
    previewSlug,
    revokeAllSessions,
    logout,
    isLoading,
    error,
    hasError
  }
}
