import { describe, it, expect, vi, beforeEach } from 'vitest'

// Mock all dependencies before importing the composable
vi.mock('./useBaseService', () => ({
  useBaseService: vi.fn()
}))

vi.mock('./useSmartFetch', () => ({
  useSmartFetch: vi.fn()
}))

vi.mock('../services/accessRequestService', () => ({
  accessRequestService: vi.fn()
}))

import { useAccessRequestActions } from './useAccessRequestActions'

describe('useAccessRequestActions', () => {
  let mockAccessRequestService: any

  beforeEach(async () => {
    // Reset all mocks
    vi.clearAllMocks()

    mockAccessRequestService = {
      createAccessRequest: vi.fn()
    }

    // Configure mocked modules
    const { useBaseService } = await import('./useBaseService')
    vi.mocked(useBaseService).mockReturnValue({
      executeRequest: vi.fn().mockImplementation(async (fn) => await fn()),
      executeRequestWithSuccess: vi.fn().mockImplementation(async (fn) => await fn()),
      isLoading: { value: false } as any,
      error: { value: null } as any,
      hasError: { value: false } as any,
      setLoading: vi.fn(),
      setError: vi.fn(),
      clearError: vi.fn()
    })

    const { useSmartFetch } = await import('./useSmartFetch')
    vi.mocked(useSmartFetch).mockReturnValue(vi.fn())

    const { accessRequestService } = await import('../services/accessRequestService')
    vi.mocked(accessRequestService).mockReturnValue(mockAccessRequestService)
  })

  describe('createAccessRequest orchestration', () => {
    it('should orchestrate service call with success message', async () => {
      const message = 'I would like access to view your personal content'
      const serviceResponse = { message: 'Access request submitted successfully' }

      // Setup service mock
      mockAccessRequestService.createAccessRequest.mockResolvedValue(serviceResponse)

      const { createAccessRequest } = useAccessRequestActions()

      const result = await createAccessRequest(message)

      // Test orchestration: service called with correct data
      expect(mockAccessRequestService.createAccessRequest).toHaveBeenCalledWith(message)

      // Test orchestration: result returned
      expect(result).toEqual(serviceResponse)
    })

    it('should handle empty message', async () => {
      const message = ''
      const serviceResponse = { message: 'Access request submitted successfully' }

      mockAccessRequestService.createAccessRequest.mockResolvedValue(serviceResponse)

      const { createAccessRequest } = useAccessRequestActions()
      const result = await createAccessRequest(message)

      expect(mockAccessRequestService.createAccessRequest).toHaveBeenCalledWith('')
      expect(result).toEqual(serviceResponse)
    })

    it('should handle long message', async () => {
      const message = 'A'.repeat(1000)
      const serviceResponse = { message: 'Access request submitted successfully' }

      mockAccessRequestService.createAccessRequest.mockResolvedValue(serviceResponse)

      const { createAccessRequest } = useAccessRequestActions()
      const result = await createAccessRequest(message)

      expect(mockAccessRequestService.createAccessRequest).toHaveBeenCalledWith(message)
      expect(result).toEqual(serviceResponse)
    })
  })

  describe('error handling', () => {
    it('should handle duplicate request error', async () => {
      const error = new Error('You already have a pending access request')
      mockAccessRequestService.createAccessRequest.mockRejectedValue(error)

      const { createAccessRequest } = useAccessRequestActions()

      await expect(createAccessRequest('test message')).rejects.toThrow('You already have a pending access request')
    })

    it('should handle authentication errors', async () => {
      const error = new Error('Unauthorized')
      mockAccessRequestService.createAccessRequest.mockRejectedValue(error)

      const { createAccessRequest } = useAccessRequestActions()

      await expect(createAccessRequest('test message')).rejects.toThrow('Unauthorized')
    })

    it('should handle network errors', async () => {
      const error = new Error('Network error')
      mockAccessRequestService.createAccessRequest.mockRejectedValue(error)

      const { createAccessRequest } = useAccessRequestActions()

      await expect(createAccessRequest('test message')).rejects.toThrow('Network error')
    })
  })

  describe('service instantiation', () => {
    it('should create accessRequestService with correct fetcher', async () => {
      useAccessRequestActions()

      // Test orchestration: service created with fetcher
      const { accessRequestService } = await import('../services/accessRequestService')
      expect(accessRequestService).toHaveBeenCalledWith(expect.any(Function))
    })
  })

  describe('interface contract', () => {
    it('should expose all expected methods and state', () => {
      const actions = useAccessRequestActions()

      // Test interface: all methods present
      expect(actions).toHaveProperty('createAccessRequest')

      // Test interface: state from useBaseService exposed
      expect(actions).toHaveProperty('isLoading')
      expect(actions).toHaveProperty('error')
      expect(actions).toHaveProperty('hasError')

      // Test interface: method is function
      expect(typeof actions.createAccessRequest).toBe('function')
    })
  })
})
