/**
 * Enhanced Phrases Store - Centralized state management with actions
 * Refactored from action composable pattern to store-with-actions pattern
 */

import type { Phrase, PhraseSuggestion, PhraseWithExclusion } from '#shared/types/phrases'
import { phraseService } from '~/services/phraseService'
import { useSmartFetch } from '#shared/composables/useSmartFetch'

export const usePhrasesStore = defineStore('phrases', () => {
  // State
  const userPhrases = ref<PhraseWithExclusion[]>([])
  const adminPhrases = ref<Phrase[]>([])
  const userSuggestions = ref<PhraseSuggestion[]>([])
  const adminSuggestions = ref<PhraseSuggestion[]>([])
  const currentPhrase = ref<string | null>(null)
  
  // Transient state (moved from useBaseService)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Computed
  const hasUserPhrases = computed(() => userPhrases.value.length > 0)
  const hasAdminPhrases = computed(() => adminPhrases.value.length > 0)
  const hasUserSuggestions = computed(() => userSuggestions.value.length > 0)
  const hasAdminSuggestions = computed(() => adminSuggestions.value.length > 0)
  const hasError = computed(() => !!error.value)
  const activePhrases = computed(() => adminPhrases.value.filter(p => p.active))
  const pendingSuggestions = computed(() => adminSuggestions.value.filter(s => s.status === 'pending'))

  // Service instance
  const smartFetch = useSmartFetch()
  const phraseServiceInstance = phraseService(smartFetch)

  // Private action handler (replaces useBaseService logic)
  const _handleAction = async <T>(
    action: () => Promise<T>,
    context?: string
  ): Promise<T | undefined> => {
    isLoading.value = true
    error.value = null
    
    try {
      const result = await action()
      return result
    } catch (err: any) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      error.value = errorMessage
      console.error(`[PhrasesStore] Error${context ? ` in ${context}` : ''}:`, errorMessage)
      
      // Handle authentication errors gracefully (don't crash during SSR)
      if (err.statusCode === 401) {
        console.log(`[PhrasesStore] Authentication error in ${context}, returning null instead of crashing`)
        return undefined
      }
      
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // Private success handler
  const _handleSuccess = (message: string): void => {
    console.log(`[PhrasesStore] Success: ${message}`)
    // TODO: Add toast notifications here
  }

  // Actions (migrated from usePhrasesActions)
  const loadPhrasesForUser = async () => {
    const data = await _handleAction(() => phraseServiceInstance.fetchUserPhrases(), 'loadPhrasesForUser')
    if (data) {
      userPhrases.value = data.phrases
    }
    return data
  }

  const loadAllPhrasesForAdmin = async () => {
    const data = await _handleAction(() => phraseServiceInstance.fetchAllPhrases(), 'loadAllPhrasesForAdmin')
    if (data) {
      adminPhrases.value = data.phrases
    }
    return data
  }

  const loadSuggestionsForUser = async () => {
    const data = await _handleAction(() => phraseServiceInstance.fetchPhraseSuggestions(), 'loadSuggestionsForUser')
    if (data) {
      userSuggestions.value = data.suggestions
    }
    return data
  }

  const loadAllSuggestionsForAdmin = async () => {
    const data = await _handleAction(() => phraseServiceInstance.fetchPhraseSuggestions(), 'loadAllSuggestionsForAdmin')
    if (data) {
      adminSuggestions.value = data.suggestions
    }
    return data
  }

  const togglePhraseExclusion = async (phraseId: string) => {
    const phrase = userPhrases.value.find(p => p.id === phraseId)
    if (!phrase) throw new Error('Phrase not found')

    if (phrase.is_excluded) {
      await _handleAction(() => phraseServiceInstance.removePhraseExclusion(phraseId), 'removePhraseExclusion')
      _handleSuccess('Phrase exclusion removed successfully')
    } else {
      await _handleAction(() => phraseServiceInstance.excludePhrase(phraseId), 'excludePhrase')
      _handleSuccess('Phrase excluded successfully')
    }

    // Update local state
    phrase.is_excluded = !phrase.is_excluded
  }

  const submitSuggestion = async (phraseText: string) => {
    const data = await _handleAction(() => phraseServiceInstance.submitPhraseSuggestion(phraseText), 'submitSuggestion')
    _handleSuccess('Phrase suggestion submitted successfully')
    
    if (data) {
      userSuggestions.value.unshift(data.suggestion)
    }
    return data
  }

  const approveSuggestion = async (suggestionId: string, reason?: string) => {
    await _handleAction(() => phraseServiceInstance.approveSuggestion(suggestionId, reason || ''), 'approveSuggestion')
    _handleSuccess('Suggestion approved successfully')
    
    // Update local state
    const suggestion = adminSuggestions.value.find(s => s.id === suggestionId)
    if (suggestion) {
      suggestion.status = 'approved'
      if (reason) {
        suggestion.admin_reason = reason
      }
    }
    return suggestion
  }

  const rejectSuggestion = async (suggestionId: string, reason?: string) => {
    await _handleAction(() => phraseServiceInstance.rejectSuggestion(suggestionId, reason || ''), 'rejectSuggestion')
    _handleSuccess('Suggestion rejected successfully')
    
    // Update local state
    const suggestion = adminSuggestions.value.find(s => s.id === suggestionId)
    if (suggestion) {
      suggestion.status = 'rejected'
      if (reason) {
        suggestion.admin_reason = reason
      }
    }
    return suggestion
  }

  const fetchRandomPhraseAuth = async () => {
    const data = await _handleAction(() => phraseServiceInstance.fetchRandomPhraseAuth(), 'fetchRandomPhraseAuth')
    if (data) {
      currentPhrase.value = data
    }
    return data
  }

  const fetchRandomPhraseClient = async (userSlug: string) => {
    const data = await _handleAction(() => phraseServiceInstance.fetchRandomPhraseClient(userSlug), 'fetchRandomPhraseClient')
    if (data) {
      currentPhrase.value = data
    }
    return data
  }

  // Get backend URL based on environment
  const getBackendUrl = () => {
    // During SSR, use internal Docker network
    if (process.server) {
      return 'http://backend:8080/backend'
    }
    // On client, use public URL
    return 'https://localhost/backend'
  }

  // SSR-compatible random phrase fetching
  const fetchRandomPhraseSSR = async (userSlug?: string): Promise<string | null> => {
    const result = await _handleAction(async () => {
      if (userSlug) {
        // Public endpoint - use existing service method
        const response = await phraseServiceInstance.fetchRandomPhraseClient(userSlug)
        currentPhrase.value = response
        return response
      } else {
        // Authenticated endpoint - use existing service method
        const response = await phraseServiceInstance.fetchRandomPhraseAuth()
        currentPhrase.value = response
        return response
      }
    }, 'fetchRandomPhraseSSR')
    
    return result || null
  }

  // Pure state management functions (kept for backward compatibility)
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

  const togglePhraseExclusionLocal = (phraseId: string) => {
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
    currentPhrase: readonly(currentPhrase),
    isLoading: readonly(isLoading),
    error: readonly(error),
    
    // Computed
    hasUserPhrases,
    hasAdminPhrases,
    hasUserSuggestions,
    hasAdminSuggestions,
    hasError,
    activePhrases,
    pendingSuggestions,
    
    // Actions (migrated from usePhrasesActions)
    loadPhrasesForUser,
    loadAllPhrasesForAdmin,
    loadSuggestionsForUser,
    loadAllSuggestionsForAdmin,
    togglePhraseExclusion,
    submitSuggestion,
    approveSuggestion,
    rejectSuggestion,
    fetchRandomPhraseAuth,
    fetchRandomPhraseClient,
    fetchRandomPhraseSSR,
    
    // Pure state management functions (kept for backward compatibility)
    setUserPhrases,
    setAdminPhrases,
    setUserSuggestions,
    setAdminSuggestions,
    addUserSuggestion,
    updateSuggestionStatus,
    removeSuggestion,
    
    // Utilities
    clearUserData,
    clearAdminData,
    clearAllData
  }
})
