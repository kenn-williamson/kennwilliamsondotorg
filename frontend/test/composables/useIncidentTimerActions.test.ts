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

vi.mock('~/services/incidentTimerService', () => ({
  incidentTimerService: vi.fn()
}))

vi.mock('~/stores/incident-timers', () => ({
  useIncidentTimerStore: vi.fn()
}))

import { useIncidentTimerActions } from '~/composables/useIncidentTimerActions'

describe('useIncidentTimerActions', () => {
  let mockIncidentTimerService: any
  let mockIncidentTimerStore: any

  beforeEach(async () => {
    // Reset all mocks
    vi.clearAllMocks()

    mockIncidentTimerService = {
      getUserTimers: vi.fn(),
      getPublicTimer: vi.fn(),
      createTimer: vi.fn(),
      updateTimer: vi.fn(),
      deleteTimer: vi.fn(),
      quickReset: vi.fn()
    }
    
    mockIncidentTimerStore = {
      setTimers: vi.fn(),
      setPublicTimer: vi.fn(),
      addTimer: vi.fn(),
      setCurrentTimer: vi.fn(),
      updateTimer: vi.fn(),
      removeTimer: vi.fn(),
      currentTimer: { id: 'timer-1' }, // Match the timer ID used in tests
      startLiveTimerUpdates: vi.fn(),
      stopLiveTimerUpdates: vi.fn()
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
    
    const { incidentTimerService } = await import('~/services/incidentTimerService')
    vi.mocked(incidentTimerService).mockReturnValue(mockIncidentTimerService)
    
    const { useIncidentTimerStore } = await import('~/stores/incident-timers')
    vi.mocked(useIncidentTimerStore).mockReturnValue(mockIncidentTimerStore)
  })

  describe('loadUserTimers orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const mockTimers = [
        { id: 'timer-1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Test timer 1' },
        { id: 'timer-2', reset_timestamp: '2024-01-02T12:00:00Z', notes: 'Test timer 2' }
      ]
      
      // Setup service mock
      mockIncidentTimerService.getUserTimers.mockResolvedValue(mockTimers)

      const { loadUserTimers } = useIncidentTimerActions()
      const result = await loadUserTimers()

      // Test orchestration: service called
      expect(mockIncidentTimerService.getUserTimers).toHaveBeenCalled()
      
      // Test orchestration: store updated with timers
      expect(mockIncidentTimerStore.setTimers).toHaveBeenCalledWith(mockTimers)
      
      // Test orchestration: live updates started (timers present)
      expect(mockIncidentTimerStore.startLiveTimerUpdates).toHaveBeenCalled()
      
      // Test orchestration: result returned
      expect(result).toEqual(mockTimers)
    })

    it('should not start live updates when no timers', async () => {
      // Setup service mock to return empty array
      mockIncidentTimerService.getUserTimers.mockResolvedValue([])

      const { loadUserTimers } = useIncidentTimerActions()
      await loadUserTimers()

      // Test orchestration: store updated with empty array
      expect(mockIncidentTimerStore.setTimers).toHaveBeenCalledWith([])
      
      // Test orchestration: live updates not started (no timers)
      expect(mockIncidentTimerStore.startLiveTimerUpdates).not.toHaveBeenCalled()
    })
  })

  describe('loadPublicTimer orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const userSlug = 'test-user'
      const mockTimer = { id: 'timer-1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Public timer' }
      
      // Setup service mock
      mockIncidentTimerService.getPublicTimer.mockResolvedValue(mockTimer)

      const { loadPublicTimer } = useIncidentTimerActions()
      const result = await loadPublicTimer(userSlug)

      // Test orchestration: service called with correct slug
      expect(mockIncidentTimerService.getPublicTimer).toHaveBeenCalledWith(userSlug)
      
      // Test orchestration: store updated with timer and slug
      expect(mockIncidentTimerStore.setPublicTimer).toHaveBeenCalledWith(mockTimer, userSlug)
      
      // Test orchestration: live updates started (timer present)
      expect(mockIncidentTimerStore.startLiveTimerUpdates).toHaveBeenCalled()
      
      // Test orchestration: result returned
      expect(result).toEqual(mockTimer)
    })

    it('should not start live updates when no timer', async () => {
      const userSlug = 'test-user'
      
      // Setup service mock to return null
      mockIncidentTimerService.getPublicTimer.mockResolvedValue(null)

      const { loadPublicTimer } = useIncidentTimerActions()
      await loadPublicTimer(userSlug)

      // Test orchestration: store updated with null timer
      expect(mockIncidentTimerStore.setPublicTimer).toHaveBeenCalledWith(null, userSlug)
      
      // Test orchestration: live updates not started (no timer)
      expect(mockIncidentTimerStore.startLiveTimerUpdates).not.toHaveBeenCalled()
    })
  })

  describe('createTimer orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const timerData = {
        reset_timestamp: '2024-01-01T12:00:00Z',
        notes: 'New timer'
      }
      const createdTimer = { id: 'new-timer-id', ...timerData }
      
      // Setup service mock
      mockIncidentTimerService.createTimer.mockResolvedValue(createdTimer)

      const { createTimer } = useIncidentTimerActions()
      const result = await createTimer(timerData)

      // Test orchestration: service called with correct data
      expect(mockIncidentTimerService.createTimer).toHaveBeenCalledWith(timerData)
      
      // Test orchestration: store updated with new timer
      expect(mockIncidentTimerStore.addTimer).toHaveBeenCalledWith(createdTimer)
      expect(mockIncidentTimerStore.setCurrentTimer).toHaveBeenCalledWith(createdTimer)
      
      // Test orchestration: result returned
      expect(result).toEqual(createdTimer)
    })
  })

  describe('updateTimer orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const timerId = 'timer-1'
      const timerData = {
        reset_timestamp: '2024-01-01T13:00:00Z',
        notes: 'Updated timer'
      }
      const updatedTimer = { id: timerId, ...timerData }
      
      // Setup service mock
      mockIncidentTimerService.updateTimer.mockResolvedValue(updatedTimer)

      const { updateTimer } = useIncidentTimerActions()
      const result = await updateTimer(timerId, timerData)

      // Test orchestration: service called with correct parameters
      expect(mockIncidentTimerService.updateTimer).toHaveBeenCalledWith(timerId, timerData)
      
      // Test orchestration: store updated with updated timer
      expect(mockIncidentTimerStore.updateTimer).toHaveBeenCalledWith(timerId, updatedTimer)
      
      // Test orchestration: current timer updated (matches current timer ID)
      expect(mockIncidentTimerStore.setCurrentTimer).toHaveBeenCalledWith(updatedTimer)
      
      // Test orchestration: result returned
      expect(result).toEqual(updatedTimer)
    })

    it('should not update current timer when different timer', async () => {
      const timerId = 'different-timer-id'
      const timerData = { notes: 'Updated timer' }
      const updatedTimer = { id: timerId, ...timerData }
      
      // Setup service mock
      mockIncidentTimerService.updateTimer.mockResolvedValue(updatedTimer)

      const { updateTimer } = useIncidentTimerActions()
      await updateTimer(timerId, timerData)

      // Test orchestration: store updated with updated timer
      expect(mockIncidentTimerStore.updateTimer).toHaveBeenCalledWith(timerId, updatedTimer)
      
      // Test orchestration: current timer not updated (different timer ID)
      expect(mockIncidentTimerStore.setCurrentTimer).not.toHaveBeenCalled()
    })
  })

  describe('deleteTimer orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const timerId = 'timer-1'
      
      // Setup service mock
      mockIncidentTimerService.deleteTimer.mockResolvedValue(undefined)

      const { deleteTimer } = useIncidentTimerActions()
      await deleteTimer(timerId)

      // Test orchestration: service called with correct ID
      expect(mockIncidentTimerService.deleteTimer).toHaveBeenCalledWith(timerId)
      
      // Test orchestration: store updated to remove timer
      expect(mockIncidentTimerStore.removeTimer).toHaveBeenCalledWith(timerId)
      
      // Test orchestration: current timer cleared (matches current timer ID)
      expect(mockIncidentTimerStore.setCurrentTimer).toHaveBeenCalledWith(null)
    })

    it('should not clear current timer when different timer', async () => {
      const timerId = 'different-timer-id'
      
      // Setup service mock
      mockIncidentTimerService.deleteTimer.mockResolvedValue(undefined)

      const { deleteTimer } = useIncidentTimerActions()
      await deleteTimer(timerId)

      // Test orchestration: store updated to remove timer
      expect(mockIncidentTimerStore.removeTimer).toHaveBeenCalledWith(timerId)
      
      // Test orchestration: current timer not cleared (different timer ID)
      expect(mockIncidentTimerStore.setCurrentTimer).not.toHaveBeenCalled()
    })
  })

  describe('quickReset orchestration', () => {
    it('should orchestrate service call and store updates', async () => {
      const timerId = 'timer-1'
      const resetTimer = { id: timerId, reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Reset timer' }
      
      // Setup service mock
      mockIncidentTimerService.quickReset.mockResolvedValue(resetTimer)

      const { quickReset } = useIncidentTimerActions()
      const result = await quickReset(timerId)

      // Test orchestration: service called with correct ID
      expect(mockIncidentTimerService.quickReset).toHaveBeenCalledWith(timerId)
      
      // Test orchestration: store updated with reset timer
      expect(mockIncidentTimerStore.updateTimer).toHaveBeenCalledWith(timerId, resetTimer)
      
      // Test orchestration: current timer updated (matches current timer ID)
      expect(mockIncidentTimerStore.setCurrentTimer).toHaveBeenCalledWith(resetTimer)
      
      // Test orchestration: result returned
      expect(result).toEqual(resetTimer)
    })
  })

  describe('backward compatibility aliases', () => {
    it('should provide fetchTimers alias for loadUserTimers', () => {
      const { fetchTimers, loadUserTimers } = useIncidentTimerActions()
      
      // Test interface: aliases are the same function
      expect(fetchTimers).toBe(loadUserTimers)
    })

    it('should provide fetchPublicTimer alias for loadPublicTimer', () => {
      const { fetchPublicTimer, loadPublicTimer } = useIncidentTimerActions()
      
      // Test interface: aliases are the same function
      expect(fetchPublicTimer).toBe(loadPublicTimer)
    })
  })

  describe('service instantiation', () => {
    it('should create incidentTimerService with correct fetcher', async () => {
      useIncidentTimerActions()

      // Test orchestration: service created with fetcher
      const { incidentTimerService } = await import('~/services/incidentTimerService')
      expect(incidentTimerService).toHaveBeenCalledWith(expect.any(Function))
    })
  })

  describe('interface contract', () => {
    it('should expose all expected methods and state', () => {
      const actions = useIncidentTimerActions()

      // Test interface: all methods present
      expect(actions).toHaveProperty('loadUserTimers')
      expect(actions).toHaveProperty('loadPublicTimer')
      expect(actions).toHaveProperty('createTimer')
      expect(actions).toHaveProperty('updateTimer')
      expect(actions).toHaveProperty('deleteTimer')
      expect(actions).toHaveProperty('quickReset')
      expect(actions).toHaveProperty('fetchTimers')
      expect(actions).toHaveProperty('fetchPublicTimer')
      expect(actions).toHaveProperty('startLiveTimerUpdates')
      expect(actions).toHaveProperty('stopLiveTimerUpdates')
      
      // Test interface: state from useBaseService exposed
      expect(actions).toHaveProperty('isLoading')
      expect(actions).toHaveProperty('error')
      expect(actions).toHaveProperty('hasError')
      
      // Test interface: methods are functions
      expect(typeof actions.loadUserTimers).toBe('function')
      expect(typeof actions.loadPublicTimer).toBe('function')
      expect(typeof actions.createTimer).toBe('function')
      expect(typeof actions.updateTimer).toBe('function')
      expect(typeof actions.deleteTimer).toBe('function')
      expect(typeof actions.quickReset).toBe('function')
      expect(typeof actions.fetchTimers).toBe('function')
      expect(typeof actions.fetchPublicTimer).toBe('function')
      expect(typeof actions.startLiveTimerUpdates).toBe('function')
      expect(typeof actions.stopLiveTimerUpdates).toBe('function')
    })
  })
})
