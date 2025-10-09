<template>
  <div class="users-tab">
    <!-- Search Box -->
    <UserSearchBox />

    <!-- Loading State -->
    <div v-if="adminStore.isLoading" class="flex justify-center items-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
    </div>

    <!-- Error State -->
    <div v-else-if="adminStore.error" class="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
      <p class="text-red-800 text-sm">{{ adminStore.error }}</p>
      <button 
        @click="refreshUsers"
        class="mt-2 text-sm text-red-600 hover:text-red-700 underline"
      >
        Try again
      </button>
    </div>

    <!-- Users List -->
    <div v-else-if="adminStore.users.length > 0" class="space-y-4">
      <div 
        v-for="user in adminStore.users" 
        :key="user.id"
        class="bg-white rounded-lg shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow"
      >
        <div class="flex items-center justify-between">
          <div class="flex-1">
            <div class="flex items-center space-x-3">
              <div class="flex-shrink-0">
                <div class="w-10 h-10 bg-gray-100 rounded-full flex items-center justify-center">
                  <span class="text-gray-600 font-medium text-sm">
                    {{ user.display_name.charAt(0).toUpperCase() }}
                  </span>
                </div>
              </div>
              <div class="flex-1 min-w-0">
                <h3 class="text-lg font-medium text-gray-900 truncate">
                  {{ user.display_name }}
                </h3>
                <p class="text-sm text-gray-500 truncate">{{ user.email }}</p>
                <p class="text-xs text-gray-400">@{{ user.slug }}</p>
              </div>
            </div>
          </div>
          
          <div class="flex items-center space-x-4">
            <!-- Status Badge -->
            <span 
              :class="[
                'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium',
                user.active 
                  ? 'bg-green-100 text-green-800' 
                  : 'bg-red-100 text-red-800'
              ]"
            >
              {{ user.active ? 'Active' : 'Inactive' }}
            </span>

            <!-- Role Badge -->
            <span 
              v-if="user.roles.includes('admin')"
              class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
            >
              Admin
            </span>

            <!-- Actions -->
            <div class="flex items-center space-x-2">
              <button
                @click="viewUser(user)"
                class="text-gray-400 hover:text-gray-600 transition-colors"
                title="View details"
              >
                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                </svg>
              </button>
            </div>
          </div>
        </div>

        <!-- User Details (when expanded) -->
        <div v-if="adminStore.selectedUser?.id === user.id" class="mt-4 pt-4 border-t border-gray-200">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <h4 class="text-sm font-medium text-gray-900 mb-2">Account Information</h4>
              <dl class="space-y-1">
                <div>
                  <dt class="text-xs text-gray-500">User ID</dt>
                  <dd class="text-sm text-gray-900 font-mono">{{ user.id }}</dd>
                </div>
                <div>
                  <dt class="text-xs text-gray-500">Created</dt>
                  <dd class="text-sm text-gray-900">{{ formatDate(user.created_at) }}</dd>
                </div>
                <div>
                  <dt class="text-xs text-gray-500">Roles</dt>
                  <dd class="text-sm text-gray-900">{{ user.roles.join(', ') }}</dd>
                </div>
              </dl>
            </div>
            
            <div>
              <h4 class="text-sm font-medium text-gray-900 mb-2">Roles</h4>
              <div class="space-y-2 mb-4">
                <!-- User role (base, immutable) -->
                <div class="flex items-center justify-between p-2 bg-gray-50 rounded-md">
                  <div class="flex items-center">
                    <input
                      type="checkbox"
                      :checked="user.roles.includes('user')"
                      disabled
                      class="h-4 w-4 rounded border-gray-300 text-gray-400 cursor-not-allowed"
                    />
                    <label class="ml-2 text-sm text-gray-700">
                      user <span class="text-xs text-gray-500">(base role)</span>
                    </label>
                  </div>
                </div>

                <!-- Email Verified role (manageable) -->
                <div class="flex items-center justify-between p-2 bg-gray-50 rounded-md hover:bg-gray-100">
                  <div class="flex items-center">
                    <input
                      type="checkbox"
                      :checked="user.roles.includes('email-verified')"
                      @change="toggleRole(user, 'email-verified')"
                      class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <label class="ml-2 text-sm text-gray-700 cursor-pointer">
                      email-verified
                    </label>
                  </div>
                </div>

                <!-- Trusted Contact role (manageable) -->
                <div class="flex items-center justify-between p-2 bg-gray-50 rounded-md hover:bg-gray-100">
                  <div class="flex items-center">
                    <input
                      type="checkbox"
                      :checked="user.roles.includes('trusted-contact')"
                      @change="toggleRole(user, 'trusted-contact')"
                      class="h-4 w-4 rounded border-gray-300 text-amber-600 focus:ring-amber-500"
                    />
                    <label class="ml-2 text-sm text-gray-700 cursor-pointer">
                      trusted-contact
                    </label>
                  </div>
                </div>

                <!-- Admin role (manageable) -->
                <div class="flex items-center justify-between p-2 bg-gray-50 rounded-md hover:bg-gray-100">
                  <div class="flex items-center">
                    <input
                      type="checkbox"
                      :checked="user.roles.includes('admin')"
                      @change="toggleRole(user, 'admin')"
                      class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <label class="ml-2 text-sm text-gray-700 cursor-pointer">
                      admin
                    </label>
                  </div>
                </div>
              </div>

              <h4 class="text-sm font-medium text-gray-900 mb-2 mt-4">Actions</h4>
              <div class="space-y-2">
                <button
                  @click="toggleUserStatus(user)"
                  :class="[
                    'w-full px-3 py-2 text-sm rounded-md transition-colors',
                    user.active
                      ? 'bg-red-100 text-red-700 hover:bg-red-200'
                      : 'bg-green-100 text-green-700 hover:bg-green-200'
                  ]"
                >
                  {{ user.active ? 'Deactivate User' : 'Activate User' }}
                </button>

                <button
                  @click="resetPassword(user)"
                  class="w-full px-3 py-2 text-sm bg-yellow-100 text-yellow-700 rounded-md hover:bg-yellow-200 transition-colors"
                >
                  Reset Password
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="text-center py-12">
      <div class="text-gray-400 text-6xl mb-4">👥</div>
      <h3 class="text-lg font-medium text-gray-900 mb-2">
        {{ adminStore.searchQuery ? 'No Users Found' : 'No Users Available' }}
      </h3>
      <p class="text-gray-500 mb-4">
        {{ adminStore.searchQuery 
          ? `No users match "${adminStore.searchQuery}"` 
          : 'Unable to load users.' 
        }}
      </p>
      <button 
        v-if="!adminStore.searchQuery"
        @click="refreshUsers"
        class="px-4 py-2 bg-gray-900 text-white rounded-md hover:bg-gray-800 transition-colors"
      >
        Refresh
      </button>
    </div>

    <!-- Password Reset Modal -->
    <div v-if="adminStore.newPassword" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg shadow-xl max-w-md w-full mx-4 p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Password Reset</h3>
        <p class="text-sm text-gray-600 mb-4">
          New password for <strong>{{ adminStore.selectedUser?.display_name }}</strong>:
        </p>
        <div class="bg-gray-100 rounded-md p-3 mb-4">
          <code class="text-sm font-mono text-gray-900">{{ adminStore.newPassword }}</code>
        </div>
        <p class="text-xs text-gray-500 mb-4">
          Please provide this password to the user. It will be required on their next login.
        </p>
        <div class="flex justify-end space-x-3">
          <button
            @click="copyPassword"
            class="px-4 py-2 text-sm bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors"
          >
            Copy Password
          </button>
          <button
            @click="adminStore.clearNewPassword()"
            class="px-4 py-2 text-sm bg-gray-900 text-white rounded-md hover:bg-gray-800 transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAdminStore } from '~/stores/admin'

