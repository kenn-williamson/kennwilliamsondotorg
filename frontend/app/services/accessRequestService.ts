/**
 * Pure Access Request Service - No Vue context, accepts fetcher as parameter
 * Easy to test with mock fetchers
 */

import { API_ROUTES } from '#shared/config/api-routes'
import type { Fetcher } from '#shared/types'

interface CreateAccessRequestResponse {
  message: string
}

export const accessRequestService = (fetcher: Fetcher) => ({
  createAccessRequest: async (message: string): Promise<CreateAccessRequestResponse> => {
    return fetcher<CreateAccessRequestResponse>(
      API_ROUTES.PROTECTED.ACCESS_REQUESTS.CREATE,
      {
        method: 'POST',
        body: { message },
      }
    )
  },
})
