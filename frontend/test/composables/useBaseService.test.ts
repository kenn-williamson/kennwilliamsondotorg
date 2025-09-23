import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { nextTick } from 'vue'

// Mock Vue's getCurrentInstance and onUnmounted
vi.mock('vue', async () => {
  const actual = await vi.importActual('vue')
  return {
    ...actual,
    getCurrentInstance: vi.fn(),
    onUnmounted: vi.fn()
  }
})

// Mock console methods to avoid noise in tests
const mockConsoleError = vi.spyOn(console, 'error').mockImplementation(() => {})
const mockConsoleLog = vi.spyOn(console, 'log').mockImplementation(() => {})

import { useBaseService } from '~/composables/useBaseService'

describe('useBaseService', () => {
  beforeEach(() => {
    // Reset all mocks
    vi.clearAllMocks()
    mockConsoleError.mockClear()
    mockConsoleLog.mockClear()
  })

  afterEach(() => {
    // Clean up any pending timers
    vi.clearAllTimers()
  })

  describe('initial state', () => {
    it('should initialize with correct default state', () => {
      const service = useBaseService()

      // Test initial state
      expect(service.isLoading.value).toBe(false)
      expect(service.error.value).toBe(null)
      expect(service.hasError.value).toBe(false)
    })
  })

  describe('state management', () => {
    it('should update loading state', () => {
      const service = useBaseService()

      // Test setting loading to true
      service.setLoading(true)
      expect(service.isLoading.value).toBe(true)

      // Test setting loading to false
      service.setLoading(false)
      expect(service.isLoading.value).toBe(false)
    })

    it('should update error state', () => {
      const service = useBaseService()

      // Test setting error
      const errorMessage = 'Test error message'
      service.setError(errorMessage)
      expect(service.error.value).toBe(errorMessage)
      expect(service.hasError.value).toBe(true)

      // Test clearing error
      service.clearError()
      expect(service.error.value).toBe(null)
      expect(service.hasError.value).toBe(false)
    })

    it('should handle null error state', () => {
      const service = useBaseService()

      // Test setting error to null
      service.setError(null)
      expect(service.error.value).toBe(null)
      expect(service.hasError.value).toBe(false)
    })
  })

  describe('executeRequest success scenarios', () => {
    it('should execute successful request and update loading state', async () => {
      const service = useBaseService()
      const mockData = { id: 1, name: 'test' }
      const mockRequest = vi.fn().mockResolvedValue(mockData)

      // Execute request
      const result = await service.executeRequest(mockRequest, 'testContext')

      // Test result
      expect(result).toEqual(mockData)
      expect(mockRequest).toHaveBeenCalledOnce()

      // Test loading state transitions
      expect(service.isLoading.value).toBe(false)
      expect(service.error.value).toBe(null)
      expect(service.hasError.value).toBe(false)
    })

    it('should handle loading state transitions during request', async () => {
      const service = useBaseService()
      let loadingStateDuringRequest = false
      
      const mockRequest = vi.fn().mockImplementation(async () => {
        // Check loading state during request execution
        loadingStateDuringRequest = service.isLoading.value
        return { success: true }
      })

      // Execute request
      await service.executeRequest(mockRequest)

      // Test loading state was true during request
      expect(loadingStateDuringRequest).toBe(true)
      
      // Test loading state is false after request
      expect(service.isLoading.value).toBe(false)
    })

    it('should clear error before executing request', async () => {
      const service = useBaseService()
      
      // Set initial error
      service.setError('Previous error')
      expect(service.hasError.value).toBe(true)

      const mockRequest = vi.fn().mockResolvedValue({ success: true })

      // Execute request
      await service.executeRequest(mockRequest)

      // Test error was cleared
      expect(service.error.value).toBe(null)
      expect(service.hasError.value).toBe(false)
    })
  })

  describe('executeRequest error scenarios', () => {
    it('should handle Error objects and update error state', async () => {
      const service = useBaseService()
      const errorMessage = 'Request failed'
      const mockError = new Error(errorMessage)
      const mockRequest = vi.fn().mockRejectedValue(mockError)

      // Execute request and expect it to throw
      await expect(service.executeRequest(mockRequest, 'testContext')).rejects.toThrow(errorMessage)

      // Test error state
      expect(service.error.value).toBe(errorMessage)
      expect(service.hasError.value).toBe(true)
      expect(service.isLoading.value).toBe(false)

      // Test error logging
      expect(mockConsoleError).toHaveBeenCalledWith(
        '[BaseService] Error in testContext:',
        errorMessage
      )
    })

    it('should handle non-Error objects and update error state', async () => {
      const service = useBaseService()
      const mockRequest = vi.fn().mockRejectedValue('String error')

      // Execute request and expect it to throw
      await expect(service.executeRequest(mockRequest)).rejects.toThrow('String error')

      // Test error state
      expect(service.error.value).toBe('An unexpected error occurred')
      expect(service.hasError.value).toBe(true)
      expect(service.isLoading.value).toBe(false)

      // Test error logging
      expect(mockConsoleError).toHaveBeenCalledWith(
        '[BaseService] Error:',
        'An unexpected error occurred'
      )
    })

    it('should handle null/undefined errors gracefully', async () => {
      const service = useBaseService()
      const mockRequest = vi.fn().mockRejectedValue(null)

      // Execute request and expect it to throw
      await expect(service.executeRequest(mockRequest)).rejects.toThrow()

      // Test error state
      expect(service.error.value).toBe('An unexpected error occurred')
      expect(service.hasError.value).toBe(true)
    })

    it('should ensure loading state is false even when request fails', async () => {
      const service = useBaseService()
      const mockRequest = vi.fn().mockRejectedValue(new Error('Request failed'))

      // Execute request
      try {
        await service.executeRequest(mockRequest)
      } catch (error) {
        // Expected to throw
      }

      // Test loading state is false after error
      expect(service.isLoading.value).toBe(false)
    })
  })

  describe('executeRequestWithSuccess scenarios', () => {
    it('should execute successful request and log success message', async () => {
      const service = useBaseService()
      const mockData = { id: 1, name: 'test' }
      const mockRequest = vi.fn().mockResolvedValue(mockData)
      const successMessage = 'Operation completed successfully'

      // Execute request with success handling
      const result = await service.executeRequestWithSuccess(mockRequest, successMessage, 'testContext')

      // Test result
      expect(result).toEqual(mockData)
      expect(mockRequest).toHaveBeenCalledOnce()

      // Test success logging
      expect(mockConsoleLog).toHaveBeenCalledWith('[BaseService] Success: Operation completed successfully')

      // Test state
      expect(service.isLoading.value).toBe(false)
      expect(service.error.value).toBe(null)
      expect(service.hasError.value).toBe(false)
    })

    it('should handle errors in executeRequestWithSuccess', async () => {
      const service = useBaseService()
      const mockError = new Error('Request failed')
      const mockRequest = vi.fn().mockRejectedValue(mockError)
      const successMessage = 'This should not be logged'

      // Execute request and expect it to throw
      await expect(service.executeRequestWithSuccess(mockRequest, successMessage, 'testContext')).rejects.toThrow('Request failed')

      // Test success message was not logged
      expect(mockConsoleLog).not.toHaveBeenCalledWith('[BaseService] Success: This should not be logged')

      // Test error was logged
      expect(mockConsoleError).toHaveBeenCalledWith(
        '[BaseService] Error in testContext:',
        'Request failed'
      )

      // Test error state
      expect(service.error.value).toBe('Request failed')
      expect(service.hasError.value).toBe(true)
    })
  })

  describe('component lifecycle integration', () => {
    it('should handle component unmount cleanup', async () => {
      const mockOnUnmounted = vi.fn()
      const mockGetCurrentInstance = vi.fn().mockReturnValue({})

      // Mock Vue functions
      const { getCurrentInstance, onUnmounted } = await import('vue')
      vi.mocked(getCurrentInstance).mockReturnValue({})
      vi.mocked(onUnmounted).mockImplementation(mockOnUnmounted)

      // Create service instance
      const service = useBaseService()

      // Test that onUnmounted was called
      expect(mockOnUnmounted).toHaveBeenCalledOnce()

      // Test that the cleanup function clears error
      const cleanupFunction = mockOnUnmounted.mock.calls[0][0]
      
      // Set an error
      service.setError('Test error')
      expect(service.error.value).toBe('Test error')

      // Call cleanup function
      cleanupFunction()

      // Test error was cleared
      expect(service.error.value).toBe(null)
    })

    it('should not set up cleanup when not in component context', async () => {
      const mockOnUnmounted = vi.fn()
      const mockGetCurrentInstance = vi.fn().mockReturnValue(null)

      // Mock Vue functions
      const { getCurrentInstance, onUnmounted } = await import('vue')
      vi.mocked(getCurrentInstance).mockReturnValue(null)
      vi.mocked(onUnmounted).mockImplementation(mockOnUnmounted)

      // Create service instance
      useBaseService()

      // Test that onUnmounted was not called
      expect(mockOnUnmounted).not.toHaveBeenCalled()
    })
  })

  describe('multiple instances', () => {
    it('should maintain separate state between instances', () => {
      const service1 = useBaseService()
      const service2 = useBaseService()

      // Set different states
      service1.setLoading(true)
      service1.setError('Service 1 error')
      
      service2.setLoading(false)
      service2.setError('Service 2 error')

      // Test states are independent
      expect(service1.isLoading.value).toBe(true)
      expect(service1.error.value).toBe('Service 1 error')
      expect(service1.hasError.value).toBe(true)

      expect(service2.isLoading.value).toBe(false)
      expect(service2.error.value).toBe('Service 2 error')
      expect(service2.hasError.value).toBe(true)
    })
  })

  describe('interface contract', () => {
    it('should expose all expected methods and state', () => {
      const service = useBaseService()

      // Test state properties
      expect(service).toHaveProperty('isLoading')
      expect(service).toHaveProperty('error')
      expect(service).toHaveProperty('hasError')

      // Test state management methods
      expect(service).toHaveProperty('setLoading')
      expect(service).toHaveProperty('setError')
      expect(service).toHaveProperty('clearError')

      // Test request execution methods
      expect(service).toHaveProperty('executeRequest')
      expect(service).toHaveProperty('executeRequestWithSuccess')

      // Test methods are functions
      expect(typeof service.setLoading).toBe('function')
      expect(typeof service.setError).toBe('function')
      expect(typeof service.clearError).toBe('function')
      expect(typeof service.executeRequest).toBe('function')
      expect(typeof service.executeRequestWithSuccess).toBe('function')

      // Test state properties are reactive
      expect(service.isLoading.value).toBeDefined()
      expect(service.error.value).toBeDefined()
      expect(service.hasError.value).toBeDefined()
    })
  })

  describe('edge cases', () => {
    it('should handle empty string error messages', () => {
      const service = useBaseService()

      service.setError('')
      expect(service.error.value).toBe('')
      expect(service.hasError.value).toBe(false) // Empty string is falsy
    })

    it('should handle very long error messages', () => {
      const service = useBaseService()
      const longError = 'A'.repeat(1000)

      service.setError(longError)
      expect(service.error.value).toBe(longError)
      expect(service.hasError.value).toBe(true)
    })

    it('should handle rapid state changes', async () => {
      const service = useBaseService()

      // Rapid state changes
      service.setLoading(true)
      service.setError('Error 1')
      service.setLoading(false)
      service.setError('Error 2')
      service.clearError()

      // Test final state
      expect(service.isLoading.value).toBe(false)
      expect(service.error.value).toBe(null)
      expect(service.hasError.value).toBe(false)
    })
  })
})
