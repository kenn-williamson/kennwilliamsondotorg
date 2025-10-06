import { describe, it, expect, vi, beforeEach } from 'vitest'

// Mock all dependencies before importing the composable
vi.mock('./useBaseService', () => ({
  useBaseService: vi.fn()
}))

vi.mock('./useSmartFetch', () => ({
  useSmartFetch: vi.fn()
}))

vi.mock('../services/authProfileService', () => ({
  authProfileService: vi.fn()
}))

import { useAuthProfileActions } from './useAuthProfileActions'

describe('useAuthProfileActions', () => {
  let mockUserSession: any
  let mockAuthProfileService: any

  beforeEach(async () => {
    // Reset all mocks
    vi.clearAllMocks()

    mockUserSession = {
      fetch: vi.fn().mockResolvedValue(undefined),
      clear: vi.fn().mockResolvedValue(undefined)
    }
    
    mockAuthProfileService = {
      updateProfile: vi.fn(),
      changePassword: vi.fn(),
      previewSlug: vi.fn(),
      validateSlug: vi.fn(),
      deleteAccount: vi.fn()
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
    
    const { authProfileService } = await import('../services/authProfileService')
    vi.mocked(authProfileService).mockReturnValue(mockAuthProfileService)
    
    // Configure the useUserSession mock
    ;(global as any).useUserSession.mockReturnValue(mockUserSession)
  })

  describe('updateProfile orchestration', () => {
    it('should orchestrate service call and session refresh', async () => {
      const profileData = {
        display_name: 'John Doe',
        slug: 'john-doe'
      }
      const serviceResponse = { message: 'Profile updated successfully' }
      
      // Setup service mock
      mockAuthProfileService.updateProfile.mockResolvedValue(serviceResponse)

      const { updateProfile } = useAuthProfileActions()
      
      const result = await updateProfile(profileData)

      // Test orchestration: service called with correct data
      expect(mockAuthProfileService.updateProfile).toHaveBeenCalledWith(profileData)
      
      // Test orchestration: session refresh called
      expect(mockUserSession.fetch).toHaveBeenCalled()
      
      // Test orchestration: result returned
      expect(result).toEqual(serviceResponse)
    })
  })

  describe('changePassword orchestration', () => {
    it('should orchestrate service call', async () => {
      const passwordData = {
        current_password: 'oldPassword123',
        new_password: 'newPassword456'
      }
      const serviceResponse = { message: 'Password changed successfully' }
      
      // Setup service mock
      mockAuthProfileService.changePassword.mockResolvedValue(serviceResponse)

      const { changePassword } = useAuthProfileActions()
      const result = await changePassword(passwordData)

      // Test orchestration: service called with correct data
      expect(mockAuthProfileService.changePassword).toHaveBeenCalledWith(passwordData)
      
      // Test orchestration: result returned
      expect(result).toEqual(serviceResponse)
    })
  })

  describe('previewSlug orchestration', () => {
    it('should orchestrate service call', async () => {
      const displayName = 'John Doe'
      const serviceResponse = {
        slug: 'john-doe',
        available: true,
        final_slug: 'john-doe'
      }
      
      // Setup service mock
      mockAuthProfileService.previewSlug.mockResolvedValue(serviceResponse)

      const { previewSlug } = useAuthProfileActions()
      const result = await previewSlug(displayName)

      // Test orchestration: service called with correct data
      expect(mockAuthProfileService.previewSlug).toHaveBeenCalledWith(displayName)
      
      // Test orchestration: result returned
      expect(result).toEqual(serviceResponse)
    })
  })

  describe('deleteAccount orchestration', () => {
    it('should orchestrate service call and session clear', async () => {
      const serviceResponse = { message: 'Account deleted successfully' }
      
      // Setup service mock
      mockAuthProfileService.deleteAccount.mockResolvedValue(serviceResponse)

      const { deleteAccount } = useAuthProfileActions()
      const result = await deleteAccount()

      // Test orchestration: service called
      expect(mockAuthProfileService.deleteAccount).toHaveBeenCalledTimes(1)
      
      // Test orchestration: session cleared after successful deletion
      expect(mockUserSession.clear).toHaveBeenCalledTimes(1)
      
      // Test orchestration: result returned
      expect(result).toEqual(serviceResponse)
    })

    it('should handle deletion errors', async () => {
      const error = new Error('Account deletion failed')
      mockAuthProfileService.deleteAccount.mockRejectedValue(error)

      const { deleteAccount } = useAuthProfileActions()

      await expect(deleteAccount()).rejects.toThrow('Account deletion failed')
    })

    it('should not clear session if deletion fails', async () => {
      const error = new Error('Account deletion failed')
      mockAuthProfileService.deleteAccount.mockRejectedValue(error)

      const { deleteAccount } = useAuthProfileActions()

      try {
        await deleteAccount()
      } catch (e) {
        // Expected to throw
      }

      expect(mockUserSession.clear).not.toHaveBeenCalled()
    })
  })

  describe('service instantiation', () => {
    it('should create authProfileService with correct fetchers', async () => {
      useAuthProfileActions()

      // Test orchestration: service created with fetcher
      const { authProfileService } = await import('../services/authProfileService')
      expect(authProfileService).toHaveBeenCalledWith(expect.any(Function))
    })
  })

  describe('interface contract', () => {
    it('should expose all expected methods and state', () => {
      const actions = useAuthProfileActions()

      // Test interface: all methods present
      expect(actions).toHaveProperty('updateProfile')
      expect(actions).toHaveProperty('changePassword')
      expect(actions).toHaveProperty('previewSlug')
      expect(actions).toHaveProperty('validateSlug')
      expect(actions).toHaveProperty('deleteAccount')
      
      // Test interface: state from useBaseService exposed
      expect(actions).toHaveProperty('isLoading')
      expect(actions).toHaveProperty('error')
      expect(actions).toHaveProperty('hasError')
      
      // Test interface: methods are functions
      expect(typeof actions.updateProfile).toBe('function')
      expect(typeof actions.changePassword).toBe('function')
      expect(typeof actions.previewSlug).toBe('function')
      expect(typeof actions.validateSlug).toBe('function')
      expect(typeof actions.deleteAccount).toBe('function')
    })
  })

})