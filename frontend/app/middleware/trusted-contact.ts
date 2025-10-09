export default defineNuxtRouteMiddleware(async (to, from) => {
  const { loggedIn, user } = useUserSession()

  // First check if user is authenticated
  if (!loggedIn.value) {
    console.log('User not authenticated, redirecting to login with redirect=', to.path)
    return navigateTo(`/login?redirect=${encodeURIComponent(to.path)}`)
  }

  // Check if user has trusted-contact role
  const roles = user.value?.roles || []
  const hasTrustedContactRole = roles.includes('trusted-contact')
  const isAdmin = roles.includes('admin')

  // Allow access if user has trusted-contact role or is admin
  if (!hasTrustedContactRole && !isAdmin) {
    console.log('User does not have trusted-contact role, redirecting to request access')
    return navigateTo('/about/request-access')
  }
})
