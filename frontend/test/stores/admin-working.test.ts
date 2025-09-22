import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { 
  createMockUser, 
  createMockPhraseSuggestion 
} from '../utils/test-helpers'

// Mock the composable before importing the store
vi.mock('~/composables/useAdminService', () => ({
  useAdminService: () => ({
    getStats: vi.fn().mockResolvedValue({ total_users: 10, active_users: 8, pending_suggestions: 2, total_phrases: 5 }),
    getUsers: vi.fn().mockResolvedValue({ users: [createMockUser()], total: 1 }),
    getSuggestions: vi.fn().mockResolvedValue({ suggestions: [createMockPhraseSuggestion()], total: 1 }),
    deactivateUser: vi.fn().mockResolvedValue({ message: 'User deactivated successfully' }),
    activateUser: vi.fn().mockResolvedValue({ message: 'User activated successfully' }),
    resetUserPassword: vi.fn().mockResolvedValue({ new_password: 'newPassword123' }),
    promoteUser: vi.fn().mockResolvedValue({ message: 'User promoted to admin successfully' }),
    approveSuggestion: vi.fn().mockResolvedValue({ message: 'Suggestion approved successfully' }),
    rejectSuggestion: vi.fn().mockResolvedValue({ message: 'Suggestion rejected successfully' }),
    isLoading: { value: false },
    error: { value: null },
    hasError: { value: false },
  })
}))

// Import the store after mocking
import { useAdminStore } from '~/stores/admin'

describe('useAdminStore - Working Tests', () => {
  beforeEach(() => {
    // Create a fresh pinia and make it active
    setActivePinia(createPinia())
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

  describe('fetchStats', () => {
    it('should fetch stats and update state', async () => {
      const store = useAdminStore()
      const mockStats = {
        total_users: 25,
        active_users: 23,
        pending_suggestions: 3,
        total_phrases: 15
      }
      
      // Mock the service to return our test data
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
      
      // Mock the service to return our test data
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
      
      // Mock the service to return our test data
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
      
      // Mock the service to return our test data
      const mockService = global.useAdminService()
      mockService.getSuggestions.mockResolvedValue(mockResponse)
      
      const result = await store.fetchSuggestions()
      
      expect(mockService.getSuggestions).toHaveBeenCalled()
      expect(store.suggestions).toEqual(mockSuggestions)
      expect(result).toEqual(mockResponse)
    })
  })

  describe('computed properties', () => {
    it('should filter users based on search query', async () => {
      const store = useAdminStore()
      const users = [
        createMockUser({ display_name: 'John Doe', email: 'john@example.com' }),
        createMockUser({ display_name: 'Jane Smith', email: 'jane@example.com' }),
        createMockUser({ display_name: 'Bob Johnson', email: 'bob@example.com' })
      ]
      
      // Load users first
      const mockService = global.useAdminService()
      mockService.getUsers.mockResolvedValue({ users, total: 3 })
      await store.fetchUsers()
      
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

    it('should return all suggestions as pending suggestions', async () => {
      const store = useAdminStore()
      const suggestions = [
        createMockPhraseSuggestion({ status: 'pending' }),
        createMockPhraseSuggestion({ id: '2', status: 'pending' })
      ]
      
      // Load suggestions first
      const mockService = global.useAdminService()
      mockService.getSuggestions.mockResolvedValue({ suggestions, total: 2 })
      await store.fetchSuggestions()
      
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
