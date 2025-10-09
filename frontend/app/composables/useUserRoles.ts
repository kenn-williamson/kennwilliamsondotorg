import { computed } from 'vue'

/**
 * Composable for checking user roles and permissions
 */
export function useUserRoles() {
  const { user } = useUserSession()

  const roles = computed(() => user.value?.roles || [])

  const hasTrustedContactAccess = computed(() => {
    return roles.value.includes('trusted-contact') || roles.value.includes('admin')
  })

  const isAdmin = computed(() => {
    return roles.value.includes('admin')
  })

  const hasRole = (role: string) => {
    return roles.value.includes(role)
  }

  return {
    roles,
    hasTrustedContactAccess,
    isAdmin,
    hasRole
  }
}
