/**
 * Test for pure phrases store
 * Tests only state management, no service calls
 */

import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { usePhrasesStore } from '~/stores/phrases-refactored'
import type { PhraseWithExclusion, Phrase, PhraseSuggestion } from '#shared/types/phrases'

describe('usePhrasesStore (refactored)', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('should set user phrases', () => {
    const store = usePhrasesStore()
    const phrases: PhraseWithExclusion[] = [
      { id: '1', phrase_text: 'Test phrase', is_excluded: false } as PhraseWithExclusion
    ]
    
    store.setUserPhrases(phrases)
    
    expect(store.userPhrases).toEqual(phrases)
    expect(store.hasUserPhrases).toBe(true)
  })

  it('should toggle phrase exclusion', () => {
    const store = usePhrasesStore()
    const phrases: PhraseWithExclusion[] = [
      { id: '1', phrase_text: 'Test phrase', is_excluded: false } as PhraseWithExclusion
    ]
    
    store.setUserPhrases(phrases)
    store.togglePhraseExclusion('1')
    
    expect(store.userPhrases[0].is_excluded).toBe(true)
    
    store.togglePhraseExclusion('1')
    expect(store.userPhrases[0].is_excluded).toBe(false)
  })

  it('should add user suggestion', () => {
    const store = usePhrasesStore()
    const suggestion: PhraseSuggestion = {
      id: '1',
      user_id: 'user-123',
      phrase_text: 'New suggestion',
      status: 'pending',
      created_at: '2024-01-01T00:00:00Z',
      updated_at: '2024-01-01T00:00:00Z'
    }
    
    store.addUserSuggestion(suggestion)
    
    expect(store.userSuggestions).toContain(suggestion)
    expect(store.hasUserSuggestions).toBe(true)
  })

  it('should update suggestion status', () => {
    const store = usePhrasesStore()
    const suggestions: PhraseSuggestion[] = [
      {
        id: '1',
        user_id: 'user-123',
        phrase_text: 'Test suggestion',
        status: 'pending',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z'
      }
    ]
    
    store.setAdminSuggestions(suggestions)
    store.updateSuggestionStatus('1', 'approved', 'Good suggestion')
    
    expect(store.adminSuggestions[0].status).toBe('approved')
    expect(store.adminSuggestions[0].admin_reason).toBe('Good suggestion')
  })

  it('should clear all data', () => {
    const store = usePhrasesStore()
    
    // Set some data
    store.setUserPhrases([{ id: '1', phrase_text: 'Test', is_excluded: false } as PhraseWithExclusion])
    store.setAdminPhrases([{ id: '1', phrase_text: 'Test', active: true } as Phrase])
    store.setUserSuggestions([{ id: '1', user_id: 'user-123', phrase_text: 'Test', status: 'pending', created_at: '2024-01-01T00:00:00Z', updated_at: '2024-01-01T00:00:00Z' }])
    store.setAdminSuggestions([{ id: '1', user_id: 'user-123', phrase_text: 'Test', status: 'pending', created_at: '2024-01-01T00:00:00Z', updated_at: '2024-01-01T00:00:00Z' }])
    
    store.clearAllData()
    
    expect(store.userPhrases).toEqual([])
    expect(store.adminPhrases).toEqual([])
    expect(store.userSuggestions).toEqual([])
    expect(store.adminSuggestions).toEqual([])
    expect(store.hasUserPhrases).toBe(false)
    expect(store.hasAdminPhrases).toBe(false)
    expect(store.hasUserSuggestions).toBe(false)
    expect(store.hasAdminSuggestions).toBe(false)
  })
})
