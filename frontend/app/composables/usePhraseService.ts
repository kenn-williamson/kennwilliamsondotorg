import type { Phrase, UserExcludedPhrase, PhraseSuggestion } from '~/types/phrases'

export const usePhraseService = () => {
  const backendFetch = useBackendFetch()

  /**
   * Fetch a random phrase for a specific user (SSR-friendly)
   * This uses the SSR proxy endpoint for server-side rendering
   */
  const fetchRandomPhrase = async (userSlug: string) => {
    try {
      const response = await $fetch<string>(`/api/${userSlug}/phrase`)
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
      const response = await $fetch<string>(`${config.public.apiBase}/${userSlug}/phrase`)
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
    try {
      const response = await backendFetch<{ phrases: Phrase[] }>('/phrases')
      return response
    } catch (error: any) {
      console.error('Failed to fetch all phrases:', error)
      throw error
    }
  }

  /**
   * Get user's excluded phrases
   * Uses direct backend pattern with JWT authentication
   */
  const fetchExcludedPhrases = async () => {
    try {
      const response = await backendFetch<{ excluded_phrases: UserExcludedPhrase[] }>('/phrases/excluded')
      return response
    } catch (error: any) {
      console.error('Failed to fetch excluded phrases:', error)
      throw error
    }
  }

  /**
   * Exclude a phrase from user's feed
   * Uses direct backend pattern with JWT authentication
   */
  const excludePhrase = async (phraseId: string) => {
    try {
      const response = await backendFetch<{ message: string }>(`/phrases/exclude/${phraseId}`, {
        method: 'POST'
      })
      return response
    } catch (error: any) {
      console.error('Failed to exclude phrase:', error)
      throw error
    }
  }

  /**
   * Remove phrase exclusion
   * Uses direct backend pattern with JWT authentication
   */
  const removePhraseExclusion = async (phraseId: string) => {
    try {
      const response = await backendFetch<{ message: string }>(`/phrases/exclude/${phraseId}`, {
        method: 'DELETE'
      })
      return response
    } catch (error: any) {
      console.error('Failed to remove phrase exclusion:', error)
      throw error
    }
  }

  /**
   * Submit a phrase suggestion
   * Uses direct backend pattern with JWT authentication
   */
  const submitPhraseSuggestion = async (phraseText: string) => {
    try {
      const response = await backendFetch<{ suggestion: PhraseSuggestion }>('/phrases/suggestions', {
        method: 'POST',
        body: { phrase_text: phraseText }
      })
      return response
    } catch (error: any) {
      console.error('Failed to submit phrase suggestion:', error)
      throw error
    }
  }

  /**
   * Get user's phrase suggestions
   * Uses direct backend pattern with JWT authentication
   */
  const fetchPhraseSuggestions = async () => {
    try {
      const response = await backendFetch<{ suggestions: PhraseSuggestion[] }>('/phrases/suggestions')
      return response
    } catch (error: any) {
      console.error('Failed to fetch phrase suggestions:', error)
      throw error
    }
  }

  /**
   * Fetch a random phrase for the authenticated user (JWT-based)
   * Uses direct backend pattern with JWT authentication
   */
  const fetchRandomPhraseAuth = async () => {
    try {
      const response = await backendFetch<string>('/phrases/random')
      return response
    } catch (error: any) {
      console.error('Failed to fetch random phrase (auth):', error)
      throw error
    }
  }

  return {
    fetchRandomPhrase,
    fetchRandomPhraseClient,
    fetchRandomPhraseAuth,
    fetchAllPhrases,
    fetchExcludedPhrases,
    excludePhrase,
    removePhraseExclusion,
    submitPhraseSuggestion,
    fetchPhraseSuggestions
  }
}
