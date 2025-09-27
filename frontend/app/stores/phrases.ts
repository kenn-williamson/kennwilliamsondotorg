/**
 * Enhanced Phrases Store - Centralized state management with actions
 */

import type { Phrase, PhraseSuggestion, PhraseWithExclusion } from '#shared/types/phrases'
import { phraseService } from '~/services/phraseService'
import { useSmartFetch } from '~/composables/useSmartFetch'
import { useSessionWatcher } from '~/composables/useSessionWatcher'

export const usePhrasesStore = defineStore('phrases', () => {
  const userPhrases = ref<PhraseWithExclusion[]>([])
  const adminPhrases = ref<Phrase[]>([])
  const userSuggestions = ref<PhraseSuggestion[]>([])
  const adminSuggestions = ref<PhraseSuggestion[]>([])
  const currentPhrase = ref<string | null>(null)
  
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const hasUserPhrases = computed(() => userPhrases.value.length > 0)
  const hasAdminPhrases = computed(() => adminPhrases.value.length > 0)
  const hasUserSuggestions = computed(() => userSuggestions.value.length > 0)
  const hasAdminSuggestions = computed(() => adminSuggestions.value.length > 0)
  const hasError = computed(() => !!error.value)
  const activePhrases = computed(() => adminPhrases.value.filter(p => p.active))
  const pendingSuggestions = computed(() => adminSuggestions.value.filter(s => s.status === 'pending'))

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
      
      // Handle all errors gracefully - keep them in state for UI to display
      return undefined
    } finally {
      isLoading.value = false
    }
  }

  // Private success handler
  const _handleSuccess = (message: string): void => {
    console.log(`[PhrasesStore] Success: ${message}`)
    // TODO: Add toast notifications here
  }

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
      // Convert PhraseSuggestionResponse to PhraseSuggestion by adding missing fields
      const suggestion: PhraseSuggestion = {
        ...data,
        user_id: '', // Will be populated when we reload suggestions
        user_display_name: '' // Will be populated when we reload suggestions
      }
      userSuggestions.value.unshift(suggestion)
    }
    return data
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

  // SSR-compatible random phrase fetching
  const fetchRandomPhraseSSR = async (userSlug?: string): Promise<string | null> => {
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
    userPhrases.value = []
    adminPhrases.value = []
    userSuggestions.value = []
    adminSuggestions.value = []
    currentPhrase.value = null
    isLoading.value = false
    error.value = null
    
    console.log('🧹 [PhrasesStore] All data cleared')
  }

  // Set up session watcher for automatic cleanup on logout
  useSessionWatcher(clearAllData)

  return {
    userPhrases,
    adminPhrases,
    userSuggestions,
    adminSuggestions,
    currentPhrase,
    isLoading,
    error,
    
    hasUserPhrases,
    hasAdminPhrases,
    hasUserSuggestions,
    hasAdminSuggestions,
    hasError,
    activePhrases,
    pendingSuggestions,
    
    loadPhrasesForUser,
    loadAllPhrasesForAdmin,
    loadSuggestionsForUser,
    loadAllSuggestionsForAdmin,
    togglePhraseExclusion,
    submitSuggestion,
    fetchRandomPhraseAuth,
    fetchRandomPhraseClient,
    fetchRandomPhraseSSR,
    
    setUserPhrases,
    setAdminPhrases,
    setUserSuggestions,
    setAdminSuggestions,
    addUserSuggestion,
    
    clearUserData,
    clearAdminData,
    clearAllData
  }
})
