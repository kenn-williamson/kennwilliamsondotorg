import { describe, it, expect, vi } from 'vitest'
import { createMockUser } from '../utils/test-helpers'
import { authProfileService } from '~/services/authProfileService'

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

      expect(mockFetcher).toHaveBeenCalledWith('/protected/auth/profile', {
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

  describe('error handling', () => {
    it('should propagate fetcher errors', async () => {
      const error = new Error('Network error')
      mockFetcher.mockRejectedValue(error)

      const service = authProfileService(mockFetcher)

      await expect(service.updateProfile({ display_name: 'Test', slug: 'test' })).rejects.toThrow('Network error')
    })
  })
})
