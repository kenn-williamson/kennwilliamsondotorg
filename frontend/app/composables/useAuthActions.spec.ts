// @ts-nocheck
import { describe, it, expect, vi, beforeEach } from 'vitest'

// Mock all dependencies before importing the composable
vi.mock('./useBaseService', () => ({
  useBaseService: vi.fn()
}))

vi.mock('./useSmartFetch', () => ({
  useSmartFetch: vi.fn()
}))

vi.mock('../services/authService', () => ({
  authService: vi.fn()
}))

import { useAuthActions } from './useAuthActions'

describe('useAuthActions', () => {
  let mockUserSession: any
  let mockAuthService: any

  beforeEach(async () => {
    // Reset all mocks
    vi.clearAllMocks()

    mockUserSession = {
      clear: vi.fn().mockResolvedValue(undefined),
      fetch: vi.fn().mockResolvedValue(undefined)
    }
    
    mockAuthService = {
      login: vi.fn(),
      register: vi.fn(),
      previewSlug: vi.fn(),
      revokeAllSessions: vi.fn(),
      logout: vi.fn()
    }
    
    // Configure mocked modules
    const { useBaseService } = await import('./useBaseService')
    vi.mocked(useBaseService).mockReturnValue({
      executeRequest: vi.fn().mockImplementation(async (fn) => await fn()),
      executeRequestWithSuccess: vi.fn().mockImplementation(async (fn) => await fn()),
      isLoading: { value: false } as any,
      error: { value: null } as any,
      hasError: { value: false } as any
    })
    
    const { useSmartFetch } = await import('./useSmartFetch')
    vi.mocked(useSmartFetch).mockReturnValue(vi.fn())
    
    const { authService } = await import('../services/authService')
    vi.mocked(authService).mockReturnValue(mockAuthService)

    // Configure the useUserSession mock (already defined in test/setup.ts)
    vi.mocked(global.useUserSession).mockReturnValue(mockUserSession)
  })

  describe('login orchestration', () => {
    it('should orchestrate service call and session refresh', async () => {
      const credentials = {
        email: 'user@example.com',
        password: 'password123'
      }
      
      // Setup service mock
      mockAuthService.login.mockResolvedValue(undefined)

      const { login } = useAuthActions()
      const result = await login(credentials)

      // Test orchestration: service called with correct data
      expect(mockAuthService.login).toHaveBeenCalledWith(credentials)
      
      // Test orchestration: session refresh called
      expect(mockUserSession.fetch).toHaveBeenCalled()
      
      // Test orchestration: result returned
      expect(result).toEqual({ success: true })
    })
  })

  describe('register orchestration', () => {
    it('should orchestrate service call and session refresh', async () => {
      const userData = {
        email: 'user@example.com',
        password: 'password123',
        display_name: 'John Doe'
      }
      
      // Setup service mock
      mockAuthService.register.mockResolvedValue(undefined)

      const { register } = useAuthActions()
      const result = await register(userData)

      // Test orchestration: service called with correct data
      expect(mockAuthService.register).toHaveBeenCalledWith(userData)
      
      // Test orchestration: session refresh called
      expect(mockUserSession.fetch).toHaveBeenCalled()
      
      // Test orchestration: result returned
      expect(result).toEqual({ success: true })
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
      mockAuthService.previewSlug.mockResolvedValue(serviceResponse)

      const { previewSlug } = useAuthActions()
      const result = await previewSlug(displayName)

      // Test orchestration: service called with correct data
      expect(mockAuthService.previewSlug).toHaveBeenCalledWith(displayName)
      
      // Test orchestration: result returned
      expect(result).toEqual(serviceResponse)
    })
  })

  describe('revokeAllSessions orchestration', () => {
    it('should orchestrate service call and session clear', async () => {
      // Setup service mock
      mockAuthService.revokeAllSessions.mockResolvedValue(undefined)

      const { revokeAllSessions } = useAuthActions()
      const result = await revokeAllSessions()

      // Test orchestration: service called
      expect(mockAuthService.revokeAllSessions).toHaveBeenCalled()
      
      // Test orchestration: session cleared
      expect(mockUserSession.clear).toHaveBeenCalled()
      
      // Test orchestration: result returned
      expect(result).toEqual({ success: true })
    })
  })

  describe('logout orchestration', () => {
    it('should orchestrate service call and session clear', async () => {
      // Setup service mock
      mockAuthService.logout.mockResolvedValue(undefined)

      const { logout } = useAuthActions()
      await logout()

      // Test orchestration: service called
      expect(mockAuthService.logout).toHaveBeenCalled()
      
      // Test orchestration: session cleared
      expect(mockUserSession.clear).toHaveBeenCalled()
    })
  })

  describe('service instantiation', () => {
    it('should create authService with correct fetcher', async () => {
      useAuthActions()

      // Test orchestration: service created with fetcher
      const { authService } = await import('../services/authService')
      expect(authService).toHaveBeenCalledWith(expect.any(Function))
    })
  })

  describe('interface contract', () => {
    it('should expose all expected methods and state', () => {
      const actions = useAuthActions()

      // Test interface: all methods present
      expect(actions).toHaveProperty('login')
      expect(actions).toHaveProperty('register')
      expect(actions).toHaveProperty('previewSlug')
      expect(actions).toHaveProperty('revokeAllSessions')
      expect(actions).toHaveProperty('logout')
      
      // Test interface: state from useBaseService exposed
      expect(actions).toHaveProperty('isLoading')
      expect(actions).toHaveProperty('error')
      expect(actions).toHaveProperty('hasError')
      
      // Test interface: methods are functions
      expect(typeof actions.login).toBe('function')
      expect(typeof actions.register).toBe('function')
      expect(typeof actions.previewSlug).toBe('function')
      expect(typeof actions.revokeAllSessions).toBe('function')
      expect(typeof actions.logout).toBe('function')
    })
  })
})
