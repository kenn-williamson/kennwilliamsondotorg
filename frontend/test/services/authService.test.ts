import { describe, it, expect, vi } from 'vitest'
import { createMockAuthResponse, createMockUser } from '../utils/test-helpers'
import { authService } from '~/services/authService'

describe('authService', () => {
  let mockFetcher: any

  beforeEach(() => {
    mockFetcher = vi.fn()
  })

  describe('login', () => {
    it('should call correct endpoint with POST method and credentials', async () => {
      const credentials = {
        email: 'user@example.com',
        password: 'password123'
      }
      const mockResponse = { success: true }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = authService(mockFetcher)
      const result = await service.login(credentials)

      expect(mockFetcher).toHaveBeenCalledWith('/api/auth/login', {
        method: 'POST',
        body: credentials
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('register', () => {
    it('should call correct endpoint with POST method and user data', async () => {
      const userData = {
        email: 'newuser@example.com',
        password: 'password123',
        display_name: 'New User'
      }
      const mockResponse = { success: true }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = authService(mockFetcher)
      const result = await service.register(userData)

      expect(mockFetcher).toHaveBeenCalledWith('/api/auth/register', {
        method: 'POST',
        body: userData
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

      const service = authService(mockFetcher)
      const result = await service.previewSlug(displayName)

      expect(mockFetcher).toHaveBeenCalledWith('/public/auth/preview-slug', {
        method: 'POST',
        body: { display_name: displayName }
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('revokeAllSessions', () => {
    it('should call correct endpoint with POST method', async () => {
      const mockResponse = { success: true }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = authService(mockFetcher)
      const result = await service.revokeAllSessions()

      expect(mockFetcher).toHaveBeenCalledWith('/protected/auth/revoke-all', {
        method: 'POST'
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('logout', () => {
    it('should call correct endpoint with POST method', async () => {
      mockFetcher.mockResolvedValue(undefined)

      const service = authService(mockFetcher)
      const result = await service.logout()

      expect(mockFetcher).toHaveBeenCalledWith('/api/auth/logout', {
        method: 'POST'
      })
      expect(result).toBeUndefined()
    })
  })

  describe('error handling', () => {
    it('should propagate fetcher errors', async () => {
      const error = new Error('Network error')
      mockFetcher.mockRejectedValue(error)

      const service = authService(mockFetcher)

      await expect(service.login({ email: 'test@example.com', password: 'password' })).rejects.toThrow('Network error')
    })
  })
})
