<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 px-4">
    <div class="max-w-md w-full">
      <!-- Loading State -->
      <div v-if="isLoading" class="text-center">
        <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-sky-600 mb-4"></div>
        <h2 class="text-xl font-semibold text-gray-900">Completing sign in...</h2>
        <p class="text-gray-600 mt-2">Please wait while we verify your Google account</p>
      </div>

      <!-- Error State -->
      <div v-else-if="hasError" class="text-center">
        <div class="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-red-100 mb-4">
          <svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </div>
        <h2 class="text-xl font-semibold text-gray-900 mb-2">Authentication Failed</h2>
        <p class="text-gray-600 mb-6">{{ error }}</p>
        <NuxtLink
          to="/login"
          class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-sky-600 hover:bg-sky-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-500"
        >
          Return to Login
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useGoogleOAuth } from '~/composables/useGoogleOAuth'

definePageMeta({
  middleware: [],  // No auth required for callback page
  layout: false,    // No layout for cleaner loading experience
})

const route = useRoute()
const router = useRouter()

const { handleOAuthCallback, isLoading, error: oauthError, hasError } = useGoogleOAuth()

// Local error state for this component
const error = ref<string | null>(null)

// Handle OAuth callback on mount
onMounted(async () => {
  const code = route.query.code as string
  const state = route.query.state as string
  const errorParam = route.query.error as string

  // Check for OAuth errors from Google
  if (errorParam) {
    error.value = 'Google authentication was cancelled or failed'
    return
  }

  // Validate required parameters
  if (!code || !state) {
    error.value = 'Missing authorization code or state parameter. Please try signing in again.'
    return
  }

  try {
    // Complete OAuth flow
    await handleOAuthCallback(code, state)

    // Refresh the session to pick up the new user data
    const { fetch: refreshSession } = useUserSession()
    await refreshSession()

    // Success - redirect to home page
    await router.push('/')
  } catch (err) {
    // Use the error from the composable or fallback to a generic message
    error.value = oauthError.value || 'Authentication failed'
    console.error('OAuth callback error:', err)
  }
})
</script>
