import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest'
import { TimerManager } from '../../app/utils/timer-manager'

// Mock global objects
const mockSetInterval = vi.fn()
const mockClearInterval = vi.fn()
const mockAddEventListener = vi.fn()
const mockRemoveEventListener = vi.fn()

// Mock document and window
Object.defineProperty(global, 'document', {
  value: {
    hidden: false,
    addEventListener: mockAddEventListener,
    removeEventListener: mockRemoveEventListener,
  },
  writable: true,
})

Object.defineProperty(global, 'window', {
  value: {
    addEventListener: mockAddEventListener,
    removeEventListener: mockRemoveEventListener,
  },
  writable: true,
})

Object.defineProperty(global, 'setInterval', {
  value: mockSetInterval,
  writable: true,
})

Object.defineProperty(global, 'clearInterval', {
  value: mockClearInterval,
  writable: true,
})

describe('TimerManager', () => {
  let timerManager: TimerManager
  let mockUpdateCallback: vi.Mock
  let mockRefreshCallback: vi.Mock
  let mockActiveTimerProvider: vi.Mock

  beforeEach(() => {
    vi.clearAllMocks()
    mockSetInterval.mockReturnValue(123) // Mock interval ID
    
    mockUpdateCallback = vi.fn()
    mockRefreshCallback = vi.fn()
    mockActiveTimerProvider = vi.fn()

    timerManager = new TimerManager(
      mockUpdateCallback,
      mockRefreshCallback,
      mockActiveTimerProvider
    )
  })

  afterEach(() => {
    timerManager.cleanup()
  })

  describe('basic timer functionality', () => {
    it('should start timer when active timer exists', () => {
      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })

      timerManager.start()

      expect(mockSetInterval).toHaveBeenCalledWith(expect.any(Function), 1000)
    })

    it('should not start timer when no active timer exists', () => {
      mockActiveTimerProvider.mockReturnValue(null)

      timerManager.start()

      expect(mockSetInterval).not.toHaveBeenCalled()
      expect(mockUpdateCallback).toHaveBeenCalledWith({
        years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0
      })
    })

    it('should stop timer', () => {
      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })

      timerManager.start()
      timerManager.stop()

      expect(mockClearInterval).toHaveBeenCalledWith(123)
      expect(mockUpdateCallback).toHaveBeenCalledWith({
        years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0
      })
    })

    it('should cleanup properly', () => {
      // Start timer first to setup event listeners
      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })
      timerManager.start()
      
      timerManager.cleanup()

      expect(mockRemoveEventListener).toHaveBeenCalledWith('visibilitychange', expect.any(Function))
      expect(mockRemoveEventListener).toHaveBeenCalledWith('focus', expect.any(Function))
      expect(mockRemoveEventListener).toHaveBeenCalledWith('blur', expect.any(Function))
    })
  })

  describe('timer calculation', () => {
    it('should calculate elapsed time correctly', () => {
      const mockNow = new Date('2024-01-01T12:30:45Z')
      vi.setSystemTime(mockNow)

      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })

      timerManager.start()

      // Get the interval callback and call it
      const intervalCallback = mockSetInterval.mock.calls[0][0]
      intervalCallback()

      expect(mockUpdateCallback).toHaveBeenCalledWith({
        years: 0,
        months: 0,
        weeks: 0,
        days: 0,
        hours: 2,
        minutes: 30,
        seconds: 45
      })
    })

    it('should stop timer when active timer disappears', () => {
      mockActiveTimerProvider
        .mockReturnValueOnce({
          id: 'test-timer',
          reset_timestamp: '2024-01-01T10:00:00Z',
          isPublic: false
        })
        .mockReturnValueOnce(null) // Timer disappears on second call

      timerManager.start()

      // Get the interval callback and call it
      const intervalCallback = mockSetInterval.mock.calls[0][0]
      intervalCallback()

      expect(mockClearInterval).toHaveBeenCalledWith(123)
    })
  })

  describe('browser event handling', () => {
    it('should setup event listeners on start', () => {
      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })

      timerManager.start()

      expect(mockAddEventListener).toHaveBeenCalledWith('visibilitychange', expect.any(Function))
      expect(mockAddEventListener).toHaveBeenCalledWith('focus', expect.any(Function))
      expect(mockAddEventListener).toHaveBeenCalledWith('blur', expect.any(Function))
    })

    it('should not setup event listeners twice', () => {
      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })

      timerManager.start()
      timerManager.start()

      // Should only setup listeners once
      expect(mockAddEventListener).toHaveBeenCalledTimes(3) // visibilitychange, focus, blur
    })

    it('should handle page visibility change', async () => {
      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })

      timerManager.start()

      // Get the visibility change handler
      const visibilityHandler = mockAddEventListener.mock.calls.find(
        call => call[0] === 'visibilitychange'
      )[1]

      // Simulate page becoming visible (was hidden, now visible)
      Object.defineProperty(global.document, 'hidden', { value: true })
      await visibilityHandler() // First call - page was visible, now hidden
      
      Object.defineProperty(global.document, 'hidden', { value: false })
      await visibilityHandler() // Second call - page was hidden, now visible

      expect(mockRefreshCallback).toHaveBeenCalled()
    })

    it('should handle window focus', async () => {
      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })

      timerManager.start()

      // Get the blur and focus handlers
      const blurHandler = mockAddEventListener.mock.calls.find(
        call => call[0] === 'blur'
      )[1]
      const focusHandler = mockAddEventListener.mock.calls.find(
        call => call[0] === 'focus'
      )[1]

      // Simulate window blur first (sets isWindowFocused = false)
      blurHandler()

      // Simulate window focus (was not focused, now focused)
      await focusHandler()

      expect(mockRefreshCallback).toHaveBeenCalled()
    })

    it('should handle window blur', () => {
      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })

      timerManager.start()

      // Get the blur handler
      const blurHandler = mockAddEventListener.mock.calls.find(
        call => call[0] === 'blur'
      )[1]

      // Simulate window blur
      blurHandler()

      // Should not call refresh on blur
      expect(mockRefreshCallback).not.toHaveBeenCalled()
    })
  })

  describe('refresh throttling', () => {
    it('should throttle rapid refresh calls', async () => {
      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })

      timerManager.start()

      // Get the visibility change handler
      const visibilityHandler = mockAddEventListener.mock.calls.find(
        call => call[0] === 'visibilitychange'
      )[1]

      // Simulate rapid visibility changes
      Object.defineProperty(global.document, 'hidden', { value: true })
      await visibilityHandler() // First call - page was visible, now hidden
      
      Object.defineProperty(global.document, 'hidden', { value: false })
      await visibilityHandler() // Second call - page was hidden, now visible
      
      Object.defineProperty(global.document, 'hidden', { value: true })
      await visibilityHandler() // Third call - page was visible, now hidden
      
      Object.defineProperty(global.document, 'hidden', { value: false })
      await visibilityHandler() // Fourth call - page was hidden, now visible

      // Should only call refresh once due to throttling
      expect(mockRefreshCallback).toHaveBeenCalledTimes(1)
    })
  })

  describe('state management', () => {
    it('should return correct state', () => {
      const state = timerManager.getState()

      expect(state).toEqual({
        isRunning: false,
        isPageVisible: true,
        isWindowFocused: true,
        lastRefreshTime: 0
      })
    })

    it('should update state when timer starts', () => {
      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })

      timerManager.start()

      const state = timerManager.getState()
      expect(state.isRunning).toBe(true)
    })
  })

  describe('edge cases', () => {
    it('should handle refresh callback errors', async () => {
      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: '2024-01-01T10:00:00Z',
        isPublic: false
      })

      mockRefreshCallback.mockRejectedValue(new Error('Refresh failed'))

      timerManager.start()

      // Get the visibility change handler
      const visibilityHandler = mockAddEventListener.mock.calls.find(
        call => call[0] === 'visibilitychange'
      )[1]

      // Should not throw error
      Object.defineProperty(global.document, 'hidden', { value: true })
      await visibilityHandler() // First call - page was visible, now hidden
      
      Object.defineProperty(global.document, 'hidden', { value: false })
      await visibilityHandler() // Second call - page was hidden, now visible

      expect(mockRefreshCallback).toHaveBeenCalled()
    })

    it('should handle future timestamps', () => {
      const futureTime = new Date()
      futureTime.setHours(futureTime.getHours() + 1)

      mockActiveTimerProvider.mockReturnValue({
        id: 'test-timer',
        reset_timestamp: futureTime.toISOString(),
        isPublic: false
      })

      timerManager.start()

      // Get the interval callback and call it
      const intervalCallback = mockSetInterval.mock.calls[0][0]
      intervalCallback()

      expect(mockUpdateCallback).toHaveBeenCalledWith({
        years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0
      })
    })
  })
})
