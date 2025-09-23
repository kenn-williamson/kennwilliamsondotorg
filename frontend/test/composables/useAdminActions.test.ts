import { describe, it, expect, vi, beforeEach } from 'vitest'

// Mock all dependencies before importing the composable
vi.mock('~/composables/useBaseService', () => ({
  useBaseService: vi.fn()
}))

vi.mock('~/composables/useBackendFetch', () => ({
  useBackendFetch: vi.fn()
}))

vi.mock('~/composables/useAuthFetch', () => ({
  useAuthFetch: vi.fn()
}))

vi.mock('~/services/adminService', () => ({
  adminService: vi.fn()
}))

vi.mock('~/stores/admin', () => ({
  useAdminStore: vi.fn()
}))

import { useAdminActions } from '~/composables/useAdminActions'

describe('useAdminActions', () => {
  let mockAdminService: any
  let mockAdminStore: any

  beforeEach(async () => {
    // Reset all mocks
    vi.clearAllMocks()

    mockAdminService = {
      getStats: vi.fn(),
      getUsers: vi.fn(),
      getSuggestions: vi.fn(),
      deactivateUser: vi.fn(),
      activateUser: vi.fn(),
      resetUserPassword: vi.fn(),
      promoteUser: vi.fn(),
      approveSuggestion: vi.fn(),
      rejectSuggestion: vi.fn()
    }
    
    mockAdminStore = {
      setUsers: vi.fn(),
      setSuggestions: vi.fn(),
      setStats: vi.fn(),
      setSearchQuery: vi.fn(),
      setSelectedUser: vi.fn(),
      setNewPassword: vi.fn(),
      updateUserActiveStatus: vi.fn(),
      updateUserRoles: vi.fn(),
      removeSuggestion: vi.fn(),
      clearNewPassword: vi.fn(),
      clearState: vi.fn(),
      users: [
        { id: 'user-1', email: 'user1@example.com', display_name: 'User One', active: true, roles: ['user'] },
        { id: 'user-2', email: 'user2@example.com', display_name: 'User Two', active: false, roles: ['user'] }
      ],
      searchQuery: 'test search'
    }
    
    // Configure mocked modules
    const { useBaseService } = await import('~/composables/useBaseService')
    vi.mocked(useBaseService).mockReturnValue({
      executeRequest: vi.fn().mockImplementation(async (fn) => await fn()),
      executeRequestWithSuccess: vi.fn().mockImplementation(async (fn) => await fn()),
      isLoading: { value: false },
      error: { value: null },
      hasError: { value: false }
    })
    
    const { useBackendFetch } = await import('~/composables/useBackendFetch')
    vi.mocked(useBackendFetch).mockReturnValue(vi.fn())
    
    const { useAuthFetch } = await import('~/composables/useAuthFetch')
    vi.mocked(useAuthFetch).mockReturnValue(vi.fn())
    
    const { adminService } = await import('~/services/adminService')
    vi.mocked(adminService).mockReturnValue(mockAdminService)
    
    const { useAdminStore } = await import('~/stores/admin')
    vi.mocked(useAdminStore).mockReturnValue(mockAdminStore)
  })

  describe('fetchStats orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const mockStats = {
        total_users: 25,
        active_users: 23,
        pending_suggestions: 3,
        total_phrases: 15
      }
      
      // Setup service mock
      mockAdminService.getStats.mockResolvedValue(mockStats)

      const { fetchStats } = useAdminActions()
      const result = await fetchStats()

      // Test orchestration: service called
      expect(mockAdminService.getStats).toHaveBeenCalled()
      
      // Test orchestration: store updated with stats
      expect(mockAdminStore.setStats).toHaveBeenCalledWith(mockStats)
      
      // Test orchestration: result returned
      expect(result).toEqual(mockStats)
    })
  })

  describe('fetchUsers orchestration', () => {
    it('should orchestrate service call and store updates with search query', async () => {
      const searchQuery = 'test search'
      const mockResponse = {
        users: [
          { id: 'user-1', email: 'user1@example.com', display_name: 'User One', active: true, roles: ['user'] }
        ],
        total: 1
      }
      
      // Setup service mock
      mockAdminService.getUsers.mockResolvedValue(mockResponse)

      const { fetchUsers } = useAdminActions()
      const result = await fetchUsers(searchQuery)

      // Test orchestration: service called with search query
      expect(mockAdminService.getUsers).toHaveBeenCalledWith(searchQuery)
      
      // Test orchestration: store updated with users
      expect(mockAdminStore.setUsers).toHaveBeenCalledWith(mockResponse.users)
      
      // Test orchestration: result returned
      expect(result).toEqual(mockResponse)
    })

    it('should use store search query when no parameter provided', async () => {
      const mockResponse = {
        users: [],
        total: 0
      }
      
      // Setup service mock
      mockAdminService.getUsers.mockResolvedValue(mockResponse)

      const { fetchUsers } = useAdminActions()
      await fetchUsers()

      // Test orchestration: service called with store search query
      expect(mockAdminService.getUsers).toHaveBeenCalledWith(mockAdminStore.searchQuery)
      
      // Test orchestration: store updated with users
      expect(mockAdminStore.setUsers).toHaveBeenCalledWith(mockResponse.users)
    })
  })

  describe('fetchSuggestions orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const mockResponse = {
        suggestions: [
          { id: 'suggestion-1', phrase_text: 'Test suggestion 1', status: 'pending' },
          { id: 'suggestion-2', phrase_text: 'Test suggestion 2', status: 'pending' }
        ],
        total: 2
      }
      
      // Setup service mock
      mockAdminService.getSuggestions.mockResolvedValue(mockResponse)

      const { fetchSuggestions } = useAdminActions()
      const result = await fetchSuggestions()

      // Test orchestration: service called
      expect(mockAdminService.getSuggestions).toHaveBeenCalled()
      
      // Test orchestration: store updated with suggestions
      expect(mockAdminStore.setSuggestions).toHaveBeenCalledWith(mockResponse.suggestions)
      
      // Test orchestration: result returned
      expect(result).toEqual(mockResponse)
    })
  })

  describe('deactivateUser orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const userId = 'user-1'
      
      // Setup service mock
      mockAdminService.deactivateUser.mockResolvedValue(undefined)

      const { deactivateUser } = useAdminActions()
      await deactivateUser(userId)

      // Test orchestration: service called with correct user ID
      expect(mockAdminService.deactivateUser).toHaveBeenCalledWith(userId)
      
      // Test orchestration: store updated to deactivate user
      expect(mockAdminStore.updateUserActiveStatus).toHaveBeenCalledWith(userId, false)
    })
  })

  describe('activateUser orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const userId = 'user-2'
      
      // Setup service mock
      mockAdminService.activateUser.mockResolvedValue(undefined)

      const { activateUser } = useAdminActions()
      await activateUser(userId)

      // Test orchestration: service called with correct user ID
      expect(mockAdminService.activateUser).toHaveBeenCalledWith(userId)
      
      // Test orchestration: store updated to activate user
      expect(mockAdminStore.updateUserActiveStatus).toHaveBeenCalledWith(userId, true)
    })
  })

  describe('resetUserPassword orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const userId = 'user-1'
      const mockResponse = {
        new_password: 'generatedPassword123'
      }
      
      // Setup service mock
      mockAdminService.resetUserPassword.mockResolvedValue(mockResponse)

      const { resetUserPassword } = useAdminActions()
      const result = await resetUserPassword(userId)

      // Test orchestration: service called with correct user ID
      expect(mockAdminService.resetUserPassword).toHaveBeenCalledWith(userId)
      
      // Test orchestration: store updated with new password
      expect(mockAdminStore.setNewPassword).toHaveBeenCalledWith(mockResponse.new_password)
      
      // Test orchestration: result returned
      expect(result).toEqual(mockResponse)
    })
  })

  describe('promoteUser orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const userId = 'user-1'
      const user = mockAdminStore.users[0] // { id: 'user-1', roles: ['user'] }
      
      // Setup service mock
      mockAdminService.promoteUser.mockResolvedValue(undefined)

      const { promoteUser } = useAdminActions()
      await promoteUser(userId)

      // Test orchestration: service called with correct user ID
      expect(mockAdminService.promoteUser).toHaveBeenCalledWith(userId)
      
      // Test orchestration: store updated to add admin role
      expect(mockAdminStore.updateUserRoles).toHaveBeenCalledWith(userId, ['user', 'admin'])
    })

    it('should not update roles when user not found', async () => {
      const userId = 'non-existent-user'
      
      // Setup service mock
      mockAdminService.promoteUser.mockResolvedValue(undefined)

      const { promoteUser } = useAdminActions()
      await promoteUser(userId)

      // Test orchestration: service called with correct user ID
      expect(mockAdminService.promoteUser).toHaveBeenCalledWith(userId)
      
      // Test orchestration: store not updated (user not found)
      expect(mockAdminStore.updateUserRoles).not.toHaveBeenCalled()
    })
  })

  describe('approveSuggestion orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const suggestionId = 'suggestion-1'
      const adminReason = 'Great suggestion!'
      
      // Setup service mock
      mockAdminService.approveSuggestion.mockResolvedValue(undefined)

      const { approveSuggestion } = useAdminActions()
      await approveSuggestion(suggestionId, adminReason)

      // Test orchestration: service called with correct parameters
      expect(mockAdminService.approveSuggestion).toHaveBeenCalledWith(suggestionId, adminReason)
      
      // Test orchestration: store updated to remove suggestion
      expect(mockAdminStore.removeSuggestion).toHaveBeenCalledWith(suggestionId)
    })
  })

  describe('rejectSuggestion orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const suggestionId = 'suggestion-2'
      const adminReason = 'Too similar to existing phrase'
      
      // Setup service mock
      mockAdminService.rejectSuggestion.mockResolvedValue(undefined)

      const { rejectSuggestion } = useAdminActions()
      await rejectSuggestion(suggestionId, adminReason)

      // Test orchestration: service called with correct parameters
      expect(mockAdminService.rejectSuggestion).toHaveBeenCalledWith(suggestionId, adminReason)
      
      // Test orchestration: store updated to remove suggestion
      expect(mockAdminStore.removeSuggestion).toHaveBeenCalledWith(suggestionId)
    })
  })

  describe('backward compatibility aliases', () => {
    it('should provide fetchPhraseSuggestions alias for fetchSuggestions', () => {
      const { fetchPhraseSuggestions, fetchSuggestions } = useAdminActions()
      
      // Test interface: aliases are the same function
      expect(fetchPhraseSuggestions).toBe(fetchSuggestions)
    })
  })

  describe('service instantiation', () => {
    it('should create adminService with correct fetcher', async () => {
      useAdminActions()

      // Test orchestration: service created with fetcher
      const { adminService } = await import('~/services/adminService')
      expect(adminService).toHaveBeenCalledWith(expect.any(Function))
    })
  })

  describe('interface contract', () => {
    it('should expose all expected methods and state', () => {
      const actions = useAdminActions()

      // Test interface: all methods present
      expect(actions).toHaveProperty('fetchStats')
      expect(actions).toHaveProperty('fetchUsers')
      expect(actions).toHaveProperty('fetchSuggestions')
      expect(actions).toHaveProperty('deactivateUser')
      expect(actions).toHaveProperty('activateUser')
      expect(actions).toHaveProperty('resetUserPassword')
      expect(actions).toHaveProperty('promoteUser')
      expect(actions).toHaveProperty('approveSuggestion')
      expect(actions).toHaveProperty('rejectSuggestion')
      expect(actions).toHaveProperty('fetchPhraseSuggestions')
      
      // Test interface: state from useBaseService exposed
      expect(actions).toHaveProperty('isLoading')
      expect(actions).toHaveProperty('error')
      expect(actions).toHaveProperty('hasError')
      
      // Test interface: methods are functions
      expect(typeof actions.fetchStats).toBe('function')
      expect(typeof actions.fetchUsers).toBe('function')
      expect(typeof actions.fetchSuggestions).toBe('function')
      expect(typeof actions.deactivateUser).toBe('function')
      expect(typeof actions.activateUser).toBe('function')
      expect(typeof actions.resetUserPassword).toBe('function')
      expect(typeof actions.promoteUser).toBe('function')
      expect(typeof actions.approveSuggestion).toBe('function')
      expect(typeof actions.rejectSuggestion).toBe('function')
      expect(typeof actions.fetchPhraseSuggestions).toBe('function')
    })
  })
})
