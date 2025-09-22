import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { 
  createMockPhrase, 
  createMockPhraseSuggestion
} from '../utils/test-helpers'

// Mock the composables before importing the store
vi.mock('~/composables/usePhraseService', () => ({
  usePhraseService: () => ({
    fetchAllPhrases: vi.fn().mockResolvedValue({ phrases: [createMockPhrase()] }),
    fetchUserPhrases: vi.fn().mockResolvedValue({ phrases: [createMockPhrase()] }),
    fetchExcludedPhrases: vi.fn().mockResolvedValue({ excluded_phrases: [] }),
    excludePhrase: vi.fn().mockResolvedValue({ message: 'Phrase excluded successfully' }),
    removePhraseExclusion: vi.fn().mockResolvedValue({ message: 'Phrase exclusion removed successfully' }),
    submitPhraseSuggestion: vi.fn().mockResolvedValue({ suggestion: createMockPhraseSuggestion() }),
    fetchPhraseSuggestions: vi.fn().mockResolvedValue({ suggestions: [createMockPhraseSuggestion()] }),
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

describe('usePhrasesStore - Working Tests', () => {
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

  describe('loadPhrasesForUser', () => {
    it('should load user phrases and update state', async () => {
      const store = usePhrasesStore()
      const mockPhrases = [createMockPhrase(), createMockPhrase({ id: '2' })]
      const mockResponse = { phrases: mockPhrases }
      
      // Mock the service to return our test data
      const mockService = global.usePhraseService()
      mockService.fetchUserPhrases.mockResolvedValue(mockResponse)
      
      const result = await store.loadPhrasesForUser()
      
      expect(mockService.fetchUserPhrases).toHaveBeenCalled()
      expect(store.userPhrases).toEqual(mockPhrases)
      expect(result).toEqual(mockResponse)
      expect(store.hasUserPhrases).toBe(true)
    })
  })

  describe('loadAllPhrasesForAdmin', () => {
    it('should load admin phrases and update state', async () => {
      const store = usePhrasesStore()
      const mockPhrases = [createMockPhrase(), createMockPhrase({ id: '2' })]
      const mockResponse = { phrases: mockPhrases }
      
      // Mock the service to return our test data
      const mockService = global.usePhraseService()
      mockService.fetchAllPhrases.mockResolvedValue(mockResponse)
      
      const result = await store.loadAllPhrasesForAdmin()
      
      expect(mockService.fetchAllPhrases).toHaveBeenCalled()
      expect(store.adminPhrases).toEqual(mockPhrases)
      expect(result).toEqual(mockResponse)
      expect(store.hasAdminPhrases).toBe(true)
    })
  })

  describe('loadSuggestionsForUser', () => {
    it('should load user suggestions and update state', async () => {
      const store = usePhrasesStore()
      const mockSuggestions = [createMockPhraseSuggestion(), createMockPhraseSuggestion({ id: '2' })]
      const mockResponse = { suggestions: mockSuggestions }
      
      // Mock the service to return our test data
      const mockService = global.usePhraseService()
      mockService.fetchPhraseSuggestions.mockResolvedValue(mockResponse)
      
      const result = await store.loadSuggestionsForUser()
      
      expect(mockService.fetchPhraseSuggestions).toHaveBeenCalled()
      expect(store.userSuggestions).toEqual(mockSuggestions)
      expect(result).toEqual(mockResponse)
      expect(store.hasUserSuggestions).toBe(true)
    })
  })

  describe('loadAllSuggestionsForAdmin', () => {
    it('should load admin suggestions and update state', async () => {
      const store = usePhrasesStore()
      const mockSuggestions = [createMockPhraseSuggestion(), createMockPhraseSuggestion({ id: '2' })]
      const mockResponse = { suggestions: mockSuggestions }
      
      // Mock the service to return our test data
      const mockService = global.usePhraseService()
      mockService.fetchPhraseSuggestions.mockResolvedValue(mockResponse)
      
      const result = await store.loadAllSuggestionsForAdmin()
      
      expect(mockService.fetchPhraseSuggestions).toHaveBeenCalled()
      expect(store.adminSuggestions).toEqual(mockSuggestions)
      expect(result).toEqual(mockResponse)
      expect(store.hasAdminSuggestions).toBe(true)
    })
  })

  describe('submitSuggestion', () => {
    it('should submit suggestion and add to user suggestions', async () => {
      const store = usePhrasesStore()
      const mockSuggestion = createMockPhraseSuggestion()
      const mockResponse = { suggestion: mockSuggestion }
      
      // Mock the service to return our test data
      const mockService = global.usePhraseService()
      mockService.submitPhraseSuggestion.mockResolvedValue(mockResponse)
      
      const result = await store.submitSuggestion('New phrase suggestion')
      
      expect(mockService.submitPhraseSuggestion).toHaveBeenCalledWith('New phrase suggestion')
      expect(store.userSuggestions).toContain(mockSuggestion)
      expect(result).toEqual(mockSuggestion)
    })
  })

  describe('utility functions', () => {
    it('should clear user data', () => {
      const store = usePhrasesStore()
      // First load some data
      store.userPhrases.splice(0, store.userPhrases.length, createMockPhrase())
      store.userSuggestions.splice(0, store.userSuggestions.length, createMockPhraseSuggestion())
      
      store.clearUserData()
      
      expect(store.userPhrases).toEqual([])
      expect(store.userSuggestions).toEqual([])
    })

    it('should clear admin data', () => {
      const store = usePhrasesStore()
      // First load some data
      store.adminPhrases.splice(0, store.adminPhrases.length, createMockPhrase())
      store.adminSuggestions.splice(0, store.adminSuggestions.length, createMockPhraseSuggestion())
      
      store.clearAdminData()
      
      expect(store.adminPhrases).toEqual([])
      expect(store.adminSuggestions).toEqual([])
    })

    it('should clear all data', () => {
      const store = usePhrasesStore()
      // First load some data
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
