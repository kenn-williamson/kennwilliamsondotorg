import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { 
  createMockPhrase, 
  createMockPhraseSuggestion
} from '../utils/test-helpers'

// Mock the composables before importing the store
vi.mock('~/composables/usePhraseService', () => ({
  usePhraseService: () => ({
    fetchAllPhrases: vi.fn().mockResolvedValue({ phrases: [] }),
    fetchUserPhrases: vi.fn().mockResolvedValue({ phrases: [] }),
    fetchExcludedPhrases: vi.fn().mockResolvedValue({ excluded_phrases: [] }),
    excludePhrase: vi.fn().mockResolvedValue({ message: 'Phrase excluded successfully' }),
    removePhraseExclusion: vi.fn().mockResolvedValue({ message: 'Phrase exclusion removed successfully' }),
    submitPhraseSuggestion: vi.fn().mockResolvedValue({ suggestion: createMockPhraseSuggestion() }),
    fetchPhraseSuggestions: vi.fn().mockResolvedValue({ suggestions: [] }),
    isLoading: { value: false },
    error: { value: null },
    hasError: { value: false },
  })
}))

vi.mock('~/composables/useAdminService', () => ({
  useAdminService: () => ({
    approveSuggestion: vi.fn().mockResolvedValue({ message: 'Suggestion approved successfully' }),
    rejectSuggestion: vi.fn().mockResolvedValue({ message: 'Suggestion rejected successfully' }),
  })
}))

// Import the store after mocking
import { usePhrasesStore } from '~/stores/phrases'

describe('usePhrasesStore - Simple Tests', () => {
  beforeEach(() => {
    // Create a fresh pinia and make it active
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  describe('initial state', () => {
    it('should initialize with empty state', () => {
      const store = usePhrasesStore()
      
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

  describe('computed properties', () => {
    it('should correctly compute hasUserPhrases', () => {
      const store = usePhrasesStore()
      
      // Initially empty
      expect(store.hasUserPhrases).toBe(false)
      
      // Add phrases by directly modifying the array
      const phrase = createMockPhrase()
      store.userPhrases.splice(0, store.userPhrases.length, phrase)
      expect(store.hasUserPhrases).toBe(true)
    })

    it('should correctly compute hasAdminPhrases', () => {
      const store = usePhrasesStore()
      
      // Initially empty
      expect(store.hasAdminPhrases).toBe(false)
      
      // Add phrases by directly modifying the array
      const phrase = createMockPhrase()
      store.adminPhrases.splice(0, store.adminPhrases.length, phrase)
      expect(store.hasAdminPhrases).toBe(true)
    })

    it('should correctly compute hasUserSuggestions', () => {
      const store = usePhrasesStore()
      
      // Initially empty
      expect(store.hasUserSuggestions).toBe(false)
      
      // Add suggestions by directly modifying the array
      const suggestion = createMockPhraseSuggestion()
      store.userSuggestions.splice(0, store.userSuggestions.length, suggestion)
      expect(store.hasUserSuggestions).toBe(true)
    })

    it('should correctly compute hasAdminSuggestions', () => {
      const store = usePhrasesStore()
      
      // Initially empty
      expect(store.hasAdminSuggestions).toBe(false)
      
      // Add suggestions by directly modifying the array
      const suggestion = createMockPhraseSuggestion()
      store.adminSuggestions.splice(0, store.adminSuggestions.length, suggestion)
      expect(store.hasAdminSuggestions).toBe(true)
    })
  })

  describe('utility functions', () => {
    it('should clear user data', () => {
      const store = usePhrasesStore()
      store.userPhrases.splice(0, store.userPhrases.length, createMockPhrase())
      store.userSuggestions.splice(0, store.userSuggestions.length, createMockPhraseSuggestion())
      
      store.clearUserData()
      
      expect(store.userPhrases).toEqual([])
      expect(store.userSuggestions).toEqual([])
    })

    it('should clear admin data', () => {
      const store = usePhrasesStore()
      store.adminPhrases.splice(0, store.adminPhrases.length, createMockPhrase())
      store.adminSuggestions.splice(0, store.adminSuggestions.length, createMockPhraseSuggestion())
      
      store.clearAdminData()
      
      expect(store.adminPhrases).toEqual([])
      expect(store.adminSuggestions).toEqual([])
    })

    it('should clear all data', () => {
      const store = usePhrasesStore()
      store.userPhrases.splice(0, store.userPhrases.length, createMockPhrase())
      store.adminPhrases.splice(0, store.adminPhrases.length, createMockPhrase())
      store.userSuggestions.splice(0, store.userSuggestions.length, createMockPhraseSuggestion())
      store.adminSuggestions.splice(0, store.adminSuggestions.length, createMockPhraseSuggestion())
      
      store.clearAllData()
      
      expect(store.userPhrases).toEqual([])
      expect(store.adminPhrases).toEqual([])
      expect(store.userSuggestions).toEqual([])
      expect(store.adminSuggestions).toEqual([])
    })
  })
})
