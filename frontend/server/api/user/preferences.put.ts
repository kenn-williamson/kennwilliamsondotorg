import { defineEventHandler, createError, readBody } from 'h3'
import { useRuntimeConfig } from '#imports'
import { requireValidJwtToken } from '../../utils/jwt-handler'
import { API_ROUTES } from '#shared/config/api-routes'
import type { User, UpdatePreferencesRequest } from '#shared/types'

export default defineEventHandler(async (event) => {
  try {
    const jwtToken = await requireValidJwtToken(event)
    const body = await readBody<UpdatePreferencesRequest>(event)
    const config = useRuntimeConfig()

    console.log('🔍 [Preferences API] Updating user preferences:', body)

    // Call backend preferences endpoint
    const user = await $fetch<User>(
      `${config.apiBase}${API_ROUTES.PROTECTED.AUTH.PREFERENCES}`,
      {
        method: 'PUT',
        headers: { 'Authorization': `Bearer ${jwtToken}` },
        body
      }
    )

    console.log('✅ [Preferences API] Preferences updated successfully')

    // Update session with full user object
    const session = await getUserSession(event)
    await setUserSession(event, {
      ...session,
      user
    })

    console.log('✅ [Preferences API] Updated session with fresh user data')

    return { message: 'Preferences updated successfully' }
  } catch (error: any) {
    console.error('❌ [Preferences API] Failed to update preferences:', error.message)
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.data?.error || 'Failed to update preferences'
    })
  }
})
