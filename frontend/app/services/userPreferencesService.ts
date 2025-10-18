/**
 * User Preferences Service - No Vue context, accepts fetcher as parameter
 * Handles user preference updates and public timer list fetching
 */

import { API_ROUTES } from '#shared/config/api-routes'
import type {
  Fetcher,
  UpdatePreferencesRequest,
  PublicTimerListItem,
  User
} from '#shared/types'

export const userPreferencesService = (fetcher: Fetcher) => ({
  /**
   * Update user preferences (timer privacy settings)
   */
  updatePreferences: async (data: UpdatePreferencesRequest): Promise<User> => {
    return fetcher<User>(API_ROUTES.PROTECTED.AUTH.PREFERENCES, {
      method: 'PUT',
      body: data
    })
  },

  /**
   * Get public timer list with pagination and search
   * @param page - Page number (default: 1)
   * @param pageSize - Items per page (default: 20, max: 100)
   * @param search - Search query for display name (optional)
   */
  getPublicTimerList: async (
    page: number = 1,
    pageSize: number = 20,
    search?: string
  ): Promise<PublicTimerListItem[]> => {
    const query: Record<string, any> = { page, page_size: pageSize }

    // Add search parameter if provided
    if (search && search.trim()) {
      query.search = search.trim()
    }

    return fetcher<PublicTimerListItem[]>(API_ROUTES.PUBLIC.PUBLIC_TIMER_LIST, {
      method: 'GET',
      query
    })
  }
})