const adminStore = useAdminStore()

// Load users directly in setup. This runs ON THE SERVER.
// Nuxt will wait for this to complete before sending the page.
console.log('🔄 Loading admin users for UsersTab...')
await adminStore.fetchUsers()

// Refresh users function
const refreshUsers = async () => {
  await adminStore.fetchUsers()
}

// View user details
const viewUser = (user: any) => {
  if (adminStore.selectedUser?.id === user.id) {
    adminStore.setSelectedUser(null)
  } else {
    adminStore.setSelectedUser(user)
  }
}

// Toggle user status
const toggleUserStatus = async (user: any) => {
  try {
    if (user.active) {
      await adminStore.deactivateUser(user.id)
    } else {
      await adminStore.activateUser(user.id)
    }
  } catch (error) {
    console.error('Toggle user status error:', error)
  }
}

// Reset user password
const resetPassword = async (user: any) => {
  try {
    adminStore.setSelectedUser(user)
    await adminStore.resetUserPassword(user.id)
  } catch (error) {
    console.error('Reset password error:', error)
  }
}

// Promote user to admin (legacy - replaced by toggleRole)
const handlePromoteUser = async (user: any) => {
  try {
    await adminStore.promoteUser(user.id)
  } catch (error) {
    console.error('Promote user error:', error)
  }
}

// Toggle user role (add/remove)
const toggleRole = async (user: any, roleName: string) => {
  try {
    const hasRole = user.roles.includes(roleName)

    if (hasRole) {
      // Remove role
      await adminStore.removeUserRole(user.id, roleName)
    } else {
      // Add role
      await adminStore.addUserRole(user.id, roleName)
    }
  } catch (error) {
    console.error(`Toggle role ${roleName} error:`, error)
  }
}

// Copy password to clipboard
const copyPassword = async () => {
  if (adminStore.newPassword) {
    try {
      await navigator.clipboard.writeText(adminStore.newPassword)
      // You could add a toast notification here
    } catch (error) {
      console.error('Failed to copy password:', error)
    }
  }
}

// Format date helper
const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>

<style scoped>
.users-tab {
  @apply space-y-6;
}
</style>
