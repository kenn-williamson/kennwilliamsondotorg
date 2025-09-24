/**
 * TimerManager - Handles all timer-related browser behavior
 * Separates timer management from store state management
 */

export interface TimerUpdateCallback {
  (): void
}


export class TimerManager {
  private intervalId: NodeJS.Timeout | null = null
  private visibilityHandlerAttached = false
  private visibilityChangeHandler: (() => void) | null = null
  private windowFocusHandler: (() => void) | null = null
  private blurHandler: (() => void) | null = null
  private isPageVisible = true
  private isWindowFocused = true

  constructor(
    private updateCallback: TimerUpdateCallback,
  ) {}

  /**
   * Start the timer with sophisticated browser focus handling
   */
  start(): void {
    this.stop()
    this.setupBrowserEventHandlers()

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
    this.updateCallback()
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
    this.stop()
    this.start()
  }

  /**
   * Handle window focus (browser window focus)
   */
  private async handleWindowFocused(): Promise<void> {
    // Only refresh if the page is also visible
    // This prevents unnecessary refreshes when window gains focus but page is hidden
    if (this.isPageVisible) {
      this.stop()
      this.start()
    }
  }

  /**
   * Get current state for debugging
   */
  getState() {
    return {
      isRunning: this.intervalId !== null,
      isPageVisible: this.isPageVisible,
      isWindowFocused: this.isWindowFocused,
    }
  }
}
