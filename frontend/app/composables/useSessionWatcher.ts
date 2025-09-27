/**
 * Session Watcher Composable - Automatically clears store data on logout
 * 
 * This composable watches for session changes and calls a provided callback
 * when the user logs out (transitions from logged in to logged out).
 * 
 * Usage in stores:
 * ```typescript
 * export const useMyStore = defineStore('my-store', () => {
 *   // ... store state and actions ...
 *   
 *   const clearAllData = () => {
 *     // Clear all store data
 *   }
 *   
 *   // Set up automatic cleanup on logout
 *   useSessionWatcher(clearAllData)
 *   
 *   return { clearAllData }
 * })
 * ```
 */

export const useSessionWatcher = (clearCallback: () => void) => {
  const { loggedIn } = useUserSession()
  
  // Watch for session changes
  watch(loggedIn, (newValue, oldValue) => {
    // Clear stores when user logs out (goes from logged in to logged out)
    if (oldValue === true && newValue === false) {
      console.log('🔄 [SessionWatcher] User logged out, clearing store data')
      clearCallback()
    }
  }, { immediate: false })
}
