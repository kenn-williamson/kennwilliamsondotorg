export default defineNuxtRouteMiddleware(async (to, from) => {
  const { user } = useUserSession()
  
  if (!user.value) {
    return navigateTo('/login')
  }
  if (!user.value || !('roles' in user.value) || !Array.isArray(user.value.roles) || !user.value.roles.includes('admin')) {
    return navigateTo('/')
  }
})
