import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'

// Mock all dependencies before importing the composable
vi.mock('./useJwtManager', () => ({
  useJwtManager: vi.fn()
}))

// Mock $fetch globally
global.$fetch = vi.fn()

// Mock useRequestFetch globally
global.useRequestFetch = vi.fn()

// Mock useRuntimeConfig globally
global.useRuntimeConfig = vi.fn()

// Mock import.meta.server globally - this needs to be set before module import
vi.stubGlobal('import', { meta: { server: false } })

import { useSmartFetch } from './useSmartFetch'

describe('useSmartFetch', () => {
  let mockJwtManager: any
  let mockRuntimeConfig: any

  beforeEach(async () => {
    // Reset all mocks
    vi.clearAllMocks()

    mockJwtManager = {
      getToken: vi.fn()
    }

    mockRuntimeConfig = {
      apiBase: 'http://backend:8080/backend',
      public: {
        apiBase: 'https://localhost/backend'
      }
    }

    // Configure mocked modules
    const { useJwtManager } = await import('./useJwtManager')
    vi.mocked(useJwtManager).mockReturnValue(mockJwtManager)

    // Configure global mocks
    global.useRuntimeConfig.mockReturnValue(mockRuntimeConfig)
    global.useRequestFetch.mockReturnValue(vi.fn())
  })

  afterEach(() => {
    // Clean up any pending timers
    vi.clearAllTimers()
  })

  describe('passthrough routes (/api/*)', () => {
    it('should handle passthrough routes with client-side $fetch', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      const result = await smartFetch('/api/phrases/random')

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with correct parameters (passthrough keeps /api prefix)
      expect($fetch).toHaveBeenCalledWith('/api/phrases/random', {})
    })
  })

  describe('direct routes (non-/api/*)', () => {
    it('should handle direct routes with client-side $fetch', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      const result = await smartFetch('/public/health')

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with correct parameters
      expect($fetch).toHaveBeenCalledWith('https://localhost/backend/public/health', {})
    })
  })

  describe('protected routes with JWT authentication', () => {
    it('should add JWT token to protected routes', async () => {
      const mockToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
      const mockResponse = { success: true }
      
      // Setup mocks
      mockJwtManager.getToken.mockResolvedValue(mockToken)
      vi.mocked($fetch).mockResolvedValue(mockResponse)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      const result = await smartFetch('/protected/incident-timers')

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test JWT manager was called
      expect(mockJwtManager.getToken).toHaveBeenCalled()
      
      // Test $fetch was called with Authorization header
      expect($fetch).toHaveBeenCalledWith('https://localhost/backend/protected/incident-timers', {
        headers: {
          Authorization: `Bearer ${mockToken}`
        }
      })
    })

    it('should handle JWT manager errors', async () => {
      // Setup mocks - JWT manager throws error
      mockJwtManager.getToken.mockRejectedValue(new Error('JWT error'))
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      
      // Test that error is propagated
      await expect(smartFetch('/protected/incident-timers')).rejects.toThrow('JWT error')
      
      // Test JWT manager was called
      expect(mockJwtManager.getToken).toHaveBeenCalled()
      
      // Test $fetch was not called due to error
      expect($fetch).not.toHaveBeenCalled()
    })
  })

  describe('public routes without JWT authentication', () => {
    it('should not call JWT manager for public routes', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      const result = await smartFetch('/public/health')

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test JWT manager was not called
      expect(mockJwtManager.getToken).not.toHaveBeenCalled()
      
      // Test $fetch was called without Authorization header
      expect($fetch).toHaveBeenCalledWith('https://localhost/backend/public/health', {})
    })
  })

  describe('query parameters', () => {
    it('should handle query parameters correctly', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      const result = await smartFetch('/public/users', {
        query: {
          search: 'test',
          limit: 10,
          page: 1
        }
      })

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with query parameters
      expect($fetch).toHaveBeenCalledWith('https://localhost/backend/public/users?search=test&limit=10&page=1', {})
    })

    it('should handle empty query parameters', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      const result = await smartFetch('/public/health', {
        query: {}
      })

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called without query parameters
      expect($fetch).toHaveBeenCalledWith('https://localhost/backend/public/health', {})
    })

    it('should filter out undefined and null query values', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      const result = await smartFetch('/public/users', {
        query: {
          search: 'test',
          limit: 10,
          page: null,
          filter: undefined,
          active: true
        }
      })

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with filtered query parameters
      expect($fetch).toHaveBeenCalledWith('https://localhost/backend/public/users?search=test&limit=10&active=true', {})
    })
  })

  describe('request options', () => {
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
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      const result = await smartFetch('/public/test', requestOptions)

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with all options preserved
      expect($fetch).toHaveBeenCalledWith('https://localhost/backend/public/test', {
        method: 'POST',
        body: { data: 'test' },
        headers: { 'Content-Type': 'application/json' },
        timeout: 5000
      })
    })

    it('should handle empty options object', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      const result = await smartFetch('/public/health', {})

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with minimal options
      expect($fetch).toHaveBeenCalledWith('https://localhost/backend/public/health', {})
    })
  })

  describe('error handling', () => {
    it('should propagate network errors', async () => {
      const mockError = new Error('Network error')
      
      // Setup mocks
      vi.mocked($fetch).mockRejectedValue(mockError)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      
      // Test error is propagated
      await expect(smartFetch('/public/health')).rejects.toThrow('Network error')
    })

    it('should propagate server errors', async () => {
      const mockError = new Error('Server error')
      const mockToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
      
      // Setup mocks - JWT manager succeeds, but $fetch fails
      mockJwtManager.getToken.mockResolvedValue(mockToken)
      vi.mocked($fetch).mockRejectedValue(mockError)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      
      // Test error is propagated
      await expect(smartFetch('/protected/auth/me')).rejects.toThrow('Server error')
    })
  })

  describe('interface contract', () => {
    it('should return a function', () => {
      const smartFetch = useSmartFetch()

      // Test interface: returns a function
      expect(typeof smartFetch).toBe('function')
    })

    it('should accept route and options parameters', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      
      // Test function accepts parameters
      await smartFetch('/public/health')
      await smartFetch('/public/health', { method: 'POST' })
      
      // Test both calls worked
      expect($fetch).toHaveBeenCalledTimes(2)
    })
  })

  describe('edge cases', () => {
    it('should handle empty route', async () => {
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      const result = await smartFetch('')

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with empty route
      expect($fetch).toHaveBeenCalledWith('https://localhost/backend', {})
    })


    it('should handle very long routes', async () => {
      const longRoute = '/public/test/' + 'A'.repeat(1000)
      const mockResponse = { success: true }
      
      // Setup mocks
      vi.mocked($fetch).mockResolvedValue(mockResponse)
      // Mock import.meta.server to false for client-side behavior
      vi.stubGlobal('import', { meta: { server: false } })

      const smartFetch = useSmartFetch()
      const result = await smartFetch(longRoute)

      // Test result
      expect(result).toEqual(mockResponse)
      
      // Test $fetch was called with long route
      expect($fetch).toHaveBeenCalledWith(`https://localhost/backend${longRoute}`, {})
    })
  })
})