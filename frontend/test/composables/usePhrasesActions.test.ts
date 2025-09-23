import { describe, it, expect, vi, beforeEach } from 'vitest'

// Mock all dependencies before importing the composable
vi.mock('~/composables/useBaseService', () => ({
  useBaseService: vi.fn()
}))

vi.mock('~/composables/useBackendFetch', () => ({
  useBackendFetch: vi.fn()
}))

vi.mock('~/composables/useAuthFetch', () => ({
  useAuthFetch: vi.fn()
}))

vi.mock('~/services/phraseService', () => ({
  phraseService: vi.fn()
}))

vi.mock('~/stores/phrases', () => ({
  usePhrasesStore: vi.fn()
}))

import { usePhrasesActions } from '~/composables/usePhrasesActions'

describe('usePhrasesActions', () => {
  let mockPhraseService: any
  let mockPhrasesStore: any

  beforeEach(async () => {
    // Reset all mocks
    vi.clearAllMocks()

    mockPhraseService = {
      fetchUserPhrases: vi.fn(),
      fetchAllPhrases: vi.fn(),
      excludePhrase: vi.fn(),
      removePhraseExclusion: vi.fn(),
      submitPhraseSuggestion: vi.fn(),
      fetchPhraseSuggestions: vi.fn(),
      fetchRandomPhraseAuth: vi.fn(),
      fetchRandomPhraseClient: vi.fn(),
      approveSuggestion: vi.fn(),
      rejectSuggestion: vi.fn()
    }
    
    mockPhrasesStore = {
      setUserPhrases: vi.fn(),
      setAdminPhrases: vi.fn(),
      setUserSuggestions: vi.fn(),
      setAdminSuggestions: vi.fn(),
      addUserSuggestion: vi.fn(),
      togglePhraseExclusion: vi.fn(),
      updateSuggestionStatus: vi.fn(),
      userPhrases: [
        { id: 'phrase-1', phrase_text: 'Test phrase 1', is_excluded: false },
        { id: 'phrase-2', phrase_text: 'Test phrase 2', is_excluded: true }
      ],
      adminSuggestions: [
        { id: 'suggestion-1', phrase_text: 'Test suggestion 1', status: 'pending' },
        { id: 'suggestion-2', phrase_text: 'Test suggestion 2', status: 'pending' }
      ]
    }
    
    // Configure mocked modules
    const { useBaseService } = await import('~/composables/useBaseService')
    vi.mocked(useBaseService).mockReturnValue({
      executeRequest: vi.fn().mockImplementation(async (fn) => await fn()),
      executeRequestWithSuccess: vi.fn().mockImplementation(async (fn) => await fn()),
      isLoading: { value: false },
      error: { value: null },
      hasError: { value: false }
    })
    
    const { useBackendFetch } = await import('~/composables/useBackendFetch')
    vi.mocked(useBackendFetch).mockReturnValue(vi.fn())
    
    const { useAuthFetch } = await import('~/composables/useAuthFetch')
    vi.mocked(useAuthFetch).mockReturnValue(vi.fn())
    
    const { phraseService } = await import('~/services/phraseService')
    vi.mocked(phraseService).mockReturnValue(mockPhraseService)
    
    const { usePhrasesStore } = await import('~/stores/phrases')
    vi.mocked(usePhrasesStore).mockReturnValue(mockPhrasesStore)
  })

  describe('loadPhrasesForUser orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const mockResponse = {
        phrases: [
          { id: 'phrase-1', phrase_text: 'Test phrase 1', is_excluded: false },
          { id: 'phrase-2', phrase_text: 'Test phrase 2', is_excluded: true }
        ]
      }
      
      // Setup service mock
      mockPhraseService.fetchUserPhrases.mockResolvedValue(mockResponse)

      const { loadPhrasesForUser } = usePhrasesActions()
      const result = await loadPhrasesForUser()

      // Test orchestration: service called
      expect(mockPhraseService.fetchUserPhrases).toHaveBeenCalled()
      
      // Test orchestration: store updated with phrases
      expect(mockPhrasesStore.setUserPhrases).toHaveBeenCalledWith(mockResponse.phrases)
      
      // Test orchestration: result returned
      expect(result).toEqual(mockResponse)
    })
  })

  describe('loadAllPhrasesForAdmin orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const mockResponse = {
        phrases: [
          { id: 'phrase-1', phrase_text: 'Admin phrase 1', active: true },
          { id: 'phrase-2', phrase_text: 'Admin phrase 2', active: false }
        ]
      }
      
      // Setup service mock
      mockPhraseService.fetchAllPhrases.mockResolvedValue(mockResponse)

      const { loadAllPhrasesForAdmin } = usePhrasesActions()
      const result = await loadAllPhrasesForAdmin()

      // Test orchestration: service called
      expect(mockPhraseService.fetchAllPhrases).toHaveBeenCalled()
      
      // Test orchestration: store updated with phrases
      expect(mockPhrasesStore.setAdminPhrases).toHaveBeenCalledWith(mockResponse.phrases)
      
      // Test orchestration: result returned
      expect(result).toEqual(mockResponse)
    })
  })

  describe('loadSuggestionsForUser orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const mockResponse = {
        suggestions: [
          { id: 'suggestion-1', phrase_text: 'User suggestion 1', status: 'pending' },
          { id: 'suggestion-2', phrase_text: 'User suggestion 2', status: 'approved' }
        ]
      }
      
      // Setup service mock
      mockPhraseService.fetchPhraseSuggestions.mockResolvedValue(mockResponse)

      const { loadSuggestionsForUser } = usePhrasesActions()
      const result = await loadSuggestionsForUser()

      // Test orchestration: service called
      expect(mockPhraseService.fetchPhraseSuggestions).toHaveBeenCalled()
      
      // Test orchestration: store updated with suggestions
      expect(mockPhrasesStore.setUserSuggestions).toHaveBeenCalledWith(mockResponse.suggestions)
      
      // Test orchestration: result returned
      expect(result).toEqual(mockResponse)
    })
  })

  describe('loadAllSuggestionsForAdmin orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const mockResponse = {
        suggestions: [
          { id: 'suggestion-1', phrase_text: 'Admin suggestion 1', status: 'pending' },
          { id: 'suggestion-2', phrase_text: 'Admin suggestion 2', status: 'rejected' }
        ]
      }
      
      // Setup service mock
      mockPhraseService.fetchPhraseSuggestions.mockResolvedValue(mockResponse)

      const { loadAllSuggestionsForAdmin } = usePhrasesActions()
      const result = await loadAllSuggestionsForAdmin()

      // Test orchestration: service called
      expect(mockPhraseService.fetchPhraseSuggestions).toHaveBeenCalled()
      
      // Test orchestration: store updated with suggestions
      expect(mockPhrasesStore.setAdminSuggestions).toHaveBeenCalledWith(mockResponse.suggestions)
      
      // Test orchestration: result returned
      expect(result).toEqual(mockResponse)
    })
  })

  describe('togglePhraseExclusion orchestration', () => {
    it('should exclude phrase when not excluded', async () => {
      const phraseId = 'phrase-1' // is_excluded: false
      
      // Setup service mock
      mockPhraseService.excludePhrase.mockResolvedValue(undefined)

      const { togglePhraseExclusion } = usePhrasesActions()
      await togglePhraseExclusion(phraseId)

      // Test orchestration: service called to exclude phrase
      expect(mockPhraseService.excludePhrase).toHaveBeenCalledWith(phraseId)
      
      // Test orchestration: store updated to toggle exclusion
      expect(mockPhrasesStore.togglePhraseExclusion).toHaveBeenCalledWith(phraseId)
    })

    it('should remove exclusion when phrase is excluded', async () => {
      const phraseId = 'phrase-2' // is_excluded: true
      
      // Setup service mock
      mockPhraseService.removePhraseExclusion.mockResolvedValue(undefined)

      const { togglePhraseExclusion } = usePhrasesActions()
      await togglePhraseExclusion(phraseId)

      // Test orchestration: service called to remove exclusion
      expect(mockPhraseService.removePhraseExclusion).toHaveBeenCalledWith(phraseId)
      
      // Test orchestration: store updated to toggle exclusion
      expect(mockPhrasesStore.togglePhraseExclusion).toHaveBeenCalledWith(phraseId)
    })

    it('should throw error when phrase not found', async () => {
      const phraseId = 'non-existent-phrase'
      
      const { togglePhraseExclusion } = usePhrasesActions()
      
      // Test orchestration: error thrown when phrase not found
      await expect(togglePhraseExclusion(phraseId)).rejects.toThrow('Phrase not found')
      
      // Test orchestration: no service calls made
      expect(mockPhraseService.excludePhrase).not.toHaveBeenCalled()
      expect(mockPhraseService.removePhraseExclusion).not.toHaveBeenCalled()
      expect(mockPhrasesStore.togglePhraseExclusion).not.toHaveBeenCalled()
    })
  })

  describe('submitSuggestion orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const phraseText = 'New phrase suggestion'
      const mockResponse = {
        suggestion: { id: 'new-suggestion-id', phrase_text: phraseText, status: 'pending' }
      }
      
      // Setup service mock
      mockPhraseService.submitPhraseSuggestion.mockResolvedValue(mockResponse)

      const { submitSuggestion } = usePhrasesActions()
      const result = await submitSuggestion(phraseText)

      // Test orchestration: service called with correct text
      expect(mockPhraseService.submitPhraseSuggestion).toHaveBeenCalledWith(phraseText)
      
      // Test orchestration: store updated with new suggestion
      expect(mockPhrasesStore.addUserSuggestion).toHaveBeenCalledWith(mockResponse.suggestion)
      
      // Test orchestration: result returned
      expect(result).toEqual(mockResponse.suggestion)
    })
  })

  describe('approveSuggestion orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const suggestionId = 'suggestion-1'
      const reason = 'Great suggestion!'
      
      // Setup service mock
      mockPhraseService.approveSuggestion.mockResolvedValue(undefined)

      const { approveSuggestion } = usePhrasesActions()
      const result = await approveSuggestion(suggestionId, reason)

      // Test orchestration: service called with correct parameters
      expect(mockPhraseService.approveSuggestion).toHaveBeenCalledWith(suggestionId, reason)
      
      // Test orchestration: store updated with approval status
      expect(mockPhrasesStore.updateSuggestionStatus).toHaveBeenCalledWith(suggestionId, 'approved', reason)
      
      // Test orchestration: result returned (found suggestion)
      expect(result).toEqual(mockPhrasesStore.adminSuggestions[0])
    })

    it('should use empty string for service but undefined for store when no reason provided', async () => {
      const suggestionId = 'suggestion-1'
      
      // Setup service mock
      mockPhraseService.approveSuggestion.mockResolvedValue(undefined)

      const { approveSuggestion } = usePhrasesActions()
      await approveSuggestion(suggestionId)

      // Test orchestration: service called with empty string for reason
      expect(mockPhraseService.approveSuggestion).toHaveBeenCalledWith(suggestionId, '')
      
      // Test orchestration: store updated with approval status and undefined reason
      expect(mockPhrasesStore.updateSuggestionStatus).toHaveBeenCalledWith(suggestionId, 'approved', undefined)
    })
  })

  describe('rejectSuggestion orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const suggestionId = 'suggestion-2'
      const reason = 'Too similar to existing phrase'
      
      // Setup service mock
      mockPhraseService.rejectSuggestion.mockResolvedValue(undefined)

      const { rejectSuggestion } = usePhrasesActions()
      const result = await rejectSuggestion(suggestionId, reason)

      // Test orchestration: service called with correct parameters
      expect(mockPhraseService.rejectSuggestion).toHaveBeenCalledWith(suggestionId, reason)
      
      // Test orchestration: store updated with rejection status
      expect(mockPhrasesStore.updateSuggestionStatus).toHaveBeenCalledWith(suggestionId, 'rejected', reason)
      
      // Test orchestration: result returned (found suggestion)
      expect(result).toEqual(mockPhrasesStore.adminSuggestions[1])
    })

    it('should use empty string for service but undefined for store when no reason provided', async () => {
      const suggestionId = 'suggestion-2'
      
      // Setup service mock
      mockPhraseService.rejectSuggestion.mockResolvedValue(undefined)

      const { rejectSuggestion } = usePhrasesActions()
      await rejectSuggestion(suggestionId)

      // Test orchestration: service called with empty string for reason
      expect(mockPhraseService.rejectSuggestion).toHaveBeenCalledWith(suggestionId, '')
      
      // Test orchestration: store updated with rejection status and undefined reason
      expect(mockPhrasesStore.updateSuggestionStatus).toHaveBeenCalledWith(suggestionId, 'rejected', undefined)
    })
  })

  describe('fetchRandomPhraseAuth orchestration', () => {
    it('should orchestrate service call', async () => {
      const mockPhrase = 'Random motivational phrase'
      
      // Setup service mock
      mockPhraseService.fetchRandomPhraseAuth.mockResolvedValue(mockPhrase)

      const { fetchRandomPhraseAuth } = usePhrasesActions()
      const result = await fetchRandomPhraseAuth()

      // Test orchestration: service called
      expect(mockPhraseService.fetchRandomPhraseAuth).toHaveBeenCalled()
      
      // Test orchestration: result returned
      expect(result).toEqual(mockPhrase)
    })
  })

  describe('fetchRandomPhraseClient orchestration', () => {
    it('should orchestrate service call with user slug', async () => {
      const userSlug = 'test-user'
      const mockPhrase = 'Random phrase for user'
      
      // Setup service mock
      mockPhraseService.fetchRandomPhraseClient.mockResolvedValue(mockPhrase)

      const { fetchRandomPhraseClient } = usePhrasesActions()
      const result = await fetchRandomPhraseClient(userSlug)

      // Test orchestration: service called with correct slug
      expect(mockPhraseService.fetchRandomPhraseClient).toHaveBeenCalledWith(userSlug)
      
      // Test orchestration: result returned
      expect(result).toEqual(mockPhrase)
    })
  })

  describe('backward compatibility aliases', () => {
    it('should provide fetchPhraseSuggestions alias for loadSuggestionsForUser', () => {
      const { fetchPhraseSuggestions, loadSuggestionsForUser } = usePhrasesActions()
      
      // Test interface: aliases are the same function
      expect(fetchPhraseSuggestions).toBe(loadSuggestionsForUser)
    })

    it('should provide submitPhraseSuggestion alias for submitSuggestion', () => {
      const { submitPhraseSuggestion, submitSuggestion } = usePhrasesActions()
      
      // Test interface: aliases are the same function
      expect(submitPhraseSuggestion).toBe(submitSuggestion)
    })
  })

  describe('service instantiation', () => {
    it('should create phraseService with correct fetcher', async () => {
      usePhrasesActions()

      // Test orchestration: service created with fetcher
      const { phraseService } = await import('~/services/phraseService')
      expect(phraseService).toHaveBeenCalledWith(expect.any(Function))
    })
  })

  describe('interface contract', () => {
    it('should expose all expected methods and state', () => {
      const actions = usePhrasesActions()

      // Test interface: all methods present
      expect(actions).toHaveProperty('loadPhrasesForUser')
      expect(actions).toHaveProperty('loadAllPhrasesForAdmin')
      expect(actions).toHaveProperty('loadSuggestionsForUser')
      expect(actions).toHaveProperty('loadAllSuggestionsForAdmin')
      expect(actions).toHaveProperty('togglePhraseExclusion')
      expect(actions).toHaveProperty('submitSuggestion')
      expect(actions).toHaveProperty('approveSuggestion')
      expect(actions).toHaveProperty('rejectSuggestion')
      expect(actions).toHaveProperty('fetchRandomPhraseAuth')
      expect(actions).toHaveProperty('fetchRandomPhraseClient')
      expect(actions).toHaveProperty('fetchPhraseSuggestions')
      expect(actions).toHaveProperty('submitPhraseSuggestion')
      
      // Test interface: state from useBaseService exposed
      expect(actions).toHaveProperty('isLoading')
      expect(actions).toHaveProperty('error')
      expect(actions).toHaveProperty('hasError')
      
      // Test interface: methods are functions
      expect(typeof actions.loadPhrasesForUser).toBe('function')
      expect(typeof actions.loadAllPhrasesForAdmin).toBe('function')
      expect(typeof actions.loadSuggestionsForUser).toBe('function')
      expect(typeof actions.loadAllSuggestionsForAdmin).toBe('function')
      expect(typeof actions.togglePhraseExclusion).toBe('function')
      expect(typeof actions.submitSuggestion).toBe('function')
      expect(typeof actions.approveSuggestion).toBe('function')
      expect(typeof actions.rejectSuggestion).toBe('function')
      expect(typeof actions.fetchRandomPhraseAuth).toBe('function')
      expect(typeof actions.fetchRandomPhraseClient).toBe('function')
      expect(typeof actions.fetchPhraseSuggestions).toBe('function')
      expect(typeof actions.submitPhraseSuggestion).toBe('function')
    })
  })
})
