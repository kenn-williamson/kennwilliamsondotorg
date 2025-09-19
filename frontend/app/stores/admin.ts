// Auto-imported: defineStore, ref, computed, readonly
import { useAdminService } from '~/composables/useAdminService'

interface User {
  id: string
  email: string
  display_name: string
  slug: string
  active: boolean
  roles: string[]
  created_at: string
}

interface PhraseSuggestion {
  id: string
  phrase_text: string
  user_display_name: string
  created_at: string
}

interface AdminStats {
  total_users: number
  active_users: number
  pending_suggestions: number
  total_phrases: number
}

export const useAdminStore = defineStore('admin', () => {
  // Destructure from service
  const { 
    getStats, 
    getUsers, 
    getSuggestions, 
    deactivateUser, 
    activateUser, 
    resetUserPassword, 
    promoteUser, 
    approveSuggestion, 
    rejectSuggestion,
    isLoading,
    error,
    hasError
  } = useAdminService()

  // State
  const users = ref<User[]>([])
  const suggestions = ref<PhraseSuggestion[]>([])
  const stats = ref<AdminStats | null>(null)
  const searchQuery = ref('')
  const selectedUser = ref<User | null>(null)
  const newPassword = ref<string | null>(null)

  // Computed properties
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

  // Actions
  const clearNewPassword = () => {
    newPassword.value = null
  }

  const setSearchQuery = (query: string) => {
    searchQuery.value = query
  }

  const setSelectedUser = (user: User | null) => {
    selectedUser.value = user
  }

  const fetchStats = async () => {
    const data = await getStats()
    stats.value = data
    return data
  }

  const fetchUsers = async (searchQueryParam?: string) => {
    const data = await getUsers(searchQueryParam || searchQuery.value)
    users.value = data.users
    return data
  }

  const fetchSuggestions = async () => {
    const data = await getSuggestions()
    suggestions.value = data.suggestions
    return data
  }

  const deactivateUserAction = async (userId: string) => {
    await deactivateUser(userId)

    // Update user in local state
    const userIndex = users.value.findIndex(user => user.id === userId)
    if (userIndex !== -1 && users.value[userIndex]) {
      users.value[userIndex].active = false
    }
  }

  const activateUserAction = async (userId: string) => {
    await activateUser(userId)

    // Update user in local state
    const userIndex = users.value.findIndex(user => user.id === userId)
    if (userIndex !== -1 && users.value[userIndex]) {
      users.value[userIndex].active = true
    }
  }

  const resetUserPasswordAction = async (userId: string) => {
    const data = await resetUserPassword(userId)
    newPassword.value = data.new_password
    return data
  }

  const promoteUserAction = async (userId: string) => {
    await promoteUser(userId)

    // Update user in local state
    const userIndex = users.value.findIndex(user => user.id === userId)
    if (userIndex !== -1 && users.value[userIndex]) {
      users.value[userIndex].roles = [...users.value[userIndex].roles, 'admin']
    }
  }

  const approveSuggestionAction = async (suggestionId: string, adminReason: string) => {
    await approveSuggestion(suggestionId, adminReason)

    // Remove suggestion from local state
    suggestions.value = suggestions.value.filter(suggestion => suggestion.id !== suggestionId)
  }

  const rejectSuggestionAction = async (suggestionId: string, adminReason: string) => {
    await rejectSuggestion(suggestionId, adminReason)

    // Remove suggestion from local state
    suggestions.value = suggestions.value.filter(suggestion => suggestion.id !== suggestionId)
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
    
    // Service state
    isLoading,
    error,
    hasError,
    
    // Computed
    filteredUsers,
    pendingSuggestions,
    
    // Actions
    fetchStats,
    fetchUsers,
    fetchSuggestions,
    deactivateUser: deactivateUserAction,
    activateUser: activateUserAction,
    resetUserPassword: resetUserPasswordAction,
    promoteUser: promoteUserAction,
    approveSuggestion: approveSuggestionAction,
    rejectSuggestion: rejectSuggestionAction,
    clearNewPassword,
    setSearchQuery,
    setSelectedUser,
    clearState
  }
})
