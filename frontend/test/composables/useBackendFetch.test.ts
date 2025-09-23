import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'

// Mock all dependencies before importing the composable
vi.mock('~/composables/useJwtManager', () => ({
  useJwtManager: vi.fn()
}))

// Mock $fetch globally
global.$fetch = vi.fn()

// Mock useRuntimeConfig globally
global.useRuntimeConfig = vi.fn()

// Mock console methods to avoid noise in tests
const mockConsoleLog = vi.spyOn(console, 'log').mockImplementation(() => {})

import { useBackendFetch } from '~/composables/useBackendFetch'

describe('useBackendFetch', () => {
  let mockJwtManager: any
  let mockRuntimeConfig: any

  beforeEach(async () => {
    // Reset all mocks
    vi.clearAllMocks()
    mockConsoleLog.mockClear()

    mockJwtManager = {
      getToken: vi.fn()
    }

    mockRuntimeConfig = {
      public: {
        apiBase: 'https://localhost/backend'
      }
    }

    // Configure mocked modules
    const { useJwtManager } = await import('~/composables/useJwtManager')
    vi.mocked(useJwtManager).mockReturnValue(mockJwtManager)

    // Configure global mocks
    global.useRuntimeConfig.mockReturnValue(mockRuntimeConfig)
  })

  afterEach(() => {
    // Clean up any pending timers
    vi.clearAllTimers()
  })

  describe('protected routes', () => {
    it('should add JWT token to protected routes when token is available', async () => {
      const mockToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
      const mockResponse = { success: true }
      
      // Setup mocks
      mockJwtManager.getToken.mockResolvedValue(mockToken)
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      const result = await backendFetch('/protected/auth/me')

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test JWT manager was called
      expect(mockJwtManager.getToken).toHaveBeenCalled()
      
      // Test $fetch was called with Authorization header
      expect($fetch).toHaveBeenCalledWith('/protected/auth/me', {
        headers: {
          Authorization: `Bearer ${mockToken}`
        },
        baseURL: 'https://localhost/backend'
      })
      
      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('✅ [useBackendFetch] Added JWT token to protected request')
      expect(mockConsoleLog).toHaveBeenCalledWith('🔄 [useBackendFetch] GET /protected/auth/me')
      expect(mockConsoleLog).toHaveBeenCalledWith('🔍 [useBackendFetch] Full URL: https://localhost/backend/protected/auth/me')
    })

    it('should handle protected routes when no token is available', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      mockJwtManager.getToken.mockResolvedValue(null)
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      const result = await backendFetch('/protected/auth/me')

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test JWT manager was called
      expect(mockJwtManager.getToken).toHaveBeenCalled()
      
      // Test $fetch was called without Authorization header
      expect($fetch).toHaveBeenCalledWith('/protected/auth/me', {
        baseURL: 'https://localhost/backend'
      })
      
      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('❌ [useBackendFetch] No JWT token available for protected request')
    })

    it('should preserve existing headers when adding JWT token', async () => {
      const mockToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
      const mockResponse = { success: true }
      const existingHeaders = {
        'Content-Type': 'application/json',
        'Custom-Header': 'custom-value'
      }
      
      // Setup mocks
      mockJwtManager.getToken.mockResolvedValue(mockToken)
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      const result = await backendFetch('/protected/auth/me', {
        headers: existingHeaders
      })

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with merged headers
      expect($fetch).toHaveBeenCalledWith('/protected/auth/me', {
        headers: {
          ...existingHeaders,
          Authorization: `Bearer ${mockToken}`
        },
        baseURL: 'https://localhost/backend'
      })
    })

    it('should handle different HTTP methods for protected routes', async () => {
      const mockToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
      const mockResponse = { success: true }
      
      // Setup mocks
      mockJwtManager.getToken.mockResolvedValue(mockToken)
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      
      // Test POST request
      await backendFetch('/protected/incident-timers', {
        method: 'POST',
        body: { notes: 'Test timer' }
      })

      // Test PUT request
      await backendFetch('/protected/incident-timers/123', {
        method: 'PUT',
        body: { notes: 'Updated timer' }
      })

      // Test DELETE request
      await backendFetch('/protected/incident-timers/123', {
        method: 'DELETE'
      })

      // Test all calls included JWT token
      expect($fetch).toHaveBeenCalledTimes(3)
      expect($fetch).toHaveBeenNthCalledWith(1, '/protected/incident-timers', expect.objectContaining({
        method: 'POST',
        headers: { Authorization: `Bearer ${mockToken}` }
      }))
      expect($fetch).toHaveBeenNthCalledWith(2, '/protected/incident-timers/123', expect.objectContaining({
        method: 'PUT',
        headers: { Authorization: `Bearer ${mockToken}` }
      }))
      expect($fetch).toHaveBeenNthCalledWith(3, '/protected/incident-timers/123', expect.objectContaining({
        method: 'DELETE',
        headers: { Authorization: `Bearer ${mockToken}` }
      }))
    })
  })

  describe('public routes', () => {
    it('should not add JWT token to public routes', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      const result = await backendFetch('/public/health')

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test JWT manager was not called
      expect(mockJwtManager.getToken).not.toHaveBeenCalled()
      
      // Test $fetch was called without Authorization header
      expect($fetch).toHaveBeenCalledWith('/public/health', {
        baseURL: 'https://localhost/backend'
      })
      
      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('ℹ️ [useBackendFetch] Public route, no JWT token needed')
    })

    it('should handle public routes with different HTTP methods', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      
      // Test POST request
      await backendFetch('/public/auth/login', {
        method: 'POST',
        body: { email: 'test@example.com', password: 'password' }
      })

      // Test $fetch was called without Authorization header
      expect($fetch).toHaveBeenCalledWith('/public/auth/login', {
        method: 'POST',
        body: { email: 'test@example.com', password: 'password' },
        baseURL: 'https://localhost/backend'
      })
    })
  })

  describe('request configuration', () => {
    it('should use correct base URL from runtime config', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      await backendFetch('/public/health')

      // Test $fetch was called with correct base URL
      expect($fetch).toHaveBeenCalledWith('/public/health', {
        baseURL: 'https://localhost/backend'
      })
    })

    it('should preserve all request options', async () => {
      const mockResponse = { success: true }
      const requestOptions = {
        method: 'POST',
        body: { data: 'test' },
        headers: { 'Content-Type': 'application/json' },
        timeout: 5000
      }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      await backendFetch('/public/test', requestOptions)

      // Test $fetch was called with all options preserved
      expect($fetch).toHaveBeenCalledWith('/public/test', {
        ...requestOptions,
        baseURL: 'https://localhost/backend'
      })
    })

    it('should handle empty options object', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      await backendFetch('/public/health', {})

      // Test $fetch was called with minimal options
      expect($fetch).toHaveBeenCalledWith('/public/health', {
        baseURL: 'https://localhost/backend'
      })
    })
  })

  describe('error handling', () => {
    it('should propagate network errors', async () => {
      const mockError = new Error('Network error')
      
      // Setup mocks
      vi.mocked($fetch).mockRejectedValue(mockError)

      const backendFetch = useBackendFetch()
      
      // Test error is propagated
      await expect(backendFetch('/public/health')).rejects.toThrow('Network error')
    })

    it('should propagate server errors', async () => {
      const mockError = new Error('Server error')
      
      // Setup mocks
      vi.mocked($fetch).mockRejectedValue(mockError)

      const backendFetch = useBackendFetch()
      
      // Test error is propagated
      await expect(backendFetch('/protected/auth/me')).rejects.toThrow('Server error')
    })

    it('should propagate JWT manager errors', async () => {
      // Setup mocks - JWT manager throws error
      mockJwtManager.getToken.mockRejectedValue(new Error('JWT error'))

      const backendFetch = useBackendFetch()
      
      // Test error is propagated (JWT manager errors are not handled gracefully)
      await expect(backendFetch('/protected/auth/me')).rejects.toThrow('JWT error')
      
      // Test $fetch was not called due to error
      expect($fetch).not.toHaveBeenCalled()
    })
  })

  describe('console logging', () => {
    it('should log all request details', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      await backendFetch('/public/health', { method: 'POST' })

      // Test all console logs
      expect(mockConsoleLog).toHaveBeenCalledWith('ℹ️ [useBackendFetch] Public route, no JWT token needed')
      expect(mockConsoleLog).toHaveBeenCalledWith('🔄 [useBackendFetch] POST /public/health')
      expect(mockConsoleLog).toHaveBeenCalledWith('🔍 [useBackendFetch] Full URL: https://localhost/backend/public/health')
    })

    it('should log protected route details', async () => {
      const mockToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
      const mockResponse = { success: true }
      
      // Setup mocks
      mockJwtManager.getToken.mockResolvedValue(mockToken)
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      await backendFetch('/protected/auth/me', { method: 'GET' })

      // Test all console logs
      expect(mockConsoleLog).toHaveBeenCalledWith('✅ [useBackendFetch] Added JWT token to protected request')
      expect(mockConsoleLog).toHaveBeenCalledWith('🔄 [useBackendFetch] GET /protected/auth/me')
      expect(mockConsoleLog).toHaveBeenCalledWith('🔍 [useBackendFetch] Full URL: https://localhost/backend/protected/auth/me')
    })
  })

  describe('interface contract', () => {
    it('should return a function', () => {
      const backendFetch = useBackendFetch()

      // Test interface: returns a function
      expect(typeof backendFetch).toBe('function')
    })

    it('should accept url and options parameters', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      
      // Test function accepts parameters
      await backendFetch('/public/health')
      await backendFetch('/public/health', { method: 'POST' })
      
      // Test both calls worked
      expect($fetch).toHaveBeenCalledTimes(2)
    })
  })

  describe('edge cases', () => {
    it('should handle empty URL', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      await backendFetch('')

      // Test $fetch was called with empty URL
      expect($fetch).toHaveBeenCalledWith('', {
        baseURL: 'https://localhost/backend'
      })
    })

    it('should handle URLs with query parameters', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      await backendFetch('/public/users?search=test&limit=10')

      // Test $fetch was called with query parameters
      expect($fetch).toHaveBeenCalledWith('/public/users?search=test&limit=10', {
        baseURL: 'https://localhost/backend'
      })
    })

    it('should handle very long URLs', async () => {
      const longUrl = '/public/test/' + 'A'.repeat(1000)
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const backendFetch = useBackendFetch()
      await backendFetch(longUrl)

      // Test $fetch was called with long URL
      expect($fetch).toHaveBeenCalledWith(longUrl, {
        baseURL: 'https://localhost/backend'
      })
    })
  })
})
