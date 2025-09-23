import { describe, it, expect, vi } from 'vitest'
import { createMockPhrase, createMockPhraseSuggestion, createMockPhraseWithExclusion } from '../utils/test-helpers'
import { phraseService } from '~/services/phraseService'

describe('phraseService', () => {
  let mockFetcher: any

  beforeEach(() => {
    mockFetcher = vi.fn()
  })

  describe('fetchUserPhrases', () => {
    it('should call correct endpoint', async () => {
      const mockResponse = { phrases: [createMockPhraseWithExclusion()] }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = phraseService(mockFetcher)
      const result = await service.fetchUserPhrases()

      expect(mockFetcher).toHaveBeenCalledWith('/protected/phrases/user')
      expect(result).toEqual(mockResponse)
    })
  })

  describe('fetchAllPhrases', () => {
    it('should call correct endpoint', async () => {
      const mockResponse = { phrases: [createMockPhrase()], total: 1 }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = phraseService(mockFetcher)
      const result = await service.fetchAllPhrases()

      expect(mockFetcher).toHaveBeenCalledWith('/protected/phrases')
      expect(result).toEqual(mockResponse)
    })
  })

  describe('excludePhrase', () => {
    it('should call correct endpoint with POST method', async () => {
      const phraseId = 'test-phrase'
      const mockResponse = { message: 'Phrase excluded successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = phraseService(mockFetcher)
      const result = await service.excludePhrase(phraseId)

      expect(mockFetcher).toHaveBeenCalledWith(`/protected/phrases/exclude/${phraseId}`, {
        method: 'POST'
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('removePhraseExclusion', () => {
    it('should call correct endpoint with DELETE method', async () => {
      const phraseId = 'test-phrase'
      const mockResponse = { message: 'Phrase exclusion removed successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = phraseService(mockFetcher)
      const result = await service.removePhraseExclusion(phraseId)

      expect(mockFetcher).toHaveBeenCalledWith(`/protected/phrases/exclude/${phraseId}`, {
        method: 'DELETE'
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('submitPhraseSuggestion', () => {
    it('should call correct endpoint with POST method and body', async () => {
      const phraseText = 'Test phrase suggestion'
      const mockResponse = { suggestion: createMockPhraseSuggestion({ phrase_text: phraseText }) }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = phraseService(mockFetcher)
      const result = await service.submitPhraseSuggestion(phraseText)

      expect(mockFetcher).toHaveBeenCalledWith('/protected/phrases/suggestions', {
        method: 'POST',
        body: { phrase_text: phraseText }
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('fetchPhraseSuggestions', () => {
    it('should call correct endpoint', async () => {
      const mockResponse = { suggestions: [createMockPhraseSuggestion()], total: 1 }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = phraseService(mockFetcher)
      const result = await service.fetchPhraseSuggestions()

      expect(mockFetcher).toHaveBeenCalledWith('/protected/phrases/suggestions')
      expect(result).toEqual(mockResponse)
    })
  })

  describe('fetchRandomPhraseAuth', () => {
    it('should call correct endpoint', async () => {
      const mockPhrase = 'Test random phrase'
      mockFetcher.mockResolvedValue(mockPhrase)

      const service = phraseService(mockFetcher)
      const result = await service.fetchRandomPhraseAuth()

      expect(mockFetcher).toHaveBeenCalledWith('/protected/phrases/random')
      expect(result).toEqual(mockPhrase)
    })
  })

  describe('fetchRandomPhraseClient', () => {
    it('should call correct endpoint with user slug', async () => {
      const userSlug = 'test-user'
      const mockPhrase = 'Test random phrase'
      mockFetcher.mockResolvedValue(mockPhrase)

      const service = phraseService(mockFetcher)
      const result = await service.fetchRandomPhraseClient(userSlug)

      expect(mockFetcher).toHaveBeenCalledWith(`/public/${userSlug}/phrase`)
      expect(result).toEqual(mockPhrase)
    })
  })

  describe('approveSuggestion', () => {
    it('should call correct endpoint with POST method and body', async () => {
      const suggestionId = 'test-suggestion'
      const adminReason = 'Great suggestion!'
      const mockResponse = { message: 'Suggestion approved successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = phraseService(mockFetcher)
      const result = await service.approveSuggestion(suggestionId, adminReason)

      expect(mockFetcher).toHaveBeenCalledWith(`/protected/admin/suggestions/${suggestionId}/approve`, {
        method: 'POST',
        body: { admin_reason: adminReason }
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('rejectSuggestion', () => {
    it('should call correct endpoint with POST method and body', async () => {
      const suggestionId = 'test-suggestion'
      const adminReason = 'Too similar to existing content'
      const mockResponse = { message: 'Suggestion rejected successfully' }
      mockFetcher.mockResolvedValue(mockResponse)

      const service = phraseService(mockFetcher)
      const result = await service.rejectSuggestion(suggestionId, adminReason)

      expect(mockFetcher).toHaveBeenCalledWith(`/protected/admin/suggestions/${suggestionId}/reject`, {
        method: 'POST',
        body: { admin_reason: adminReason }
      })
      expect(result).toEqual(mockResponse)
    })
  })

  describe('error handling', () => {
    it('should propagate fetcher errors', async () => {
      const error = new Error('Network error')
      mockFetcher.mockRejectedValue(error)

      const service = phraseService(mockFetcher)

      await expect(service.fetchUserPhrases()).rejects.toThrow('Network error')
    })
  })
})