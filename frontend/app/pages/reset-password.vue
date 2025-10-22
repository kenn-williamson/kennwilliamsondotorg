<template>
  <div class="min-h-screen flex items-center justify-center px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-slate-50 via-sky-50 to-blue-50">
    <div class="max-w-md w-full">
      <!-- Success State -->
      <div v-if="resetStatus === 'success'" class="text-center">
        <div class="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-green-100 mb-6">
          <svg class="h-10 w-10 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        </div>
        <h1 class="text-3xl font-bold text-nautical-900 mb-2">Password Reset!</h1>
        <p class="text-nautical-600 mb-6">Your password has been successfully reset.</p>
        <p class="text-sm text-nautical-500 mb-4">Redirecting to login page...</p>
      </div>

      <!-- Error State -->
      <div v-else-if="resetStatus === 'error'" class="text-center">
        <div class="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-red-100 mb-6">
          <svg class="h-10 w-10 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </div>
        <h1 class="text-3xl font-bold text-nautical-900 mb-2">Reset Failed</h1>
        <p class="text-nautical-600 mb-6">{{ errorMessage }}</p>

        <div class="space-y-3">
          <NuxtLink
            to="/forgot-password"
            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-sky-600 hover:bg-sky-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-500 transition-colors duration-200"
          >
            Request New Reset Link
          </NuxtLink>
        </div>
      </div>

      <!-- Form State -->
      <div v-else-if="resetStatus === 'form'">
        <!-- Header -->
        <div class="text-center mb-8">
          <h1 class="text-3xl font-bold text-nautical-900 mb-2">Create New Password</h1>
          <p class="text-nautical-600">Enter your new password below.</p>
        </div>

        <!-- Password Form -->
        <div class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-sky-200 p-8">
          <form @submit.prevent="onSubmit" class="space-y-6">
            <!-- Password Field -->
            <div>
              <label for="password" class="block text-sm font-medium text-nautical-700 mb-2">
                New Password
              </label>
              <Field
                name="password"
                type="password"
                v-model="form.password"
                :class="[
                  'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
                  errors.password ? 'border-red-300 bg-red-50' : 'border-nautical-300'
                ]"
                placeholder="Enter new password"
              />
              <ErrorMessage name="password" class="text-red-600 text-sm mt-1" />
            </div>

            <!-- Confirm Password Field -->
            <div>
              <label for="confirmPassword" class="block text-sm font-medium text-nautical-700 mb-2">
                Confirm New Password
              </label>
              <Field
                name="confirmPassword"
                type="password"
                v-model="form.confirmPassword"
                :class="[
                  'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
                  errors.confirmPassword ? 'border-red-300 bg-red-50' : 'border-nautical-300'
                ]"
                placeholder="Confirm new password"
              />
              <ErrorMessage name="confirmPassword" class="text-red-600 text-sm mt-1" />
            </div>

            <!-- Server Error -->
            <div v-if="serverError" class="bg-red-50 border border-red-200 rounded-md p-4">
              <p class="text-red-800 text-sm">{{ serverError }}</p>
            </div>

            <!-- Submit Button -->
            <button
              type="submit"
              :disabled="isLoading || !meta.valid"
              class="w-full flex justify-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-sky-600 hover:bg-sky-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
            >
              <svg v-if="isLoading" class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
              </svg>
              {{ isLoading ? 'Resetting Password...' : 'Reset Password' }}
            </button>
          </form>
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
        <h1 class="text-2xl font-bold text-nautical-900 mb-2">Verifying Reset Link...</h1>
        <p class="text-nautical-600">Please wait while we verify your password reset link.</p>
      </div>

      <!-- Decorative Elements -->
      <div class="mt-8 flex justify-center opacity-20">
        <svg class="w-8 h-12 text-sky-400" fill="currentColor" viewBox="0 0 24 32">
          <path d="M12 0L0 20h6l6-8 6 8h6L12 0z"/>
        </svg>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useForm, Field, ErrorMessage } from 'vee-validate'
import { resetPasswordSchema } from '#shared/schemas/auth'
import { usePasswordResetActions } from '~/composables/usePasswordResetActions'

// Page meta
useHead({
  title: 'Reset Password',
  meta: [
    { name: 'description', content: 'Reset your password for KennWilliamson.org.' }
  ]
})

const route = useRoute()
const router = useRouter()

const resetStatus = ref<'loading' | 'form' | 'success' | 'error'>('loading')
const errorMessage = ref('Invalid or expired reset link. Please request a new password reset.')
const serverError = ref('')
const resetToken = ref('')

// Form validation schema
const validationSchema = resetPasswordSchema

// Form state
const form = reactive({
  password: '',
  confirmPassword: '',
})

// Password reset actions
const { resetPassword, isLoading } = usePasswordResetActions()

// Form validation composable
const { errors, meta, handleSubmit } = useForm({
  validationSchema,
  initialValues: form,
})

// Verify token exists on mount
onMounted(async () => {
  const token = route.query.token as string

  if (!token) {
    resetStatus.value = 'error'
    errorMessage.value = 'No reset token provided. Please request a new password reset.'
    return
  }

  // Store token and show form
  resetToken.value = token
  resetStatus.value = 'form'

  // Auto-focus password field
  await nextTick()
  const passwordInput = document.querySelector('input[name="password"]')
  if (passwordInput) {
    (passwordInput as HTMLInputElement).focus()
  }
})

// Handle form submission
const handlePasswordReset = async () => {
  try {
    serverError.value = ''

    await resetPassword(resetToken.value, form.password)

    // Show success state
    resetStatus.value = 'success'

    // Redirect to login after 3 seconds
    setTimeout(() => {
      router.push('/login')
    }, 3000)
  } catch (error: any) {
    console.error('Password reset error:', error)

    if (error.statusCode === 400) {
      resetStatus.value = 'error'
      errorMessage.value = 'This reset link has expired or is invalid. Please request a new password reset.'
    } else {
      serverError.value = error.message || 'Password reset failed. Please try again.'
    }
  }
}

// Create the submit handler using vee-validate's handleSubmit
const onSubmit = handleSubmit(handlePasswordReset)
</script>

<style scoped>
/* Custom focus styles for better accessibility */
input:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(14, 165, 233, 0.1);
}
</style>
