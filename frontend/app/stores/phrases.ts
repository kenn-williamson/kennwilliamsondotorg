/**
 * Pure Phrases Store - Only state management, no service calls
 * Refactored to follow proper separation of concerns
 */

import type { Phrase, PhraseSuggestion, PhraseWithExclusion } from '#shared/types/phrases'

export const usePhrasesStore = defineStore('phrases', () => {
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

  // Pure state management functions
  const setUserPhrases = (phrases: PhraseWithExclusion[]) => {
    userPhrases.value = phrases
  }

  const setAdminPhrases = (phrases: Phrase[]) => {
    adminPhrases.value = phrases
  }

  const setUserSuggestions = (suggestions: PhraseSuggestion[]) => {
    userSuggestions.value = suggestions
  }

  const setAdminSuggestions = (suggestions: PhraseSuggestion[]) => {
    adminSuggestions.value = suggestions
  }

  const addUserSuggestion = (suggestion: PhraseSuggestion) => {
    userSuggestions.value.unshift(suggestion)
  }

  const togglePhraseExclusion = (phraseId: string) => {
    const phrase = userPhrases.value.find(p => p.id === phraseId)
    if (phrase) {
      phrase.is_excluded = !phrase.is_excluded
    }
  }

  const updateSuggestionStatus = (suggestionId: string, status: string, adminReason?: string) => {
    const suggestion = adminSuggestions.value.find(s => s.id === suggestionId)
    if (suggestion) {
      suggestion.status = status as 'pending' | 'approved' | 'rejected'
      if (adminReason) {
        suggestion.admin_reason = adminReason
      }
    }
  }

  const removeSuggestion = (suggestionId: string) => {
    adminSuggestions.value = adminSuggestions.value.filter(s => s.id !== suggestionId)
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
    
    // Actions
    setUserPhrases,
    setAdminPhrases,
    setUserSuggestions,
    setAdminSuggestions,
    addUserSuggestion,
    togglePhraseExclusion,
    updateSuggestionStatus,
    removeSuggestion,
    
    // Utilities
    clearUserData,
    clearAdminData,
    clearAllData
  }
})
