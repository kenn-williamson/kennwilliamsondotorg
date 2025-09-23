import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { createMockPhrase, createMockPhraseSuggestion, createMockPhraseWithExclusion } from '../utils/test-helpers'

// Import the store directly - no mocking needed for pure stores
import { usePhrasesStore } from '~/stores/phrases'

describe('usePhrasesStore', () => {
  beforeEach(() => {
    // Create a fresh pinia and make it active
    setActivePinia(createPinia())
  })

  describe('store state', () => {
    it('should initialize with empty state', () => {
      const store = usePhrasesStore()
      
      expect(store.userPhrases).toEqual([])
      expect(store.adminPhrases).toEqual([])
      expect(store.userSuggestions).toEqual([])
      expect(store.adminSuggestions).toEqual([])
    })
  })

  describe('computed properties', () => {
    it('should correctly identify when user has phrases', () => {
      const store = usePhrasesStore()
      const phrases = [createMockPhraseWithExclusion()]
      
      store.setUserPhrases(phrases)
      
      expect(store.hasUserPhrases).toBe(true)
    })

    it('should correctly identify when user has no phrases', () => {
      const store = usePhrasesStore()
      
      expect(store.hasUserPhrases).toBe(false)
    })

    it('should correctly identify when admin has phrases', () => {
      const store = usePhrasesStore()
      const phrases = [createMockPhrase()]
      
      store.setAdminPhrases(phrases)
      
      expect(store.hasAdminPhrases).toBe(true)
    })

    it('should correctly identify when user has suggestions', () => {
      const store = usePhrasesStore()
      const suggestions = [createMockPhraseSuggestion()]
      
      store.setUserSuggestions(suggestions)
      
      expect(store.hasUserSuggestions).toBe(true)
    })

    it('should correctly identify when admin has suggestions', () => {
      const store = usePhrasesStore()
      const suggestions = [createMockPhraseSuggestion()]
      
      store.setAdminSuggestions(suggestions)
      
      expect(store.hasAdminSuggestions).toBe(true)
    })
  })

  describe('pure state management functions', () => {
    it('should set user phrases correctly', () => {
      const store = usePhrasesStore()
      const phrases = [createMockPhraseWithExclusion(), createMockPhraseWithExclusion()]
      
      store.setUserPhrases(phrases)
      
      expect(store.userPhrases).toEqual(phrases)
    })

    it('should set admin phrases correctly', () => {
      const store = usePhrasesStore()
      const phrases = [createMockPhrase(), createMockPhrase()]
      
      store.setAdminPhrases(phrases)
      
      expect(store.adminPhrases).toEqual(phrases)
    })

    it('should set user suggestions correctly', () => {
      const store = usePhrasesStore()
      const suggestions = [createMockPhraseSuggestion(), createMockPhraseSuggestion()]
      
      store.setUserSuggestions(suggestions)
      
      expect(store.userSuggestions).toEqual(suggestions)
    })

    it('should set admin suggestions correctly', () => {
      const store = usePhrasesStore()
      const suggestions = [createMockPhraseSuggestion(), createMockPhraseSuggestion()]
      
      store.setAdminSuggestions(suggestions)
      
      expect(store.adminSuggestions).toEqual(suggestions)
    })

    it('should add user suggestion correctly', () => {
      const store = usePhrasesStore()
      const existingSuggestion = createMockPhraseSuggestion({ id: 'existing' })
      const newSuggestion = createMockPhraseSuggestion({ id: 'new' })
      store.setUserSuggestions([existingSuggestion])
      
      store.addUserSuggestion(newSuggestion)
      
      expect(store.userSuggestions).toHaveLength(2)
      expect(store.userSuggestions[0].id).toBe('new') // Should be added to beginning
    })

    it('should toggle phrase exclusion correctly', () => {
      const store = usePhrasesStore()
      const phrase = createMockPhraseWithExclusion({ id: 'test-phrase', is_excluded: false })
      store.setUserPhrases([phrase])
      
      store.togglePhraseExclusion('test-phrase')
      
      expect(store.userPhrases[0].is_excluded).toBe(true)
      
      store.togglePhraseExclusion('test-phrase')
      
      expect(store.userPhrases[0].is_excluded).toBe(false)
    })

    it('should update suggestion status correctly', () => {
      const store = usePhrasesStore()
      const suggestion = createMockPhraseSuggestion({ 
        id: 'test-suggestion', 
        status: 'pending' 
      })
      store.setAdminSuggestions([suggestion])
      
      store.updateSuggestionStatus('test-suggestion', 'approved', 'Great suggestion!')
      
      expect(store.adminSuggestions[0].status).toBe('approved')
      expect(store.adminSuggestions[0].admin_reason).toBe('Great suggestion!')
    })

    it('should remove suggestion correctly', () => {
      const store = usePhrasesStore()
      const suggestion1 = createMockPhraseSuggestion({ id: 'suggestion-1' })
      const suggestion2 = createMockPhraseSuggestion({ id: 'suggestion-2' })
      store.setAdminSuggestions([suggestion1, suggestion2])
      
      store.removeSuggestion('suggestion-1')
      
      expect(store.adminSuggestions).toHaveLength(1)
      expect(store.adminSuggestions[0].id).toBe('suggestion-2')
    })
  })

  describe('utility functions', () => {
    it('should clear user data correctly', () => {
      const store = usePhrasesStore()
      store.setUserPhrases([createMockPhraseWithExclusion()])
      store.setUserSuggestions([createMockPhraseSuggestion()])
      
      store.clearUserData()
      
      expect(store.userPhrases).toEqual([])
      expect(store.userSuggestions).toEqual([])
    })

    it('should clear admin data correctly', () => {
      const store = usePhrasesStore()
      store.setAdminPhrases([createMockPhrase()])
      store.setAdminSuggestions([createMockPhraseSuggestion()])
      
      store.clearAdminData()
      
      expect(store.adminPhrases).toEqual([])
      expect(store.adminSuggestions).toEqual([])
    })

    it('should clear all data correctly', () => {
      const store = usePhrasesStore()
      store.setUserPhrases([createMockPhraseWithExclusion()])
      store.setUserSuggestions([createMockPhraseSuggestion()])
      store.setAdminPhrases([createMockPhrase()])
      store.setAdminSuggestions([createMockPhraseSuggestion()])
      
      store.clearAllData()
      
      expect(store.userPhrases).toEqual([])
      expect(store.userSuggestions).toEqual([])
      expect(store.adminPhrases).toEqual([])
      expect(store.adminSuggestions).toEqual([])
    })
  })
})