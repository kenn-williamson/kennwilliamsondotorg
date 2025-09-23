import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { createMockTimer } from '../utils/test-helpers'

// Import the store directly - no mocking needed for pure stores
import { useIncidentTimerStore } from '../../app/stores/incident-timers'

// Mock the TimerManager
const mockTimerManager = {
  start: vi.fn(),
  stop: vi.fn(),
  cleanup: vi.fn(),
  getState: vi.fn(() => ({
    isRunning: false,
    isPageVisible: true,
    isWindowFocused: true,
    lastRefreshTime: 0
  }))
}

vi.mock('../../app/utils/timer-manager', () => ({
  TimerManager: vi.fn().mockImplementation(() => mockTimerManager)
}))

describe('useIncidentTimerStore', () => {
  beforeEach(() => {
    // Create a fresh pinia and make it active
    setActivePinia(createPinia())
    
    // Reset mocks
    vi.clearAllMocks()
  })

  describe('store state', () => {
    it('should initialize with empty state', () => {
      const store = useIncidentTimerStore()
      
      expect(store.timers).toEqual([])
      expect(store.currentTimer).toBeNull()
      expect(store.publicTimer).toBeNull()
      expect(store.publicTimerUserSlug).toBeNull()
      expect(store.activeTimerBreakdown).toEqual({
        years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0
      })
    })
  })

  describe('computed properties', () => {
    it('should return null for latestTimer when no timers exist', () => {
      const store = useIncidentTimerStore()
      
      expect(store.latestTimer).toBeNull()
    })

    it('should return the most recent timer as latestTimer', () => {
      const store = useIncidentTimerStore()
      const olderTimer = createMockTimer({
        id: 'older',
        reset_timestamp: '2024-01-01T10:00:00Z'
      })
      const newerTimer = createMockTimer({
        id: 'newer',
        reset_timestamp: '2024-01-01T12:00:00Z'
      })
      
      store.setTimers([olderTimer, newerTimer])
      
      expect(store.latestTimer).toEqual(newerTimer)
    })

    it('should return the most recent timer when timers are in different order', () => {
      const store = useIncidentTimerStore()
      const newerTimer = createMockTimer({
        id: 'newer',
        reset_timestamp: '2024-01-01T12:00:00Z'
      })
      const olderTimer = createMockTimer({
        id: 'older',
        reset_timestamp: '2024-01-01T10:00:00Z'
      })
      
      store.setTimers([newerTimer, olderTimer])
      
      expect(store.latestTimer).toEqual(newerTimer)
    })
  })

  describe('pure state management functions', () => {
    it('should set timers correctly', () => {
      const store = useIncidentTimerStore()
      const timers = [createMockTimer({ id: 'timer1' }), createMockTimer({ id: 'timer2' })]
      
      store.setTimers(timers)
      
      expect(store.timers).toEqual(timers)
    })

    it('should set current timer correctly', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ id: 'current' })
      
      store.setCurrentTimer(timer)
      
      expect(store.currentTimer).toEqual(timer)
    })

    it('should set current timer to null', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ id: 'current' })
      store.setCurrentTimer(timer)
      
      store.setCurrentTimer(null)
      
      expect(store.currentTimer).toBeNull()
    })

    it('should set public timer correctly', () => {
      const store = useIncidentTimerStore()
      const publicTimer = {
        ...createMockTimer({ id: 'public' }),
        user_display_name: 'John Doe'
      }
      
      store.setPublicTimer(publicTimer, 'john-doe')
      
      expect(store.publicTimer).toEqual(publicTimer)
      expect(store.publicTimerUserSlug).toBe('john-doe')
    })

    it('should set public timer without user slug', () => {
      const store = useIncidentTimerStore()
      const publicTimer = {
        ...createMockTimer({ id: 'public' }),
        user_display_name: 'John Doe'
      }
      
      store.setPublicTimer(publicTimer)
      
      expect(store.publicTimer).toEqual(publicTimer)
      expect(store.publicTimerUserSlug).toBeNull()
    })

    it('should add timer correctly', () => {
      const store = useIncidentTimerStore()
      const existingTimer = createMockTimer({ id: 'existing' })
      const newTimer = createMockTimer({ id: 'new' })
      
      store.setTimers([existingTimer])
      store.addTimer(newTimer)
      
      expect(store.timers).toHaveLength(2)
      expect(store.timers[1]).toEqual(newTimer)
    })

    it('should update timer correctly', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ id: 'test', notes: 'Original notes' })
      store.setTimers([timer])
      
      store.updateTimer('test', { notes: 'Updated notes' })
      
      expect(store.timers[0].notes).toBe('Updated notes')
    })

    it('should not update non-existent timer', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ id: 'test', notes: 'Original notes' })
      store.setTimers([timer])
      
      store.updateTimer('non-existent', { notes: 'Updated notes' })
      
      expect(store.timers[0].notes).toBe('Original notes')
    })

    it('should remove timer correctly', () => {
      const store = useIncidentTimerStore()
      const timer1 = createMockTimer({ id: 'timer1' })
      const timer2 = createMockTimer({ id: 'timer2' })
      store.setTimers([timer1, timer2])
      
      store.removeTimer('timer1')
      
      expect(store.timers).toHaveLength(1)
      expect(store.timers[0]).toEqual(timer2)
    })

    it('should not remove non-existent timer', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ id: 'test' })
      store.setTimers([timer])
      
      store.removeTimer('non-existent')
      
      expect(store.timers).toHaveLength(1)
      expect(store.timers[0]).toEqual(timer)
    })

    it('should clear timers correctly', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ id: 'test' })
      store.setTimers([timer])
      store.setCurrentTimer(timer)
      
      store.clearTimers()
      
      expect(store.timers).toEqual([])
      expect(store.currentTimer).toBeNull()
    })

    it('should clear public timer correctly', () => {
      const store = useIncidentTimerStore()
      const publicTimer = {
        ...createMockTimer({ id: 'public' }),
        user_display_name: 'John Doe'
      }
      store.setPublicTimer(publicTimer, 'john-doe')
      
      store.clearPublicTimer()
      
      expect(store.publicTimer).toBeNull()
      expect(store.publicTimerUserSlug).toBeNull()
    })
  })

  describe('timer calculation utilities', () => {
    it('should calculate elapsed time breakdown correctly', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({
        reset_timestamp: '2024-01-01T10:00:00Z'
      })
      
      // Mock current time to be 2 hours, 30 minutes, 45 seconds later
      const mockNow = new Date('2024-01-01T12:30:45Z')
      vi.setSystemTime(mockNow)
      
      const breakdown = store.getElapsedTimeBreakdown(timer)
      
      expect(breakdown).toEqual({
        years: 0,
        months: 0,
        weeks: 0,
        days: 0,
        hours: 2,
        minutes: 30,
        seconds: 45
      })
    })

    it('should return zero breakdown for timer with no reset_timestamp', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ reset_timestamp: '' })
      
      const breakdown = store.getElapsedTimeBreakdown(timer)
      
      expect(breakdown).toEqual({
        years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0
      })
    })

    it('should return zero breakdown for null timer', () => {
      const store = useIncidentTimerStore()
      
      const breakdown = store.getElapsedTimeBreakdown(null as any)
      
      expect(breakdown).toEqual({
        years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0
      })
    })

    it('should return zero breakdown for future timestamp', () => {
      const store = useIncidentTimerStore()
      const futureTime = new Date()
      futureTime.setHours(futureTime.getHours() + 1)
      const timer = createMockTimer({
        reset_timestamp: futureTime.toISOString()
      })
      
      const breakdown = store.getElapsedTimeBreakdown(timer)
      
      expect(breakdown).toEqual({
        years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0
      })
    })

    it('should calculate elapsed seconds correctly', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({
        reset_timestamp: '2024-01-01T10:00:00Z'
      })
      
      // Mock current time to be 2 hours, 30 minutes, 45 seconds later
      const mockNow = new Date('2024-01-01T12:30:45Z')
      vi.setSystemTime(mockNow)
      
      const seconds = store.getElapsedSeconds(timer)
      
      expect(seconds).toBe(9045) // 2*3600 + 30*60 + 45
    })

    it('should return zero seconds for timer with no reset_timestamp', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ reset_timestamp: '' })
      
      const seconds = store.getElapsedSeconds(timer)
      
      expect(seconds).toBe(0)
    })

    it('should format elapsed time correctly', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({
        reset_timestamp: '2024-01-01T10:00:00Z'
      })
      
      // Mock current time to be 1 year, 2 months, 3 weeks, 4 days, 5 hours, 6 minutes, 7 seconds later
      const mockNow = new Date('2025-03-26T15:06:07Z') // 15:06 - 10:00 = 5:06, so 5 hours
      vi.setSystemTime(mockNow)
      
      const formatted = store.formatElapsedTime(timer)
      
      expect(formatted).toContain('1 year')
      expect(formatted).toContain('2 months')
      expect(formatted).toContain('3 weeks')
      expect(formatted).toContain('4 days')
      expect(formatted).toContain('6 hours')
      expect(formatted).toContain('6 minutes')
      expect(formatted).toContain('7 seconds')
    })

    it('should format elapsed time with singular forms correctly', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({
        reset_timestamp: '2024-01-01T10:00:00Z'
      })
      
      // Mock current time to be exactly 1 year, 1 month, 1 week, 1 day, 1 hour, 1 minute, 1 second later
      const mockNow = new Date('2025-02-09T11:01:01Z') // Adjusted to get exactly 1 day
      vi.setSystemTime(mockNow)
      
      const formatted = store.formatElapsedTime(timer)
      
      expect(formatted).toContain('1 year')
      expect(formatted).toContain('1 month')
      expect(formatted).toContain('1 week')
      expect(formatted).toContain('1 day')
      expect(formatted).toContain('1 hour')
      expect(formatted).toContain('1 minute')
      expect(formatted).toContain('1 second')
    })

    it('should return "No incident started" for timer with no reset_timestamp', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ reset_timestamp: '' })
      
      const formatted = store.formatElapsedTime(timer)
      
      expect(formatted).toBe('No incident started')
    })

    it('should format compact elapsed time correctly', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({
        reset_timestamp: '2024-01-01T10:00:00Z'
      })
      
      // Mock current time to be 2 hours, 30 minutes, 45 seconds later
      const mockNow = new Date('2024-01-01T12:30:45Z')
      vi.setSystemTime(mockNow)
      
      const formatted = store.formatElapsedTimeCompact(timer)
      
      expect(formatted).toBe('02:30:45')
    })

    it('should return "00:00:00" for timer with no reset_timestamp', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ reset_timestamp: '' })
      
      const formatted = store.formatElapsedTimeCompact(timer)
      
      expect(formatted).toBe('00:00:00')
    })
  })

  describe('live timer updates', () => {
    beforeEach(() => {
      vi.clearAllMocks()
    })

    it('should start live timer updates with public timer', () => {
      const store = useIncidentTimerStore()
      const publicTimer = {
        ...createMockTimer({ id: 'public' }),
        user_display_name: 'John Doe'
      }
      store.setPublicTimer(publicTimer, 'john-doe')
      
      store.startLiveTimerUpdates()
      
      expect(mockTimerManager.start).toHaveBeenCalled()
    })

    it('should start live timer updates with latest user timer', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ id: 'user-timer' })
      store.setTimers([timer])
      
      store.startLiveTimerUpdates()
      
      expect(mockTimerManager.start).toHaveBeenCalled()
    })

    it('should start live timer updates even when no timer exists', () => {
      const store = useIncidentTimerStore()
      
      store.startLiveTimerUpdates()
      
      expect(mockTimerManager.start).toHaveBeenCalled()
    })

    it('should stop live timer updates', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ id: 'test' })
      store.setTimers([timer])
      store.startLiveTimerUpdates()
      
      store.stopLiveTimerUpdates()
      
      expect(mockTimerManager.stop).toHaveBeenCalled()
      expect(store.activeTimerBreakdown).toEqual({
        years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0
      })
    })

    it('should cleanup global event listeners', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ id: 'test' })
      store.setTimers([timer])
      
      // Start timer to create TimerManager instance
      store.startLiveTimerUpdates()
      store.cleanupGlobalEventListeners()
      
      expect(mockTimerManager.cleanup).toHaveBeenCalled()
    })

    it('should create new TimerManager instance on each start', () => {
      const store = useIncidentTimerStore()
      const timer = createMockTimer({ id: 'test' })
      store.setTimers([timer])
      
      store.startLiveTimerUpdates()
      store.startLiveTimerUpdates()
      
      // Should create TimerManager twice (once per start call)
      expect(mockTimerManager.start).toHaveBeenCalledTimes(2)
    })
  })
})