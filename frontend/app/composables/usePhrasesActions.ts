/**
 * Phrases Action Composable - Orchestrates services + stores
 * Handles context-aware operations and bridges between services and stores
 */

import { usePhrasesStore } from '~/stores/phrases'
import { phraseService } from '~/services/phraseService'
import { useBaseService } from '~/composables/useBaseService'
import { useBackendFetch } from '~/composables/useBackendFetch'
import { useAuthFetch } from '~/composables/useAuthFetch'

export const usePhrasesActions = () => {
  // Create dependencies at the top level
  const backendFetch = useBackendFetch()
  
  // Use base service for request execution
  const { executeRequest, executeRequestWithSuccess, isLoading, error, hasError } = useBaseService()
  
  // Create service instance
  const phraseServiceBackend = phraseService(backendFetch)
  
  // Destructure service methods
  const { 
    fetchUserPhrases, 
    fetchAllPhrases, 
    excludePhrase, 
    removePhraseExclusion, 
    submitPhraseSuggestion: submitPhraseSuggestionService, 
    fetchPhraseSuggestions: fetchPhraseSuggestionsService, 
    fetchRandomPhraseAuth: fetchRandomPhraseAuthService, 
    fetchRandomPhraseClient: fetchRandomPhraseClientService 
  } = phraseServiceBackend

  // Destructure store methods and state
  const { 
    setUserPhrases, 
    setAdminPhrases, 
    setUserSuggestions, 
    setAdminSuggestions, 
    addUserSuggestion, 
    togglePhraseExclusion: togglePhraseExclusionStore, 
    updateSuggestionStatus,
    userPhrases,
    adminSuggestions
  } = usePhrasesStore()

  const loadPhrasesForUser = async () => {
    const response = await executeRequest(() => fetchUserPhrases(), 'loadPhrasesForUser')
    setUserPhrases(response.phrases)
    return response
  }

  const loadAllPhrasesForAdmin = async () => {
    const response = await executeRequest(() => fetchAllPhrases(), 'loadAllPhrasesForAdmin')
    setAdminPhrases(response.phrases)
    return response
  }

  const loadSuggestionsForUser = async () => {
    const response = await executeRequest(() => fetchPhraseSuggestionsService(), 'loadSuggestionsForUser')
    setUserSuggestions(response.suggestions)
    return response
  }

  const loadAllSuggestionsForAdmin = async () => {
    const response = await executeRequest(() => fetchPhraseSuggestionsService(), 'loadAllSuggestionsForAdmin')
    setAdminSuggestions(response.suggestions)
    return response
  }

  const togglePhraseExclusion = async (phraseId: string) => {
    const phrase = userPhrases.find(p => p.id === phraseId)
    if (!phrase) throw new Error('Phrase not found')

    if (phrase.is_excluded) {
      await executeRequestWithSuccess(
        () => removePhraseExclusion(phraseId),
        'Phrase exclusion removed successfully',
        'removePhraseExclusion'
      )
    } else {
      await executeRequestWithSuccess(
        () => excludePhrase(phraseId),
        'Phrase excluded successfully',
        'excludePhrase'
      )
    }

    togglePhraseExclusionStore(phraseId)
  }

  const submitSuggestion = async (phraseText: string) => {
    const response = await executeRequestWithSuccess(
      () => submitPhraseSuggestionService(phraseText),
      'Phrase suggestion submitted successfully',
      'submitSuggestion'
    )
    addUserSuggestion(response.suggestion)
    return response.suggestion
  }

  const approveSuggestion = async (suggestionId: string, reason?: string) => {
    await executeRequestWithSuccess(
      () => phraseServiceBackend.approveSuggestion(suggestionId, reason || ''),
      'Suggestion approved successfully',
      'approveSuggestion'
    )
    updateSuggestionStatus(suggestionId, 'approved', reason)
    return adminSuggestions.find(s => s.id === suggestionId)
  }

  const rejectSuggestion = async (suggestionId: string, reason?: string) => {
    await executeRequestWithSuccess(
      () => phraseServiceBackend.rejectSuggestion(suggestionId, reason || ''),
      'Suggestion rejected successfully',
      'rejectSuggestion'
    )
    updateSuggestionStatus(suggestionId, 'rejected', reason)
    return adminSuggestions.find(s => s.id === suggestionId)
  }

  const fetchRandomPhraseAuth = async () => {
    return executeRequest(() => fetchRandomPhraseAuthService(), 'fetchRandomPhraseAuth')
  }

  const fetchRandomPhraseClient = async (userSlug: string) => {
    return executeRequest(() => fetchRandomPhraseClientService(userSlug), 'fetchRandomPhraseClient')
  }

  // Aliases for backward compatibility
  const fetchPhraseSuggestions = loadSuggestionsForUser
  const submitPhraseSuggestion = submitSuggestion

  return {
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
    // Aliases for backward compatibility
    fetchPhraseSuggestions,
    submitPhraseSuggestion,
    isLoading,
    error,
    hasError
  }
}
