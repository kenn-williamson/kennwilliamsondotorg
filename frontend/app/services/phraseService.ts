/**
 * Pure Phrase Service - No Vue context, accepts fetcher as parameter
 * Easy to test with mock fetchers
 */

import { API_ROUTES } from '#shared/config/api-routes'
import type { 
  Phrase, 
  PhraseWithExclusion, 
  PhraseSuggestion,
  PhraseExclusionResponse,
  PhraseSuggestionResponse,
  PhraseSuggestionsResponse,
  AllPhrasesResponse,
  Fetcher
} from '#shared/types'

export const phraseService = (fetcher: Fetcher) => ({
  fetchUserPhrases: async (params?: { limit?: number; offset?: number; search?: string }): Promise<{ phrases: PhraseWithExclusion[] }> => {
    const searchParams = new URLSearchParams()
    if (params?.limit) searchParams.set('limit', params.limit.toString())
    if (params?.offset) searchParams.set('offset', params.offset.toString())
    if (params?.search) searchParams.set('search', params.search)
    
    const url = searchParams.toString() 
      ? `${API_ROUTES.PROTECTED.PHRASES.USER}?${searchParams.toString()}`
      : API_ROUTES.PROTECTED.PHRASES.USER
    
    return fetcher<{ phrases: PhraseWithExclusion[] }>(url)
  },

  fetchAllPhrases: async (params?: { limit?: number; offset?: number; search?: string; include_inactive?: boolean }): Promise<AllPhrasesResponse> => {
    const searchParams = new URLSearchParams()
    if (params?.limit) searchParams.set('limit', params.limit.toString())
    if (params?.offset) searchParams.set('offset', params.offset.toString())
    if (params?.search) searchParams.set('search', params.search)
    if (params?.include_inactive) searchParams.set('include_inactive', params.include_inactive.toString())
    
    const url = searchParams.toString() 
      ? `${API_ROUTES.PROTECTED.PHRASES.LIST}?${searchParams.toString()}`
      : API_ROUTES.PROTECTED.PHRASES.LIST
    
    return fetcher<AllPhrasesResponse>(url)
  },

  excludePhrase: async (phraseId: string): Promise<PhraseExclusionResponse> => {
    return fetcher<PhraseExclusionResponse>(API_ROUTES.PROTECTED.PHRASES.EXCLUDE(phraseId), {
      method: 'POST'
    })
  },

  removePhraseExclusion: async (phraseId: string): Promise<PhraseExclusionResponse> => {
    return fetcher<PhraseExclusionResponse>(API_ROUTES.PROTECTED.PHRASES.EXCLUDE(phraseId), {
      method: 'DELETE'
    })
  },

  submitPhraseSuggestion: async (phraseText: string): Promise<PhraseSuggestionResponse> => {
    return fetcher<PhraseSuggestionResponse>(API_ROUTES.PROTECTED.PHRASES.SUGGESTIONS, {
      method: 'POST',
      body: { phrase_text: phraseText }
    })
  },

  fetchPhraseSuggestions: async (): Promise<PhraseSuggestionsResponse> => {
    return fetcher<PhraseSuggestionsResponse>(API_ROUTES.PROTECTED.PHRASES.SUGGESTIONS)
  },

  fetchRandomPhraseAuth: async (): Promise<string> => {
    return fetcher<string>(API_ROUTES.PROTECTED.PHRASES.RANDOM)
  },

  fetchRandomPhraseClient: async (userSlug: string): Promise<string> => {
    return fetcher<string>(API_ROUTES.PUBLIC.PHRASES.BY_USER_SLUG(userSlug))
  }
})
