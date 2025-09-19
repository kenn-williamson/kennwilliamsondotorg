/**
 * useIncidentTimerService - Incident timer operations for client-side use
 * 
 * Uses direct backend fetch for all operations with JWT authentication.
 * For SSR prefetching, use $fetch('/api/*') directly in pages.
 */

import { useBaseService } from './useBaseService'
import { API_ROUTES } from '#shared/config/api-routes'

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
  reset_timestamp?: string
  notes?: string
}

interface UpdateTimerRequest {
  reset_timestamp?: string
  notes?: string
}

export function useIncidentTimerService() {
  const { executeRequest, executeRequestWithSuccess, backendFetch, isLoading, error, hasError } = useBaseService()

  return {
    // Get all timers for current user (protected) - direct backend call with JWT
    async getUserTimers(): Promise<IncidentTimer[]> {
      return executeRequest(
        () => backendFetch<IncidentTimer[]>(API_ROUTES.PROTECTED.TIMERS.LIST),
        'getUserTimers'
      )
    },

    // Refresh timers (client-side) - direct backend call for better performance
    async refreshUserTimers(): Promise<IncidentTimer[]> {
      return executeRequest(
        () => backendFetch<IncidentTimer[]>(API_ROUTES.PROTECTED.TIMERS.LIST),
        'refreshUserTimers'
      )
    },

    // Get public timer by user slug (no auth required) - direct backend call
    async getPublicTimer(userSlug: string): Promise<PublicIncidentTimer> {
      return executeRequest(
        () => backendFetch<PublicIncidentTimer>(API_ROUTES.PUBLIC.TIMERS.BY_USER_SLUG(userSlug)),
        'getPublicTimer'
      )
    },

    // Create new timer (protected) - direct backend call
    async createTimer(timerData: CreateTimerRequest): Promise<IncidentTimer> {
      return executeRequestWithSuccess(
        () => backendFetch<IncidentTimer>(API_ROUTES.PROTECTED.TIMERS.CREATE, {
          method: 'POST',
          body: timerData,
        }),
        'Timer created successfully',
        'createTimer'
      )
    },

    // Update existing timer (protected) - direct backend call
    async updateTimer(id: string, updates: UpdateTimerRequest): Promise<IncidentTimer> {
      return executeRequestWithSuccess(
        () => backendFetch<IncidentTimer>(API_ROUTES.PROTECTED.TIMERS.UPDATE(id), {
          method: 'PUT',
          body: updates,
        }),
        'Timer updated successfully',
        'updateTimer'
      )
    },

    // Delete timer (protected) - direct backend call
    async deleteTimer(id: string): Promise<void> {
      return executeRequestWithSuccess(
        () => backendFetch<void>(API_ROUTES.PROTECTED.TIMERS.DELETE(id), {
          method: 'DELETE',
        }),
        'Timer deleted successfully',
        'deleteTimer'
      )
    },

    // Quick reset - create new timer with current timestamp (protected)
    async quickReset(notes?: string): Promise<IncidentTimer> {
      return this.createTimer({
        reset_timestamp: new Date().toISOString(),
        notes,
      })
    },

    // Expose base service state for components
    isLoading,
    error,
    hasError,
  }
}