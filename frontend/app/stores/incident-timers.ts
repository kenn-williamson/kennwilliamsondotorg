// Auto-imported: defineStore, ref, computed

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

// Global timer update interval - shared across all store instances
let globalTimerInterval: NodeJS.Timeout | null = null

// Global page visibility handler - shared across all store instances
let visibilityHandlerAttached = false
let visibilityChangeHandler: (() => void) | null = null
let windowFocusHandler: (() => void) | null = null

export const useIncidentTimerStore = defineStore('incident-timers', () => {
  // Destructure from service
  const { 
    getUserTimers, 
    getPublicTimer, 
    createTimer, 
    updateTimer, 
    deleteTimer, 
    quickReset,
    isLoading,
    error,
    hasError
  } = useIncidentTimerService()

  // State
  const timers = ref<IncidentTimer[]>([])
  const currentTimer = ref<IncidentTimer | null>(null)
  const publicTimer = ref<PublicIncidentTimer | null>(null)
  const publicTimerUserSlug = ref<string | null>(null)
  const activeTimerBreakdown = ref({ years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 })

  // Computed properties
  const getElapsedSeconds = (timer: IncidentTimer): number => {
    if (!timer?.reset_timestamp) return 0
    const startTime = new Date(timer.reset_timestamp).getTime()
    const now = Date.now()
    return Math.floor((now - startTime) / 1000)
  }

  const getElapsedTimeBreakdown = (timer: IncidentTimer) => {
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
  }

  const formatElapsedTime = (timer: IncidentTimer): string => {
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
  }

  const formatElapsedTimeCompact = (timer: IncidentTimer): string => {
    if (!timer?.reset_timestamp) return '00:00:00'
    
    // Calculate total seconds directly
    const startTime = new Date(timer.reset_timestamp).getTime()
    const now = Date.now()
    const totalSeconds = Math.floor((now - startTime) / 1000)
    
    const hours = Math.floor(totalSeconds / 3600)
    const minutes = Math.floor((totalSeconds % 3600) / 60)
    const seconds = totalSeconds % 60

    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
  }

  const latestTimer = computed((): IncidentTimer | null => {
    if (timers.value.length === 0) return null
    return timers.value.reduce((latest, timer) =>
      new Date(timer.reset_timestamp) > new Date(latest.reset_timestamp) ? timer : latest
    )
  })

  // Actions
  const startLiveTimerUpdates = () => {
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
          console.log('👁️ Page visible again, refreshing data and restarting timer')
          // Get current store instance and refresh data
          const timerStore = useIncidentTimerStore()
          if (timerStore.publicTimer && timerStore.publicTimerUserSlug) {
            // Refresh public timer data
            timerStore.fetchPublicTimer(timerStore.publicTimerUserSlug)
          } else if (timerStore.latestTimer) {
            // Refresh user timers data
            timerStore.fetchUserTimers()
          }
        }
      }

      windowFocusHandler = () => {
        console.log('🎯 Window focused, refreshing data and restarting timer')
        const timerStore = useIncidentTimerStore()
        if (timerStore.publicTimer && timerStore.publicTimerUserSlug) {
          // Refresh public timer data
          timerStore.fetchPublicTimer(timerStore.publicTimerUserSlug)
        } else if (timerStore.latestTimer) {
          // Refresh user timers data
          timerStore.fetchUserTimers()
        }
      }

      document.addEventListener('visibilitychange', visibilityChangeHandler)
      window.addEventListener('focus', windowFocusHandler)
    }

    // Determine which timer to track - prioritize public timer, fallback to latest user timer
    const activeTimer = publicTimer.value || latestTimer.value
    if (!activeTimer?.reset_timestamp) {
      console.log('🔴 No active timer to track')
      activeTimerBreakdown.value = { years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 }
      return
    }

    console.log('🟢 Starting live timer updates for:', {
      id: activeTimer.id,
      reset_timestamp: activeTimer.reset_timestamp,
      isPublic: !!publicTimer.value
    })

    // Update immediately (this recalculates from actual timestamps, so catches up automatically)
    activeTimerBreakdown.value = getElapsedTimeBreakdown(activeTimer)
    console.log('⏱️ Initial breakdown:', activeTimerBreakdown.value)

    // Set up fresh interval to update every second
    globalTimerInterval = setInterval(() => {
      const currentActiveTimer = publicTimer.value || latestTimer.value
      if (currentActiveTimer?.reset_timestamp) {
        activeTimerBreakdown.value = getElapsedTimeBreakdown(currentActiveTimer)
      } else {
        console.log('🔴 No active timer found, stopping updates')
        stopLiveTimerUpdates()
      }
    }, 1000)
  }

  const stopLiveTimerUpdates = () => {
    if (globalTimerInterval) {
      console.log('⏹️ Stopping live timer updates')
      clearInterval(globalTimerInterval)
      globalTimerInterval = null
    }
    activeTimerBreakdown.value = { years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 }
  }

  const cleanupGlobalEventListeners = () => {
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
  }

  const fetchUserTimers = async () => {
    console.log('🔍 Fetching user timers...')
    const data = await getUserTimers()
    console.log('📦 User timers data:', data)

    timers.value = data
    console.log('🎯 Latest timer:', latestTimer.value)
    
    // Start live updates if we have timers
    if (latestTimer.value) {
      console.log('📊 Loaded user timers, starting live updates')
      startLiveTimerUpdates()
    } else {
      console.log('❌ No latest timer found')
    }
    
    return data
  }

  const fetchTimers = async () => {
    return fetchUserTimers()
  }

  const fetchPublicTimer = async (userSlug: string) => {
    const data = await getPublicTimer(userSlug)
    publicTimer.value = data
    publicTimerUserSlug.value = userSlug // Store the user slug for refresh
    
    // Start live updates for public timer
    console.log('🌐 Loaded public timer, starting live updates')
    startLiveTimerUpdates()
    
    return data
  }

  const createTimerAction = async (timerData: CreateTimerRequest) => {
    const data = await createTimer(timerData)

    // Add new timer to the beginning of the list
    timers.value.unshift(data)
    currentTimer.value = data

    // Start live updates for the new timer
    console.log('✅ Created new timer, starting live updates')
    startLiveTimerUpdates()

    return data
  }

  const updateTimerAction = async (id: string, updates: UpdateTimerRequest) => {
    const data = await updateTimer(id, updates)

    // Update timer in the list
    const index = timers.value.findIndex(timer => timer.id === id)
    if (index !== -1) {
      timers.value[index] = data
    }

    // Update current timer if it's the same one
    if (currentTimer.value?.id === id) {
      currentTimer.value = data
    }

    // If this is the latest timer, restart live updates to reflect changes
    if (latestTimer.value?.id === id) {
      console.log('✅ Updated latest timer, restarting live updates')
      startLiveTimerUpdates()
    }

    return data
  }

  const deleteTimerAction = async (id: string) => {
    await deleteTimer(id)

    // Remove timer from the list
    timers.value = timers.value.filter(timer => timer.id !== id)

    // Clear current timer if it was the deleted one
    if (currentTimer.value?.id === id) {
      currentTimer.value = null
    }

    // Restart live updates with the new latest timer (or stop if no timers left)
    if (latestTimer.value) {
      console.log('✅ Timer deleted, restarting live updates with new latest timer')
      startLiveTimerUpdates()
    } else {
      console.log('❌ No timers left after deletion, stopping live updates')
      stopLiveTimerUpdates()
    }
  }

  const quickResetAction = async (notes?: string) => {
    const timer = await quickReset(notes)
    
    timers.value.unshift(timer)
    currentTimer.value = timer
    return timer
  }

  const clearState = () => {
    stopLiveTimerUpdates()
    cleanupGlobalEventListeners()
    timers.value = []
    currentTimer.value = null
    publicTimer.value = null
    publicTimerUserSlug.value = null
  }

  return {
    // State
    timers: readonly(timers),
    currentTimer: readonly(currentTimer),
    publicTimer: readonly(publicTimer),
    publicTimerUserSlug: readonly(publicTimerUserSlug),
    activeTimerBreakdown: readonly(activeTimerBreakdown),
    
    // Service state
    isLoading,
    error,
    hasError,
    
    // Computed
    latestTimer,
    
    // Utility functions
    getElapsedSeconds,
    getElapsedTimeBreakdown,
    formatElapsedTime,
    formatElapsedTimeCompact,
    
    // Actions
    fetchUserTimers,
    fetchTimers,
    fetchPublicTimer,
    createTimer: createTimerAction,
    updateTimer: updateTimerAction,
    deleteTimer: deleteTimerAction,
    quickReset: quickResetAction,
    startLiveTimerUpdates,
    stopLiveTimerUpdates,
    cleanupGlobalEventListeners,
    clearState
  }
})