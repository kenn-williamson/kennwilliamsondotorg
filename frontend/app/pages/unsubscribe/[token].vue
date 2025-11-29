<template>
  <div class="min-h-screen flex items-center justify-center px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-slate-50 via-sky-50 to-blue-50">
    <div class="max-w-md w-full">
      <!-- Success State -->
      <div v-if="status === 'success'" class="text-center">
        <div class="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-green-100 mb-6">
          <svg class="h-10 w-10 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        </div>
        <h1 class="text-3xl font-bold text-nautical-900 mb-2">Unsubscribed</h1>
        <p class="text-nautical-600 mb-6">
          You have been unsubscribed from blog post notifications.
        </p>
        <p class="text-sm text-nautical-500 mb-6">
          You can re-enable notifications anytime from your account preferences.
        </p>
        <NuxtLink
          to="/"
          class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-sky-600 hover:bg-sky-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-500 transition-colors duration-200"
        >
          Go to Homepage
        </NuxtLink>
      </div>

      <!-- Error State -->
      <div v-else-if="status === 'error'" class="text-center">
        <div class="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-red-100 mb-6">
          <svg class="h-10 w-10 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </div>
        <h1 class="text-3xl font-bold text-nautical-900 mb-2">Unsubscribe Failed</h1>
        <p class="text-nautical-600 mb-6">{{ errorMessage }}</p>

        <div class="space-y-3">
          <NuxtLink
            to="/"
            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-sky-600 hover:bg-sky-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-500 transition-colors duration-200"
          >
            Go to Homepage
          </NuxtLink>
        </div>
      </div>

      <!-- Loading State -->
      <div v-else class="text-center">
        <div class="mx-auto flex items-center justify-center h-16 w-16 mb-6">
          <svg class="animate-spin h-12 w-12 text-sky-600" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
          </svg>
        </div>
        <h1 class="text-2xl font-bold text-nautical-900 mb-2">Unsubscribing...</h1>
        <p class="text-nautical-600">Please wait while we process your request.</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSmartFetch } from '~/composables/useSmartFetch'
import { API_ROUTES } from '#shared/config/api-routes'

useHead({
  title: 'Unsubscribe',
  meta: [
    { name: 'description', content: 'Unsubscribe from email notifications' }
  ]
})

const route = useRoute()
const smartFetch = useSmartFetch()
const { loggedIn, fetch: refreshSession } = useUserSession()

const status = ref<'loading' | 'success' | 'error'>('loading')
const errorMessage = ref('Invalid or expired unsubscribe link.')

onMounted(async () => {
  const token = route.params.token as string

  if (!token) {
    status.value = 'error'
    errorMessage.value = 'No unsubscribe token provided.'
    return
  }

  try {
    await smartFetch(API_ROUTES.PUBLIC.EMAIL.UNSUBSCRIBE, {
      method: 'POST',
      body: { token }
    })

    // Refresh session if user is logged in so preferences toggle updates
    if (loggedIn.value) {
      // Fetch fresh user data from backend (updates server session)
      await smartFetch(API_ROUTES.API.AUTH.ME)
      // Refresh client-side reactive state
      await refreshSession()
    }

    status.value = 'success'
  } catch (error: any) {
    console.error('Unsubscribe failed:', error)
    status.value = 'error'

    if (error.statusCode === 404) {
      errorMessage.value = 'This unsubscribe link is invalid or has expired.'
    } else if (error.statusCode === 410) {
      errorMessage.value = 'You have already been unsubscribed from these notifications.'
    } else {
      errorMessage.value = 'An error occurred while processing your request. Please try again later.'
    }
  }
})
</script>
