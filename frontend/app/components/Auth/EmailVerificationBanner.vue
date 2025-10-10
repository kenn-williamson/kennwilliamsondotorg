<template>
  <div
    v-if="shouldShowBanner"
    class="bg-amber-50 border-l-4 border-amber-400 p-4"
  >
    <div class="flex items-center justify-between">
      <div class="flex items-center flex-1">
        <!-- Warning Icon -->
        <div class="flex-shrink-0">
          <svg class="h-5 w-5 text-amber-400" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
          </svg>
        </div>

        <!-- Message -->
        <div class="ml-3 flex-1">
          <p class="text-sm text-amber-800">
            <span class="font-medium">Email not verified.</span>
            {{ ' ' }}
            Please check your inbox and verify your email address to access all features.
          </p>
        </div>

        <!-- Resend Button -->
        <div class="flex-shrink-0 ml-4">
          <button
            @click="handleResend"
            :disabled="isLoading || emailSent"
            class="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-amber-700 bg-amber-100 hover:bg-amber-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-amber-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
          >
            <svg v-if="isLoading" class="animate-spin -ml-0.5 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
            </svg>
            {{ emailSent ? 'Email sent!' : 'Resend verification email' }}
          </button>
        </div>
      </div>

      <!-- Dismiss Button -->
      <div class="ml-4 flex-shrink-0">
        <button
          @click="dismissBanner"
          class="inline-flex text-amber-400 hover:text-amber-500 focus:outline-none focus:text-amber-500 transition-colors duration-200"
        >
          <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Success Message -->
    <div v-if="emailSent" class="mt-2 ml-8">
      <p class="text-sm text-green-700">
        Verification email sent successfully. Please check your inbox.
      </p>
    </div>

    <!-- Error Message -->
    <div v-if="error" class="mt-2 ml-8">
      <p class="text-sm text-red-700">
        Failed to send verification email. Please try again later.
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmailVerificationActions } from '~/composables/useEmailVerificationActions'

const { user, loggedIn } = useUserSession()

// Check if user needs to verify email (logged in and doesn't have email-verified role)
const needsVerification = computed(() => {
  if (!loggedIn.value || !user.value) return false
  return !user.value.roles?.includes('email-verified')
})

// Check if banner is dismissed (expires after 24 hours)
const dismissedUntil = ref<number | null>(null)

onMounted(() => {
  const stored = localStorage.getItem('email-verification-banner-dismissed')
  if (stored) {
    dismissedUntil.value = parseInt(stored, 10)
  }
})

const isDismissed = computed(() => {
  if (!dismissedUntil.value) return false
  return Date.now() < dismissedUntil.value
})

const shouldShowBanner = computed(() => {
  return needsVerification.value && !isDismissed.value
})

// Email verification actions
const { sendVerificationEmail, isLoading, error } = useEmailVerificationActions()
const emailSent = ref(false)

const handleResend = async () => {
  try {
    await sendVerificationEmail()
    emailSent.value = true

    // Reset email sent status after 5 seconds
    setTimeout(() => {
      emailSent.value = false
    }, 5000)
  } catch (err) {
    console.error('Failed to resend verification email:', err)
  }
}

const dismissBanner = () => {
  // Dismiss for 24 hours
  const dismissUntil = Date.now() + 24 * 60 * 60 * 1000
  dismissedUntil.value = dismissUntil
  localStorage.setItem('email-verification-banner-dismissed', dismissUntil.toString())
}
</script>
