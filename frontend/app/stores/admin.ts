/**
 * Pure Admin Store - Only state management, no service calls
 * Refactored to follow proper separation of concerns
 */

import type { User, AdminStats } from '#shared/types'
import type { PhraseSuggestion } from '#shared/types/phrases'

export const useAdminStore = defineStore('admin', () => {
  // State
  const users = ref<User[]>([])
  const suggestions = ref<PhraseSuggestion[]>([])
  const stats = ref<AdminStats | null>(null)
  const searchQuery = ref('')
  const selectedUser = ref<User | null>(null)
  const newPassword = ref<string | null>(null)

  // Computed
  const filteredUsers = computed((): User[] => {
    if (!searchQuery.value.trim()) return users.value
    
    const query = searchQuery.value.toLowerCase()
    return users.value.filter(user => 
      user.display_name.toLowerCase().includes(query) ||
      user.email.toLowerCase().includes(query)
    )
  })

  const pendingSuggestions = computed((): PhraseSuggestion[] => {
    return suggestions.value
  })

  // Pure state management functions
  const setUsers = (usersList: User[]) => {
    users.value = usersList
  }

  const setSuggestions = (suggestionsList: PhraseSuggestion[]) => {
    suggestions.value = suggestionsList
  }

  const setStats = (statsData: AdminStats) => {
    stats.value = statsData
  }

  const setSearchQuery = (query: string) => {
    searchQuery.value = query
  }

  const setSelectedUser = (user: User | null) => {
    selectedUser.value = user
  }

  const setNewPassword = (password: string | null) => {
    newPassword.value = password
  }

  const updateUserActiveStatus = (userId: string, active: boolean) => {
    const user = users.value.find(u => u.id === userId)
    if (user) {
      user.active = active
    }
  }

  const updateUserRoles = (userId: string, roles: string[]) => {
    const user = users.value.find(u => u.id === userId)
    if (user) {
      user.roles = roles
    }
  }

  const removeSuggestion = (suggestionId: string) => {
    suggestions.value = suggestions.value.filter(s => s.id !== suggestionId)
  }

  const clearNewPassword = () => {
    newPassword.value = null
  }

  const clearState = () => {
    users.value = []
    suggestions.value = []
    stats.value = null
    searchQuery.value = ''
    selectedUser.value = null
    newPassword.value = null
  }

  return {
    // State
    users: readonly(users),
    suggestions: readonly(suggestions),
    stats: readonly(stats),
    searchQuery: readonly(searchQuery),
    selectedUser: readonly(selectedUser),
    newPassword: readonly(newPassword),
    
    // Computed
    filteredUsers,
    pendingSuggestions,
    
    // Actions
    setUsers,
    setSuggestions,
    setStats,
    setSearchQuery,
    setSelectedUser,
    setNewPassword,
    updateUserActiveStatus,
    updateUserRoles,
    removeSuggestion,
    clearNewPassword,
    clearState
  }
})
