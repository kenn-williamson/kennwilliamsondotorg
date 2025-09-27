<template>
  <div class="min-h-screen flex items-center justify-center px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-slate-50 via-sky-50 to-blue-50">
    <div class="max-w-md w-full">
      <!-- Header -->
      <div class="text-center mb-8">
        <h1 class="text-3xl font-bold text-gray-900 mb-2">Create Account</h1>
        <p class="text-gray-600">Join the digital sanctuary</p>
      </div>

      <!-- Registration Form -->
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

          <!-- Display Name Field -->
          <div>
            <label for="displayName" class="block text-sm font-medium text-gray-700 mb-2">
              Display Name
              <span class="text-xs text-gray-500 ml-1">(how you want to be called)</span>
            </label>
            <Field
              name="displayName"
              type="text"
              v-model="form.display_name"
              :class="[
                'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
                errors.displayName ? 'border-red-300 bg-red-50' : 'border-gray-300'
              ]"
              placeholder="John Doe"
            />
            <ErrorMessage name="displayName" class="text-red-600 text-sm mt-1" />
            <!-- Dynamic Slug Preview -->
            <div v-if="slugPreview && form.display_name.length >= 2" class="mt-2 p-3 bg-sky-50 border border-sky-200 rounded-md">
              <p class="text-xs text-gray-600 mb-2">Your public incident timer will be available at:</p>
              <p class="text-sm font-mono text-sky-700 break-all mb-2">
                kennwilliamson.org/<span class="font-bold">{{ slugPreview.final_slug }}</span>/incident-timer
              </p>
              <div v-if="!slugPreview.available" class="flex items-center text-xs text-amber-600">
                <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                </svg>
                "{{ slugPreview.slug }}" is taken, using "{{ slugPreview.final_slug }}" instead
              </div>
            </div>
            <p v-else-if="form.display_name.length < 2" class="text-xs text-gray-500 mt-1">
              Your public incident timer will be available at kennwilliamson.org/{auto-generated-slug}/incident-timer
            </p>
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
              placeholder="Choose a strong password"
            />
            <ErrorMessage name="password" class="text-red-600 text-sm mt-1" />
          </div>

          <!-- Confirm Password Field -->
          <div>
            <label for="confirmPassword" class="block text-sm font-medium text-gray-700 mb-2">
              Confirm Password
            </label>
            <Field
              name="confirmPassword"
              type="password"
              v-model="form.confirmPassword"
              :class="[
                'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
                errors.confirmPassword ? 'border-red-300 bg-red-50' : 'border-gray-300'
              ]"
              placeholder="Confirm your password"
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
            {{ isLoading ? 'Creating Account...' : 'Create Account' }}
          </button>
        </form>

        <!-- Login Link -->
        <div class="mt-6 text-center">
          <p class="text-sm text-gray-600">
            Already have an account?
            <NuxtLink 
              to="/login" 
              class="text-sky-600 hover:text-sky-700 font-medium hover:underline transition-colors duration-200"
            >
              Sign in here
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
import { registerSchema, generateSlug } from '#shared/schemas/auth'
import { useAuthActions } from '~/composables/useAuthActions'

// Page meta
useHead({
  title: 'Create Account',
  meta: [
    { name: 'description', content: 'Create your KennWilliamson.org account to access personal features and manage your incident timers.' }
  ]
})

const { loggedIn } = useUserSession()
const router = useRouter()

if (loggedIn.value) {
  await navigateTo('/')
}

// Form validation schema
const validationSchema = registerSchema

// Form state
const form = reactive({
  email: '',
  display_name: '',
  password: '',
  confirmPassword: '',
})

const serverError = ref('')
const slugPreview = ref(null)
const slugPreviewLoading = ref(false)

// Auth actions
const { register, previewSlug, isLoading, error } = useAuthActions()

// Form validation composable
const { errors, meta, handleSubmit } = useForm({
  validationSchema,
  initialValues: form,
})

// Watch for changes in display_name and get slug preview
const debouncedSlugPreview = useDebounceFn(async (displayName) => {
  if (!displayName || displayName.length < 2) {
    slugPreview.value = null
    return
  }

  try {
    slugPreviewLoading.value = true
    const preview = await previewSlug(displayName.trim())
    slugPreview.value = preview
  } catch (error) {
    console.error('Failed to get slug preview:', error)
    slugPreview.value = null
  } finally {
    slugPreviewLoading.value = false
  }
}, 300)

watch(() => form.display_name, (newValue) => {
  debouncedSlugPreview(newValue)
})

// Handle form submission using VeeValidate's handleSubmit
const onSubmit = handleSubmit(async (values) => {
  try {
    serverError.value = ''

    // Use auth service for registration
    const result = await register({
      email: values.email,
      display_name: values.displayName,
      password: values.password,
    })

    if (result.success) {
      // Get redirect parameter from URL or default to homepage
      const route = useRoute()
      const redirectTo = String(route.query.redirect || '/')
      
      // Validate redirect to prevent open redirects
      const isValidRedirect = redirectTo.startsWith('/') && !redirectTo.startsWith('//')
      const targetPath = isValidRedirect ? redirectTo : '/'
      
      console.log('Registration successful, redirecting to:', targetPath)
      await router.push(targetPath)
    }
  } catch (error) {
    console.error('Registration error:', error)
    serverError.value = error.message || 'Registration failed. Please try again.'
  }
})

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