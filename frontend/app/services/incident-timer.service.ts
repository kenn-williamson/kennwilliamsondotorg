import { BaseService } from './base.service'

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

export class IncidentTimerService extends BaseService {
  constructor(apiBase: string) {
    super(apiBase)
  }

  async getUserTimers(authStore: any): Promise<IncidentTimer[]> {
    return this.makeRequest<IncidentTimer[]>('/incident-timers', {
      headers: this.getAuthHeaders(authStore),
    })
  }

  async getPublicTimer(userSlug: string): Promise<IncidentTimer> {
    return this.makeRequest<IncidentTimer>(`/${userSlug}/incident-timers`)
  }

  async createTimer(timerData: CreateTimerRequest, authStore: any): Promise<IncidentTimer> {
    return this.makeRequest<IncidentTimer>('/incident-timers', {
      method: 'POST',
      headers: this.getAuthHeaders(authStore),
      body: timerData,
    })
  }

  async updateTimer(id: string, updates: UpdateTimerRequest, authStore: any): Promise<IncidentTimer> {
    return this.makeRequest<IncidentTimer>(`/incident-timers/${id}`, {
      method: 'PUT',
      headers: this.getAuthHeaders(authStore),
      body: updates,
    })
  }

  async deleteTimer(id: string, authStore: any): Promise<void> {
    return this.makeRequest<void>(`/incident-timers/${id}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(authStore),
    })
  }

  async quickReset(authStore: any, notes?: string): Promise<IncidentTimer> {
    return this.createTimer({
      reset_timestamp: new Date().toISOString(),
      notes,
    }, authStore)
  }
}

// Export factory function instead of singleton
export function createIncidentTimerService(apiBase: string): IncidentTimerService {
  return new IncidentTimerService(apiBase)
}