import { describe, it, expect, vi, beforeEach } from 'vitest'
import { authService } from './authService'

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
        email: 'user@example.com',
        password: 'password123',
        display_name: 'John Doe'
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

  describe('getCurrentUser', () => {
    it('should call correct endpoint with GET method', async () => {
      const mockResponse = {
        id: '123',
        email: 'user@example.com',
        display_name: 'John Doe',
        roles: ['user']
      }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = authService(mockFetcher)
      const result = await service.getCurrentUser()

      expect(mockFetcher).toHaveBeenCalledWith('/api/auth/me')
      expect(result).toEqual(mockResponse)
    })
  })
})
