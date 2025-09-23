/**
 * TimerManager - Handles all timer-related browser behavior
 * Separates timer management from store state management
 */

export interface TimerUpdateCallback {
  (breakdown: {
    years: number
    months: number
    weeks: number
    days: number
    hours: number
    minutes: number
    seconds: number
  }): void
}

export interface DataRefreshCallback {
  (): Promise<void>
}

export interface ActiveTimerProvider {
  (): {
    id: string
    reset_timestamp: string
    isPublic?: boolean
  } | null
}

export class TimerManager {
  private intervalId: NodeJS.Timeout | null = null
  private visibilityHandlerAttached = false
  private visibilityChangeHandler: (() => void) | null = null
  private windowFocusHandler: (() => void) | null = null
  private blurHandler: (() => void) | null = null
  private isPageVisible = true
  private isWindowFocused = true
  private lastRefreshTime = 0
  private refreshThrottleMs = 1000 // Prevent excessive refreshes

  constructor(
    private updateCallback: TimerUpdateCallback,
    private refreshCallback: DataRefreshCallback,
    private activeTimerProvider: ActiveTimerProvider
  ) {}

  /**
   * Start the timer with sophisticated browser focus handling
   */
  start(): void {
    this.stop()
    this.setupBrowserEventHandlers()
    
    const activeTimer = this.activeTimerProvider()
    if (!activeTimer?.reset_timestamp) {
      console.log('🔴 No active timer to track')
      this.updateCallback({
        years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0
      })
      return
    }

    console.log('🟢 Starting live timer updates for:', {
      id: activeTimer.id,
      reset_timestamp: activeTimer.reset_timestamp,
      isPublic: activeTimer.isPublic
    })

    this.intervalId = setInterval(() => {
      this.updateTimer()
    }, 1000)
  }

  /**
   * Stop the timer and reset state
   */
  stop(): void {
    if (this.intervalId) {
      console.log('⏹️ Stopping live timer updates')
      clearInterval(this.intervalId)
      this.intervalId = null
    }
    
    this.updateCallback({
      years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0
    })
  }

  /**
   * Clean up all event listeners
   */
  cleanup(): void {
    this.stop()
    this.removeBrowserEventHandlers()
  }

  /**
   * Update the timer display
   */
  private updateTimer(): void {
    const activeTimer = this.activeTimerProvider()
    if (!activeTimer?.reset_timestamp) {
      console.log('🔴 No active timer found, stopping updates')
      this.stop()
      return
    }

    const breakdown = this.calculateElapsedTimeBreakdown(activeTimer.reset_timestamp)
    this.updateCallback(breakdown)
    
    console.log('🔄 Timer tick:', {
      seconds: breakdown.seconds,
      total: `${breakdown.years}y ${breakdown.months}m ${breakdown.weeks}w ${breakdown.days}d ${breakdown.hours}h ${breakdown.minutes}min ${breakdown.seconds}s`
    })
  }

  /**
   * Setup sophisticated browser event handlers
   */
  private setupBrowserEventHandlers(): void {
    if (typeof document === 'undefined' || this.visibilityHandlerAttached) {
      return
    }

    this.visibilityHandlerAttached = true

    // Handle page visibility changes (tab switching)
    this.visibilityChangeHandler = () => {
      const wasVisible = this.isPageVisible
      this.isPageVisible = !document.hidden
      
      if (!wasVisible && this.isPageVisible) {
        console.log('👁️ Page visible again, refreshing data and restarting timer')
        this.handlePageVisible()
      }
    }

    // Handle window focus (browser window focus)
    this.windowFocusHandler = () => {
      const wasFocused = this.isWindowFocused
      this.isWindowFocused = true
      
      if (!wasFocused) {
        console.log('🎯 Window focused, refreshing data and restarting timer')
        this.handleWindowFocused()
      }
    }

    // Handle window blur (browser window loses focus)
    this.blurHandler = () => {
      this.isWindowFocused = false
      console.log('😴 Window blurred, timer continues but no refresh on focus')
    }

    document.addEventListener('visibilitychange', this.visibilityChangeHandler)
    window.addEventListener('focus', this.windowFocusHandler)
    window.addEventListener('blur', this.blurHandler)
  }

