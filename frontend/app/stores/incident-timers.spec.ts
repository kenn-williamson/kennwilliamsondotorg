/**
 * Incident Timer Store Tests
 * Tests the enhanced incident timer store with actions and transient state
 */

import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useIncidentTimerStore } from './incident-timers'
import { incidentTimerService } from '~/services/incidentTimerService'

// Mock the service layer
const mockServiceMethods = {
  getUserTimers: vi.fn(),
  getPublicTimer: vi.fn(),
  createTimer: vi.fn(),
  updateTimer: vi.fn(),
  deleteTimer: vi.fn(),
  quickReset: vi.fn(),
}

vi.mock('~/services/incidentTimerService', () => ({
  incidentTimerService: vi.fn(() => mockServiceMethods)
}))

// Mock the backend fetch composable
vi.mock('~/composables/useBackendFetch', () => ({
  useBackendFetch: vi.fn(() => vi.fn())
}))

// Mock the timer manager utility
vi.mock('~/utils/timer-manager', () => ({
  TimerManager: vi.fn().mockImplementation(() => ({
    start: vi.fn(),
    stop: vi.fn(),
    cleanup: vi.fn(),
  }))
}))

describe('Incident Timer Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  describe('Initial State', () => {
    it('should have correct initial state', () => {
      const store = useIncidentTimerStore()
      
      expect(store.timers).toEqual([])
      expect(store.currentTimer).toBe(null)
      expect(store.publicTimer).toBe(null)
      expect(store.publicTimerUserSlug).toBe(null)
      expect(store.activeTimerBreakdown).toEqual({ years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 })
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })

    it('should have correct computed properties', () => {
      const store = useIncidentTimerStore()
      
      expect(store.latestTimer).toBe(null)
      expect(store.hasError).toBe(false)
    })
  })

  describe('loadUserTimers', () => {
    it('should load user timers successfully', async () => {
      const store = useIncidentTimerStore()
      const mockTimers = [
        { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Test timer 1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' },
        { id: '2', reset_timestamp: '2024-01-02T12:00:00Z', notes: 'Test timer 2', created_at: '2024-01-02T12:00:00Z', updated_at: '2024-01-02T12:00:00Z' }
      ]
      
      mockServiceMethods.getUserTimers.mockResolvedValue(mockTimers)

      const result = await store.loadUserTimers()

      expect(mockServiceMethods.getUserTimers).toHaveBeenCalled()
      expect(store.timers).toEqual(mockTimers)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(mockTimers)
    })

    it('should handle error when loading user timers', async () => {
      const store = useIncidentTimerStore()
      const mockError = new Error('API Failure')
      
      mockServiceMethods.getUserTimers.mockRejectedValue(mockError)

      await expect(store.loadUserTimers()).rejects.toThrow('API Failure')
      
      expect(store.error).toBe('API Failure')
      expect(store.isLoading).toBe(false)
      expect(store.timers).toEqual([])
    })
  })

  describe('loadPublicTimer', () => {
    it('should load public timer successfully', async () => {
      const store = useIncidentTimerStore()
      const mockTimer = { 
        id: '1', 
        reset_timestamp: '2024-01-01T12:00:00Z', 
        notes: 'Public timer',
        user_display_name: 'Test User',
        created_at: '2024-01-01T12:00:00Z',
        updated_at: '2024-01-01T12:00:00Z'
      }
      
      mockServiceMethods.getPublicTimer.mockResolvedValue(mockTimer)

      const result = await store.loadPublicTimer('test-user')

      expect(mockServiceMethods.getPublicTimer).toHaveBeenCalledWith('test-user')
      expect(store.publicTimer).toEqual(mockTimer)
      expect(store.publicTimerUserSlug).toBe('test-user')
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(mockTimer)
    })

    it('should handle error when loading public timer', async () => {
      const store = useIncidentTimerStore()
      const mockError = new Error('Public timer not found')
      
      mockServiceMethods.getPublicTimer.mockRejectedValue(mockError)

      await expect(store.loadPublicTimer('test-user')).rejects.toThrow('Public timer not found')
      
      expect(store.error).toBe('Public timer not found')
      expect(store.isLoading).toBe(false)
      expect(store.publicTimer).toBe(null)
    })
  })

  describe('createTimer', () => {
    it('should create timer successfully', async () => {
      const store = useIncidentTimerStore()
      const mockTimer = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'New timer', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      const timerData = { reset_timestamp: '2024-01-01T12:00:00Z', notes: 'New timer' }
      
      mockServiceMethods.createTimer.mockResolvedValue(mockTimer)

      const result = await store.createTimer(timerData)

      expect(mockServiceMethods.createTimer).toHaveBeenCalledWith(timerData)
      expect(store.timers).toContain(mockTimer)
      expect(store.currentTimer).toEqual(mockTimer)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(mockTimer)
    })

    it('should handle error when creating timer', async () => {
      const store = useIncidentTimerStore()
      const mockError = new Error('Creation failed')
      const timerData = { reset_timestamp: '2024-01-01T12:00:00Z', notes: 'New timer' }
      
      mockServiceMethods.createTimer.mockRejectedValue(mockError)

      await expect(store.createTimer(timerData)).rejects.toThrow('Creation failed')
      
      expect(store.error).toBe('Creation failed')
      expect(store.isLoading).toBe(false)
      expect(store.timers).toEqual([])
    })
  })

  describe('updateTimer', () => {
    it('should update timer successfully', async () => {
      const store = useIncidentTimerStore()
      const existingTimer = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Old notes', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      const updatedTimer = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'New notes', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      const updateData = { notes: 'New notes' }
      
      store.timers = [existingTimer]
      store.currentTimer = existingTimer
      
      mockServiceMethods.updateTimer.mockResolvedValue(updatedTimer)

      const result = await store.updateTimer('1', updateData)

      expect(mockServiceMethods.updateTimer).toHaveBeenCalledWith('1', updateData)
      expect(store.timers[0]).toEqual(updatedTimer)
      expect(store.currentTimer).toEqual(updatedTimer)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(updatedTimer)
    })

    it('should handle error when updating timer', async () => {
      const store = useIncidentTimerStore()
      const mockError = new Error('Update failed')
      const updateData = { notes: 'New notes' }
      
      mockServiceMethods.updateTimer.mockRejectedValue(mockError)

      await expect(store.updateTimer('1', updateData)).rejects.toThrow('Update failed')
      
      expect(store.error).toBe('Update failed')
      expect(store.isLoading).toBe(false)
    })
  })

  describe('deleteTimer', () => {
    it('should delete timer successfully', async () => {
      const store = useIncidentTimerStore()
      const timerToDelete = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Timer to delete', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      const remainingTimer = { id: '2', reset_timestamp: '2024-01-02T12:00:00Z', notes: 'Remaining timer', created_at: '2024-01-02T12:00:00Z', updated_at: '2024-01-02T12:00:00Z' }
      
      store.timers = [timerToDelete, remainingTimer]
      store.currentTimer = timerToDelete
      
      mockServiceMethods.deleteTimer.mockResolvedValue(undefined)

      await store.deleteTimer('1')

      expect(mockServiceMethods.deleteTimer).toHaveBeenCalledWith('1')
      expect(store.timers).toEqual([remainingTimer])
      expect(store.currentTimer).toBe(null)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
    })

    it('should handle error when deleting timer', async () => {
      const store = useIncidentTimerStore()
      const mockError = new Error('Delete failed')
      
      mockServiceMethods.deleteTimer.mockRejectedValue(mockError)

      await expect(store.deleteTimer('1')).rejects.toThrow('Delete failed')
      
      expect(store.error).toBe('Delete failed')
      expect(store.isLoading).toBe(false)
    })
  })

  describe('quickReset', () => {
    it('should reset timer successfully', async () => {
      const store = useIncidentTimerStore()
      const originalTimer = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Original timer', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      const resetTimer = { id: '1', reset_timestamp: '2024-01-02T12:00:00Z', notes: 'Reset timer', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-02T12:00:00Z' }
      
      store.timers = [originalTimer]
      store.currentTimer = originalTimer
      
      mockServiceMethods.quickReset.mockResolvedValue(resetTimer)

      const result = await store.quickReset('1')

      expect(mockServiceMethods.quickReset).toHaveBeenCalledWith('1')
      expect(store.timers[0]).toEqual(resetTimer)
      expect(store.currentTimer).toEqual(resetTimer)
      expect(store.isLoading).toBe(false)
      expect(store.error).toBe(null)
      expect(result).toEqual(resetTimer)
    })

    it('should handle error when resetting timer', async () => {
      const store = useIncidentTimerStore()
      const mockError = new Error('Reset failed')
      
      mockServiceMethods.quickReset.mockRejectedValue(mockError)

      await expect(store.quickReset('1')).rejects.toThrow('Reset failed')
      
      expect(store.error).toBe('Reset failed')
      expect(store.isLoading).toBe(false)
    })
  })

  describe('Computed Properties', () => {
    it('should correctly compute latestTimer', () => {
      const store = useIncidentTimerStore()
      const timers = [
        { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Older timer', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' },
        { id: '2', reset_timestamp: '2024-01-02T12:00:00Z', notes: 'Newer timer', created_at: '2024-01-02T12:00:00Z', updated_at: '2024-01-02T12:00:00Z' }
      ]
      store.timers = timers
      
      expect(store.latestTimer?.id).toBe('2')
      expect(store.latestTimer?.notes).toBe('Newer timer')
    })

    it('should return null for latestTimer when no timers', () => {
      const store = useIncidentTimerStore()
      store.timers = []
      
      expect(store.latestTimer).toBe(null)
    })

    it('should correctly compute hasError', () => {
      const store = useIncidentTimerStore()
      
      expect(store.hasError).toBe(false)
      
      store.error = 'Some error'
      expect(store.hasError).toBe(true)
    })
  })

  describe('Utility Functions', () => {
    it('should format elapsed time correctly', () => {
      const store = useIncidentTimerStore()
      const timer = {
        id: '1',
        reset_timestamp: new Date(Date.now() - 3661000).toISOString(), // 1 hour, 1 minute, 1 second ago
        notes: 'Test timer',
        created_at: '2024-01-01T12:00:00Z',
        updated_at: '2024-01-01T12:00:00Z'
      }
      
      const formatted = store.formatElapsedTime(timer)
      expect(formatted).toContain('1 hour')
      expect(formatted).toContain('1 minute')
      expect(formatted).toContain('1 second')
    })

    it('should format compact elapsed time correctly', () => {
      const store = useIncidentTimerStore()
      const timer = {
        id: '1',
        reset_timestamp: new Date(Date.now() - 3661000).toISOString(), // 1 hour, 1 minute, 1 second ago
        notes: 'Test timer',
        created_at: '2024-01-01T12:00:00Z',
        updated_at: '2024-01-01T12:00:00Z'
      }
      
      const formatted = store.formatElapsedTimeCompact(timer)
      expect(formatted).toMatch(/^\d{2}:\d{2}:\d{2}$/)
    })

    it('should get elapsed seconds correctly', () => {
      const store = useIncidentTimerStore()
      const timer = {
        id: '1',
        reset_timestamp: new Date(Date.now() - 5000).toISOString(), // 5 seconds ago
        notes: 'Test timer',
        created_at: '2024-01-01T12:00:00Z',
        updated_at: '2024-01-01T12:00:00Z'
      }
      
      const seconds = store.getElapsedSeconds(timer)
      expect(seconds).toBeGreaterThanOrEqual(4)
      expect(seconds).toBeLessThanOrEqual(6)
    })
  })

  describe('State Management Functions', () => {
    it('should set timers correctly', () => {
      const store = useIncidentTimerStore()
      const timers = [
        { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Timer 1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' },
        { id: '2', reset_timestamp: '2024-01-02T12:00:00Z', notes: 'Timer 2', created_at: '2024-01-02T12:00:00Z', updated_at: '2024-01-02T12:00:00Z' }
      ]
      
      store.setTimers(timers)
      
      expect(store.timers).toEqual(timers)
    })

    it('should set current timer correctly', () => {
      const store = useIncidentTimerStore()
      const timer = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Current timer', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      
      store.setCurrentTimer(timer)
      
      expect(store.currentTimer).toEqual(timer)
    })

    it('should set public timer correctly', () => {
      const store = useIncidentTimerStore()
      const timer = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Public timer', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' }
      
      store.setPublicTimer(timer, 'test-user')
      
      expect(store.publicTimer).toEqual(timer)
      expect(store.publicTimerUserSlug).toBe('test-user')
    })

    it('should add timer correctly', () => {
      const store = useIncidentTimerStore()
      const timer = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'New timer', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      
      store.addTimer(timer)
      
      expect(store.timers).toContain(timer)
    })

    it('should update timer store correctly', () => {
      const store = useIncidentTimerStore()
      const originalTimer = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Original', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      const updates = { notes: 'Updated' }
      
      store.timers = [originalTimer]
      store.updateTimerStore('1', updates)
      
      expect(store.timers[0]?.notes).toBe('Updated')
    })

    it('should remove timer correctly', () => {
      const store = useIncidentTimerStore()
      const timer1 = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Timer 1', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      const timer2 = { id: '2', reset_timestamp: '2024-01-02T12:00:00Z', notes: 'Timer 2', created_at: '2024-01-02T12:00:00Z', updated_at: '2024-01-02T12:00:00Z' }
      
      store.timers = [timer1, timer2]
      store.removeTimer('1')
      
      expect(store.timers).toEqual([timer2])
    })

    it('should clear timers correctly', () => {
      const store = useIncidentTimerStore()
      const timer = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Timer', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z' }
      
      store.timers = [timer]
      store.currentTimer = timer
      store.clearTimers()
      
      expect(store.timers).toEqual([])
      expect(store.currentTimer).toBe(null)
    })

    it('should clear public timer correctly', () => {
      const store = useIncidentTimerStore()
      const timer = { id: '1', reset_timestamp: '2024-01-01T12:00:00Z', notes: 'Public timer', created_at: '2024-01-01T12:00:00Z', updated_at: '2024-01-01T12:00:00Z', user_display_name: 'Test User' }
      
      store.publicTimer = timer
      store.publicTimerUserSlug = 'test-user'
      store.clearPublicTimer()
      
      expect(store.publicTimer).toBe(null)
      expect(store.publicTimerUserSlug).toBe(null)
    })
  })
})
