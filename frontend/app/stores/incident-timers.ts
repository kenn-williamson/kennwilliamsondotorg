/**
 * Enhanced Incident Timer Store - Centralized state management with actions
 */

import type { IncidentTimer, PublicTimerResponse, CreateTimerRequest, UpdateTimerRequest } from '#shared/types/timers'
import type { PublicTimerListItem, UpdatePreferencesRequest } from '#shared/types'
import { incidentTimerService } from '~/services/incidentTimerService'
import { userPreferencesService } from '~/services/userPreferencesService'
import { useSmartFetch } from '~/composables/useSmartFetch'
import { useSessionWatcher } from '~/composables/useSessionWatcher'
import { TimerManager, type TimerUpdateCallback } from '~/utils/timer-manager'

export const useIncidentTimerStore = defineStore('incident-timers', () => {
  const timers = ref<IncidentTimer[]>([])
  const currentTimer = ref<IncidentTimer | null>(null)
  const publicTimer = ref<PublicTimerResponse | null>(null)
  const publicTimerUserSlug = ref<string | null>(null)
  const activeTimerBreakdown = ref({ years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 })

  // Public timer list state
  const publicTimersList = ref<PublicTimerListItem[]>([])
  const publicTimersPage = ref(1)
  const publicTimersPageSize = ref(20)
  const publicTimersLoading = ref(false)

  const isLoading = ref(false)
  const error = ref<string | null>(null)

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

  const updateTimerStore = (timerId: string, updates: Partial<IncidentTimer>) => {
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
    // Reset breakdown to zero when clearing public timer
    activeTimerBreakdown.value = { years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 }
  }

  const clearAllData = () => {
    timers.value = []
    currentTimer.value = null
    publicTimer.value = null
    publicTimerUserSlug.value = null
    activeTimerBreakdown.value = { years: 0, months: 0, weeks: 0, days: 0, hours: 0, minutes: 0, seconds: 0 }
    isLoading.value = false
    error.value = null

    // Stop any running timers
    stopLiveTimerUpdates()
    cleanupGlobalEventListeners()

    console.log('🧹 [IncidentTimerStore] All data cleared')
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

  const hasError = computed(() => !!error.value)

  // Service instances - uses useSmartFetch for automatic routing
  const smartFetch = useSmartFetch()
  const incidentTimerServiceInstance = incidentTimerService(smartFetch)
  const userPreferencesServiceInstance = userPreferencesService(smartFetch)

  // Private action handler (replaces useBaseService logic)
  const _handleAction = async <T>(
    action: () => Promise<T>,
    context?: string
  ): Promise<T | undefined> => {
    isLoading.value = true
    error.value = null
    
    try {
      const result = await action()
      return result
    } catch (err: any) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      error.value = errorMessage
      console.error(`[IncidentTimerStore] Error${context ? ` in ${context}` : ''}:`, errorMessage)
      
      // Handle all errors gracefully - keep them in state for UI to display
      return undefined
    } finally {
      isLoading.value = false
    }
  }

  // Private success handler
  const _handleSuccess = (message: string): void => {
    console.log(`[IncidentTimerStore] Success: ${message}`)
    // TODO: Add toast notifications here
  }

  // Timer manager instance
  let timerManager: TimerManager | null = null

  // Timer update callback for TimerManager
  const timerUpdateCallback: TimerUpdateCallback = () => {
    // Use public timer if available (for public pages), otherwise use latest timer (for authenticated users)
    const timerToUse = publicTimer.value || latestTimer.value
    if (timerToUse) {
      activeTimerBreakdown.value = getElapsedTimeBreakdown(timerToUse)
    }
  }

  // Timer update methods
  const startLiveTimerUpdates = () => {
    // Only start timers on client-side (not during SSR)
    if (import.meta.client) {
      // Create timer manager if it doesn't exist
      if (!timerManager) {
        timerManager = new TimerManager(
          timerUpdateCallback,
        )
      }

      timerManager.start()
      console.log('🔄 [Timer Store] Started live timer updates on client-side')
    } else {
      console.log('ℹ️ [Timer Store] Skipping timer start during SSR')
    }
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

  // Clear public timer when navigating away from public pages
  const clearPublicTimerOnNavigation = () => {
    if (publicTimer.value) {
      console.log('🔄 [Timer Store] Clearing public timer on navigation away from public page')
      clearPublicTimer()
    }
  }

  const loadUserTimers = async () => {
    console.log('🔄 [IncidentTimerStore] loadUserTimers called')
    console.log('🔄 [IncidentTimerStore] Environment:', import.meta.server ? 'SERVER' : 'CLIENT')
    console.log('🔄 [IncidentTimerStore] Current timers length:', timers.value.length)
    
    const data = await _handleAction(() => incidentTimerServiceInstance.getUserTimers(), 'loadUserTimers')
    if (data) {
      timers.value = data
      // Note: latestTimer computed property will automatically pick the most recent timer
      
      // Calculate initial breakdown for SSR (find most recent timer directly from data)
      if (data.length > 0) {
        const mostRecentTimer = [...data]
          .sort((a, b) => new Date(b.reset_timestamp).getTime() - new Date(a.reset_timestamp).getTime())[0]
        
        if (mostRecentTimer) {
          const initialBreakdown = getElapsedTimeBreakdown(mostRecentTimer)
          activeTimerBreakdown.value = initialBreakdown
          console.log('✅ [Timer Store] Set initial breakdown for SSR:', initialBreakdown)
        }
      }
      
      // Note: Timer starting is handled separately on client-side after hydration
    }
    return data
  }

  const loadPublicTimer = async (userSlug: string) => {
    const data = await _handleAction(() => incidentTimerServiceInstance.getPublicTimer(userSlug), 'loadPublicTimer')
    if (data) {
      publicTimer.value = data
      publicTimerUserSlug.value = userSlug
      
      // Calculate initial breakdown for SSR
      if (data) {
        const initialBreakdown = getElapsedTimeBreakdown(data)
        activeTimerBreakdown.value = initialBreakdown
        console.log('✅ [Timer Store] Set initial breakdown for public timer SSR:', initialBreakdown)
      }
      
      // Note: Timer starting is handled separately on client-side after hydration
    }
    return data
  }

  const createTimer = async (timerData: CreateTimerRequest) => {
    const data = await _handleAction(() => incidentTimerServiceInstance.createTimer(timerData), 'createTimer')
    _handleSuccess('Timer created successfully')
    
    if (data) {
      timers.value.push(data)
      currentTimer.value = data
    }
    return data
  }

  const updateTimer = async (timerId: string, timerData: UpdateTimerRequest) => {
    const data = await _handleAction(() => incidentTimerServiceInstance.updateTimer(timerId, timerData), 'updateTimer')
    _handleSuccess('Timer updated successfully')
    
    if (data) {
      const index = timers.value.findIndex(t => t.id === timerId)
      if (index !== -1) {
        timers.value[index] = data
      }
      
      // Update current timer if it's the same one
      if (currentTimer.value?.id === timerId) {
        currentTimer.value = data
      }
    }
    return data
  }

  const deleteTimer = async (timerId: string) => {
    await _handleAction(() => incidentTimerServiceInstance.deleteTimer(timerId), 'deleteTimer')
    _handleSuccess('Timer deleted successfully')
    
    timers.value = timers.value.filter(t => t.id !== timerId)
    
    // Clear current timer if it was the deleted one
    if (currentTimer.value?.id === timerId) {
      currentTimer.value = null
    }
  }

  const quickReset = async (timerId: string) => {
    const data = await _handleAction(() => incidentTimerServiceInstance.quickReset(timerId), 'quickReset')
    _handleSuccess('Timer reset successfully')

    if (data) {
      const index = timers.value.findIndex(t => t.id === timerId)
      if (index !== -1) {
        timers.value[index] = data
      }

      // Update current timer if it's the same one
      if (currentTimer.value?.id === timerId) {
        currentTimer.value = data
      }
    }
    return data
  }

  // Public timer list actions
  const loadPublicTimerList = async (page: number = 1, pageSize: number = 20, search?: string) => {
    publicTimersLoading.value = true
    const data = await _handleAction(
      () => userPreferencesServiceInstance.getPublicTimerList(page, pageSize, search),
      'loadPublicTimerList'
    )
    publicTimersLoading.value = false

    if (data) {
      publicTimersList.value = data
      publicTimersPage.value = page
      publicTimersPageSize.value = pageSize
    }
    return data
  }

  // User preferences actions
  const updateUserPreferences = async (preferences: UpdatePreferencesRequest): Promise<{ message: string } | undefined> => {
    const data = await _handleAction(
      () => userPreferencesServiceInstance.updatePreferences(preferences),
      'updateUserPreferences'
    )
    _handleSuccess('Preferences updated successfully')

    return data
  }

  // Set up session watcher for automatic cleanup on logout
  useSessionWatcher(clearAllData)

  return {
    timers,
    currentTimer,
    publicTimer,
    publicTimerUserSlug,
    publicTimersList,
    publicTimersPage,
    publicTimersPageSize,
    publicTimersLoading,
    isLoading,
    error,

    activeTimerBreakdown,
    latestTimer,
    hasError,

    loadUserTimers,
    loadPublicTimer,
    loadPublicTimerList,
    createTimer,
    updateTimer,
    deleteTimer,
    quickReset,
    updateUserPreferences,

    setTimers,
    setCurrentTimer,
    setPublicTimer,
    addTimer,
    updateTimerStore,
    removeTimer,
    clearTimers,
    clearPublicTimer,
    clearAllData,

    startLiveTimerUpdates,
    stopLiveTimerUpdates,
    cleanupGlobalEventListeners,
    clearPublicTimerOnNavigation,

    getElapsedTimeBreakdown,
    getElapsedSeconds,
    formatElapsedTime,
    formatElapsedTimeCompact
  }
})
