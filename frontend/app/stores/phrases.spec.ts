/**
 * Phrases Store Tests
 * Tests the enhanced phrases store with actions and transient state
 */

import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { usePhrasesStore } from './phrases'
import { phraseService } from '~/services/phraseService'

// Mock the service layer
const mockPhraseServiceMethods = {
  fetchUserPhrases: vi.fn(),
  fetchAllPhrases: vi.fn(),
  fetchPhraseSuggestions: vi.fn(),
  excludePhrase: vi.fn(),
  removePhraseExclusion: vi.fn(),
  submitPhraseSuggestion: vi.fn(),
  approveSuggestion: vi.fn(),
  rejectSuggestion: vi.fn(),
  fetchRandomPhraseAuth: vi.fn(),
  fetchRandomPhraseClient: vi.fn(),
}

vi.mock('~/services/phraseService', () => ({
  phraseService: vi.fn(() => mockPhraseServiceMethods)
}))

// Mock the backend fetch composable
vi.mock('~/composables/useBackendFetch', () => ({
  useBackendFetch: vi.fn(() => vi.fn())
}))

describe('Phrases Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  describe('Initial State', () => {
    it('should have correct initial state', () => {
      const store = usePhrasesStore()
      
      expect(store.userPhrases).toEqual([])
      expect(store.adminPhrases).toEqual([])
      expect(store.userSuggestions).toEqual([])
      expect(store.adminSuggestions).toEqual([])
      expect(store.currentPhrase).toBe(null)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })

    it('should have correct computed properties', () => {
      const store = usePhrasesStore()
      
      expect(store.hasUserPhrases).toBe(false)
      expect(store.hasAdminPhrases).toBe(false)
      expect(store.hasUserSuggestions).toBe(false)
      expect(store.hasAdminSuggestions).toBe(false)
      expect(store.hasError).toBe(false)
      expect(store.activePhrases).toEqual([])
      expect(store.pendingSuggestions).toEqual([])
    })
  })

  describe('loadPhrasesForUser', () => {
    it('should load user phrases successfully', async () => {
      const store = usePhrasesStore()
      const mockPhrases = [
        { id: '1', phrase_text: 'Test phrase 1', is_excluded: false, active: true, created_by: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' },
        { id: '2', phrase_text: 'Test phrase 2', is_excluded: true, active: true, created_by: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      ]
      const mockResponse = { phrases: mockPhrases }
      
      mockPhraseServiceMethods.fetchUserPhrases.mockResolvedValue(mockResponse)

      const result = await store.loadPhrasesForUser()

      expect(mockPhraseServiceMethods.fetchUserPhrases).toHaveBeenCalled()
      expect(store.userPhrases).toEqual(mockPhrases)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(mockResponse)
    })

    it('should handle error when loading user phrases', async () => {
      const store = usePhrasesStore()
      const mockError = new Error('API Failure')
      
      mockPhraseServiceMethods.fetchUserPhrases.mockRejectedValue(mockError)

      await expect(store.loadPhrasesForUser()).rejects.toThrow('API Failure')
      
      expect(store.error).toBe('API Failure')
      expect(store.isLoading).toBe(false)
      expect(store.userPhrases).toEqual([])
    })
  })

  describe('loadAllPhrasesForAdmin', () => {
    it('should load admin phrases successfully', async () => {
      const store = usePhrasesStore()
      const mockPhrases = [
        { id: '1', phrase_text: 'Admin phrase 1', active: true, created_by: 'admin1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' },
        { id: '2', phrase_text: 'Admin phrase 2', active: false, created_by: 'admin1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      ]
      const mockResponse = { phrases: mockPhrases, total: 2 }
      
      mockPhraseServiceMethods.fetchAllPhrases.mockResolvedValue(mockResponse)

      const result = await store.loadAllPhrasesForAdmin()

      expect(mockPhraseServiceMethods.fetchAllPhrases).toHaveBeenCalled()
      expect(store.adminPhrases).toEqual(mockPhrases)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(mockResponse)
    })

    it('should handle error when loading admin phrases', async () => {
      const store = usePhrasesStore()
      const mockError = new Error('Admin API Failure')
      
      mockPhraseServiceMethods.fetchAllPhrases.mockRejectedValue(mockError)

      await expect(store.loadAllPhrasesForAdmin()).rejects.toThrow('Admin API Failure')
      
      expect(store.error).toBe('Admin API Failure')
      expect(store.isLoading).toBe(false)
      expect(store.adminPhrases).toEqual([])
    })
  })

  describe('loadSuggestionsForUser', () => {
    it('should load user suggestions successfully', async () => {
      const store = usePhrasesStore()
      const mockSuggestions = [
        { id: '1', phrase_text: 'User suggestion 1', status: 'pending', user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' },
        { id: '2', phrase_text: 'User suggestion 2', status: 'approved', user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' }
      ]
      const mockResponse = { suggestions: mockSuggestions, total: 2 }
      
      mockPhraseServiceMethods.fetchPhraseSuggestions.mockResolvedValue(mockResponse)

      const result = await store.loadSuggestionsForUser()

      expect(mockPhraseServiceMethods.fetchPhraseSuggestions).toHaveBeenCalled()
      expect(store.userSuggestions).toEqual(mockSuggestions)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(mockResponse)
    })
  })

  describe('loadAllSuggestionsForAdmin', () => {
    it('should load admin suggestions successfully', async () => {
      const store = usePhrasesStore()
      const mockSuggestions = [
        { id: '1', phrase_text: 'Admin suggestion 1', status: 'pending', user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' },
        { id: '2', phrase_text: 'Admin suggestion 2', status: 'rejected', user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' }
      ]
      const mockResponse = { suggestions: mockSuggestions, total: 2 }
      
      mockPhraseServiceMethods.fetchPhraseSuggestions.mockResolvedValue(mockResponse)

      const result = await store.loadAllSuggestionsForAdmin()

      expect(mockPhraseServiceMethods.fetchPhraseSuggestions).toHaveBeenCalled()
      expect(store.adminSuggestions).toEqual(mockSuggestions)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(mockResponse)
    })
  })

  describe('togglePhraseExclusion', () => {
    it('should exclude phrase successfully', async () => {
      const store = usePhrasesStore()
      const mockPhrase = { id: '1', phrase_text: 'Test phrase', is_excluded: false, active: true, created_by: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      store.userPhrases = [mockPhrase]
      
      mockPhraseServiceMethods.excludePhrase.mockResolvedValue({ message: 'Phrase excluded successfully' })

      await store.togglePhraseExclusion('1')

      expect(mockPhraseServiceMethods.excludePhrase).toHaveBeenCalledWith('1')
      expect(mockPhrase.is_excluded).toBe(true)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })

    it('should remove phrase exclusion successfully', async () => {
      const store = usePhrasesStore()
      const mockPhrase = { id: '1', phrase_text: 'Test phrase', is_excluded: true, active: true, created_by: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      store.userPhrases = [mockPhrase]
      
      mockPhraseServiceMethods.removePhraseExclusion.mockResolvedValue({ message: 'Phrase exclusion removed successfully' })

      await store.togglePhraseExclusion('1')

      expect(mockPhraseServiceMethods.removePhraseExclusion).toHaveBeenCalledWith('1')
      expect(mockPhrase.is_excluded).toBe(false)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })

    it('should throw error if phrase not found', async () => {
      const store = usePhrasesStore()
      
      await expect(store.togglePhraseExclusion('nonexistent')).rejects.toThrow('Phrase not found')
    })
  })

  describe('submitSuggestion', () => {
    it('should submit suggestion successfully', async () => {
      const store = usePhrasesStore()
      const mockSuggestion = { id: '1', phrase_text: 'New suggestion', status: 'pending', user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' }
      const mockResponse = { suggestion: mockSuggestion }
      
      mockPhraseServiceMethods.submitPhraseSuggestion.mockResolvedValue(mockResponse)

      const result = await store.submitSuggestion('New suggestion')

      expect(mockPhraseServiceMethods.submitPhraseSuggestion).toHaveBeenCalledWith('New suggestion')
      expect(store.userSuggestions).toContain(mockSuggestion)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(mockResponse)
    })

    it('should handle error when submitting suggestion', async () => {
      const store = usePhrasesStore()
      const mockError = new Error('Submission failed')
      
      mockPhraseServiceMethods.submitPhraseSuggestion.mockRejectedValue(mockError)

      await expect(store.submitSuggestion('New suggestion')).rejects.toThrow('Submission failed')
      
      expect(store.error).toBe('Submission failed')
      expect(store.isLoading).toBe(false)
      expect(store.userSuggestions).toEqual([])
    })
  })

  describe('approveSuggestion', () => {
    it('should approve suggestion successfully', async () => {
      const store = usePhrasesStore()
      const mockSuggestion = { id: '1', phrase_text: 'Test suggestion', status: 'pending' as const, user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User', admin_reason: undefined }
      store.adminSuggestions = [mockSuggestion]
      
      mockPhraseServiceMethods.approveSuggestion.mockResolvedValue({ message: 'Suggestion approved successfully' })

      const result = await store.approveSuggestion('1', 'Great suggestion!')

      expect(mockPhraseServiceMethods.approveSuggestion).toHaveBeenCalledWith('1', 'Great suggestion!')
      expect(mockSuggestion.status).toBe('approved')
      expect(mockSuggestion.admin_reason).toBe('Great suggestion!')
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(mockSuggestion)
    })
  })

  describe('rejectSuggestion', () => {
    it('should reject suggestion successfully', async () => {
      const store = usePhrasesStore()
      const mockSuggestion = { id: '1', phrase_text: 'Test suggestion', status: 'pending' as const, user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User', admin_reason: undefined }
      store.adminSuggestions = [mockSuggestion]
      
      mockPhraseServiceMethods.rejectSuggestion.mockResolvedValue({ message: 'Suggestion rejected successfully' })

      const result = await store.rejectSuggestion('1', 'Not appropriate')

      expect(mockPhraseServiceMethods.rejectSuggestion).toHaveBeenCalledWith('1', 'Not appropriate')
      expect(mockSuggestion.status).toBe('rejected')
      expect(mockSuggestion.admin_reason).toBe('Not appropriate')
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(mockSuggestion)
    })
  })

  describe('fetchRandomPhraseAuth', () => {
    it('should fetch random phrase for authenticated user', async () => {
      const store = usePhrasesStore()
      const mockPhrase = 'Vigilance Maintained - Until the Next Challenge Arises'
      
      mockPhraseServiceMethods.fetchRandomPhraseAuth.mockResolvedValue(mockPhrase)

      const result = await store.fetchRandomPhraseAuth()

      expect(mockPhraseServiceMethods.fetchRandomPhraseAuth).toHaveBeenCalled()
      expect(store.currentPhrase).toBe(mockPhrase)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toBe(mockPhrase)
    })
  })

  describe('fetchRandomPhraseClient', () => {
    it('should fetch random phrase for public user', async () => {
      const store = usePhrasesStore()
      const mockPhrase = 'Public phrase display'
      
      mockPhraseServiceMethods.fetchRandomPhraseClient.mockResolvedValue(mockPhrase)

      const result = await store.fetchRandomPhraseClient('test-user')

      expect(mockPhraseServiceMethods.fetchRandomPhraseClient).toHaveBeenCalledWith('test-user')
      expect(store.currentPhrase).toBe(mockPhrase)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toBe(mockPhrase)
    })
  })

  describe('Computed Properties', () => {
    it('should correctly compute hasUserPhrases', () => {
      const store = usePhrasesStore()
      
      expect(store.hasUserPhrases).toBe(false)
      
      store.userPhrases = [{ id: '1', phrase_text: 'Test', is_excluded: false, active: true, created_by: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }]
      expect(store.hasUserPhrases).toBe(true)
    })

    it('should correctly compute activePhrases', () => {
      const store = usePhrasesStore()
      store.adminPhrases = [
        { id: '1', phrase_text: 'Active phrase', active: true, created_by: 'admin1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' },
        { id: '2', phrase_text: 'Inactive phrase', active: false, created_by: 'admin1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      ]
      
      expect(store.activePhrases).toHaveLength(1)
      expect(store.activePhrases[0]?.phrase_text).toBe('Active phrase')
    })

    it('should correctly compute pendingSuggestions', () => {
      const store = usePhrasesStore()
      store.adminSuggestions = [
        { id: '1', phrase_text: 'Pending suggestion', status: 'pending', user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' },
        { id: '2', phrase_text: 'Approved suggestion', status: 'approved', user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' }
      ]
      
      expect(store.pendingSuggestions).toHaveLength(1)
      expect(store.pendingSuggestions[0]?.phrase_text).toBe('Pending suggestion')
    })
  })

  describe('Utility Functions', () => {
    it('should clear user data', () => {
      const store = usePhrasesStore()
      store.userPhrases = [{ id: '1', phrase_text: 'Test', is_excluded: false, active: true, created_by: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }]
      store.userSuggestions = [{ id: '1', phrase_text: 'Test', status: 'pending', user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' }]
      
      store.clearUserData()
      
      expect(store.userPhrases).toEqual([])
      expect(store.userSuggestions).toEqual([])
    })

    it('should clear admin data', () => {
      const store = usePhrasesStore()
      store.adminPhrases = [{ id: '1', phrase_text: 'Test', active: true, created_by: 'admin1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }]
      store.adminSuggestions = [{ id: '1', phrase_text: 'Test', status: 'pending', user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' }]
      
      store.clearAdminData()
      
      expect(store.adminPhrases).toEqual([])
      expect(store.adminSuggestions).toEqual([])
    })

    it('should clear all data', () => {
      const store = usePhrasesStore()
      store.userPhrases = [{ id: '1', phrase_text: 'Test', is_excluded: false, active: true, created_by: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }]
      store.adminPhrases = [{ id: '1', phrase_text: 'Test', active: true, created_by: 'admin1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }]
      store.userSuggestions = [{ id: '1', phrase_text: 'Test', status: 'pending', user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' }]
      store.adminSuggestions = [{ id: '1', phrase_text: 'Test', status: 'pending', user_id: 'user1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' }]
      
      store.clearAllData()
      
      expect(store.userPhrases).toEqual([])
      expect(store.adminPhrases).toEqual([])
      expect(store.userSuggestions).toEqual([])
      expect(store.adminSuggestions).toEqual([])
    })
  })
})
