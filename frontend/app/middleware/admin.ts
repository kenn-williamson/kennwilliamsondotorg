export default defineNuxtRouteMiddleware(async (to, from) => {
  const { user } = useUserSession()
  
  // Check if user is authenticated
  if (!user.value) {
    return navigateTo('/login')
  }
  
  // Check if user has admin role
  if (!user.value || !('roles' in user.value) || !Array.isArray(user.value.roles) || !user.value.roles.includes('admin')) {
    return navigateTo('/')
  }
})
