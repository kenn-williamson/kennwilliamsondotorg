import { describe, it, expect, vi, beforeEach } from 'vitest'
import { accessRequestService } from './accessRequestService'

describe('accessRequestService', () => {
  let mockFetcher: any

  beforeEach(() => {
    mockFetcher = vi.fn()
  })

  describe('createAccessRequest', () => {
    it('should call correct endpoint with POST method and message', async () => {
      const message = 'I would like access to view your personal content'
      const mockResponse = { message: 'Access request submitted successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = accessRequestService(mockFetcher)
      const result = await service.createAccessRequest(message)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/access-requests', {
        method: 'POST',
        body: { message }
      })
      expect(result).toEqual(mockResponse)
    })

    it('should handle successful submission response', async () => {
      const message = 'Access request message'
      const mockResponse = { message: 'Access request submitted successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = accessRequestService(mockFetcher)
      const result = await service.createAccessRequest(message)

      expect(result).toEqual(mockResponse)
    })

    it('should propagate fetcher errors', async () => {
      const error = new Error('Network error')
      mockFetcher.mockRejectedValue(error)

      const service = accessRequestService(mockFetcher)

      await expect(service.createAccessRequest('test message')).rejects.toThrow('Network error')
    })

    it('should handle duplicate request error', async () => {
      const error = new Error('You already have a pending access request')
      mockFetcher.mockRejectedValue(error)

      const service = accessRequestService(mockFetcher)

      await expect(service.createAccessRequest('test message')).rejects.toThrow('You already have a pending access request')
    })

    it('should handle empty message', async () => {
      const message = ''
      const mockResponse = { message: 'Access request submitted successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = accessRequestService(mockFetcher)
      const result = await service.createAccessRequest(message)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/access-requests', {
        method: 'POST',
        body: { message: '' }
      })
      expect(result).toEqual(mockResponse)
    })

    it('should handle long message', async () => {
      const message = 'A'.repeat(1000)
      const mockResponse = { message: 'Access request submitted successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = accessRequestService(mockFetcher)
      const result = await service.createAccessRequest(message)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/access-requests', {
        method: 'POST',
        body: { message }
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('error handling', () => {
    it('should propagate authentication errors', async () => {
      const error = new Error('Unauthorized')
      mockFetcher.mockRejectedValue(error)

      const service = accessRequestService(mockFetcher)

      await expect(service.createAccessRequest('test message')).rejects.toThrow('Unauthorized')
    })

    it('should propagate server errors', async () => {
      const error = new Error('Internal server error')
      mockFetcher.mockRejectedValue(error)

      const service = accessRequestService(mockFetcher)

      await expect(service.createAccessRequest('test message')).rejects.toThrow('Internal server error')
    })
  })
})
