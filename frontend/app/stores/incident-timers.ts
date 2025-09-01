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
}

export const useIncidentTimerStore = defineStore('incident-timers', {
  state: (): TimerState => ({
    timers: [],
    currentTimer: null,
    publicTimer: null,
    loading: false,
    error: undefined,
  }),

  getters: {
    // Calculate total elapsed seconds for a timer (simple calculation)
    getElapsedSeconds: (state) => (timer: IncidentTimer): number => {
      if (!timer?.reset_timestamp) return 0
      const startTime = new Date(timer.reset_timestamp).getTime()
      const now = Date.now()
      return Math.floor((now - startTime) / 1000)
    },

    // Calculate elapsed time breakdown using hybrid approach
    // Years/months via date arithmetic, remaining time via milliseconds
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
      
      // Step 1: Calculate years and months using date arithmetic
      let years = now.getFullYear() - startDate.getFullYear()
      let months = now.getMonth() - startDate.getMonth()
      
      // Adjust for negative months
      if (months < 0) {
        years--
        months += 12
      }
      
      // Step 2: Create adjusted start date after subtracting years and months
      const adjustedStart = new Date(startDate)
      adjustedStart.setFullYear(startDate.getFullYear() + years)
      adjustedStart.setMonth(startDate.getMonth() + months)
      
      // Step 3: Calculate remaining time in milliseconds
      const remainingMs = now.getTime() - adjustedStart.getTime()
      const remainingTotalSeconds = Math.floor(remainingMs / 1000)
      
      // Step 4: Break down remaining seconds into weeks, days, hours, minutes, seconds
      const weeks = Math.floor(remainingTotalSeconds / (7 * 24 * 60 * 60))
      const remainingAfterWeeks = remainingTotalSeconds % (7 * 24 * 60 * 60)
      
      const days = Math.floor(remainingAfterWeeks / (24 * 60 * 60))
      const remainingAfterDays = remainingAfterWeeks % (24 * 60 * 60)
      
      const hours = Math.floor(remainingAfterDays / (60 * 60))
      const remainingAfterHours = remainingAfterDays % (60 * 60)
      
      const minutes = Math.floor(remainingAfterHours / 60)
      const seconds = remainingAfterHours % 60
      
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

    // Get the most recent timer
    latestTimer: (state): IncidentTimer | null => {
      if (state.timers.length === 0) return null
      return state.timers.reduce((latest, timer) => 
        new Date(timer.created_at) > new Date(latest.created_at) ? timer : latest
      )
    },
  },

  actions: {
    // Fetch user's timers
    async fetchUserTimers(): Promise<{ success: boolean; error?: string }> {
      try {
        this.loading = true
        this.error = undefined

        const incidentTimerService = useIncidentTimerService()
        const data = await incidentTimerService.getUserTimers()

        this.timers = data
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
      this.timers = []
      this.currentTimer = null
      this.publicTimer = null
      this.error = undefined
      this.loading = false
    },
  },
})