  /**
   * Remove all browser event handlers
   */
  private removeBrowserEventHandlers(): void {
    if (typeof document === 'undefined' || !this.visibilityHandlerAttached) {
      return
    }

    console.log('🧹 Cleaning up page visibility event listeners')

    if (this.visibilityChangeHandler) {
      document.removeEventListener('visibilitychange', this.visibilityChangeHandler)
      this.visibilityChangeHandler = null
    }

    if (this.windowFocusHandler) {
      window.removeEventListener('focus', this.windowFocusHandler)
      this.windowFocusHandler = null
    }

    if (this.blurHandler) {
      window.removeEventListener('blur', this.blurHandler)
      this.blurHandler = null
    }

    this.visibilityHandlerAttached = false
  }

  /**
   * Handle page becoming visible (tab focus)
   */
  private async handlePageVisible(): Promise<void> {
    // Always refresh when page becomes visible (tab focus)
    await this.throttledRefresh()
  }

  /**
   * Handle window focus (browser window focus)
   */
  private async handleWindowFocused(): Promise<void> {
    // Only refresh if the page is also visible
    // This prevents unnecessary refreshes when window gains focus but page is hidden
    if (this.isPageVisible) {
      await this.throttledRefresh()
    }
  }

  /**
   * Throttled refresh to prevent excessive API calls
   */
  private async throttledRefresh(): Promise<void> {
    const now = Date.now()
    if (now - this.lastRefreshTime < this.refreshThrottleMs) {
      console.log('⏱️ Refresh throttled, skipping')
      return
    }

    this.lastRefreshTime = now
    try {
      await this.refreshCallback()
    } catch (error) {
      console.error('❌ Error refreshing timer data:', error)
    }
  }

  /**
   * Calculate elapsed time breakdown (extracted from store)
   */
  private calculateElapsedTimeBreakdown(resetTimestamp: string): {
    years: number
    months: number
    weeks: number
    days: number
    hours: number
    minutes: number
    seconds: number
  } {
    const startDate = new Date(resetTimestamp)
    const now = new Date()
    
    if (now < startDate) {
      return { years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 }
    }
    
    // Step 1: Calculate years and months properly
    let years = now.getFullYear() - startDate.getFullYear()
    let months = now.getMonth() - startDate.getMonth()
    let days = now.getDate() - startDate.getDate()
    
    // Adjust if the current day is before the start day
    if (days < 0) {
      months--
      // Add the days from the previous month
      const prevMonth = new Date(now.getFullYear(), now.getMonth(), 0)
      days += prevMonth.getDate()
    }
    
    // Adjust if months is negative
    if (months < 0) {
      years--
      months += 12
    }
    
    // Step 2: Calculate remaining time (hours, minutes, seconds)
    let hours = now.getHours() - startDate.getHours()
    let minutes = now.getMinutes() - startDate.getMinutes()
    let seconds = now.getSeconds() - startDate.getSeconds()
    
    // Adjust seconds
    if (seconds < 0) {
      minutes--
      seconds += 60
    }
    
    // Adjust minutes
    if (minutes < 0) {
      hours--
      minutes += 60
    }
    
    // Adjust hours
    if (hours < 0) {
      days--
      hours += 24
    }
    
    // Adjust days again if needed
    if (days < 0) {
      months--
      if (months < 0) {
        years--
        months += 12
      }
      const prevMonth = new Date(now.getFullYear(), now.getMonth(), 0)
      days += prevMonth.getDate()
    }
    
    // Calculate weeks from remaining days
    const weeks = Math.floor(days / 7)
    days = days % 7
    
    return { years, months, weeks, days, hours, minutes, seconds }
  }

  /**
   * Get current state for debugging
   */
  getState() {
    return {
      isRunning: this.intervalId !== null,
      isPageVisible: this.isPageVisible,
      isWindowFocused: this.isWindowFocused,
      lastRefreshTime: this.lastRefreshTime
    }
  }
}
