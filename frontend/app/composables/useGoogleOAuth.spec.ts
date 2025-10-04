import { describe, it, expect, vi, beforeEach } from 'vitest'
import { useGoogleOAuth } from './useGoogleOAuth'
import type { GoogleOAuthUrlResponse } from '#shared/types'
import { API_ROUTES } from '#shared/config/api-routes'

// Mock useSmartFetch
const mockSmartFetch = vi.fn()
vi.mock('~/composables/useSmartFetch', () => ({
  useSmartFetch: () => mockSmartFetch,
}))

// Mock useRouter
const mockPush = vi.fn()
const mockRouter = { push: mockPush }
const mockUseRouter = vi.fn(() => mockRouter)
vi.mock('#app/composables/router', () => ({
  useRouter: mockUseRouter,
}))

// Also mock for direct imports
global.useRouter = mockUseRouter as any

// Mock useUserSession
const mockRefreshSession = vi.fn()
const mockUseUserSession = vi.fn(() => ({
  fetch: mockRefreshSession,
}))
vi.mock('#app', () => ({
  useUserSession: mockUseUserSession,
}))

// Mock window.location
delete (window as any).location
window.location = { href: '' } as any

describe('useGoogleOAuth', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    window.location.href = ''
  })

  describe('initiateOAuth', () => {
    it('should fetch OAuth URL and redirect to Google', async () => {
      // Arrange
      const mockOAuthUrl = 'https://accounts.google.com/o/oauth2/v2/auth?client_id=...'
      const mockResponse: GoogleOAuthUrlResponse = {
        url: mockOAuthUrl,
      }
      mockSmartFetch.mockResolvedValue(mockResponse)

      // Act
      const { initiateOAuth, isLoading, error } = useGoogleOAuth()
      await initiateOAuth()

      // Assert
      expect(mockSmartFetch).toHaveBeenCalledWith(
        API_ROUTES.API.AUTH.GOOGLE_URL,
        { method: 'GET' }
      )
      expect(window.location.href).toBe(mockOAuthUrl)
      expect(isLoading.value).toBe(false)
      expect(error.value).toBe(null)
    })

    it('should handle error when fetching OAuth URL fails', async () => {
      // Arrange
      const mockError = new Error('Failed to get OAuth URL')
      mockSmartFetch.mockRejectedValue(mockError)

      // Act
      const { initiateOAuth, error } = useGoogleOAuth()

      await expect(initiateOAuth()).rejects.toThrow('Failed to get OAuth URL')

      // Assert
      expect(error.value).toBe('Failed to get OAuth URL')
      expect(window.location.href).toBe('')
    })

    it('should set loading state correctly during OAuth initiation', async () => {
      // Arrange
      const mockResponse: GoogleOAuthUrlResponse = {
        url: 'https://accounts.google.com/o/oauth2/v2/auth',
      }
      let resolvePromise: (value: any) => void
      const promise = new Promise((resolve) => {
        resolvePromise = resolve
      })
      mockSmartFetch.mockReturnValue(promise)

      // Act
      const { initiateOAuth, isLoading } = useGoogleOAuth()
      const requestPromise = initiateOAuth()

      // Assert - loading should be true
      expect(isLoading.value).toBe(true)

      // Resolve the promise
      resolvePromise!(mockResponse)
      await requestPromise

      // Assert - loading should be false
      expect(isLoading.value).toBe(false)
    })
  })

  describe('handleOAuthCallback', () => {
    it('should call API with code and state parameters', async () => {
      // Arrange
      const mockCode = 'test-auth-code-123'
      const mockState = 'test-state-token-456'
      mockSmartFetch.mockResolvedValue({ success: true })

      // Act
      const { handleOAuthCallback } = useGoogleOAuth()
      await handleOAuthCallback(mockCode, mockState)

      // Assert
      expect(mockSmartFetch).toHaveBeenCalledWith(
        API_ROUTES.API.AUTH.GOOGLE_CALLBACK,
        {
          method: 'POST',
          body: { code: mockCode, state: mockState }
        }
      )
    })

    it('should handle error when callback fails', async () => {
      // Arrange
      const mockError = new Error('Invalid state parameter')
      mockSmartFetch.mockRejectedValue(mockError)

      // Act
      const { handleOAuthCallback, error } = useGoogleOAuth()

      await expect(handleOAuthCallback('code', 'state')).rejects.toThrow('Invalid state parameter')

      // Assert
      expect(error.value).toBe('Invalid state parameter')
    })

    it('should set loading state correctly during callback', async () => {
      // Arrange
      let resolvePromise: (value: any) => void
      const promise = new Promise((resolve) => {
        resolvePromise = resolve
      })
      mockSmartFetch.mockReturnValue(promise)

      // Act
      const { handleOAuthCallback, isLoading } = useGoogleOAuth()
      const requestPromise = handleOAuthCallback('code', 'state')

      // Assert - loading should be true
      expect(isLoading.value).toBe(true)

      // Resolve the promise
      resolvePromise!({ success: true })
      await requestPromise

      // Assert - loading should be false
      expect(isLoading.value).toBe(false)
    })

    it('should complete successfully with valid code and state', async () => {
      // Arrange
      const mockResponse = {
        success: true,
        user: {
          id: 'user-123',
          email: 'test@example.com',
          display_name: 'Test User'
        }
      }
      mockSmartFetch.mockResolvedValue(mockResponse)

      // Act
      const { handleOAuthCallback, error } = useGoogleOAuth()
      await handleOAuthCallback('valid-code', 'valid-state')

      // Assert
      expect(error.value).toBe(null)
    })
  })

  describe('error handling', () => {
    it('should clear error state on successful request after error', async () => {
      // Arrange
      const mockError = new Error('First request failed')
      const mockSuccess: GoogleOAuthUrlResponse = {
        url: 'https://accounts.google.com/o/oauth2/v2/auth',
      }
      mockSmartFetch
        .mockRejectedValueOnce(mockError)
        .mockResolvedValueOnce(mockSuccess)

      // Act
      const { initiateOAuth, error } = useGoogleOAuth()

      // First request fails
      await expect(initiateOAuth()).rejects.toThrow('First request failed')
      expect(error.value).toBe('First request failed')

      // Second request succeeds
      await initiateOAuth()

      // Assert - error should be cleared
      expect(error.value).toBe(null)
    })
  })

  describe('hasError computed', () => {
    it('should return true when error exists', async () => {
      // Arrange
      const mockError = new Error('Test error')
      mockSmartFetch.mockRejectedValue(mockError)

      // Act
      const { initiateOAuth, hasError } = useGoogleOAuth()
      await expect(initiateOAuth()).rejects.toThrow('Test error')

      // Assert
      expect(hasError.value).toBe(true)
    })

    it('should return false when no error exists', () => {
      // Act
      const { hasError } = useGoogleOAuth()

      // Assert
      expect(hasError.value).toBe(false)
    })
  })
})
