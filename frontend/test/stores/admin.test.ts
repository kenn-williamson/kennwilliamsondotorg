import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { 
  createMockUser, 
  createMockPhraseSuggestion 
} from '../utils/test-helpers'

// Import the store (composables are mocked globally in setup.ts)
import { useAdminStore } from '~/stores/admin'

describe('useAdminStore', () => {
  beforeEach(() => {
    // Create a fresh pinia and make it active
    setActivePinia(createPinia())
    
    // Reset all mocks before each test
    vi.clearAllMocks()
  })

  describe('initial state', () => {
    it('should initialize with empty state', () => {
      const store = useAdminStore()
      
      expect(store.users).toEqual([])
      expect(store.suggestions).toEqual([])
      expect(store.stats).toBeNull()
      expect(store.searchQuery).toBe('')
      expect(store.selectedUser).toBeNull()
      expect(store.newPassword).toBeNull()
    })
  })

  describe('computed properties', () => {
    it('should filter users based on search query', () => {
      const store = useAdminStore()
      const users = [
        createMockUser({ display_name: 'John Doe', email: 'john@example.com' }),
        createMockUser({ display_name: 'Jane Smith', email: 'jane@example.com' }),
        createMockUser({ display_name: 'Bob Johnson', email: 'bob@example.com' })
      ]
      // Directly set the users array since it's readonly
      store.users.splice(0, store.users.length, ...users)
      
      // No search query - should return all users
      expect(store.filteredUsers).toEqual(users)
      
      // Search by display name
      store.setSearchQuery('John')
      expect(store.filteredUsers).toHaveLength(2)
      expect(store.filteredUsers).toContain(users[0]) // John Doe
      expect(store.filteredUsers).toContain(users[2]) // Bob Johnson
      
      // Search by email
      store.setSearchQuery('jane@example.com')
      expect(store.filteredUsers).toHaveLength(1)
      expect(store.filteredUsers).toContain(users[1]) // Jane Smith
      
      // Case insensitive search
      store.setSearchQuery('JOHN')
      expect(store.filteredUsers).toHaveLength(2)
      
      // Empty search query
      store.setSearchQuery('')
      expect(store.filteredUsers).toEqual(users)
    })

    it('should return all suggestions as pending suggestions', () => {
      const store = useAdminStore()
      const suggestions = [
        createMockPhraseSuggestion({ status: 'pending' }),
        createMockPhraseSuggestion({ id: '2', status: 'pending' })
      ]
      // Directly set the suggestions array since it's readonly
      store.suggestions.splice(0, store.suggestions.length, ...suggestions)
      
      expect(store.pendingSuggestions).toEqual(suggestions)
    })
  })

  describe('setSearchQuery', () => {
    it('should update search query', () => {
      const store = useAdminStore()
      
      store.setSearchQuery('test query')
      expect(store.searchQuery).toBe('test query')
    })
  })

  describe('setSelectedUser', () => {
    it('should update selected user', () => {
      const store = useAdminStore()
      const user = createMockUser()
      
      store.setSelectedUser(user)
      expect(store.selectedUser).toEqual(user)
      
      store.setSelectedUser(null)
      expect(store.selectedUser).toBeNull()
    })
  })

  describe('clearNewPassword', () => {
    it('should clear new password', () => {
      const store = useAdminStore()
      store.newPassword = 'test-password'
      
      store.clearNewPassword()
      expect(store.newPassword).toBeNull()
    })
  })

  describe('fetchStats', () => {
    it('should fetch stats and update state', async () => {
      const store = useAdminStore()
      const mockStats = {
        total_users: 25,
        active_users: 23,
        pending_suggestions: 3,
        total_phrases: 15
      }
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.getStats.mockResolvedValue(mockStats)
      
      const result = await store.fetchStats()
      
      expect(mockService.getStats).toHaveBeenCalled()
      expect(store.stats).toEqual(mockStats)
      expect(result).toEqual(mockStats)
    })
  })

  describe('fetchUsers', () => {
    it('should fetch users and update state', async () => {
      const store = useAdminStore()
      const mockUsers = [createMockUser(), createMockUser({ id: '2' })]
      const mockResponse = { users: mockUsers, total: 2 }
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.getUsers.mockResolvedValue(mockResponse)
      
      const result = await store.fetchUsers()
      
      expect(mockService.getUsers).toHaveBeenCalledWith('')
      expect(store.users).toEqual(mockUsers)
      expect(result).toEqual(mockResponse)
    })

    it('should use provided search query parameter', async () => {
      const store = useAdminStore()
      const mockUsers = [createMockUser()]
      const mockResponse = { users: mockUsers, total: 1 }
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.getUsers.mockResolvedValue(mockResponse)
      
      await store.fetchUsers('test search')
      
      expect(mockService.getUsers).toHaveBeenCalledWith('test search')
    })
  })

  describe('fetchSuggestions', () => {
    it('should fetch suggestions and update state', async () => {
      const store = useAdminStore()
      const mockSuggestions = [createMockPhraseSuggestion(), createMockPhraseSuggestion({ id: '2' })]
      const mockResponse = { suggestions: mockSuggestions, total: 2 }
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.getSuggestions.mockResolvedValue(mockResponse)
      
      const result = await store.fetchSuggestions()
      
      expect(mockService.getSuggestions).toHaveBeenCalled()
      expect(store.suggestions).toEqual(mockSuggestions)
      expect(result).toEqual(mockResponse)
    })
  })

  describe('deactivateUser', () => {
    it('should deactivate user and update local state', async () => {
      const store = useAdminStore()
      const user = createMockUser({ active: true })
      store.users.splice(0, store.users.length, user)
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.deactivateUser.mockResolvedValue({ message: 'User deactivated successfully' })
      
      await store.deactivateUser(user.id)
      
      expect(mockService.deactivateUser).toHaveBeenCalledWith(user.id)
      expect(user.active).toBe(false)
    })

    it('should handle user not found in local state', async () => {
      const store = useAdminStore()
      store.users = []
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.deactivateUser.mockResolvedValue({ message: 'User deactivated successfully' })
      
      // Should not throw error even if user not found in local state
      await expect(store.deactivateUser('nonexistent-id')).resolves.toBeUndefined()
    })
  })

  describe('activateUser', () => {
    it('should activate user and update local state', async () => {
      const store = useAdminStore()
      const user = createMockUser({ active: false })
      store.users.splice(0, store.users.length, user)
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.activateUser.mockResolvedValue({ message: 'User activated successfully' })
      
      await store.activateUser(user.id)
      
      expect(mockService.activateUser).toHaveBeenCalledWith(user.id)
      expect(user.active).toBe(true)
    })
  })

  describe('resetUserPassword', () => {
    it('should reset user password and store new password', async () => {
      const store = useAdminStore()
      const mockResponse = { new_password: 'newPassword123' }
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.resetUserPassword.mockResolvedValue(mockResponse)
      
      const result = await store.resetUserPassword('user-id')
      
      expect(mockService.resetUserPassword).toHaveBeenCalledWith('user-id')
      expect(store.newPassword).toBe('newPassword123')
      expect(result).toEqual(mockResponse)
    })
  })

  describe('promoteUser', () => {
    it('should promote user and update local state', async () => {
      const store = useAdminStore()
      const user = createMockUser({ roles: ['user'] })
      store.users.splice(0, store.users.length, user)
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.promoteUser.mockResolvedValue({ message: 'User promoted to admin successfully' })
      
      await store.promoteUser(user.id)
      
      expect(mockService.promoteUser).toHaveBeenCalledWith(user.id)
      expect(user.roles).toContain('admin')
    })
  })

  describe('approveSuggestion', () => {
    it('should approve suggestion and remove from local state', async () => {
      const store = useAdminStore()
      const suggestion = createMockPhraseSuggestion()
      store.suggestions.splice(0, store.suggestions.length, suggestion)
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.approveSuggestion.mockResolvedValue({ message: 'Suggestion approved successfully' })
      
      await store.approveSuggestion(suggestion.id, 'Great suggestion!')
      
      expect(mockService.approveSuggestion).toHaveBeenCalledWith(suggestion.id, 'Great suggestion!')
      expect(store.suggestions).not.toContain(suggestion)
    })
  })

  describe('rejectSuggestion', () => {
    it('should reject suggestion and remove from local state', async () => {
      const store = useAdminStore()
      const suggestion = createMockPhraseSuggestion()
      store.suggestions.splice(0, store.suggestions.length, suggestion)
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.rejectSuggestion.mockResolvedValue({ message: 'Suggestion rejected successfully' })
      
      await store.rejectSuggestion(suggestion.id, 'Too similar to existing content')
      
      expect(mockService.rejectSuggestion).toHaveBeenCalledWith(suggestion.id, 'Too similar to existing content')
      expect(store.suggestions).not.toContain(suggestion)
    })
  })

  describe('clearState', () => {
    it('should clear all state', () => {
      const store = useAdminStore()
      store.users.splice(0, store.users.length, createMockUser())
      store.suggestions.splice(0, store.suggestions.length, createMockPhraseSuggestion())
      store.stats = { total_users: 10, active_users: 8, pending_suggestions: 2, total_phrases: 5 }
      store.searchQuery = 'test'
      store.selectedUser = createMockUser()
      store.newPassword = 'password123'
      
      store.clearState()
      
      expect(store.users).toEqual([])
      expect(store.suggestions).toEqual([])
      expect(store.stats).toBeNull()
      expect(store.searchQuery).toBe('')
      expect(store.selectedUser).toBeNull()
      expect(store.newPassword).toBeNull()
    })
  })
})
