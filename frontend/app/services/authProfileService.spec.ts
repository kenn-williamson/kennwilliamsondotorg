import { describe, it, expect, vi, beforeEach } from 'vitest'
import { authProfileService } from './authProfileService'

describe('authProfileService', () => {
  let mockFetcher: any

  beforeEach(() => {
    mockFetcher = vi.fn()
  })

  describe('updateProfile', () => {
    it('should call correct endpoint with PUT method and profile data', async () => {
      const profileData = {
        display_name: 'Updated Name',
        slug: 'updated-slug'
      }
      const mockResponse = { message: 'Profile updated successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = authProfileService(mockFetcher)
      const result = await service.updateProfile(profileData)

      expect(mockFetcher).toHaveBeenCalledWith('/api/auth/profile', {
        method: 'PUT',
        body: profileData
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('changePassword', () => {
    it('should call correct endpoint with PUT method and password data', async () => {
      const passwordData = {
        current_password: 'oldPassword123',
        new_password: 'newPassword456'
      }
      const mockResponse = { message: 'Password changed successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = authProfileService(mockFetcher)
      const result = await service.changePassword(passwordData)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/auth/change-password', {
        method: 'PUT',
        body: passwordData
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('previewSlug', () => {
    it('should call correct endpoint with POST method and display name', async () => {
      const displayName = 'John Doe'
      const mockResponse = {
        slug: 'john-doe',
        available: true,
        final_slug: 'john-doe'
      }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = authProfileService(mockFetcher)
      const result = await service.previewSlug(displayName)

      expect(mockFetcher).toHaveBeenCalledWith('/public/auth/preview-slug', {
        method: 'POST',
        body: { display_name: displayName }
      })
      expect(result).toEqual(mockResponse)
    })

    it('should handle unavailable slug response', async () => {
      const displayName = 'John Doe'
      const mockResponse = {
        slug: 'john-doe',
        available: false,
        final_slug: 'john-doe-2'
      }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = authProfileService(mockFetcher)
      const result = await service.previewSlug(displayName)

      expect(mockFetcher).toHaveBeenCalledWith('/public/auth/preview-slug', {
        method: 'POST',
        body: { display_name: displayName }
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('deleteAccount', () => {
    it('should call correct endpoint with DELETE method', async () => {
      const mockResponse = { message: 'Account deleted successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = authProfileService(mockFetcher)
      const result = await service.deleteAccount()

      expect(mockFetcher).toHaveBeenCalledWith('/protected/auth/delete-account', {
        method: 'DELETE'
      })
      expect(result).toEqual(mockResponse)
    })

    it('should handle successful deletion response', async () => {
      const mockResponse = { message: 'Account deleted successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = authProfileService(mockFetcher)
      const result = await service.deleteAccount()

      expect(result).toEqual(mockResponse)
    })

    it('should propagate fetcher errors', async () => {
      const error = new Error('Account deletion failed')
      mockFetcher.mockRejectedValue(error)

      const service = authProfileService(mockFetcher)

      await expect(service.deleteAccount()).rejects.toThrow('Account deletion failed')
    })

    it('should handle system user deletion error', async () => {
      const error = new Error('Cannot delete system user')
      mockFetcher.mockRejectedValue(error)

      const service = authProfileService(mockFetcher)

      await expect(service.deleteAccount()).rejects.toThrow('Cannot delete system user')
    })

    it('should handle user not found error', async () => {
      const error = new Error('User not found')
      mockFetcher.mockRejectedValue(error)

      const service = authProfileService(mockFetcher)

      await expect(service.deleteAccount()).rejects.toThrow('User not found')
    })
  })

  describe('error handling', () => {
    it('should propagate fetcher errors', async () => {
      const error = new Error('Network error')
      mockFetcher.mockRejectedValue(error)

      const service = authProfileService(mockFetcher)

      await expect(service.updateProfile({ display_name: 'Test', slug: 'test' })).rejects.toThrow('Network error')
    })
  })
})
