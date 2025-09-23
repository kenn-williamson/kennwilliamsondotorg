import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { createMockUser, createMockPhraseSuggestion, createMockAdminStats } from '../utils/test-helpers'

// Import the store directly - no mocking needed for pure stores
import { useAdminStore } from '../../app/stores/admin'

describe('useAdminStore', () => {
  beforeEach(() => {
    // Create a fresh pinia and make it active
    setActivePinia(createPinia())
  })

  describe('store state', () => {
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
    it('should return all users when search query is empty', () => {
      const store = useAdminStore()
      const users = [createMockUser({ id: 'user1' }), createMockUser({ id: 'user2' })]
      store.setUsers(users)
      
      expect(store.filteredUsers).toEqual(users)
    })

    it('should filter users by display name', () => {
      const store = useAdminStore()
      const users = [
        createMockUser({ id: 'user1', display_name: 'John Doe' }),
        createMockUser({ id: 'user2', display_name: 'Jane Smith' }),
        createMockUser({ id: 'user3', display_name: 'Bob Johnson' })
      ]
      store.setUsers(users)
      store.setSearchQuery('John')
      
      expect(store.filteredUsers).toHaveLength(2)
      expect(store.filteredUsers[0].display_name).toBe('John Doe')
      expect(store.filteredUsers[1].display_name).toBe('Bob Johnson')
    })

    it('should filter users by email', () => {
      const store = useAdminStore()
      const users = [
        createMockUser({ id: 'user1', email: 'john@example.com' }),
        createMockUser({ id: 'user2', email: 'jane@test.com' }),
        createMockUser({ id: 'user3', email: 'bob@example.org' })
      ]
      store.setUsers(users)
      store.setSearchQuery('example')
      
      expect(store.filteredUsers).toHaveLength(2)
      expect(store.filteredUsers[0].email).toBe('john@example.com')
      expect(store.filteredUsers[1].email).toBe('bob@example.org')
    })

    it('should be case insensitive when filtering', () => {
      const store = useAdminStore()
      const users = [
        createMockUser({ id: 'user1', display_name: 'John Doe' }),
        createMockUser({ id: 'user2', display_name: 'JANE SMITH' }),
        createMockUser({ id: 'user3', display_name: 'bob johnson' })
      ]
      store.setUsers(users)
      store.setSearchQuery('JOHN')
      
      expect(store.filteredUsers).toHaveLength(2)
      expect(store.filteredUsers[0].display_name).toBe('John Doe')
      expect(store.filteredUsers[1].display_name).toBe('bob johnson')
    })

    it('should return empty array when no users match search', () => {
      const store = useAdminStore()
      const users = [createMockUser({ display_name: 'John Doe' })]
      store.setUsers(users)
      store.setSearchQuery('NonExistent')
      
      expect(store.filteredUsers).toEqual([])
    })

    it('should return all suggestions as pending suggestions', () => {
      const store = useAdminStore()
      const suggestions = [
        createMockPhraseSuggestion({ id: 'suggestion1' }),
        createMockPhraseSuggestion({ id: 'suggestion2' })
      ]
      store.setSuggestions(suggestions)
      
      expect(store.pendingSuggestions).toEqual(suggestions)
    })
  })

  describe('pure state management functions', () => {
    it('should set users correctly', () => {
      const store = useAdminStore()
      const users = [createMockUser({ id: 'user1' }), createMockUser({ id: 'user2' })]
      
      store.setUsers(users)
      
      expect(store.users).toEqual(users)
    })

    it('should set suggestions correctly', () => {
      const store = useAdminStore()
      const suggestions = [
        createMockPhraseSuggestion({ id: 'suggestion1' }),
        createMockPhraseSuggestion({ id: 'suggestion2' })
      ]
      
      store.setSuggestions(suggestions)
      
      expect(store.suggestions).toEqual(suggestions)
    })

    it('should set stats correctly', () => {
      const store = useAdminStore()
      const stats = createMockAdminStats()
      
      store.setStats(stats)
      
      expect(store.stats).toEqual(stats)
    })

    it('should set search query correctly', () => {
      const store = useAdminStore()
      
      store.setSearchQuery('test query')
      
      expect(store.searchQuery).toBe('test query')
    })

    it('should set selected user correctly', () => {
      const store = useAdminStore()
      const user = createMockUser({ id: 'selected' })
      
      store.setSelectedUser(user)
      
      expect(store.selectedUser).toEqual(user)
    })

    it('should set selected user to null', () => {
      const store = useAdminStore()
      const user = createMockUser({ id: 'selected' })
      store.setSelectedUser(user)
      
      store.setSelectedUser(null)
      
      expect(store.selectedUser).toBeNull()
    })

    it('should set new password correctly', () => {
      const store = useAdminStore()
      
      store.setNewPassword('newPassword123')
      
      expect(store.newPassword).toBe('newPassword123')
    })

    it('should set new password to null', () => {
      const store = useAdminStore()
      store.setNewPassword('newPassword123')
      
      store.setNewPassword(null)
      
      expect(store.newPassword).toBeNull()
    })

    it('should update user active status correctly', () => {
      const store = useAdminStore()
      const user = createMockUser({ id: 'user1', active: true })
      store.setUsers([user])
      
      store.updateUserActiveStatus('user1', false)
      
      expect(store.users[0].active).toBe(false)
    })

    it('should not update non-existent user active status', () => {
      const store = useAdminStore()
      const user = createMockUser({ id: 'user1', active: true })
      store.setUsers([user])
      
      store.updateUserActiveStatus('non-existent', false)
      
      expect(store.users[0].active).toBe(true)
    })

    it('should update user roles correctly', () => {
      const store = useAdminStore()
      const user = createMockUser({ id: 'user1', roles: ['user'] })
      store.setUsers([user])
      
      store.updateUserRoles('user1', ['user', 'admin'])
      
      expect(store.users[0].roles).toEqual(['user', 'admin'])
    })

    it('should not update non-existent user roles', () => {
      const store = useAdminStore()
      const user = createMockUser({ id: 'user1', roles: ['user'] })
      store.setUsers([user])
      
      store.updateUserRoles('non-existent', ['admin'])
      
      expect(store.users[0].roles).toEqual(['user'])
    })

    it('should remove suggestion correctly', () => {
      const store = useAdminStore()
      const suggestion1 = createMockPhraseSuggestion({ id: 'suggestion1' })
      const suggestion2 = createMockPhraseSuggestion({ id: 'suggestion2' })
      store.setSuggestions([suggestion1, suggestion2])
      
      store.removeSuggestion('suggestion1')
      
      expect(store.suggestions).toHaveLength(1)
      expect(store.suggestions[0]).toEqual(suggestion2)
    })

    it('should not remove non-existent suggestion', () => {
      const store = useAdminStore()
      const suggestion = createMockPhraseSuggestion({ id: 'suggestion1' })
      store.setSuggestions([suggestion])
      
      store.removeSuggestion('non-existent')
      
      expect(store.suggestions).toHaveLength(1)
      expect(store.suggestions[0]).toEqual(suggestion)
    })

    it('should clear new password correctly', () => {
      const store = useAdminStore()
      store.setNewPassword('password123')
      
      store.clearNewPassword()
      
      expect(store.newPassword).toBeNull()
    })

    it('should clear all state correctly', () => {
      const store = useAdminStore()
      const users = [createMockUser({ id: 'user1' })]
      const suggestions = [createMockPhraseSuggestion({ id: 'suggestion1' })]
      const stats = createMockAdminStats()
      
      store.setUsers(users)
      store.setSuggestions(suggestions)
      store.setStats(stats)
      store.setSearchQuery('test')
      store.setSelectedUser(users[0])
      store.setNewPassword('password123')
      
      store.clearState()
      
      expect(store.users).toEqual([])
      expect(store.suggestions).toEqual([])
      expect(store.stats).toBeNull()
      expect(store.searchQuery).toBe('')
      expect(store.selectedUser).toBeNull()
      expect(store.newPassword).toBeNull()
    })
  })

  describe('edge cases', () => {
    it('should handle empty search query with whitespace', () => {
      const store = useAdminStore()
      const users = [createMockUser({ display_name: 'John Doe' })]
      store.setUsers(users)
      store.setSearchQuery('   ')
      
      expect(store.filteredUsers).toEqual(users)
    })

    it('should handle partial matches in search', () => {
      const store = useAdminStore()
      const users = [
        createMockUser({ display_name: 'Johnny Doe' }),
        createMockUser({ display_name: 'John Smith' }),
        createMockUser({ display_name: 'Bob Wilson' }) // Changed from Johnson to avoid matching
      ]
      store.setUsers(users)
      store.setSearchQuery('John')
      
      expect(store.filteredUsers).toHaveLength(2)
    })

    it('should handle special characters in search', () => {
      const store = useAdminStore()
      const users = [
        createMockUser({ display_name: 'John-Doe' }),
        createMockUser({ display_name: 'John_Doe' }),
        createMockUser({ display_name: 'John.Doe' })
      ]
      store.setUsers(users)
      store.setSearchQuery('John')
      
      expect(store.filteredUsers).toHaveLength(3)
    })

    it('should handle updating user that appears multiple times', () => {
      const store = useAdminStore()
      const user1 = createMockUser({ id: 'user1', active: true })
      const user2 = createMockUser({ id: 'user1', active: true }) // Same ID
      store.setUsers([user1, user2])
      
      store.updateUserActiveStatus('user1', false)
      
      // Should update the first occurrence
      expect(store.users[0].active).toBe(false)
      expect(store.users[1].active).toBe(true)
    })
  })
})