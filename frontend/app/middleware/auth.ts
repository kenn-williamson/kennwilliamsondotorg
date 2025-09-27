export default defineNuxtRouteMiddleware(async (to, from) => {
  const { loggedIn } = useUserSession()
  
  // Check authentication status - Nuxt Auth Utils handles SSR timing automatically
  if (!loggedIn.value) {
    console.log('User not authenticated, redirecting to login with redirect=', to.path)
    return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`)
  }
})