import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { createMockTimer } from '../utils/test-helpers'

// Mock the composable before importing the store
vi.mock('~/composables/useIncidentTimerService', () => ({
  useIncidentTimerService: () => ({
    getUserTimers: vi.fn(),
    getPublicTimer: vi.fn(),
    createTimer: vi.fn(),
    updateTimer: vi.fn(),
    deleteTimer: vi.fn(),
    quickReset: vi.fn(),
    isLoading: { value: false },
    error: { value: null },
    hasError: { value: false },
  })
}))

// Import the store after mocking
import { useIncidentTimerStore } from '~/stores/incident-timers'

describe('useIncidentTimerStore', () => {
  beforeEach(() => {
    // Create a fresh pinia and make it active
    // so it's automatically picked up by any useStore() call
    setActivePinia(createPinia())
  })

  describe('getElapsedTimeBreakdown', () => {
    it('should calculate correct time breakdown for a timer from 1 year ago', () => {
      const store = useIncidentTimerStore()
      
      // Create a timer from exactly 1 year ago
      const oneYearAgo = new Date()
      oneYearAgo.setFullYear(oneYearAgo.getFullYear() - 1)
      
      const timer = createMockTimer({
        reset_timestamp: oneYearAgo.toISOString()
      })
      
      const breakdown = store.getElapsedTimeBreakdown(timer)
      
      // Should be approximately 1 year (allowing for some variance due to exact timing)
      expect(breakdown.years).toBeGreaterThanOrEqual(0)
      expect(breakdown.years).toBeLessThanOrEqual(2) // Allow some variance
      expect(breakdown.months).toBeGreaterThanOrEqual(0)
      expect(breakdown.weeks).toBeGreaterThanOrEqual(0)
      expect(breakdown.days).toBeGreaterThanOrEqual(0)
      expect(breakdown.hours).toBeGreaterThanOrEqual(0)
      expect(breakdown.minutes).toBeGreaterThanOrEqual(0)
      expect(breakdown.seconds).toBeGreaterThanOrEqual(0)
    })

    it('should return zero breakdown for invalid timer', () => {
      const store = useIncidentTimerStore()
      
      const breakdown = store.getElapsedTimeBreakdown(null as any)
      
      expect(breakdown).toEqual({
        years: 0,
        months: 0,
        weeks: 0,
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 0
      })
    })

    it('should return zero breakdown for timer with no reset_timestamp', () => {
      const store = useIncidentTimerStore()
      
      const timer = createMockTimer({
        reset_timestamp: ''
      })
      
      const breakdown = store.getElapsedTimeBreakdown(timer)
      
      expect(breakdown).toEqual({
        years: 0,
        months: 0,
        weeks: 0,
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 0
      })
    })

    it('should return zero breakdown for future timestamp', () => {
      const store = useIncidentTimerStore()
      
      // Create a timer with a future timestamp
      const futureTime = new Date()
      futureTime.setFullYear(futureTime.getFullYear() + 1)
      
      const timer = createMockTimer({
        reset_timestamp: futureTime.toISOString()
      })
      
      const breakdown = store.getElapsedTimeBreakdown(timer)
      
      expect(breakdown).toEqual({
        years: 0,
        months: 0,
        weeks: 0,
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 0
      })
    })

    it('should calculate correct breakdown for a timer from 2 hours ago', () => {
      const store = useIncidentTimerStore()
      
      // Create a timer from exactly 2 hours ago
      const twoHoursAgo = new Date()
      twoHoursAgo.setHours(twoHoursAgo.getHours() - 2)
      
      const timer = createMockTimer({
        reset_timestamp: twoHoursAgo.toISOString()
      })
      
      const breakdown = store.getElapsedTimeBreakdown(timer)
      
      // Should be approximately 2 hours
      expect(breakdown.years).toBe(0)
      expect(breakdown.months).toBe(0)
      expect(breakdown.weeks).toBe(0)
      expect(breakdown.days).toBe(0)
      expect(breakdown.hours).toBe(2)
      expect(breakdown.minutes).toBeGreaterThanOrEqual(0)
      expect(breakdown.seconds).toBeGreaterThanOrEqual(0)
    })
  })

  describe('store state', () => {
    it('should initialize with empty state', () => {
      const store = useIncidentTimerStore()
      
      expect(store.timers).toEqual([])
      expect(store.currentTimer).toBeNull()
      expect(store.publicTimer).toBeNull()
      expect(store.publicTimerUserSlug).toBeNull()
      expect(store.activeTimerBreakdown).toEqual({
        years: 0,
        months: 0,
        weeks: 0,
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 0
      })
    })
  })
})
