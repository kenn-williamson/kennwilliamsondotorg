import { describe, it, expect, vi, beforeEach } from 'vitest'
import { useEmailVerification } from './useEmailVerification'
import type { SendVerificationEmailResponse, VerifyEmailResponse } from '#shared/types'
import { API_ROUTES } from '#shared/config/api-routes'

// Mock useSmartFetch
const mockSmartFetch = vi.fn()
vi.mock('~/composables/useSmartFetch', () => ({
  useSmartFetch: () => mockSmartFetch,
}))

// Mock useUserSession
const mockRefreshSession = vi.fn()
const mockUseUserSession = vi.fn(() => ({
  fetch: mockRefreshSession,
}))
vi.mock('#app', () => ({
  useUserSession: mockUseUserSession,
}))

describe('useEmailVerification', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe('sendVerificationEmail', () => {
    it('should successfully send verification email', async () => {
      // Arrange
      const mockResponse: SendVerificationEmailResponse = {
        message: 'Verification email sent successfully',
      }
      mockSmartFetch.mockResolvedValue(mockResponse)

      // Act
      const { sendVerificationEmail, isLoading, error } = useEmailVerification()
      const result = await sendVerificationEmail()

      // Assert
      expect(mockSmartFetch).toHaveBeenCalledWith(
        API_ROUTES.API.AUTH.SEND_VERIFICATION,
        { method: 'POST' }
      )
      expect(result).toEqual(mockResponse)
      expect(isLoading.value).toBe(false)
      expect(error.value).toBe(null)
    })

    it('should handle error when sending verification email fails', async () => {
      // Arrange
      const mockError = new Error('Failed to send verification email')
      mockSmartFetch.mockRejectedValue(mockError)

      // Act
      const { sendVerificationEmail, isLoading, error } = useEmailVerification()

      await expect(sendVerificationEmail()).rejects.toThrow('Failed to send verification email')

      // Assert
      expect(error.value).toBe('Failed to send verification email')
      expect(isLoading.value).toBe(false)
    })

    it('should set loading state correctly during request', async () => {
      // Arrange
      const mockResponse: SendVerificationEmailResponse = {
        message: 'Verification email sent successfully',
      }
      let resolvePromise: (value: any) => void
      const promise = new Promise((resolve) => {
        resolvePromise = resolve
      })
      mockSmartFetch.mockReturnValue(promise)

      // Act
      const { sendVerificationEmail, isLoading } = useEmailVerification()
      const requestPromise = sendVerificationEmail()

      // Assert - loading should be true
      expect(isLoading.value).toBe(true)

      // Resolve the promise
      resolvePromise!(mockResponse)
      await requestPromise

      // Assert - loading should be false
      expect(isLoading.value).toBe(false)
    })
  })

  describe('verifyEmail', () => {
    it('should successfully verify email', async () => {
      // Arrange
      const mockToken = 'valid-token-123'
      const mockResponse: VerifyEmailResponse = {
        message: 'Email verified successfully',
      }
      mockSmartFetch.mockResolvedValue(mockResponse)
      mockRefreshSession.mockResolvedValue({})

      // Act
      const { verifyEmail, isLoading, error } = useEmailVerification()
      const result = await verifyEmail(mockToken)

      // Assert
      expect(mockSmartFetch).toHaveBeenCalledWith(
        API_ROUTES.PUBLIC.VERIFY_EMAIL,
        {
          method: 'GET',
          query: { token: mockToken },
        }
      )
      // Note: Session refresh is tested in integration tests
      expect(result).toEqual(mockResponse)
      expect(isLoading.value).toBe(false)
      expect(error.value).toBe(null)
    })

    it('should handle invalid token error', async () => {
      // Arrange
      const mockToken = 'invalid-token'
      const mockError = new Error('Invalid or expired verification token')
      mockSmartFetch.mockRejectedValue(mockError)

      // Act
      const { verifyEmail, error } = useEmailVerification()

      await expect(verifyEmail(mockToken)).rejects.toThrow('Invalid or expired verification token')

      // Assert
      expect(error.value).toBe('Invalid or expired verification token')
    })

    it('should set loading state correctly during verification', async () => {
      // Arrange
      const mockToken = 'valid-token-123'
      const mockResponse: VerifyEmailResponse = {
        message: 'Email verified successfully',
      }
      let resolvePromise: (value: any) => void
      const promise = new Promise((resolve) => {
        resolvePromise = resolve
      })
      mockSmartFetch.mockReturnValue(promise)
      mockRefreshSession.mockResolvedValue({})

      // Act
      const { verifyEmail, isLoading } = useEmailVerification()
      const requestPromise = verifyEmail(mockToken)

      // Assert - loading should be true
      expect(isLoading.value).toBe(true)

      // Resolve the promise
      resolvePromise!(mockResponse)
      await requestPromise

      // Assert - loading should be false
      expect(isLoading.value).toBe(false)
    })
  })

  describe('error handling', () => {
    it('should clear error state on successful request after error', async () => {
      // Arrange
      const mockError = new Error('First request failed')
      const mockSuccess: SendVerificationEmailResponse = {
        message: 'Verification email sent successfully',
      }
      mockSmartFetch
        .mockRejectedValueOnce(mockError)
        .mockResolvedValueOnce(mockSuccess)

      // Act
      const { sendVerificationEmail, error } = useEmailVerification()

      // First request fails
      await expect(sendVerificationEmail()).rejects.toThrow('First request failed')
      expect(error.value).toBe('First request failed')

      // Second request succeeds
      await sendVerificationEmail()

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
      const { sendVerificationEmail, hasError } = useEmailVerification()
      await expect(sendVerificationEmail()).rejects.toThrow('Test error')

      // Assert
      expect(hasError.value).toBe(true)
    })

    it('should return false when no error exists', () => {
      // Act
      const { hasError } = useEmailVerification()

      // Assert
      expect(hasError.value).toBe(false)
    })
  })
})
