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
  fetchUserPhrases: async (): Promise<{ phrases: PhraseWithExclusion[] }> => {
    return fetcher<{ phrases: PhraseWithExclusion[] }>(API_ROUTES.PROTECTED.PHRASES.USER)
  },

  fetchAllPhrases: async (): Promise<AllPhrasesResponse> => {
    return fetcher<AllPhrasesResponse>(API_ROUTES.PROTECTED.PHRASES.LIST)
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
