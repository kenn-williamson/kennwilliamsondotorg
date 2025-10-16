// @ts-nocheck
import { setActivePinia, createPinia } from 'pinia'
import { useAdminStore } from './admin'
import { adminService } from '~/services/adminService'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import type { User, AdminStats, UsersResponse, SuggestionsResponse, AdminResetPasswordResponse } from '#shared/types'
import type { PhraseSuggestion } from '#shared/types/phrases'

// Create a shared mock service instance that will be used by all stores
const mockAdminServiceInstance = {
  getStats: vi.fn(),
  getUsers: vi.fn(),
  getSuggestions: vi.fn(),
  deactivateUser: vi.fn(),
  activateUser: vi.fn(),
  resetUserPassword: vi.fn(),
  promoteUser: vi.fn(),
  approveSuggestion: vi.fn(),
  rejectSuggestion: vi.fn(),
}

// Mock the service layer to return the shared mock instance
vi.mock('~/services/adminService', () => ({
  adminService: vi.fn(() => mockAdminServiceInstance),
}))

// Mock useSmartFetch
vi.mock('~/composables/useSmartFetch', () => ({
  useSmartFetch: vi.fn(() => vi.fn()),
}))

describe('Admin Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  describe('fetchStats', () => {
    it('should call service and update state on success', async () => {
      // Arrange
      const store = useAdminStore()
      const mockStats = {
        total_users: 25,
        active_users: 23,
        pending_suggestions: 3,
        total_phrases: 15
      }
      
      vi.mocked(mockAdminServiceInstance.getStats).mockResolvedValue(mockStats)

      // Act
      await store.fetchStats()

      // Assert
      expect(mockAdminServiceInstance.getStats).toHaveBeenCalled()
      expect(store.stats).toEqual(mockStats)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })

    it('should set error state on failure', async () => {
      // Arrange
      const store = useAdminStore()
      const mockError = new Error('API Failure')
      
      vi.mocked(mockAdminServiceInstance.getStats).mockRejectedValue(mockError)

      // Act
      await store.fetchStats()

      // Assert
      expect(store.error).toEqual('API Failure')
      expect(store.stats).toBe(null)
      expect(store.isLoading).toBe(false)
    })
  })

  describe('fetchUsers', () => {
    it('should call service and update state on success', async () => {
      // Arrange
      const store = useAdminStore()
      const mockUsers: User[] = [
        { id: '1', display_name: 'John Doe', email: 'john@example.com', slug: 'john-doe', roles: ['user'], created_at: '2024-01-01T00:00:00Z', active: true }
      ]
      const mockResponse: UsersResponse = mockUsers
      
      vi.mocked(mockAdminServiceInstance.getUsers).mockResolvedValue(mockResponse)

      // Act
      await store.fetchUsers('john')

      // Assert
      expect(mockAdminServiceInstance.getUsers).toHaveBeenCalledWith('john')
      expect(store.users).toEqual(mockUsers)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })

    it('should use searchQuery when no parameter provided', async () => {
      // Arrange
      const store = useAdminStore()
      store.setSearchQuery('test query')
      const mockUsers: User[] = []
      const mockResponse: UsersResponse = mockUsers
      
      vi.mocked(mockAdminServiceInstance.getUsers).mockResolvedValue(mockResponse)

      // Act
      await store.fetchUsers()

      // Assert
      expect(mockAdminServiceInstance.getUsers).toHaveBeenCalledWith('test query')
    })
  })

  describe('fetchSuggestions', () => {
    it('should call service and update state on success', async () => {
      // Arrange
      const store = useAdminStore()
      const mockSuggestions: PhraseSuggestion[] = [
        { id: '1', user_id: '1', phrase_text: 'Test phrase', status: 'pending', admin_id: undefined, admin_reason: undefined, created_at: '2024-01-01T00:00:00Z', updated_at: '2024-01-01T00:00:00Z', user_display_name: 'John Doe' }
      ]
      const mockResponse: SuggestionsResponse = { suggestions: mockSuggestions, total: 1 }
      
      vi.mocked(mockAdminServiceInstance.getSuggestions).mockResolvedValue(mockResponse)

      // Act
      await store.fetchSuggestions()

      // Assert
      expect(mockAdminServiceInstance.getSuggestions).toHaveBeenCalled()
      expect(store.suggestions).toEqual(mockSuggestions)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })
  })

  describe('deactivateUser', () => {
    it('should call service and update local state on success', async () => {
      // Arrange
      const store = useAdminStore()
      const mockUser: User = { id: '1', display_name: 'John Doe', email: 'john@example.com', slug: 'john-doe', roles: ['user'], created_at: '2024-01-01T00:00:00Z', active: true }
      store.setUsers([mockUser])
      
      vi.mocked(mockAdminServiceInstance.deactivateUser).mockResolvedValue({ message: 'User deactivated successfully' })

      // Act
      await store.deactivateUser('1')

      // Assert
      expect(mockAdminServiceInstance.deactivateUser).toHaveBeenCalledWith('1')
      expect(store.users[0]?.active).toBe(false)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })
  })

  describe('activateUser', () => {
    it('should call service and update local state on success', async () => {
      // Arrange
      const store = useAdminStore()
      const mockUser: User = { id: '1', display_name: 'John Doe', email: 'john@example.com', slug: 'john-doe', roles: ['user'], created_at: '2024-01-01T00:00:00Z', active: false }
      store.setUsers([mockUser])
      
      vi.mocked(mockAdminServiceInstance.activateUser).mockResolvedValue({ message: 'User activated successfully' })

      // Act
      await store.activateUser('1')

      // Assert
      expect(mockAdminServiceInstance.activateUser).toHaveBeenCalledWith('1')
      expect(store.users[0]?.active).toBe(true)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })
  })

  describe('resetUserPassword', () => {
    it('should call service and update newPassword state on success', async () => {
      // Arrange
      const store = useAdminStore()
      const mockResponse: AdminResetPasswordResponse = { new_password: 'generatedPassword123', message: 'Password reset successfully' }
      
      vi.mocked(mockAdminServiceInstance.resetUserPassword).mockResolvedValue(mockResponse)

      // Act
      await store.resetUserPassword('1')

      // Assert
      expect(mockAdminServiceInstance.resetUserPassword).toHaveBeenCalledWith('1')
      expect(store.newPassword).toBe('generatedPassword123')
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })
  })

  describe('promoteUser', () => {
    it('should call service and update user roles on success', async () => {
      // Arrange
      const store = useAdminStore()
      const mockUser: User = { id: '1', display_name: 'John Doe', email: 'john@example.com', slug: 'john-doe', roles: ['user'], created_at: '2024-01-01T00:00:00Z', active: true }
      store.setUsers([mockUser])
      
      vi.mocked(mockAdminServiceInstance.promoteUser).mockResolvedValue({ message: 'User promoted to admin successfully' })

      // Act
      await store.promoteUser('1')

      // Assert
      expect(mockAdminServiceInstance.promoteUser).toHaveBeenCalledWith('1')
      expect(store.users[0]?.roles).toEqual(['user', 'admin'])
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })
  })

  describe('approveSuggestion', () => {
    it('should call service and remove suggestion from local state on success', async () => {
      // Arrange
      const store = useAdminStore()
      const mockSuggestion: PhraseSuggestion = { id: '1', user_id: '1', phrase_text: 'Test phrase', status: 'pending', admin_id: undefined, admin_reason: undefined, created_at: '2024-01-01T00:00:00Z', updated_at: '2024-01-01T00:00:00Z', user_display_name: 'John Doe' }
      store.setSuggestions([mockSuggestion])
      
      vi.mocked(mockAdminServiceInstance.approveSuggestion).mockResolvedValue({ message: 'Suggestion approved successfully' })

      // Act
      await store.approveSuggestion('1', 'Great suggestion!')

      // Assert
      expect(mockAdminServiceInstance.approveSuggestion).toHaveBeenCalledWith('1', 'Great suggestion!')
      expect(store.suggestions).toEqual([])
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })
  })

  describe('rejectSuggestion', () => {
    it('should call service and remove suggestion from local state on success', async () => {
      // Arrange
      const store = useAdminStore()
      const mockSuggestion: PhraseSuggestion = { id: '1', user_id: '1', phrase_text: 'Test phrase', status: 'pending', admin_id: undefined, admin_reason: undefined, created_at: '2024-01-01T00:00:00Z', updated_at: '2024-01-01T00:00:00Z', user_display_name: 'John Doe' }
      store.setSuggestions([mockSuggestion])
      
      vi.mocked(mockAdminServiceInstance.rejectSuggestion).mockResolvedValue({ message: 'Suggestion rejected successfully' })

      // Act
      await store.rejectSuggestion('1', 'Too similar to existing content')

      // Assert
      expect(mockAdminServiceInstance.rejectSuggestion).toHaveBeenCalledWith('1', 'Too similar to existing content')
      expect(store.suggestions).toEqual([])
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })
  })

  describe('utility actions', () => {
    it('should set search query', () => {
      const store = useAdminStore()
      store.setSearchQuery('test query')
      expect(store.searchQuery).toBe('test query')
    })

    it('should set selected user', () => {
      const store = useAdminStore()
      const mockUser = { id: '1', display_name: 'John Doe', email: 'john@example.com', slug: 'john-doe', roles: ['user'], created_at: '2024-01-01T00:00:00Z', active: true }
      store.setSelectedUser(mockUser)
      expect(store.selectedUser).toEqual(mockUser)
    })

    it('should set active tab', () => {
      const store = useAdminStore()
      expect(store.activeTab).toBe('overview') // default
      store.setActiveTab('users')
      expect(store.activeTab).toBe('users')
      store.setActiveTab('suggestions')
      expect(store.activeTab).toBe('suggestions')
    })

    it('should clear new password', () => {
      const store = useAdminStore()
      store.setNewPassword('test password')
      store.clearNewPassword()
      expect(store.newPassword).toBe(null)
    })

    it('should clear all state', () => {
      const store = useAdminStore()
      store.setUsers([{ id: '1', display_name: 'John Doe', email: 'john@example.com', slug: 'john-doe', roles: ['user'], created_at: '2024-01-01T00:00:00Z', active: true }])
      store.setSuggestions([{ id: '1', user_id: '1', phrase_text: 'Test phrase', status: 'pending', admin_id: undefined, admin_reason: undefined, created_at: '2024-01-01T00:00:00Z', updated_at: '2024-01-01T00:00:00Z', user_display_name: 'John Doe' }])
      store.setStats({ total_users: 1, active_users: 1, pending_suggestions: 1, total_phrases: 1 })
      store.setSearchQuery('test')
      store.setSelectedUser({ id: '1', display_name: 'John Doe', email: 'john@example.com', slug: 'john-doe', roles: ['user'], created_at: '2024-01-01T00:00:00Z', active: true })
      store.setNewPassword('test password')
      
      store.clearState()
      
      expect(store.users).toEqual([])
      expect(store.suggestions).toEqual([])
      expect(store.stats).toBe(null)
      expect(store.searchQuery).toBe('')
      expect(store.selectedUser).toBe(null)
      expect(store.newPassword).toBe(null)
      expect(store.error).toBe(null)
    })
  })

  describe('computed properties', () => {
    it('should update search query when searchUsers is called', async () => {
      const store = useAdminStore()
      
      store.searchUsers('john')
      
      expect(store.searchQuery).toBe('john')
    })

    it('should clear search query when empty string provided', async () => {
      const store = useAdminStore()
      
      store.searchUsers('')
      
      expect(store.searchQuery).toBe('')
    })

    it('should return pending suggestions', () => {
      const store = useAdminStore()
      const mockSuggestions: PhraseSuggestion[] = [
        { id: '1', user_id: '1', phrase_text: 'Test phrase', status: 'pending', admin_id: undefined, admin_reason: undefined, created_at: '2024-01-01T00:00:00Z', updated_at: '2024-01-01T00:00:00Z', user_display_name: 'John Doe' }
      ]
      store.setSuggestions(mockSuggestions)
      
      expect(store.pendingSuggestions).toEqual(mockSuggestions)
    })

    it('should compute hasError correctly', () => {
      const store = useAdminStore()
      expect(store.hasError).toBe(false)
      
      store.setError('Test error')
      expect(store.hasError).toBe(true)
    })
  })
})
