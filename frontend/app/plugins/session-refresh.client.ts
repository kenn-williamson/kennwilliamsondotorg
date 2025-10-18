/**
 * Session Refresh Plugin
 *
 * Refreshes user session on app initialization to ensure fresh data from backend.
 * This catches changes from:
 * - Admin role updates
 * - Email verification status changes
 * - Updates from other browser tabs/devices
 * - Any other backend state changes
 *
 * Client-only plugin - runs after hydration on client-side
 *
 * Uses existing /api/auth/me endpoint which:
 * - Uses refresh token to get new JWT from backend
 * - Backend returns complete user object with latest data
 * - Session automatically updated via performRefresh()
 */
export default defineNuxtPlugin(async () => {
  const { loggedIn } = useUserSession()
  const smartFetch = useSmartFetch()

  // Only refresh if user is logged in
  if (loggedIn.value) {
    try {
      await smartFetch('/api/auth/me')
      console.log('✅ [Session Refresh Plugin] Session refreshed on app load')
    } catch (error) {
      console.error('❌ [Session Refresh Plugin] Failed to refresh session on load:', error)
      // Don't throw - let app continue with stale session
      // User will be prompted to log in again if session is truly invalid
    }
  }
})
