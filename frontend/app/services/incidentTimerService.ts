/**
 * Incident Timer Service - Uses useRequestFetch for SSR-safe requests
 */

import { API_ROUTES } from '#shared/config/api-routes'
import type { 
  IncidentTimer, 
  PublicTimerResponse,
  CreateTimerRequest,
  UpdateTimerRequest,
  Fetcher
} from '#shared/types'

export const incidentTimerService = (fetcher: Fetcher) => ({
  getUserTimers: async (): Promise<IncidentTimer[]> => {
    return fetcher<IncidentTimer[]>(API_ROUTES.PROTECTED.TIMERS.LIST)
  },

  getPublicTimer: async (userSlug: string): Promise<PublicTimerResponse> => {
    return fetcher<PublicTimerResponse>(API_ROUTES.PUBLIC.TIMERS.BY_USER_SLUG(userSlug))
  },

  createTimer: async (timerData: CreateTimerRequest): Promise<IncidentTimer> => {
    return fetcher<IncidentTimer>(API_ROUTES.PROTECTED.TIMERS.CREATE, {
      method: 'POST',
      body: timerData
    })
  },

  updateTimer: async (timerId: string, timerData: UpdateTimerRequest): Promise<IncidentTimer> => {
    return fetcher<IncidentTimer>(API_ROUTES.PROTECTED.TIMERS.UPDATE(timerId), {
      method: 'PUT',
      body: timerData
    })
  },

  deleteTimer: async (timerId: string): Promise<void> => {
    return fetcher<void>(API_ROUTES.PROTECTED.TIMERS.DELETE(timerId), {
      method: 'DELETE'
    })
  },

  quickReset: async (timerId: string): Promise<IncidentTimer> => {
    return fetcher<IncidentTimer>(API_ROUTES.PROTECTED.TIMERS.UPDATE(timerId), {
      method: 'POST'
    })
  }
})
