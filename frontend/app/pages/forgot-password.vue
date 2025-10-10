<template>
  <div class="min-h-screen flex items-center justify-center px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-slate-50 via-sky-50 to-blue-50">
    <div class="max-w-md w-full">
      <!-- Success State -->
      <div v-if="emailSent" class="text-center">
        <div class="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-green-100 mb-6">
          <svg class="h-10 w-10 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        </div>
        <h1 class="text-3xl font-bold text-gray-900 mb-2">Check Your Email</h1>
        <p class="text-gray-600 mb-6">
          If an account exists with that email address, you will receive a password reset link shortly.
        </p>
        <p class="text-sm text-gray-500 mb-6">
          The link will expire in 1 hour for security reasons.
        </p>
        <NuxtLink
          to="/login"
          class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-sky-600 hover:bg-sky-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-500 transition-colors duration-200"
        >
          Back to Login
        </NuxtLink>
      </div>

      <!-- Form State -->
      <div v-else>
        <!-- Header -->
        <div class="text-center mb-8">
          <h1 class="text-3xl font-bold text-gray-900 mb-2">Reset Password</h1>
          <p class="text-gray-600">Enter your email address and we'll send you a password reset link.</p>
        </div>

        <!-- Reset Form -->
        <div class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-sky-200 p-8">
          <form @submit.prevent="onSubmit" class="space-y-6">
            <!-- Email Field -->
            <div>
              <label for="email" class="block text-sm font-medium text-gray-700 mb-2">
                Email Address
              </label>
              <Field
                name="email"
                type="email"
                v-model="form.email"
                :class="[
                  'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
                  errors.email ? 'border-red-300 bg-red-50' : 'border-gray-300'
                ]"
                placeholder="your@email.com"
              />
              <ErrorMessage name="email" class="text-red-600 text-sm mt-1" />
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
              {{ isLoading ? 'Sending...' : 'Send Reset Link' }}
            </button>
          </form>

          <!-- Back to Login Link -->
          <div class="mt-6 text-center">
            <NuxtLink
              to="/login"
              class="text-sm text-sky-600 hover:text-sky-700 font-medium hover:underline transition-colors duration-200"
            >
              ← Back to login
            </NuxtLink>
          </div>
        </div>
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

<script setup>
import { useForm, Field, ErrorMessage } from 'vee-validate'
import { forgotPasswordSchema } from '#shared/schemas/auth'
import { usePasswordResetActions } from '~/composables/usePasswordResetActions'

// Page meta
useHead({
  title: 'Reset Password',
  meta: [
    { name: 'description', content: 'Request a password reset link for your KennWilliamson.org account.' }
  ]
})

const { loggedIn } = useUserSession()

// Redirect if already logged in
if (loggedIn.value) {
  await navigateTo('/')
}

// Form validation schema
const validationSchema = forgotPasswordSchema

// Form state
const form = reactive({
  email: '',
})

const serverError = ref('')
const emailSent = ref(false)

// Password reset actions
const { sendResetEmail, isLoading, error } = usePasswordResetActions()

// Form validation composable
const { errors, meta, handleSubmit } = useForm({
  validationSchema,
  initialValues: form,
})

// Handle form submission
const handleResetRequest = async () => {
  try {
    serverError.value = ''

    await sendResetEmail(form.email)

    // Show success state
    emailSent.value = true
  } catch (error) {
    console.error('Forgot password error:', error)

    // Always show the same generic message (security: no user enumeration)
    serverError.value = 'An error occurred. Please try again.'
  }
}

// Create the submit handler using vee-validate's handleSubmit
const onSubmit = handleSubmit(handleResetRequest)

// Auto-focus email field
onMounted(() => {
  const emailInput = document.querySelector('input[name="email"]')
  if (emailInput) {
    emailInput.focus()
  }
})
</script>

<style scoped>
/* Custom focus styles for better accessibility */
input:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(14, 165, 233, 0.1);
}
</style>
