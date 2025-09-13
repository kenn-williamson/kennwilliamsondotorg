<template>
  <div class="min-h-screen flex items-center justify-center px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-slate-50 via-sky-50 to-blue-50">
    <div class="max-w-md w-full">
      <!-- Header -->
      <div class="text-center mb-8">
        <h1 class="text-3xl font-bold text-gray-900 mb-2">Sign In - TEST CHANGE</h1>
        <p class="text-gray-600">Welcome back to your digital sanctuary</p>
      </div>

      <!-- Login Form -->
      <div class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-sky-200 p-8">
        <form @submit="onSubmit" class="space-y-6">
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

          <!-- Password Field -->
          <div>
            <label for="password" class="block text-sm font-medium text-gray-700 mb-2">
              Password
            </label>
            <Field
              name="password"
              type="password"
              v-model="form.password"
              :class="[
                'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
                errors.password ? 'border-red-300 bg-red-50' : 'border-gray-300'
              ]"
              placeholder="Enter your password"
            />
            <ErrorMessage name="password" class="text-red-600 text-sm mt-1" />
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
            {{ isLoading ? 'Signing In...' : 'Sign In' }}
          </button>
        </form>

        <!-- Register Link -->
        <div class="mt-6 text-center">
          <p class="text-sm text-gray-600">
            Don't have an account?
            <NuxtLink 
              to="/register" 
              class="text-sky-600 hover:text-sky-700 font-medium hover:underline transition-colors duration-200"
            >
              Create one here
            </NuxtLink>
          </p>
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
import * as yup from 'yup'

// Page meta
useHead({
  title: 'Sign In',
  meta: [
    { name: 'description', content: 'Sign in to your KennWilliamson.org account to access your personal features and incident timer management.' }
  ]
})

// Redirect if already authenticated
const { loggedIn } = useUserSession()
const router = useRouter()

// Comment out the redirect to see if this is causing the issue
// if (loggedIn.value) {
//   await navigateTo('/')
// }

// Form validation schema
const validationSchema = yup.object({
  email: yup
    .string()
    .required('Email is required')
    .email('Please enter a valid email address'),
  password: yup
    .string()
    .required('Password is required')
    .min(8, 'Password must be at least 8 characters'),
})

// Form state
const form = reactive({
  email: '',
  password: '',
})

const isLoading = ref(false)
const serverError = ref('')

// Form validation composable
const { errors, meta, handleSubmit } = useForm({
  validationSchema,
  initialValues: form,
})

// Handle form submission
const handleLogin = async () => {
  try {
    isLoading.value = true
    serverError.value = ''

    const authService = useAuthService()
    const result = await authService.login({
      email: form.email,
      password: form.password,
    })

    if (result.success) {
      // Get redirect parameter from URL or default to homepage
      const route = useRoute()
      const redirectTo = String(route.query.redirect || '/')
      
      // Validate redirect to prevent open redirects
      const isValidRedirect = redirectTo.startsWith('/') && !redirectTo.startsWith('//')
      const targetPath = isValidRedirect ? redirectTo : '/'
      
      console.log('Login successful, redirecting to:', targetPath)
      await router.push(targetPath)
    }
  } catch (error) {
    console.error('Login error:', error)
    
    // Handle specific error types
    if (error.statusCode === 401) {
      serverError.value = 'Invalid email or password. Please try again.'
    } else if (error.data?.statusMessage) {
      serverError.value = error.data.statusMessage
    } else {
      serverError.value = error.message || 'Login failed. Please try again.'
    }
  } finally {
    isLoading.value = false
  }
}

// Create the submit handler using vee-validate's handleSubmit
const onSubmit = handleSubmit(handleLogin)

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

/* Subtle sacred geometry background pattern */
.bg-pattern {
  background-image: 
    radial-gradient(circle at 1px 1px, rgba(14, 165, 233, 0.05) 1px, transparent 0);
  background-size: 20px 20px;
}
</style>