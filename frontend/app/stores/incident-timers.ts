import { defineStore } from 'pinia'

interface IncidentTimer {
  id: string
  reset_timestamp: string
  notes?: string
  created_at: string
  updated_at: string
}

interface PublicIncidentTimer extends IncidentTimer {
  user_display_name: string
}

interface CreateTimerRequest {
  reset_timestamp: string
  notes?: string
}

interface UpdateTimerRequest {
  reset_timestamp?: string
  notes?: string
}

interface TimerState {
  timers: IncidentTimer[]
  currentTimer: IncidentTimer | null
  publicTimer: PublicIncidentTimer | null
  loading: boolean
  error: string | undefined
  // Reactive timer breakdown that updates every second
  activeTimerBreakdown: { years: number, months: number, weeks: number, days: number, hours: number, minutes: number, seconds: number }
}

// Global timer update interval - shared across all store instances
let globalTimerInterval: NodeJS.Timeout | null = null

// Global page visibility handler - shared across all store instances
let visibilityHandlerAttached = false
let visibilityChangeHandler: (() => void) | null = null
let windowFocusHandler: (() => void) | null = null

export const useIncidentTimerStore = defineStore('incident-timers', {
  state: (): TimerState => ({
    timers: [],
    currentTimer: null,
    publicTimer: null,
    loading: false,
    error: undefined,
    activeTimerBreakdown: { years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 },
  }),

  getters: {
    // Calculate total elapsed seconds for a timer (simple calculation)
    getElapsedSeconds: (state) => (timer: IncidentTimer): number => {
      if (!timer?.reset_timestamp) return 0
      const startTime = new Date(timer.reset_timestamp).getTime()
      const now = Date.now()
      return Math.floor((now - startTime) / 1000)
    },

    // Calculate elapsed time breakdown using proper calendar arithmetic
    // This matches how humans actually count time intervals
    getElapsedTimeBreakdown: (state) => (timer: IncidentTimer) => {
      if (!timer?.reset_timestamp) return { 
        years: 0, months: 0, weeks: 0, days: 0, 
        hours: 0, minutes: 0, seconds: 0 
      }

      const startDate = new Date(timer.reset_timestamp)
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
    },

    // Format elapsed time with sophisticated breakdown (only shows non-zero values)
    formatElapsedTime: (state) => (timer: IncidentTimer): string => {
      if (!timer?.reset_timestamp) return 'No incident started'
      
      // Directly calculate breakdown here since we can't access other getters from state
      const startDate = new Date(timer.reset_timestamp)
      const now = new Date()
      
      // Calculate year and month differences using date arithmetic
      let years = now.getFullYear() - startDate.getFullYear()
      let months = now.getMonth() - startDate.getMonth()
      
      // Adjust for negative months
      if (months < 0) {
        years--
        months += 12
      }
      
      // Calculate the remaining time after years and months
      const adjustedStart = new Date(startDate)
      adjustedStart.setFullYear(adjustedStart.getFullYear() + years)
      adjustedStart.setMonth(adjustedStart.getMonth() + months)
      
      // Calculate remaining time in milliseconds
      const remainingMs = now.getTime() - adjustedStart.getTime()
      const remainingSeconds = Math.floor(remainingMs / 1000)
      
      // Break down remaining time
      const weeks = Math.floor(remainingSeconds / (7 * 24 * 3600))
      const days = Math.floor((remainingSeconds % (7 * 24 * 3600)) / (24 * 3600))
      const hours = Math.floor((remainingSeconds % (24 * 3600)) / 3600)
      const minutes = Math.floor((remainingSeconds % 3600) / 60)
      const seconds = remainingSeconds % 60
      
      const breakdown = { years, months, weeks, days, hours, minutes, seconds }
      const parts: string[] = []
      
      if (breakdown.years > 0) parts.push(`${breakdown.years} year${breakdown.years !== 1 ? 's' : ''}`)
      if (breakdown.months > 0) parts.push(`${breakdown.months} month${breakdown.months !== 1 ? 's' : ''}`)
      if (breakdown.weeks > 0) parts.push(`${breakdown.weeks} week${breakdown.weeks !== 1 ? 's' : ''}`)
      if (breakdown.days > 0) parts.push(`${breakdown.days} day${breakdown.days !== 1 ? 's' : ''}`)
      if (breakdown.hours > 0) parts.push(`${breakdown.hours} hour${breakdown.hours !== 1 ? 's' : ''}`)
      if (breakdown.minutes > 0) parts.push(`${breakdown.minutes} minute${breakdown.minutes !== 1 ? 's' : ''}`)
      if (breakdown.seconds > 0 || parts.length === 0) parts.push(`${breakdown.seconds} second${breakdown.seconds !== 1 ? 's' : ''}`)
      
      return parts.join(', ')
    },

    // Format elapsed time as compact HH:MM:SS (for compatibility)
    formatElapsedTimeCompact: (state) => (timer: IncidentTimer): string => {
      if (!timer?.reset_timestamp) return '00:00:00'
      
      // Calculate total seconds directly
      const startTime = new Date(timer.reset_timestamp).getTime()
      const now = Date.now()
      const totalSeconds = Math.floor((now - startTime) / 1000)
      
      const hours = Math.floor(totalSeconds / 3600)
      const minutes = Math.floor((totalSeconds % 3600) / 60)
      const seconds = totalSeconds % 60

      return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
    },

    // Get the most recent timer (by reset_timestamp - most recently started incident-free period)
    latestTimer: (state): IncidentTimer | null => {
      if (state.timers.length === 0) return null
      return state.timers.reduce((latest, timer) =>
        new Date(timer.reset_timestamp) > new Date(latest.reset_timestamp) ? timer : latest
      )
    },
  },

  actions: {
    // Start live timer updates for the active timer (public or latest user timer)
    startLiveTimerUpdates() {
      // Clear any existing interval
      if (globalTimerInterval) {
        clearInterval(globalTimerInterval)
        globalTimerInterval = null
      }

      // Set up page visibility handling (only once globally)
      if (typeof document !== 'undefined' && !visibilityHandlerAttached) {
        visibilityHandlerAttached = true

        visibilityChangeHandler = () => {
          if (!document.hidden) {
            console.log('👁️ Page visible again, restarting timer with fresh interval')
            // Get current store instance and restart timer updates
            const timerStore = useIncidentTimerStore()
            const activeTimer = timerStore.publicTimer || timerStore.latestTimer
            if (activeTimer?.reset_timestamp) {
              timerStore.startLiveTimerUpdates()
            }
          }
        }

        windowFocusHandler = () => {
          console.log('🎯 Window focused, restarting timer with fresh interval')
          const timerStore = useIncidentTimerStore()
          const activeTimer = timerStore.publicTimer || timerStore.latestTimer
          if (activeTimer?.reset_timestamp) {
            timerStore.startLiveTimerUpdates()
          }
        }

        document.addEventListener('visibilitychange', visibilityChangeHandler)
        window.addEventListener('focus', windowFocusHandler)
      }

      // Determine which timer to track - prioritize public timer, fallback to latest user timer
      const activeTimer = this.publicTimer || this.latestTimer
      if (!activeTimer?.reset_timestamp) {
        console.log('🔴 No active timer to track')
        this.activeTimerBreakdown = { years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 }
        return
      }

      console.log('🟢 Starting live timer updates for:', {
        id: activeTimer.id,
        reset_timestamp: activeTimer.reset_timestamp,
        isPublic: !!this.publicTimer
      })

      // Update immediately (this recalculates from actual timestamps, so catches up automatically)
      this.activeTimerBreakdown = this.getElapsedTimeBreakdown(activeTimer)
      console.log('⏱️ Initial breakdown:', this.activeTimerBreakdown)

      // Set up fresh interval to update every second
      globalTimerInterval = setInterval(() => {
        const currentActiveTimer = this.publicTimer || this.latestTimer
        if (currentActiveTimer?.reset_timestamp) {
          this.activeTimerBreakdown = this.getElapsedTimeBreakdown(currentActiveTimer)
          console.log('🔄 Timer tick:', {
            seconds: this.activeTimerBreakdown.seconds,
            total: `${this.activeTimerBreakdown.years}y ${this.activeTimerBreakdown.months}m ${this.activeTimerBreakdown.weeks}w ${this.activeTimerBreakdown.days}d ${this.activeTimerBreakdown.hours}h ${this.activeTimerBreakdown.minutes}min ${this.activeTimerBreakdown.seconds}s`
          })
        } else {
          console.log('🔴 No active timer found, stopping updates')
          this.stopLiveTimerUpdates()
        }
      }, 1000)
    },

    // Stop live timer updates
    stopLiveTimerUpdates() {
      if (globalTimerInterval) {
        console.log('⏹️ Stopping live timer updates')
        clearInterval(globalTimerInterval)
        globalTimerInterval = null
      }
      this.activeTimerBreakdown = { years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 }
    },

    // Clean up global event listeners (call when completely done with timers)
    cleanupGlobalEventListeners() {
      if (typeof document !== 'undefined' && visibilityHandlerAttached) {
        console.log('🧹 Cleaning up page visibility event listeners')

        if (visibilityChangeHandler) {
          document.removeEventListener('visibilitychange', visibilityChangeHandler)
          visibilityChangeHandler = null
        }

        if (windowFocusHandler) {
          window.removeEventListener('focus', windowFocusHandler)
          windowFocusHandler = null
        }

        visibilityHandlerAttached = false
      }
    },

    // Fetch user's timers
    async fetchUserTimers(): Promise<{ success: boolean; error?: string }> {
      try {
        this.loading = true
        this.error = undefined
        console.log('🔍 Fetching user timers...')

        const incidentTimerService = useIncidentTimerService()
        const data = await incidentTimerService.getUserTimers()
        console.log('📦 User timers data:', data)

        this.timers = data
        console.log('🎯 Latest timer:', this.latestTimer)
        
        // Start live updates if we have timers
        if (this.latestTimer) {
          console.log('📊 Loaded user timers, starting live updates')
          this.startLiveTimerUpdates()
        } else {
          console.log('❌ No latest timer found')
        }
        
        return { success: true }
      } catch (error: any) {
        console.error('Fetch timers error:', error)
        this.error = error.message || 'Failed to fetch timers'
        return {
          success: false,
          error: this.error,
        }
      } finally {
        this.loading = false
      }
    },

    // Fetch public timer by user slug
    async fetchPublicTimer(userSlug: string): Promise<{ success: boolean; error?: string }> {
      try {
        this.loading = true
        this.error = undefined

        const incidentTimerService = useIncidentTimerService()
        const data = await incidentTimerService.getPublicTimer(userSlug)

        this.publicTimer = data
        
        // Start live updates for public timer
        console.log('🌐 Loaded public timer, starting live updates')
        this.startLiveTimerUpdates()
        
        return { success: true }
      } catch (error: any) {
        console.error('Fetch public timer error:', error)
        this.error = error.message || 'Failed to fetch public timer'
        this.publicTimer = null
        return {
          success: false,
          error: this.error,
        }
      } finally {
        this.loading = false
      }
    },

    // Create new timer (reset incident)
    async createTimer(timerData: CreateTimerRequest): Promise<{ success: boolean; error?: string }> {
      try {
        this.loading = true
        this.error = undefined

        const incidentTimerService = useIncidentTimerService()
        const data = await incidentTimerService.createTimer(timerData)

        // Add new timer to the beginning of the list
        this.timers.unshift(data)
        this.currentTimer = data

        // Start live updates for the new timer
        console.log('✅ Created new timer, starting live updates')
        this.startLiveTimerUpdates()

        return { success: true }
      } catch (error: any) {
        console.error('Create timer error:', error)
        this.error = error.message || 'Failed to create timer'
        return {
          success: false,
          error: this.error,
        }
      } finally {
        this.loading = false
      }
    },

    // Update existing timer
    async updateTimer(id: string, updates: UpdateTimerRequest): Promise<{ success: boolean; error?: string }> {
      try {
        this.loading = true
        this.error = undefined

        const incidentTimerService = useIncidentTimerService()
        const data = await incidentTimerService.updateTimer(id, updates)

        // Update timer in the list
        const index = this.timers.findIndex(timer => timer.id === id)
        if (index !== -1) {
          this.timers[index] = data
        }

        // Update current timer if it's the same one
        if (this.currentTimer?.id === id) {
          this.currentTimer = data
        }

        // If this is the latest timer, restart live updates to reflect changes
        if (this.latestTimer?.id === id) {
          console.log('✅ Updated latest timer, restarting live updates')
          this.startLiveTimerUpdates()
        }

        return { success: true }
      } catch (error: any) {
        console.error('Update timer error:', error)
        this.error = error.message || 'Failed to update timer'
        return {
          success: false,
          error: this.error,
        }
      } finally {
        this.loading = false
      }
    },

    // Delete timer
    async deleteTimer(id: string): Promise<{ success: boolean; error?: string }> {
      try {
        this.loading = true
        this.error = undefined

        const incidentTimerService = useIncidentTimerService()
        await incidentTimerService.deleteTimer(id)

        // Remove timer from the list
        this.timers = this.timers.filter(timer => timer.id !== id)

        // Clear current timer if it was the deleted one
        if (this.currentTimer?.id === id) {
          this.currentTimer = null
        }

        // Restart live updates with the new latest timer (or stop if no timers left)
        if (this.latestTimer) {
          console.log('✅ Timer deleted, restarting live updates with new latest timer')
          this.startLiveTimerUpdates()
        } else {
          console.log('❌ No timers left after deletion, stopping live updates')
          this.stopLiveTimerUpdates()
        }

        return { success: true }
      } catch (error: any) {
        console.error('Delete timer error:', error)
        this.error = error.message || 'Failed to delete timer'
        return {
          success: false,
          error: this.error,
        }
      } finally {
        this.loading = false
      }
    },

    // Quick reset - create a new timer with current time
    async quickReset(notes?: string): Promise<{ success: boolean; error?: string }> {
      try {
        const incidentTimerService = useIncidentTimerService()
        const timer = await incidentTimerService.quickReset(notes)
        
        this.timers.unshift(timer)
        this.currentTimer = timer
        return { success: true }
      } catch (error: any) {
        return {
          success: false,
          error: error.message || 'Failed to reset timer'
        }
      }
    },

    // Clear error
    clearError(): void {
      this.error = undefined
    },

    // Clear all state
    clearState(): void {
      this.stopLiveTimerUpdates()
      this.cleanupGlobalEventListeners()
      this.timers = []
      this.currentTimer = null
      this.publicTimer = null
      this.error = undefined
      this.loading = false
    },
  },
})