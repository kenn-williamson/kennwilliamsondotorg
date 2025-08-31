/**
 * useIncidentTimerService - Incident timer operations using composable pattern
 * 
 * Replaces IncidentTimerService class with composable function.
 * Uses useAuthFetch for automatic authentication handling.
 */

interface IncidentTimer {
  id: string
  reset_timestamp: string
  notes?: string
  created_at: string
  updated_at: string
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
  const config = useRuntimeConfig()
  const authFetch = useAuthFetch()

  // Helper to create full API URL
  const apiUrl = (endpoint: string) => `${config.public.apiBase}${endpoint}`

  return {
    // Get all timers for current user (protected)
    async getUserTimers(): Promise<IncidentTimer[]> {
      return authFetch(apiUrl('/incident-timers'), {
        method: 'GET',
      })
    },

    // Get public timer by user slug (no auth required)
    async getPublicTimer(userSlug: string): Promise<IncidentTimer> {
      return authFetch(apiUrl(`/${userSlug}/incident-timer`), {
        method: 'GET',
      })
    },

    // Create new timer (protected)
    async createTimer(timerData: CreateTimerRequest): Promise<IncidentTimer> {
      return authFetch(apiUrl('/incident-timers'), {
        method: 'POST',
        body: timerData,
      })
    },

    // Update existing timer (protected)
    async updateTimer(id: string, updates: UpdateTimerRequest): Promise<IncidentTimer> {
      return authFetch(apiUrl(`/incident-timers/${id}`), {
        method: 'PUT',
        body: updates,
      })
    },

    // Delete timer (protected)
    async deleteTimer(id: string): Promise<void> {
      return authFetch(apiUrl(`/incident-timers/${id}`), {
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