import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Phrase, PhraseSuggestion, PhraseWithExclusion } from '~/types/phrases'

export const usePhrasesStore = defineStore('phrases', () => {
  const baseService = useBaseService()

  // State
  const userPhrases = ref<PhraseWithExclusion[]>([])
  const adminPhrases = ref<Phrase[]>([])
  const userSuggestions = ref<PhraseSuggestion[]>([])
  const adminSuggestions = ref<PhraseSuggestion[]>([])

  // Computed
  const hasUserPhrases = computed(() => userPhrases.value.length > 0)
  const hasAdminPhrases = computed(() => adminPhrases.value.length > 0)
  const hasUserSuggestions = computed(() => userSuggestions.value.length > 0)
  const hasAdminSuggestions = computed(() => adminSuggestions.value.length > 0)

  // Actions
  const loadPhrasesForUser = () => baseService.executeRequest(
    async () => {
      const response = await baseService.backendFetch<{ phrases: PhraseWithExclusion[], total: number }>('/phrases/user')
      userPhrases.value = response.phrases
      return response
    },
    'loadPhrasesForUser'
  )

  const loadAllPhrasesForAdmin = () => baseService.executeRequest(
    async () => {
      const response = await baseService.backendFetch<{ phrases: Phrase[], total: number }>('/phrases/admin')
      adminPhrases.value = response.phrases
      return response
    },
    'loadAllPhrasesForAdmin'
  )

  const loadSuggestionsForUser = () => baseService.executeRequest(
    async () => {
      const response = await baseService.backendFetch<{ suggestions: PhraseSuggestion[], total: number }>('/phrases/suggestions')
      userSuggestions.value = response.suggestions
      return response
    },
    'loadSuggestionsForUser'
  )

  const loadAllSuggestionsForAdmin = () => baseService.executeRequest(
    async () => {
      const response = await baseService.backendFetch<{ suggestions: PhraseSuggestion[], total: number }>('/phrases/suggestions/admin')
      adminSuggestions.value = response.suggestions
      return response
    },
    'loadAllSuggestionsForAdmin'
  )

  const togglePhraseExclusion = async (phraseId: string) => {
    const phrase = userPhrases.value.find(p => p.id === phraseId)
    if (!phrase) throw new Error('Phrase not found')

    const wasExcluded = phrase.is_excluded

    try {
      if (phrase.is_excluded) {
        // Remove exclusion
        await baseService.backendFetch(`/phrases/exclude/${phraseId}`, { method: 'DELETE' })
        phrase.is_excluded = false
      } else {
        // Add exclusion
        await baseService.backendFetch(`/phrases/exclude/${phraseId}`, { method: 'POST' })
        phrase.is_excluded = true
      }
      
      // Show success message
      console.log(`Phrase ${wasExcluded ? 'included' : 'excluded'} successfully`)
    } catch (error) {
      console.error('Error toggling phrase exclusion:', error)
      throw error
    }
  }

  const submitSuggestion = (phraseText: string) => baseService.executeRequestWithSuccess(
    async () => {
      const response = await baseService.backendFetch<{ suggestion: PhraseSuggestion }>('/phrases/suggestions', {
        method: 'POST',
        body: { phrase_text: phraseText }
      })
      
      // Add to user suggestions
      userSuggestions.value.unshift(response.suggestion)
      
      return response.suggestion
    },
    'Phrase suggestion submitted successfully',
    'submitSuggestion'
  )

  const approveSuggestion = (suggestionId: string, reason?: string) => baseService.executeRequestWithSuccess(
    async () => {
      await baseService.backendFetch(`/phrases/suggestions/${suggestionId}/approve`, {
        method: 'POST',
        body: { admin_reason: reason }
      })
      
      // Update suggestion status
      const suggestion = adminSuggestions.value.find(s => s.id === suggestionId)
      if (suggestion) {
        suggestion.status = 'approved'
        suggestion.admin_reason = reason
      }
      
      return suggestion
    },
    'Suggestion approved successfully',
    'approveSuggestion'
  )

  const rejectSuggestion = (suggestionId: string, reason?: string) => baseService.executeRequestWithSuccess(
    async () => {
      await baseService.backendFetch(`/phrases/suggestions/${suggestionId}/reject`, {
        method: 'POST',
        body: { admin_reason: reason }
      })
      
      // Update suggestion status
      const suggestion = adminSuggestions.value.find(s => s.id === suggestionId)
      if (suggestion) {
        suggestion.status = 'rejected'
        suggestion.admin_reason = reason
      }
      
      return suggestion
    },
    'Suggestion rejected successfully',
    'rejectSuggestion'
  )

  // Utility functions
  const clearUserData = () => {
    userPhrases.value = []
    userSuggestions.value = []
  }

  const clearAdminData = () => {
    adminPhrases.value = []
    adminSuggestions.value = []
  }

  const clearAllData = () => {
    clearUserData()
    clearAdminData()
  }

  return {
    // State
    userPhrases: readonly(userPhrases),
    adminPhrases: readonly(adminPhrases),
    userSuggestions: readonly(userSuggestions),
    adminSuggestions: readonly(adminSuggestions),
    
    // Computed
    hasUserPhrases,
    hasAdminPhrases,
    hasUserSuggestions,
    hasAdminSuggestions,
    
    // Base service state
    isLoading: baseService.isLoading,
    error: baseService.error,
    hasError: baseService.hasError,
    isStale: baseService.isStale,
    
    // Actions
    loadPhrasesForUser,
    loadAllPhrasesForAdmin,
    loadSuggestionsForUser,
    loadAllSuggestionsForAdmin,
    togglePhraseExclusion,
    submitSuggestion,
    approveSuggestion,
    rejectSuggestion,
    
    // Utilities
    clearUserData,
    clearAdminData,
    clearAllData
  }
})
