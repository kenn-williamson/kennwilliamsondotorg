import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'

// Mock $fetch globally
global.$fetch = vi.fn()

// Mock console methods to avoid noise in tests
const mockConsoleLog = vi.spyOn(console, 'log').mockImplementation(() => {})

import { useJwtManager } from '~/composables/useJwtManager'

describe('useJwtManager', () => {
  beforeEach(() => {
    // Reset all mocks
    vi.clearAllMocks()
    mockConsoleLog.mockClear()
  })

  afterEach(() => {
    // Clean up any pending timers
    vi.clearAllTimers()
  })

  describe('getToken success scenarios', () => {
    it('should retrieve token from server successfully', async () => {
      const mockToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
      const mockResponse = { token: mockToken }
      
      // Mock $fetch to return token
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const { getToken } = useJwtManager()
      const result = await getToken()

      // Test result
      expect(result).toBe(mockToken)
      
      // Test $fetch was called with correct endpoint
      expect($fetch).toHaveBeenCalledWith('/api/auth/jwt')
      
      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('🔄 [JWT Manager] Getting token from server')
      expect(mockConsoleLog).toHaveBeenCalledWith('✅ [JWT Manager] Got token from server')
    })

    it('should handle server response with null token', async () => {
      const mockResponse = { token: null }
      
      // Mock $fetch to return null token
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const { getToken } = useJwtManager()
      const result = await getToken()

      // Test result
      expect(result).toBe(null)
      
      // Test $fetch was called
      expect($fetch).toHaveBeenCalledWith('/api/auth/jwt')
      
      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('🔄 [JWT Manager] Getting token from server')
      expect(mockConsoleLog).toHaveBeenCalledWith('✅ [JWT Manager] Got token from server')
    })

    it('should handle server response with undefined token', async () => {
      const mockResponse = { token: undefined }
      
      // Mock $fetch to return undefined token
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const { getToken } = useJwtManager()
      const result = await getToken()

      // Test result
      expect(result).toBe(undefined)
      
      // Test $fetch was called
      expect($fetch).toHaveBeenCalledWith('/api/auth/jwt')
    })
  })

  describe('getToken error scenarios', () => {
    it('should handle network errors and return null', async () => {
      const mockError = new Error('Network error')
      
      // Mock $fetch to throw error
      vi.mocked($fetch).mockRejectedValue(mockError)

      const { getToken } = useJwtManager()
      const result = await getToken()

      // Test result
      expect(result).toBe(null)
      
      // Test $fetch was called
      expect($fetch).toHaveBeenCalledWith('/api/auth/jwt')
      
      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('🔄 [JWT Manager] Getting token from server')
      expect(mockConsoleLog).toHaveBeenCalledWith('❌ [JWT Manager] No token available:', 'Network error')
    })

    it('should handle server errors and return null', async () => {
      const mockError = new Error('Unauthorized')
      
      // Mock $fetch to throw error
      vi.mocked($fetch).mockRejectedValue(mockError)

      const { getToken } = useJwtManager()
      const result = await getToken()

      // Test result
      expect(result).toBe(null)
      
      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('❌ [JWT Manager] No token available:', 'Unauthorized')
    })

    it('should handle non-Error objects gracefully', async () => {
      // Mock $fetch to throw non-Error object
      vi.mocked($fetch).mockRejectedValue('String error')

      const { getToken } = useJwtManager()
      const result = await getToken()

      // Test result
      expect(result).toBe(null)
      
      // Test console logging (non-Error objects are converted to strings)
      expect(mockConsoleLog).toHaveBeenCalledWith('❌ [JWT Manager] No token available:', 'String error')
    })

    it('should handle null/undefined errors gracefully', async () => {
      // Mock $fetch to throw null
      vi.mocked($fetch).mockRejectedValue(null)

      const { getToken } = useJwtManager()
      const result = await getToken()

      // Test result
      expect(result).toBe(null)
      
      // Test console logging (null is converted to string 'null')
      expect(mockConsoleLog).toHaveBeenCalledWith('❌ [JWT Manager] No token available:', 'null')
    })

    it('should handle undefined errors gracefully', async () => {
      // Mock $fetch to throw undefined
      vi.mocked($fetch).mockRejectedValue(undefined)

      const { getToken } = useJwtManager()
      const result = await getToken()

      // Test result
      expect(result).toBe(null)
      
      // Test console logging (undefined is converted to string 'undefined')
      expect(mockConsoleLog).toHaveBeenCalledWith('❌ [JWT Manager] No token available:', 'undefined')
    })
  })

  describe('multiple calls', () => {
    it('should handle multiple concurrent calls', async () => {
      const mockToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
      const mockResponse = { token: mockToken }
      
      // Mock $fetch to return token
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const { getToken } = useJwtManager()
      
      // Make multiple concurrent calls
      const [result1, result2, result3] = await Promise.all([
        getToken(),
        getToken(),
        getToken()
      ])

      // Test all results are the same
      expect(result1).toBe(mockToken)
      expect(result2).toBe(mockToken)
      expect(result3).toBe(mockToken)
      
      // Test $fetch was called multiple times
      expect($fetch).toHaveBeenCalledTimes(3)
    })

    it('should handle mixed success and error calls', async () => {
      const mockToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
      const mockResponse = { token: mockToken }
      
      // Mock $fetch to alternate between success and error
      vi.mocked($fetch)
        .mockResolvedValueOnce(mockResponse)
        .mockRejectedValueOnce(new Error('Network error'))
        .mockResolvedValueOnce(mockResponse)

      const { getToken } = useJwtManager()
      
      // Make calls
      const [result1, result2, result3] = await Promise.all([
        getToken(),
        getToken(),
        getToken()
      ])

      // Test results
      expect(result1).toBe(mockToken)
      expect(result2).toBe(null)
      expect(result3).toBe(mockToken)
      
      // Test $fetch was called multiple times
      expect($fetch).toHaveBeenCalledTimes(3)
    })
  })

  describe('interface contract', () => {
    it('should expose all expected methods', () => {
      const jwtManager = useJwtManager()

      // Test interface: all methods present
      expect(jwtManager).toHaveProperty('getToken')

      // Test interface: methods are functions
      expect(typeof jwtManager.getToken).toBe('function')
    })
  })

  describe('edge cases', () => {
    it('should handle very long tokens', async () => {
      const longToken = 'A'.repeat(1000)
      const mockResponse = { token: longToken }
      
      // Mock $fetch to return long token
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const { getToken } = useJwtManager()
      const result = await getToken()

      // Test result
      expect(result).toBe(longToken)
    })

    it('should handle empty string tokens', async () => {
      const mockResponse = { token: '' }
      
      // Mock $fetch to return empty token
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const { getToken } = useJwtManager()
      const result = await getToken()

      // Test result
      expect(result).toBe('')
    })

    it('should handle malformed server responses', async () => {
      // Mock $fetch to return unexpected response structure
      vi.mocked($fetch).mockResolvedValue({ invalid: 'response' })

      const { getToken } = useJwtManager()
      const result = await getToken()

      // Test result (should be undefined since token property doesn't exist)
      expect(result).toBe(undefined)
    })
  })
})
