<template>
  <form @submit.prevent="onSubmit" class="space-y-6">
    <!-- Current Password Field (only show if user has credentials) -->
    <div v-if="hasCredentials">
      <label for="current_password" class="block text-sm font-medium text-nautical-700 mb-2">
        Current Password
      </label>
      <Field
        name="currentPassword"
        type="password"
        v-model="form.current_password"
        :class="[
          'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
          errors.currentPassword ? 'border-red-300 bg-red-50' : 'border-nautical-300'
        ]"
        placeholder="Enter your current password"
      />
      <ErrorMessage name="currentPassword" class="text-red-600 text-sm mt-1" />
    </div>

    <!-- Info message for OAuth-only users -->
    <div v-if="!hasCredentials" class="bg-primary-50 border border-primary-200 rounded-md p-4 mb-4">
      <div class="flex">
        <div class="flex-shrink-0">
          <svg class="h-5 w-5 text-primary-400" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
          </svg>
        </div>
        <div class="ml-3">
          <h3 class="text-sm font-medium text-primary-800">
            Set up password authentication
          </h3>
          <div class="mt-2 text-sm text-primary-700">
            <p>You signed in with Google. Add a password to enable password-based login.</p>
          </div>
        </div>
      </div>
    </div>

    <!-- New Password Field -->
    <div>
      <label for="new_password" class="block text-sm font-medium text-nautical-700 mb-2">
        {{ hasCredentials ? 'New Password' : 'Password' }}
      </label>
      <Field
        name="newPassword"
        type="password"
        v-model="form.new_password"
        :class="[
          'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
          errors.newPassword ? 'border-red-300 bg-red-50' : 'border-nautical-300'
        ]"
        :placeholder="hasCredentials ? 'Enter your new password' : 'Enter your password'"
      />
      <ErrorMessage name="newPassword" class="text-red-600 text-sm mt-1" />

      <!-- Password Requirements -->
      <div class="mt-2 text-xs text-nautical-600">
        <p class="mb-1">Password must contain:</p>
        <ul class="list-disc list-inside space-y-1">
          <li :class="passwordChecks.length ? 'text-green-600' : 'text-nautical-500'">
            At least 8 characters
          </li>
          <li :class="passwordChecks.lowercase ? 'text-green-600' : 'text-nautical-500'">
            One lowercase letter
          </li>
          <li :class="passwordChecks.uppercase ? 'text-green-600' : 'text-nautical-500'">
            One uppercase letter
          </li>
          <li :class="passwordChecks.number ? 'text-green-600' : 'text-nautical-500'">
            One number
          </li>
        </ul>
      </div>
    </div>

    <!-- Confirm New Password Field -->
    <div>
      <label for="confirm_password" class="block text-sm font-medium text-nautical-700 mb-2">
        Confirm Password
      </label>
      <Field
        name="confirmPassword"
        type="password"
        v-model="form.confirm_password"
        :class="[
          'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
          errors.confirmPassword ? 'border-red-300 bg-red-50' : 'border-nautical-300'
        ]"
        :placeholder="hasCredentials ? 'Confirm your new password' : 'Confirm your password'"
      />
      <ErrorMessage name="confirmPassword" class="text-red-600 text-sm mt-1" />
    </div>

    <!-- Submit Button -->
    <div class="flex justify-end">
      <button
        type="submit"
        :disabled="isSubmitting || !isFormValid"
        :class="[
          'px-6 py-3 rounded-md font-medium transition-colors duration-200',
          isSubmitting || !isFormValid
            ? 'bg-nautical-300 text-nautical-500 cursor-not-allowed'
            : 'bg-sky-600 text-white hover:bg-sky-700'
        ]"
      >
        <span v-if="isSubmitting" class="flex items-center">
          <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          {{ hasCredentials ? 'Changing...' : 'Setting...' }}
        </span>
        <span v-else>{{ hasCredentials ? 'Change Password' : 'Set Password' }}</span>
      </button>
    </div>
  </form>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useForm, Field, ErrorMessage } from 'vee-validate'
import { passwordChangeSchema, setPasswordSchema } from '#shared/schemas/auth'
import { useAuthProfileActions } from '~/composables/useAuthProfileActions'

// Get user from session
const { user } = useUserSession()

// Composables
const { changePassword, setPassword, isLoading, error, hasError } = useAuthProfileActions()

// Check if user has credentials
const hasCredentials = computed(() => user.value?.has_credentials ?? true)

// Form setup - use appropriate schema based on has_credentials
const { handleSubmit, errors, isSubmitting, setFieldValue, values } = useForm({
  validationSchema: hasCredentials.value ? passwordChangeSchema : setPasswordSchema,
  initialValues: hasCredentials.value ? {
    currentPassword: '',
    newPassword: '',
    confirmPassword: ''
  } : {
    newPassword: '',
    confirmPassword: ''
  }
})

// Reactive form data
const form = ref({
  current_password: '',
  new_password: '',
  confirm_password: ''
})

// Password validation checks
const passwordChecks = computed(() => {
  const password = form.value.new_password || ''
  return {
    length: password.length >= 8,
    lowercase: /[a-z]/.test(password),
    uppercase: /[A-Z]/.test(password),
    number: /\d/.test(password)
  }
})

// Form validation - different requirements based on has_credentials
const isFormValid = computed(() => {
  const baseValid = form.value.new_password &&
                   form.value.confirm_password &&
                   !errors.value.newPassword &&
                   !errors.value.confirmPassword &&
                   passwordChecks.value.length &&
                   passwordChecks.value.lowercase &&
                   passwordChecks.value.uppercase &&
                   passwordChecks.value.number

  if (hasCredentials.value) {
    return baseValid && form.value.current_password && !errors.value.currentPassword
  }

  return baseValid
})

// Form submission
const onSubmit = handleSubmit(async (values) => {
  try {
    if (hasCredentials.value) {
      // Change password (requires current password)
      await changePassword({
        current_password: values.currentPassword,
        new_password: values.newPassword
      })
    } else {
      // Set password (no current password required)
      await setPassword({
        new_password: values.newPassword
      })
    }

    // Clear form
    form.value.current_password = ''
    form.value.new_password = ''
    form.value.confirm_password = ''
    if (hasCredentials.value) {
      setFieldValue('currentPassword', '')
    }
    setFieldValue('newPassword', '')
    setFieldValue('confirmPassword', '')

  } catch (error) {
    console.error('Password change/set error:', error)
    // Error handling is managed by the service
  }
})

// Watch form values for validation
watch(form, (newForm) => {
  if (hasCredentials.value) {
    setFieldValue('currentPassword', newForm.current_password)
  }
  setFieldValue('newPassword', newForm.new_password)
  setFieldValue('confirmPassword', newForm.confirm_password)
}, { deep: true })
</script>
