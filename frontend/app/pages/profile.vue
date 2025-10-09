<template>
  <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- Header -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-900">Profile Settings</h1>
        <p class="mt-2 text-gray-600">Manage your account information and security settings.</p>
      </div>

      <!-- Email Verification Banner -->
      <EmailVerificationBanner />

      <!-- Email Verification Status -->
      <div v-if="user && !user.roles?.includes('email-verified')" class="bg-white shadow rounded-lg mb-8">
        <div class="px-6 py-4 border-b border-gray-200">
          <h2 class="text-lg font-medium text-gray-900">Email Verification</h2>
          <p class="mt-1 text-sm text-gray-500">Verify your email to access all features.</p>
        </div>
        <div class="px-6 py-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-700">
                <span class="font-medium">Status:</span>
                <span class="ml-2 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                  Not Verified
                </span>
              </p>
              <p class="mt-2 text-sm text-gray-600">
                A verification email has been sent to <span class="font-medium">{{ user.email }}</span>
              </p>
            </div>
            <button
              @click="handleResendVerification"
              :disabled="verificationLoading || verificationSent"
              class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-sky-600 hover:bg-sky-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
            >
              <svg v-if="verificationLoading" class="animate-spin -ml-0.5 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
              </svg>
              {{ verificationSent ? 'Email sent!' : 'Resend verification email' }}
            </button>
          </div>
          <div v-if="verificationSent" class="mt-4 text-sm text-green-700">
            Verification email sent successfully. Please check your inbox.
          </div>
        </div>
      </div>

      <!-- OAuth Accounts Section -->
      <div v-if="user" class="bg-white shadow rounded-lg mb-8">
        <div class="px-6 py-4 border-b border-gray-200">
          <h2 class="text-lg font-medium text-gray-900">Connected Accounts</h2>
          <p class="mt-1 text-sm text-gray-500">Link your account with Google for easier sign-in.</p>
        </div>
        <div class="px-6 py-4">
          <!-- Google Account -->
          <div class="flex items-center justify-between">
            <div class="flex items-center">
              <svg class="w-6 h-6" viewBox="0 0 24 24" fill="none">
                <path d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" fill="#4285F4"/>
                <path d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" fill="#34A853"/>
                <path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" fill="#FBBC05"/>
                <path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" fill="#EA4335"/>
              </svg>
              <div class="ml-3">
                <p class="text-sm font-medium text-gray-900">Google</p>
                <p v-if="user.google_user_id && user.real_name" class="text-sm text-gray-500">{{ user.real_name }}</p>
                <p v-else-if="user.google_user_id" class="text-sm text-gray-500">Connected</p>
                <p v-else class="text-sm text-gray-500">Not connected</p>
              </div>
            </div>
            <div v-if="user.google_user_id">
              <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
                <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                </svg>
                Connected
              </span>
            </div>
            <div v-else>
              <GoogleOAuthButton variant="link" />
            </div>
          </div>
        </div>
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
      <div class="bg-white shadow rounded-lg mb-8">
        <div class="px-6 py-4 border-b border-gray-200">
          <h2 class="text-lg font-medium text-gray-900">Security</h2>
          <p class="mt-1 text-sm text-gray-500">Change your password to keep your account secure.</p>
        </div>
        <div class="px-6 py-4">
          <SecurityForm />
        </div>
      </div>

      <!-- Data Export Section -->
      <DataExport class="mb-8" />

      <!-- Delete Account Section -->
      <DeleteAccountSection v-if="user" :user="user" />
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useEmailVerification } from '~/composables/useEmailVerification'

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

// Clear public timer when navigating to authenticated pages
const incidentTimerStore = useIncidentTimerStore()
incidentTimerStore.clearPublicTimerOnNavigation()

// Get user data from session
const { user, refresh: refreshSession } = useUserSession()

// Reactive references for template compatibility
const refreshUser = () => refreshSession()
const userPending = computed(() => false) // Session data is already loaded
const userError = computed(() => null) // Session handles auth errors

// Email verification
const { sendVerificationEmail, isLoading: verificationLoading } = useEmailVerification()
const verificationSent = ref(false)

const handleResendVerification = async () => {
  try {
    await sendVerificationEmail()
    verificationSent.value = true

    // Reset sent status after 5 seconds
    setTimeout(() => {
      verificationSent.value = false
    }, 5000)
  } catch (error) {
    console.error('Failed to resend verification email:', error)
  }
}

// Event handlers removed - components handle success via action composables
</script>
