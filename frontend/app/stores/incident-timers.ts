/**
 * Pure Incident Timer Store - Only state management, no service calls
 * Refactored to follow proper separation of concerns
 */

import type { IncidentTimer, PublicTimerResponse } from '#shared/types/timers'
import { TimerManager, type TimerUpdateCallback, type DataRefreshCallback, type ActiveTimerProvider } from '~/utils/timer-manager'

export const useIncidentTimerStore = defineStore('incident-timers', () => {
  // State
  const timers = ref<IncidentTimer[]>([])
  const currentTimer = ref<IncidentTimer | null>(null)
  const publicTimer = ref<PublicTimerResponse | null>(null)
  const publicTimerUserSlug = ref<string | null>(null)
  const activeTimerBreakdown = ref({ years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 })

  // Pure state management functions
  const setTimers = (timersList: IncidentTimer[]) => {
    timers.value = timersList
  }

  const setCurrentTimer = (timer: IncidentTimer | null) => {
    currentTimer.value = timer
  }

  const setPublicTimer = (timer: PublicTimerResponse | null, userSlug?: string) => {
    publicTimer.value = timer
    if (userSlug) {
      publicTimerUserSlug.value = userSlug
    }
  }

  const addTimer = (timer: IncidentTimer) => {
    timers.value.push(timer)
  }

  const updateTimer = (timerId: string, updates: Partial<IncidentTimer>) => {
    const index = timers.value.findIndex(t => t.id === timerId)
    if (index !== -1) {
      timers.value[index] = { ...timers.value[index], ...updates } as IncidentTimer
    }
  }

  const removeTimer = (timerId: string) => {
    timers.value = timers.value.filter(t => t.id !== timerId)
  }

  const clearTimers = () => {
    timers.value = []
    currentTimer.value = null
  }

  const clearPublicTimer = () => {
    publicTimer.value = null
    publicTimerUserSlug.value = null
  }

  // Utility functions for timer calculations (pure functions)
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

  const getElapsedSeconds = (timer: IncidentTimer): number => {
    if (!timer?.reset_timestamp) return 0
    const startTime = new Date(timer.reset_timestamp).getTime()
    const now = Date.now()
    return Math.floor((now - startTime) / 1000)
  }

  const formatElapsedTime = (timer: IncidentTimer): string => {
    if (!timer?.reset_timestamp) return 'No incident started'
    
    const breakdown = getElapsedTimeBreakdown(timer)
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

  // Timer manager instance
  let timerManager: TimerManager | null = null

  // Timer update callback for TimerManager
  const timerUpdateCallback: TimerUpdateCallback = (breakdown) => {
    activeTimerBreakdown.value = breakdown
  }

  // Data refresh callback for TimerManager
  const dataRefreshCallback: DataRefreshCallback = async () => {
    // This will be called by action composables when they need to refresh data
    // The store doesn't handle data fetching - that's the action composable's job
    console.log('🔄 TimerManager requested data refresh - action composable should handle this')
  }

  // Active timer provider for TimerManager
  const activeTimerProvider: ActiveTimerProvider = () => {
    const activeTimer = publicTimer.value || latestTimer.value
    if (!activeTimer?.reset_timestamp) return null
    
    return {
      id: activeTimer.id,
      reset_timestamp: activeTimer.reset_timestamp,
      isPublic: !!publicTimer.value
    }
  }

  // Timer update methods
  const startLiveTimerUpdates = () => {
    // Create timer manager if it doesn't exist
    if (!timerManager) {
      timerManager = new TimerManager(
        timerUpdateCallback,
        dataRefreshCallback,
        activeTimerProvider
      )
    }

    timerManager.start()
  }

  const stopLiveTimerUpdates = () => {
    if (timerManager) {
      timerManager.stop()
    }
  }

  const cleanupGlobalEventListeners = () => {
    if (timerManager) {
      timerManager.cleanup()
      timerManager = null
    }
  }

  return {
    // State
    timers: readonly(timers),
    currentTimer: readonly(currentTimer),
    publicTimer: readonly(publicTimer),
    publicTimerUserSlug: readonly(publicTimerUserSlug),
    
    // Computed
    activeTimerBreakdown: readonly(activeTimerBreakdown),
    latestTimer,
    
    // Actions
    setTimers,
    setCurrentTimer,
    setPublicTimer,
    addTimer,
    updateTimer,
    removeTimer,
    clearTimers,
    clearPublicTimer,
    
    // Timer update methods
    startLiveTimerUpdates,
    stopLiveTimerUpdates,
    cleanupGlobalEventListeners,
    
    // Utility functions
    getElapsedTimeBreakdown,
    getElapsedSeconds,
    formatElapsedTime,
    formatElapsedTimeCompact
  }
})
