import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { 
  createMockPhrase, 
  createMockPhraseSuggestion, 
  createMockUser 
} from '../utils/test-helpers'

// Import the store (composables are mocked globally in setup.ts)
import { usePhrasesStore } from '~/stores/phrases'

describe('usePhrasesStore', () => {
  beforeEach(() => {
    // Create a fresh pinia and make it active
    setActivePinia(createPinia())
    
    // Reset all mocks before each test
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
      
      // Add phrases
      store.userPhrases.splice(0, store.userPhrases.length, createMockPhrase())
      expect(store.hasUserPhrases).toBe(true)
    })

    it('should correctly compute hasAdminPhrases', () => {
      const store = usePhrasesStore()
      
      // Initially empty
      expect(store.hasAdminPhrases).toBe(false)
      
      // Add phrases
      store.adminPhrases.splice(0, store.adminPhrases.length, createMockPhrase())
      expect(store.hasAdminPhrases).toBe(true)
    })

    it('should correctly compute hasUserSuggestions', () => {
      const store = usePhrasesStore()
      
      // Initially empty
      expect(store.hasUserSuggestions).toBe(false)
      
      // Add suggestions
      store.userSuggestions.splice(0, store.userSuggestions.length, createMockPhraseSuggestion())
      expect(store.hasUserSuggestions).toBe(true)
    })

    it('should correctly compute hasAdminSuggestions', () => {
      const store = usePhrasesStore()
      
      // Initially empty
      expect(store.hasAdminSuggestions).toBe(false)
      
      // Add suggestions
      store.adminSuggestions.splice(0, store.adminSuggestions.length, createMockPhraseSuggestion())
      expect(store.hasAdminSuggestions).toBe(true)
    })
  })

  describe('loadPhrasesForUser', () => {
    it('should load user phrases and update state', async () => {
      const store = usePhrasesStore()
      const mockPhrases = [createMockPhrase(), createMockPhrase({ id: '2' })]
      const mockResponse = { phrases: mockPhrases }
      
      // Mock the service
      const mockService = global.usePhraseService()
      mockService.fetchUserPhrases.mockResolvedValue(mockResponse)
      
      const result = await store.loadPhrasesForUser()
      
      expect(mockService.fetchUserPhrases).toHaveBeenCalled()
      expect(store.userPhrases).toEqual(mockPhrases)
      expect(result).toEqual(mockResponse)
    })
  })

  describe('loadAllPhrasesForAdmin', () => {
    it('should load admin phrases and update state', async () => {
      const store = usePhrasesStore()
      const mockPhrases = [createMockPhrase(), createMockPhrase({ id: '2' })]
      const mockResponse = { phrases: mockPhrases }
      
      // Mock the service
      const mockService = global.usePhraseService()
      mockService.fetchAllPhrases.mockResolvedValue(mockResponse)
      
      const result = await store.loadAllPhrasesForAdmin()
      
      expect(mockService.fetchAllPhrases).toHaveBeenCalled()
      expect(store.adminPhrases).toEqual(mockPhrases)
      expect(result).toEqual(mockResponse)
    })
  })

  describe('loadSuggestionsForUser', () => {
    it('should load user suggestions and update state', async () => {
      const store = usePhrasesStore()
      const mockSuggestions = [createMockPhraseSuggestion(), createMockPhraseSuggestion({ id: '2' })]
      const mockResponse = { suggestions: mockSuggestions }
      
      // Mock the service
      const mockService = global.usePhraseService()
      mockService.fetchPhraseSuggestions.mockResolvedValue(mockResponse)
      
      const result = await store.loadSuggestionsForUser()
      
      expect(mockService.fetchPhraseSuggestions).toHaveBeenCalled()
      expect(store.userSuggestions).toEqual(mockSuggestions)
      expect(result).toEqual(mockResponse)
    })
  })

  describe('loadAllSuggestionsForAdmin', () => {
    it('should load admin suggestions and update state', async () => {
      const store = usePhrasesStore()
      const mockSuggestions = [createMockPhraseSuggestion(), createMockPhraseSuggestion({ id: '2' })]
      const mockResponse = { suggestions: mockSuggestions }
      
      // Mock the service
      const mockService = global.usePhraseService()
      mockService.fetchPhraseSuggestions.mockResolvedValue(mockResponse)
      
      const result = await store.loadAllSuggestionsForAdmin()
      
      expect(mockService.fetchPhraseSuggestions).toHaveBeenCalled()
      expect(store.adminSuggestions).toEqual(mockSuggestions)
      expect(result).toEqual(mockResponse)
    })
  })

  describe('togglePhraseExclusion', () => {
    it('should exclude phrase when not currently excluded', async () => {
      const store = usePhrasesStore()
      const phrase = createMockPhrase({ is_excluded: false })
      store.userPhrases.splice(0, store.userPhrases.length, phrase)
      
      // Mock the service
      const mockService = global.usePhraseService()
      mockService.excludePhrase.mockResolvedValue({ message: 'Phrase excluded successfully' })
      
      await store.togglePhraseExclusion(phrase.id)
      
      expect(mockService.excludePhrase).toHaveBeenCalledWith(phrase.id)
      expect(phrase.is_excluded).toBe(true)
    })

    it('should remove exclusion when currently excluded', async () => {
      const store = usePhrasesStore()
      const phrase = createMockPhrase({ is_excluded: true })
      store.userPhrases.splice(0, store.userPhrases.length, phrase)
      
      // Mock the service
      const mockService = global.usePhraseService()
      mockService.removePhraseExclusion.mockResolvedValue({ message: 'Phrase exclusion removed successfully' })
      
      await store.togglePhraseExclusion(phrase.id)
      
      expect(mockService.removePhraseExclusion).toHaveBeenCalledWith(phrase.id)
      expect(phrase.is_excluded).toBe(false)
    })

    it('should throw error when phrase not found', async () => {
      const store = usePhrasesStore()
      store.userPhrases = []
      
      await expect(store.togglePhraseExclusion('nonexistent-id'))
        .rejects.toThrow('Phrase not found')
    })
  })

  describe('submitSuggestion', () => {
    it('should submit suggestion and add to user suggestions', async () => {
      const store = usePhrasesStore()
      const mockSuggestion = createMockPhraseSuggestion()
      const mockResponse = { suggestion: mockSuggestion }
      
      // Mock the service
      const mockService = global.usePhraseService()
      mockService.submitPhraseSuggestion.mockResolvedValue(mockResponse)
      
      const result = await store.submitSuggestion('New phrase suggestion')
      
      expect(mockService.submitPhraseSuggestion).toHaveBeenCalledWith('New phrase suggestion')
      expect(store.userSuggestions).toContain(mockSuggestion)
      expect(result).toEqual(mockSuggestion)
    })
  })

  describe('approveSuggestion', () => {
    it('should approve suggestion and update status', async () => {
      const store = usePhrasesStore()
      const suggestion = createMockPhraseSuggestion({ status: 'pending' })
      store.adminSuggestions.splice(0, store.adminSuggestions.length, suggestion)
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.approveSuggestion.mockResolvedValue({ message: 'Suggestion approved successfully' })
      
      const result = await store.approveSuggestion(suggestion.id, 'Great suggestion!')
      
      expect(mockService.approveSuggestion).toHaveBeenCalledWith(suggestion.id, 'Great suggestion!')
      expect(suggestion.status).toBe('approved')
      expect(suggestion.admin_reason).toBe('Great suggestion!')
      expect(result).toEqual(suggestion)
    })

    it('should approve suggestion with empty reason when not provided', async () => {
      const store = usePhrasesStore()
      const suggestion = createMockPhraseSuggestion({ status: 'pending' })
      store.adminSuggestions.splice(0, store.adminSuggestions.length, suggestion)
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.approveSuggestion.mockResolvedValue({ message: 'Suggestion approved successfully' })
      
      await store.approveSuggestion(suggestion.id)
      
      expect(mockService.approveSuggestion).toHaveBeenCalledWith(suggestion.id, '')
    })
  })

  describe('rejectSuggestion', () => {
    it('should reject suggestion and update status', async () => {
      const store = usePhrasesStore()
      const suggestion = createMockPhraseSuggestion({ status: 'pending' })
      store.adminSuggestions.splice(0, store.adminSuggestions.length, suggestion)
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.rejectSuggestion.mockResolvedValue({ message: 'Suggestion rejected successfully' })
      
      const result = await store.rejectSuggestion(suggestion.id, 'Too similar to existing content')
      
      expect(mockService.rejectSuggestion).toHaveBeenCalledWith(suggestion.id, 'Too similar to existing content')
      expect(suggestion.status).toBe('rejected')
      expect(suggestion.admin_reason).toBe('Too similar to existing content')
      expect(result).toEqual(suggestion)
    })

    it('should reject suggestion with empty reason when not provided', async () => {
      const store = usePhrasesStore()
      const suggestion = createMockPhraseSuggestion({ status: 'pending' })
      store.adminSuggestions.splice(0, store.adminSuggestions.length, suggestion)
      
      // Mock the service
      const mockService = global.useAdminService()
      mockService.rejectSuggestion.mockResolvedValue({ message: 'Suggestion rejected successfully' })
      
      await store.rejectSuggestion(suggestion.id)
      
      expect(mockService.rejectSuggestion).toHaveBeenCalledWith(suggestion.id, '')
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
