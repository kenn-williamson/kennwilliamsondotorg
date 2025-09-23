import { describe, it, expect, vi } from 'vitest'
import { createMockAdminStats, createMockUserWithRoles, createMockPhraseSuggestion } from '../utils/test-helpers'
import { adminService } from '~/services/adminService'

describe('adminService', () => {
  let mockFetcher: any

  beforeEach(() => {
    mockFetcher = vi.fn()
  })

  describe('getStats', () => {
    it('should call correct endpoint', async () => {
      const mockStats = createMockAdminStats()
      mockFetcher.mockResolvedValue(mockStats)

      const service = adminService(mockFetcher)
      const result = await service.getStats()

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/stats')
      expect(result).toEqual(mockStats)
    })
  })

  describe('getUsers', () => {
    it('should call correct endpoint without search query', async () => {
      const mockResponse = {
        users: [createMockUserWithRoles()],
        total: 1
      }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = adminService(mockFetcher)
      const result = await service.getUsers()

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/users')
      expect(result).toEqual(mockResponse)
    })

    it('should call correct endpoint with search query', async () => {
      const searchQuery = 'john'
      const mockResponse = {
        users: [createMockUserWithRoles()],
        total: 1
      }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = adminService(mockFetcher)
      const result = await service.getUsers(searchQuery)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/users?search=john')
      expect(result).toEqual(mockResponse)
    })

    it('should handle empty search query', async () => {
      const mockResponse = {
        users: [createMockUserWithRoles()],
        total: 1
      }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = adminService(mockFetcher)
      const result = await service.getUsers('')

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/users')
      expect(result).toEqual(mockResponse)
    })

    it('should trim whitespace from search query', async () => {
      const searchQuery = '  john  '
      const mockResponse = {
        users: [createMockUserWithRoles()],
        total: 1
      }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = adminService(mockFetcher)
      const result = await service.getUsers(searchQuery)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/users?search=john')
      expect(result).toEqual(mockResponse)
    })
  })

  describe('getSuggestions', () => {
    it('should call correct endpoint', async () => {
      const mockResponse = {
        suggestions: [createMockPhraseSuggestion()],
        total: 1
      }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = adminService(mockFetcher)
      const result = await service.getSuggestions()

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/suggestions')
      expect(result).toEqual(mockResponse)
    })
  })

  describe('deactivateUser', () => {
    it('should call correct endpoint with POST method', async () => {
      const userId = 'test-user-id'
      const mockResponse = { message: 'User deactivated successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = adminService(mockFetcher)
      const result = await service.deactivateUser(userId)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/users/test-user-id/deactivate', {
        method: 'POST'
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('activateUser', () => {
    it('should call correct endpoint with POST method', async () => {
      const userId = 'test-user-id'
      const mockResponse = { message: 'User activated successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = adminService(mockFetcher)
      const result = await service.activateUser(userId)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/users/test-user-id/activate', {
        method: 'POST'
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('resetUserPassword', () => {
    it('should call correct endpoint with POST method', async () => {
      const userId = 'test-user-id'
      const mockResponse = { new_password: 'generatedPassword123' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = adminService(mockFetcher)
      const result = await service.resetUserPassword(userId)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/users/test-user-id/reset-password', {
        method: 'POST'
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('promoteUser', () => {
    it('should call correct endpoint with POST method', async () => {
      const userId = 'test-user-id'
      const mockResponse = { message: 'User promoted to admin successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = adminService(mockFetcher)
      const result = await service.promoteUser(userId)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/users/test-user-id/promote', {
        method: 'POST'
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('approveSuggestion', () => {
    it('should call correct endpoint with POST method and admin reason', async () => {
      const suggestionId = 'test-suggestion-id'
      const adminReason = 'Great suggestion!'
      const mockResponse = { message: 'Suggestion approved successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = adminService(mockFetcher)
      const result = await service.approveSuggestion(suggestionId, adminReason)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/suggestions/test-suggestion-id/approve', {
        method: 'POST',
        body: { admin_reason: adminReason }
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('rejectSuggestion', () => {
    it('should call correct endpoint with POST method and admin reason', async () => {
      const suggestionId = 'test-suggestion-id'
      const adminReason = 'Too similar to existing content'
      const mockResponse = { message: 'Suggestion rejected successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = adminService(mockFetcher)
      const result = await service.rejectSuggestion(suggestionId, adminReason)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/admin/suggestions/test-suggestion-id/reject', {
        method: 'POST',
        body: { admin_reason: adminReason }
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('error handling', () => {
    it('should propagate fetcher errors', async () => {
      const error = new Error('Network error')
      mockFetcher.mockRejectedValue(error)

      const service = adminService(mockFetcher)

      await expect(service.getStats()).rejects.toThrow('Network error')
    })
  })
})
