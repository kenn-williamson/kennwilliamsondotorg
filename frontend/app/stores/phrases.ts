import type { Phrase, PhraseSuggestion, PhraseWithExclusion } from '#shared/types/phrases'
import { useAdminService } from '~/composables/useAdminService'

export const usePhrasesStore = defineStore('phrases', () => {
  const { 
    fetchAllPhrases, 
    fetchUserPhrases,
    fetchExcludedPhrases, 
    excludePhrase, 
    removePhraseExclusion, 
    submitPhraseSuggestion, 
    fetchPhraseSuggestions,
    isLoading,
    error,
    hasError
  } = usePhraseService()
  
  const { approveSuggestion: approveSuggestionService, rejectSuggestion: rejectSuggestionService } = useAdminService()

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
  const loadPhrasesForUser = async () => {
    const response = await fetchUserPhrases()
    userPhrases.value = response.phrases
    return response
  }

  const loadAllPhrasesForAdmin = async () => {
    const response = await fetchAllPhrases()
    adminPhrases.value = response.phrases
    return response
  }

  const loadSuggestionsForUser = async () => {
    const response = await fetchPhraseSuggestions()
    userSuggestions.value = response.suggestions
    return response
  }

  const loadAllSuggestionsForAdmin = async () => {
    const response = await fetchPhraseSuggestions()
    adminSuggestions.value = response.suggestions
    return response
  }

  const togglePhraseExclusion = async (phraseId: string) => {
    const phrase = userPhrases.value.find(p => p.id === phraseId)
    if (!phrase) throw new Error('Phrase not found')

    const wasExcluded = phrase.is_excluded

    if (phrase.is_excluded) {
      // Remove exclusion
      await removePhraseExclusion(phraseId)
      phrase.is_excluded = false
    } else {
      // Add exclusion
      await excludePhrase(phraseId)
      phrase.is_excluded = true
    }
  }

  const submitSuggestion = async (phraseText: string) => {
    const response = await submitPhraseSuggestion(phraseText)
    
    // Add to user suggestions
    userSuggestions.value.unshift(response.suggestion)
    
    return response.suggestion
  }

  const approveSuggestion = async (suggestionId: string, reason?: string) => {
    await approveSuggestionService(suggestionId, reason || '')
    
    // Update suggestion status
    const suggestion = adminSuggestions.value.find(s => s.id === suggestionId)
    if (suggestion) {
      suggestion.status = 'approved'
      suggestion.admin_reason = reason
    }
    
    return suggestion
  }

  const rejectSuggestion = async (suggestionId: string, reason?: string) => {
    await rejectSuggestionService(suggestionId, reason || '')
    
    // Update suggestion status
    const suggestion = adminSuggestions.value.find(s => s.id === suggestionId)
    if (suggestion) {
      suggestion.status = 'rejected'
      suggestion.admin_reason = reason
    }
    
    return suggestion
  }

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
    isLoading,
    error,
    hasError,
    
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
