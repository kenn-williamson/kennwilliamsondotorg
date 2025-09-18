/**
 * useIncidentTimerService - Incident timer operations for client-side use
 * 
 * Uses direct backend fetch for all operations with JWT authentication.
 * For SSR prefetching, use $fetch('/api/*') directly in pages.
 */

import { useBackendFetch } from './useBackendFetch'

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
  const backendFetch = useBackendFetch()

  return {
    // Get all timers for current user (protected) - direct backend call with JWT
    async getUserTimers(): Promise<IncidentTimer[]> {
      return backendFetch<IncidentTimer[]>('/incident-timers')
    },

    // Refresh timers (client-side) - direct backend call for better performance
    async refreshUserTimers(): Promise<IncidentTimer[]> {
      return backendFetch<IncidentTimer[]>('/incident-timers')
    },

    // Get public timer by user slug (no auth required) - direct backend call
    async getPublicTimer(userSlug: string): Promise<PublicIncidentTimer> {
      return backendFetch<PublicIncidentTimer>(`/${userSlug}/incident-timer`)
    },

    // Create new timer (protected) - direct backend call
    async createTimer(timerData: CreateTimerRequest): Promise<IncidentTimer> {
      return backendFetch<IncidentTimer>('/incident-timers', {
        method: 'POST',
        body: timerData,
      })
    },

    // Update existing timer (protected) - direct backend call
    async updateTimer(id: string, updates: UpdateTimerRequest): Promise<IncidentTimer> {
      return backendFetch<IncidentTimer>(`/incident-timers/${id}`, {
        method: 'PUT',
        body: updates,
      })
    },

    // Delete timer (protected) - direct backend call
    async deleteTimer(id: string): Promise<void> {
      return backendFetch<void>(`/incident-timers/${id}`, {
        method: 'DELETE',
      })
    },

    // Quick reset - create new timer with current timestamp (protected)
    async quickReset(notes?: string): Promise<IncidentTimer> {
      return this.createTimer({
        reset_timestamp: new Date().toISOString(),
        notes,
      })
    },
  }
}