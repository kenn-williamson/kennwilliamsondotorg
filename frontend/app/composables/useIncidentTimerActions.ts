/**
 * Incident Timer Action Composable - Orchestrates services + stores
 * Handles context-aware operations and bridges between services and stores
 */

import { useIncidentTimerStore } from '~/stores/incident-timers'
import { incidentTimerService } from '~/services/incidentTimerService'
import { useBaseService } from '~/composables/useBaseService'
import { useBackendFetch } from '~/composables/useBackendFetch'
import type { CreateTimerRequest, UpdateTimerRequest } from '#shared/types/timers'

export const useIncidentTimerActions = () => {
  // Destructure base service utilities
  const { executeRequest, executeRequestWithSuccess, isLoading, error, hasError } = useBaseService()
  const backendFetch = useBackendFetch()
  
  // Create service instance once
  const incidentTimerServiceBackend = incidentTimerService(backendFetch)
  
  // Destructure service methods
  const { 
    getUserTimers, 
    getPublicTimer, 
    createTimer: createTimerService, 
    updateTimer: updateTimerService, 
    deleteTimer: deleteTimerService, 
    quickReset: quickResetService 
  } = incidentTimerServiceBackend

  // Destructure store methods and state
  const { 
    setTimers, 
    setPublicTimer, 
    addTimer, 
    setCurrentTimer, 
    updateTimer: updateTimerStore, 
    removeTimer, 
    currentTimer,
    startLiveTimerUpdates,
    stopLiveTimerUpdates
  } = useIncidentTimerStore()

  const loadUserTimers = async () => {
    const timers = await executeRequest(() => getUserTimers(), 'loadUserTimers')
    setTimers(timers)
    
    // Start live timer updates if we have timers
    if (timers.length > 0) {
      startLiveTimerUpdates()
    }
    
    return timers
  }

  const loadPublicTimer = async (userSlug: string) => {
    const timer = await executeRequest(() => getPublicTimer(userSlug), 'loadPublicTimer')
    setPublicTimer(timer, userSlug)
    
    // Start live timer updates for public timer
    if (timer) {
      startLiveTimerUpdates()
    }
    
    return timer
  }

  const createTimer = async (timerData: CreateTimerRequest) => {
    const timer = await executeRequestWithSuccess(
      () => createTimerService(timerData),
      'Timer created successfully',
      'createTimer'
    )
    addTimer(timer)
    setCurrentTimer(timer)
    return timer
  }

  const updateTimer = async (timerId: string, timerData: UpdateTimerRequest) => {
    const timer = await executeRequestWithSuccess(
      () => updateTimerService(timerId, timerData),
      'Timer updated successfully',
      'updateTimer'
    )
    updateTimerStore(timerId, timer)
    
    // Update current timer if it's the same one
    if (currentTimer?.id === timerId) {
      setCurrentTimer(timer)
    }
    
    return timer
  }

  const deleteTimer = async (timerId: string) => {
    await executeRequestWithSuccess(
      () => deleteTimerService(timerId),
      'Timer deleted successfully',
      'deleteTimer'
    )
    removeTimer(timerId)
    
    // Clear current timer if it was the deleted one
    if (currentTimer?.id === timerId) {
      setCurrentTimer(null)
    }
  }

  const quickReset = async (timerId: string) => {
    const timer = await executeRequestWithSuccess(
      () => quickResetService(timerId),
      'Timer reset successfully',
      'quickReset'
    )
    updateTimerStore(timerId, timer)
    
    // Update current timer if it's the same one
    if (currentTimer?.id === timerId) {
      setCurrentTimer(timer)
    }
    
    return timer
  }

  // Alias for backward compatibility
  const fetchTimers = loadUserTimers
  const fetchPublicTimer = loadPublicTimer

  return {
    loadUserTimers,
    loadPublicTimer,
    createTimer,
    updateTimer,
    deleteTimer,
    quickReset,
    // Aliases for backward compatibility
    fetchTimers,
    fetchPublicTimer,
    // Timer control methods
    startLiveTimerUpdates,
    stopLiveTimerUpdates,
    isLoading,
    error,
    hasError
  }
}
