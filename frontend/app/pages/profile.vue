<template>
  <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- Header -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-900">Profile Settings</h1>
        <p class="mt-2 text-gray-600">Manage your account information and security settings.</p>
      </div>

      <!-- Account Information Form -->
      <div class="bg-white shadow rounded-lg mb-8">
        <div class="px-6 py-4 border-b border-gray-200">
          <h2 class="text-lg font-medium text-gray-900">Account Information</h2>
          <p class="mt-1 text-sm text-gray-500">Update your display name and username.</p>
        </div>
        <div class="px-6 py-4">
          <div v-if="userPending" class="flex items-center justify-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-sky-600"></div>
            <span class="ml-3 text-gray-600">Loading profile...</span>
          </div>
          <AccountInformationForm 
            v-else-if="user"
            :user="user"
          />
          <div v-else-if="userError" class="text-red-600">
            Failed to load profile data: {{ userError.message }}
          </div>
          <div v-else class="text-red-600">
            No profile data available
          </div>
        </div>
      </div>

      <!-- Security Form -->
      <div class="bg-white shadow rounded-lg">
        <div class="px-6 py-4 border-b border-gray-200">
          <h2 class="text-lg font-medium text-gray-900">Security</h2>
          <p class="mt-1 text-sm text-gray-500">Change your password to keep your account secure.</p>
        </div>
        <div class="px-6 py-4">
          <SecurityForm />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

// Page meta
useHead({
  title: 'Profile Settings',
  meta: [
    { name: 'description', content: 'Manage your account information and security settings.' }
  ]
})

// Authentication check
const { loggedIn } = useUserSession()

// Redirect if not authenticated
if (!loggedIn.value) {
  await navigateTo('/login')
}

// Fetch fresh user data from backend
const { data: user, refresh: refreshUser, pending: userPending, error: userError } = await useFetch('/api/auth/me', {
  server: true,
  default: () => null
})

// Handle authentication errors
if (userError.value?.statusCode === 401) {
  await navigateTo('/login')
}

// Event handlers removed - components handle success via action composables
</script>
