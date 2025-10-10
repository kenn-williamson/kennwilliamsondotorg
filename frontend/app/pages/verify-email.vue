<template>
  <div class="min-h-screen flex items-center justify-center px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-slate-50 via-sky-50 to-blue-50">
    <div class="max-w-md w-full">
      <!-- Success State -->
      <div v-if="verificationStatus === 'success'" class="text-center">
        <div class="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-green-100 mb-6">
          <svg class="h-10 w-10 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        </div>
        <h1 class="text-3xl font-bold text-gray-900 mb-2">Email Verified!</h1>
        <p class="text-gray-600 mb-6">Your email address has been successfully verified.</p>
        <p class="text-sm text-gray-500 mb-4">Redirecting to homepage...</p>
      </div>

      <!-- Error State -->
      <div v-else-if="verificationStatus === 'error'" class="text-center">
        <div class="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-red-100 mb-6">
          <svg class="h-10 w-10 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </div>
        <h1 class="text-3xl font-bold text-gray-900 mb-2">Verification Failed</h1>
        <p class="text-gray-600 mb-6">{{ errorMessage }}</p>

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
        <h1 class="text-2xl font-bold text-gray-900 mb-2">Verifying Email...</h1>
        <p class="text-gray-600">Please wait while we verify your email address.</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmailVerificationActions } from '~/composables/useEmailVerificationActions'

// Page meta
useHead({
  title: 'Verify Email',
  meta: [
    { name: 'description', content: 'Email verification page' }
  ]
})

const route = useRoute()
const router = useRouter()

const verificationStatus = ref<'loading' | 'success' | 'error'>('loading')
const errorMessage = ref('Invalid or expired verification link. Please try resending the verification email.')

const { verifyEmail } = useEmailVerificationActions()

onMounted(async () => {
  const token = route.query.token as string

  if (!token) {
    verificationStatus.value = 'error'
    errorMessage.value = 'No verification token provided.'
    return
  }

  try {
    await verifyEmail(token)
    verificationStatus.value = 'success'

    // Redirect to homepage after 2 seconds
    setTimeout(() => {
      router.push('/')
    }, 2000)
  } catch (error: any) {
    console.error('Email verification failed:', error)
    verificationStatus.value = 'error'

    // Use specific error message if available
    if (error.statusCode === 400) {
      errorMessage.value = 'This verification link has expired or is invalid. Please request a new verification email.'
    } else if (error.statusCode === 404) {
      errorMessage.value = 'Verification token not found. Please request a new verification email.'
    }
  }
})
</script>
