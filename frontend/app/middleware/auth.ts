export default defineNuxtRouteMiddleware((to, from) => {
  const authStore = useAuthStore()
  
  // Check authentication status
  if (!authStore.isAuthenticated) {
    // Try to restore auth from cookie
    authStore.checkAuth()
    
    // If still not authenticated, redirect to login
    if (!authStore.isAuthenticated) {
      return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`)
    }
  }
})