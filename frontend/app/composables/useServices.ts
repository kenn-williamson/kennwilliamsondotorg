import { createAuthService } from '~/services/auth.service'
import { createIncidentTimerService } from '~/services/incident-timer.service'

export function useServices() {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBase

  const authService = createAuthService(apiBase)
  const incidentTimerService = createIncidentTimerService(apiBase)

  return {
    authService,
    incidentTimerService,
  }
}
