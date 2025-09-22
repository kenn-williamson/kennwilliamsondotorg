/**
 * Test for pure phrase service
 * Easy to test with mock fetchers
 */

import { describe, it, expect, vi } from 'vitest'
import { phraseService } from '~/services/phraseService'
import type { PhraseWithExclusion, PhraseSuggestion } from '#shared/types/phrases'

describe('phraseService', () => {
  it('should fetch user phrases', async () => {
    const mockFetcher = vi.fn().mockResolvedValue({ 
      phrases: [
        { id: '1', phrase_text: 'Test phrase', is_excluded: false } as PhraseWithExclusion
      ] 
    })
    const service = phraseService(mockFetcher)
    
    const result = await service.fetchUserPhrases()
    
    expect(mockFetcher).toHaveBeenCalledWith('/protected/phrases/user')
    expect(result).toEqual({
      phrases: [
        { id: '1', phrase_text: 'Test phrase', is_excluded: false }
      ]
    })
  })

  it('should exclude phrase', async () => {
    const mockFetcher = vi.fn().mockResolvedValue({ message: 'Phrase excluded' })
    const service = phraseService(mockFetcher)
    
    const result = await service.excludePhrase('phrase-123')
    
    expect(mockFetcher).toHaveBeenCalledWith('/protected/phrases/exclude/phrase-123', {
      method: 'POST'
    })
    expect(result).toEqual({ message: 'Phrase excluded' })
  })

  it('should submit phrase suggestion', async () => {
    const mockSuggestion = {
      id: 'suggestion-123',
      user_id: 'user-123',
      phrase_text: 'New phrase',
      status: 'pending' as const,
      created_at: '2024-01-01T00:00:00Z',
      updated_at: '2024-01-01T00:00:00Z'
    }
    const mockFetcher = vi.fn().mockResolvedValue({ suggestion: mockSuggestion })
    const service = phraseService(mockFetcher)
    
    const result = await service.submitPhraseSuggestion('New phrase')
    
    expect(mockFetcher).toHaveBeenCalledWith('/protected/phrases/suggestions', {
      method: 'POST',
      body: { phrase_text: 'New phrase' }
    })
    expect(result).toEqual({ suggestion: mockSuggestion })
  })

  it('should fetch random phrase for client', async () => {
    const mockFetcher = vi.fn().mockResolvedValue('Random phrase text')
    const service = phraseService(mockFetcher)
    
    const result = await service.fetchRandomPhraseClient('user-slug')
    
    expect(mockFetcher).toHaveBeenCalledWith('/public/user-slug/phrase')
    expect(result).toBe('Random phrase text')
  })
})
