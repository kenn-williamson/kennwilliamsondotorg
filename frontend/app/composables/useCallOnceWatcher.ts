/**
 * CallOnce Watcher Composable - Automatically clears user-specific callOnce cache on logout
 * 
 * This composable watches for session changes and clears only user-specific callOnce cache
 * when the user logs out, allowing components to reload data for new users while preserving
 * public/shared data.
 * 
 * Usage:
 * ```typescript
 * // Set up callOnce cache clearing on logout (app-level)
 * useCallOnceWatcher()
 * 
 * // Now callOnce will work properly across user sessions
 * await callOnce('user-phrase-suggestions', () => loadData())
 * ```
 */

export const useCallOnceWatcher = () => {
  const { loggedIn } = useUserSession()
  
  // Define callOnce keys that should be cleared on logout
  const keysToClear = [
    'user-phrase-suggestions',
    'user-suggestion-history', 
    'user-timers',
    'public-timer',        // Not parameterized by slug - needs clearing
    'random-phrase',       // Not parameterized by slug - needs clearing
    // Add new keys here as needed
  ]
  
  // Watch for session changes
  watch(loggedIn, (newValue, oldValue) => {
    // Clear callOnce cache when user logs out (goes from logged in to logged out)
    if (oldValue === true && newValue === false) {
      console.log('🔄 [CallOnceWatcher] User logged out, clearing callOnce cache')
      
      // Clear the specific callOnce keys
      clearNuxtData(keysToClear)
      
      // Also refresh all Nuxt data to ensure components reload
      refreshNuxtData()
    }
  }, { immediate: false })
}
