import type { Phrase, UserExcludedPhrase, PhraseSuggestion } from '#shared/types/phrases'
import { API_ROUTES } from '#shared/config/api-routes'

export const usePhraseService = () => {
  const { executeRequest, executeRequestWithSuccess, backendFetch, isLoading, error, hasError } = useBaseService()

  /**
   * Fetch a random phrase for a specific user (SSR-friendly)
   * This uses the SSR proxy endpoint for server-side rendering
   */
  const fetchRandomPhrase = async (userSlug: string) => {
    try {
      const response = await $fetch<string>(API_ROUTES.API.PHRASES.BY_USER_SLUG(userSlug))
      return response
    } catch (error: any) {
      console.error('Failed to fetch random phrase:', error)
      throw error
    }
  }

  /**
   * Fetch a random phrase for a specific user (client-side)
   * This calls the backend directly for client-side operations
   */
  const fetchRandomPhraseClient = async (userSlug: string) => {
    try {
      const config = useRuntimeConfig()
      const response = await $fetch<string>(`${config.public.apiBase}${API_ROUTES.PUBLIC.PHRASES.BY_USER_SLUG(userSlug)}`)
      return response
    } catch (error: any) {
      console.error('Failed to fetch random phrase (client):', error)
      throw error
    }
  }

  /**
   * Get all active phrases (for authenticated users)
   * Uses direct backend pattern with JWT authentication
   */
  const fetchAllPhrases = async () => {
    return executeRequest(
      () => backendFetch<{ phrases: Phrase[] }>(API_ROUTES.PROTECTED.PHRASES.LIST),
      'fetchAllPhrases'
    )
  }

  /**
   * Get user's phrases with exclusion status
   * Uses direct backend pattern with JWT authentication
   */
  const fetchUserPhrases = async () => {
    return executeRequest(
      () => backendFetch<{ phrases: PhraseWithExclusion[] }>(API_ROUTES.PROTECTED.PHRASES.USER),
      'fetchUserPhrases'
    )
  }

  /**
   * Get user's excluded phrases (legacy endpoint)
   * Uses direct backend pattern with JWT authentication
   */
  const fetchExcludedPhrases = async () => {
    return executeRequest(
      () => backendFetch<{ excluded_phrases: UserExcludedPhrase[] }>(API_ROUTES.PROTECTED.PHRASES.EXCLUDED),
      'fetchExcludedPhrases'
    )
  }

  /**
   * Exclude a phrase from user's feed
   * Uses direct backend pattern with JWT authentication
   */
  const excludePhrase = async (phraseId: string) => {
    return executeRequestWithSuccess(
      () => backendFetch<{ message: string }>(API_ROUTES.PROTECTED.PHRASES.EXCLUDE(phraseId), {
        method: 'POST'
      }),
      'Phrase excluded successfully',
      'excludePhrase'
    )
  }

  /**
   * Remove phrase exclusion
   * Uses direct backend pattern with JWT authentication
   */
  const removePhraseExclusion = async (phraseId: string) => {
    return executeRequestWithSuccess(
      () => backendFetch<{ message: string }>(API_ROUTES.PROTECTED.PHRASES.EXCLUDE(phraseId), {
        method: 'DELETE'
      }),
      'Phrase exclusion removed successfully',
      'removePhraseExclusion'
    )
  }

  /**
   * Submit a phrase suggestion
   * Uses direct backend pattern with JWT authentication
   */
  const submitPhraseSuggestion = async (phraseText: string) => {
    return executeRequestWithSuccess(
      () => backendFetch<{ suggestion: PhraseSuggestion }>(API_ROUTES.PROTECTED.PHRASES.SUGGESTIONS, {
        method: 'POST',
        body: { phrase_text: phraseText }
      }),
      'Phrase suggestion submitted successfully',
      'submitPhraseSuggestion'
    )
  }

  /**
   * Get user's phrase suggestions
   * Uses direct backend pattern with JWT authentication
   */
  const fetchPhraseSuggestions = async () => {
    return executeRequest(
      () => backendFetch<{ suggestions: PhraseSuggestion[] }>(API_ROUTES.PROTECTED.PHRASES.SUGGESTIONS),
      'fetchPhraseSuggestions'
    )
  }

  /**
   * Fetch a random phrase for the authenticated user (JWT-based)
   * Uses direct backend pattern with JWT authentication
   */
  const fetchRandomPhraseAuth = async () => {
    return executeRequest(
      () => backendFetch<string>(API_ROUTES.PROTECTED.PHRASES.RANDOM),
      'fetchRandomPhraseAuth'
    )
  }

  return {
    fetchRandomPhrase,
    fetchRandomPhraseClient,
    fetchRandomPhraseAuth,
    fetchAllPhrases,
    fetchUserPhrases,
    fetchExcludedPhrases,
    excludePhrase,
    removePhraseExclusion,
    submitPhraseSuggestion,
    fetchPhraseSuggestions,
    
    // Expose base service state for components
    isLoading,
    error,
    hasError,
  }
}
