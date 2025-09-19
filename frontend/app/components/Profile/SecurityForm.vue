<template>
  <form @submit.prevent="onSubmit" class="space-y-6">
    <!-- Current Password Field -->
    <div>
      <label for="current_password" class="block text-sm font-medium text-gray-700 mb-2">
        Current Password
      </label>
      <Field
        name="current_password"
        type="password"
        v-model="form.current_password"
        :class="[
          'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
          errors.current_password ? 'border-red-300 bg-red-50' : 'border-gray-300'
        ]"
        placeholder="Enter your current password"
      />
      <ErrorMessage name="current_password" class="text-red-600 text-sm mt-1" />
    </div>

    <!-- New Password Field -->
    <div>
      <label for="new_password" class="block text-sm font-medium text-gray-700 mb-2">
        New Password
      </label>
      <Field
        name="new_password"
        type="password"
        v-model="form.new_password"
        :class="[
          'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
          errors.new_password ? 'border-red-300 bg-red-50' : 'border-gray-300'
        ]"
        placeholder="Enter your new password"
      />
      <ErrorMessage name="new_password" class="text-red-600 text-sm mt-1" />
      
      <!-- Password Requirements -->
      <div class="mt-2 text-xs text-gray-600">
        <p class="mb-1">Password must contain:</p>
        <ul class="list-disc list-inside space-y-1">
          <li :class="passwordChecks.length ? 'text-green-600' : 'text-gray-500'">
            At least 8 characters
          </li>
          <li :class="passwordChecks.lowercase ? 'text-green-600' : 'text-gray-500'">
            One lowercase letter
          </li>
          <li :class="passwordChecks.uppercase ? 'text-green-600' : 'text-gray-500'">
            One uppercase letter
          </li>
          <li :class="passwordChecks.number ? 'text-green-600' : 'text-gray-500'">
            One number
          </li>
        </ul>
      </div>
    </div>

    <!-- Confirm New Password Field -->
    <div>
      <label for="confirm_password" class="block text-sm font-medium text-gray-700 mb-2">
        Confirm New Password
      </label>
      <Field
        name="confirm_password"
        type="password"
        v-model="form.confirm_password"
        :class="[
          'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
          errors.confirm_password ? 'border-red-300 bg-red-50' : 'border-gray-300'
        ]"
        placeholder="Confirm your new password"
      />
      <ErrorMessage name="confirm_password" class="text-red-600 text-sm mt-1" />
    </div>

    <!-- Submit Button -->
    <div class="flex justify-end">
      <button
        type="submit"
        :disabled="isSubmitting || !isFormValid"
        :class="[
          'px-6 py-3 rounded-md font-medium transition-colors duration-200',
          isSubmitting || !isFormValid
            ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
            : 'bg-sky-600 text-white hover:bg-sky-700'
        ]"
      >
        <span v-if="isSubmitting" class="flex items-center">
          <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          Changing...
        </span>
        <span v-else>Change Password</span>
      </button>
    </div>
  </form>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useForm, Field, ErrorMessage } from 'vee-validate'
import { passwordChangeSchema } from '#shared/schemas/auth'
import { useBackendFetch } from '~/composables/useBackendFetch'

// Emits
const emit = defineEmits(['password-changed'])

// Composables
const backendFetch = useBackendFetch()

// Form setup
const { handleSubmit, errors, isSubmitting, setFieldValue, values } = useForm({
  validationSchema: passwordChangeSchema,
  initialValues: {
    current_password: '',
    new_password: '',
    confirm_password: ''
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

// Form validation
const isFormValid = computed(() => {
  return form.value.current_password && 
         form.value.new_password && 
         form.value.confirm_password &&
         !errors.value.current_password && 
         !errors.value.new_password && 
         !errors.value.confirm_password &&
         passwordChecks.value.length &&
         passwordChecks.value.lowercase &&
         passwordChecks.value.uppercase &&
         passwordChecks.value.number
})

// Form submission
const onSubmit = handleSubmit(async (values) => {
  try {
    await backendFetch('/auth/change-password', {
      method: 'PUT',
      body: {
        current_password: values.current_password,
        new_password: values.new_password
      }
    })
    
    // Emit success event
    emit('password-changed')
    
    // Clear form
    form.value.current_password = ''
    form.value.new_password = ''
    form.value.confirm_password = ''
    setFieldValue('current_password', '')
    setFieldValue('new_password', '')
    setFieldValue('confirm_password', '')
    
  } catch (error) {
    console.error('Password change error:', error)
    // You could add toast notification here
  }
})

// Watch form values for validation
watch(form, (newForm) => {
  setFieldValue('current_password', newForm.current_password)
  setFieldValue('new_password', newForm.new_password)
  setFieldValue('confirm_password', newForm.confirm_password)
}, { deep: true })
</script>
