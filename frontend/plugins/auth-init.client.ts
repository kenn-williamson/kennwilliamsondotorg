/**
 * Auth Initialization Plugin
 * 
 * Runs on app load to initialize authentication state and handle session recovery.
 * This ensures the user's authentication state is properly restored on page refresh.
 */

export default defineNuxtPlugin(async () => {
  const authManager = useAuthManager()
  
  console.log('🚀 [Auth Plugin] Initializing authentication on app load...')
  
  // Initialize authentication state
  await authManager.initializeAuth()
  
  console.log('✅ [Auth Plugin] Authentication initialization complete')
})
