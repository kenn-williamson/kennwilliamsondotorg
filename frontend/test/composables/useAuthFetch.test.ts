import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'

// Mock $fetch globally
global.$fetch = vi.fn()

// Mock console methods to avoid noise in tests
const mockConsoleLog = vi.spyOn(console, 'log').mockImplementation(() => {})

import { useAuthFetch } from '~/composables/useAuthFetch'

describe('useAuthFetch', () => {
  beforeEach(() => {
    // Reset all mocks
    vi.clearAllMocks()
    mockConsoleLog.mockClear()
  })

  afterEach(() => {
    // Clean up any pending timers
    vi.clearAllTimers()
  })

  describe('successful requests', () => {
    it('should make GET request successfully', async () => {
      const mockResponse = { user: { id: '1', email: 'test@example.com' } }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      const result = await authFetch('/api/auth/me')

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with correct parameters
      expect($fetch).toHaveBeenCalledWith('/api/auth/me', {
        baseURL: ''
      })
      
      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('🔐 [useAuthFetch] GET /api/auth/me')
    })

    it('should make POST request successfully', async () => {
      const mockResponse = { success: true, token: 'jwt-token' }
      const requestBody = { email: 'test@example.com', password: 'password123' }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      const result = await authFetch('/api/auth/login', {
        method: 'POST',
        body: requestBody
      })

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with correct parameters
      expect($fetch).toHaveBeenCalledWith('/api/auth/login', {
        method: 'POST',
        body: requestBody,
        baseURL: ''
      })
      
      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('🔐 [useAuthFetch] POST /api/auth/login')
    })

    it('should make PUT request successfully', async () => {
      const mockResponse = { success: true }
      const requestBody = { display_name: 'New Name' }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      const result = await authFetch('/api/auth/profile', {
        method: 'PUT',
        body: requestBody
      })

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with correct parameters
      expect($fetch).toHaveBeenCalledWith('/api/auth/profile', {
        method: 'PUT',
        body: requestBody,
        baseURL: ''
      })
      
      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('🔐 [useAuthFetch] PUT /api/auth/profile')
    })

    it('should make DELETE request successfully', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      const result = await authFetch('/api/auth/logout', {
        method: 'DELETE'
      })

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with correct parameters
      expect($fetch).toHaveBeenCalledWith('/api/auth/logout', {
        method: 'DELETE',
        baseURL: ''
      })
      
      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('🔐 [useAuthFetch] DELETE /api/auth/logout')
    })
  })

  describe('request configuration', () => {
    it('should preserve all request options', async () => {
      const mockResponse = { success: true }
      const requestOptions = {
        method: 'POST',
        body: { data: 'test' },
        headers: { 'Content-Type': 'application/json' },
        timeout: 5000,
        credentials: 'include'
      }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      const result = await authFetch('/api/auth/login', requestOptions)

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with all options preserved
      expect($fetch).toHaveBeenCalledWith('/api/auth/login', {
        ...requestOptions,
        baseURL: ''
      })
    })

    it('should handle empty options object', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      const result = await authFetch('/api/auth/me', {})

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with minimal options
      expect($fetch).toHaveBeenCalledWith('/api/auth/me', {
        baseURL: ''
      })
    })

    it('should handle undefined options', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      const result = await authFetch('/api/auth/me')

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with default options
      expect($fetch).toHaveBeenCalledWith('/api/auth/me', {
        baseURL: ''
      })
    })

    it('should always use empty baseURL for internal API routes', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      await authFetch('/api/auth/me')

      // Test $fetch was called with empty baseURL
      expect($fetch).toHaveBeenCalledWith('/api/auth/me', {
        baseURL: ''
      })
    })
  })

  describe('error handling', () => {
    it('should propagate network errors', async () => {
      const mockError = new Error('Network error')
      
      // Mock $fetch to throw error
      vi.mocked($fetch).mockRejectedValue(mockError)

      const authFetch = useAuthFetch()
      
      // Test error is propagated
      await expect(authFetch('/api/auth/me')).rejects.toThrow('Network error')
      
      // Test console logging still occurred
      expect(mockConsoleLog).toHaveBeenCalledWith('🔐 [useAuthFetch] GET /api/auth/me')
    })

    it('should propagate server errors', async () => {
      const mockError = new Error('Unauthorized')
      
      // Mock $fetch to throw error
      vi.mocked($fetch).mockRejectedValue(mockError)

      const authFetch = useAuthFetch()
      
      // Test error is propagated
      await expect(authFetch('/api/auth/login', {
        method: 'POST',
        body: { email: 'test@example.com', password: 'wrong' }
      })).rejects.toThrow('Unauthorized')
      
      // Test console logging still occurred
      expect(mockConsoleLog).toHaveBeenCalledWith('🔐 [useAuthFetch] POST /api/auth/login')
    })

    it('should propagate validation errors', async () => {
      const mockError = new Error('Validation failed')
      
      // Mock $fetch to throw error
      vi.mocked($fetch).mockRejectedValue(mockError)

      const authFetch = useAuthFetch()
      
      // Test error is propagated
      await expect(authFetch('/api/auth/register', {
        method: 'POST',
        body: { email: 'invalid-email' }
      })).rejects.toThrow('Validation failed')
    })
  })

  describe('console logging', () => {
    it('should log all request details', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      await authFetch('/api/auth/me', { method: 'GET' })

      // Test console logging
      expect(mockConsoleLog).toHaveBeenCalledWith('🔐 [useAuthFetch] GET /api/auth/me')
    })

    it('should log different HTTP methods', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      
      // Test different methods
      await authFetch('/api/auth/login', { method: 'POST' })
      await authFetch('/api/auth/profile', { method: 'PUT' })
      await authFetch('/api/auth/logout', { method: 'DELETE' })

      // Test all console logs
      expect(mockConsoleLog).toHaveBeenCalledWith('🔐 [useAuthFetch] POST /api/auth/login')
      expect(mockConsoleLog).toHaveBeenCalledWith('🔐 [useAuthFetch] PUT /api/auth/profile')
      expect(mockConsoleLog).toHaveBeenCalledWith('🔐 [useAuthFetch] DELETE /api/auth/logout')
    })

    it('should log default method as GET', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      await authFetch('/api/auth/me')

      // Test console logging shows GET as default
      expect(mockConsoleLog).toHaveBeenCalledWith('🔐 [useAuthFetch] GET /api/auth/me')
    })
  })

  describe('multiple calls', () => {
    it('should handle multiple concurrent calls', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      
      // Make multiple concurrent calls
      const [result1, result2, result3] = await Promise.all([
        authFetch('/api/auth/me'),
        authFetch('/api/auth/login', { method: 'POST' }),
        authFetch('/api/auth/logout', { method: 'DELETE' })
      ])

      // Test all results are the same
      expect(result1).toEqual(mockResponse)
      expect(result2).toEqual(mockResponse)
      expect(result3).toEqual(mockResponse)
      
      // Test $fetch was called multiple times
      expect($fetch).toHaveBeenCalledTimes(3)
    })

    it('should handle mixed success and error calls', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to alternate between success and error
      vi.mocked($fetch)
        .mockResolvedValueOnce(mockResponse)
        .mockRejectedValueOnce(new Error('Network error'))
        .mockResolvedValueOnce(mockResponse)

      const authFetch = useAuthFetch()
      
      // Make calls
      const [result1, result2, result3] = await Promise.allSettled([
        authFetch('/api/auth/me'),
        authFetch('/api/auth/login'),
        authFetch('/api/auth/logout')
      ])

      // Test results
      expect(result1.status).toBe('fulfilled')
      expect(result2.status).toBe('rejected')
      expect(result3.status).toBe('fulfilled')
      
      // Test $fetch was called multiple times
      expect($fetch).toHaveBeenCalledTimes(3)
    })
  })

  describe('interface contract', () => {
    it('should return a function', () => {
      const authFetch = useAuthFetch()

      // Test interface: returns a function
      expect(typeof authFetch).toBe('function')
    })

    it('should accept url and options parameters', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      
      // Test function accepts parameters
      await authFetch('/api/auth/me')
      await authFetch('/api/auth/login', { method: 'POST' })
      
      // Test both calls worked
      expect($fetch).toHaveBeenCalledTimes(2)
    })
  })

  describe('edge cases', () => {
    it('should handle empty URL', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      await authFetch('')

      // Test $fetch was called with empty URL
      expect($fetch).toHaveBeenCalledWith('', {
        baseURL: ''
      })
    })

    it('should handle URLs with query parameters', async () => {
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      await authFetch('/api/auth/me?include=profile&format=json')

      // Test $fetch was called with query parameters
      expect($fetch).toHaveBeenCalledWith('/api/auth/me?include=profile&format=json', {
        baseURL: ''
      })
    })

    it('should handle very long URLs', async () => {
      const longUrl = '/api/auth/test/' + 'A'.repeat(1000)
      const mockResponse = { success: true }
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      await authFetch(longUrl)

      // Test $fetch was called with long URL
      expect($fetch).toHaveBeenCalledWith(longUrl, {
        baseURL: ''
      })
    })

    it('should handle URLs with special characters', async () => {
      const mockResponse = { success: true }
      const specialUrl = '/api/auth/test?param=value%20with%20spaces&other=test@example.com'
      
      // Mock $fetch to return response
      vi.mocked($fetch).mockResolvedValue(mockResponse)

      const authFetch = useAuthFetch()
      await authFetch(specialUrl)

      // Test $fetch was called with special characters preserved
      expect($fetch).toHaveBeenCalledWith(specialUrl, {
        baseURL: ''
      })
    })
  })
})
