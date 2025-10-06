<template>
  <div class="bg-white shadow rounded-lg border-l-4 border-red-500">
    <div class="px-6 py-4 border-b border-gray-200 bg-red-50">
      <div class="flex items-center">
        <svg class="w-6 h-6 text-red-600 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
        </svg>
        <h2 class="text-lg font-medium text-red-900">Danger Zone</h2>
      </div>
      <p class="mt-1 text-sm text-red-700">Permanent actions that cannot be undone.</p>
    </div>
    
    <div class="px-6 py-4">
      <div class="flex items-center justify-between">
        <div class="flex-1">
          <h3 class="text-base font-medium text-gray-900">Delete Account</h3>
          <p class="mt-1 text-sm text-gray-600">
            Permanently delete your account and all associated data. This action cannot be undone.
          </p>
          <div class="mt-2 text-sm text-red-600">
            <p class="font-medium">⚠️ This will permanently delete:</p>
            <ul class="list-disc list-inside mt-1 space-y-1">
              <li>Your profile and account information</li>
              <li>All incident timers and history</li>
              <li>All phrase exclusions and preferences</li>
              <li>All phrase suggestions (except accepted ones)</li>
              <li>All login sessions and tokens</li>
            </ul>
            <p class="mt-2 font-medium">✅ Accepted phrase suggestions will be preserved and reassigned to the system.</p>
          </div>
        </div>
        <button
          @click="showDeleteModal = true"
          :disabled="isLoading"
          class="ml-6 inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
        >
          <svg v-if="isLoading" class="animate-spin -ml-0.5 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
          </svg>
          <svg v-else class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
          Delete Account
        </button>
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteModal" class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50" @click="closeModal">
      <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white" @click.stop>
        <!-- Modal Header with Warning -->
        <div class="text-center">
          <div class="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-red-100 mb-4">
            <svg class="h-6 w-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
            </svg>
          </div>
          <h3 class="text-lg font-medium text-gray-900 mb-2">⚠️ Final Warning ⚠️</h3>
          <div class="text-sm text-red-600 mb-4">
            <p class="font-bold">🚨 CAUTION TAPE 🚨</p>
            <p class="mt-2">You are about to permanently delete your account.</p>
            <p class="font-bold mt-2">THIS CANNOT BE UNDONE!</p>
            <p class="mt-2">There is no recovery mechanism. No going back except for a brand new account.</p>
          </div>
        </div>

        <!-- Email Confirmation -->
        <div class="mt-4">
          <label for="email-confirmation" class="block text-sm font-medium text-gray-700 mb-2">
            To confirm, please type your email address:
          </label>
          <Field
            id="email-confirmation"
            name="emailConfirmation"
            type="email"
            placeholder="Enter your email address"
            class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-red-500 focus:border-red-500"
            :class="{ 'border-red-500': errors.emailConfirmation }"
          />
          <ErrorMessage name="emailConfirmation" class="mt-1 text-sm text-red-600" />
          <p v-if="!errors.emailConfirmation && emailConfirmation && !isEmailValid" class="mt-1 text-sm text-red-600">
            Email does not match your account email
          </p>
        </div>

        <!-- Action Buttons -->
        <div class="mt-6 flex justify-end space-x-3">
          <button
            @click="closeModal"
            :disabled="isLoading"
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Cancel
          </button>
          <button
            @click="confirmDelete"
            :disabled="!isEmailValid || isLoading"
            class="px-4 py-2 text-sm font-medium text-white bg-red-600 border border-transparent rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
          >
            <svg v-if="isLoading" class="animate-spin -ml-0.5 mr-2 h-4 w-4 inline" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
            </svg>
            <svg v-else class="w-4 h-4 mr-2 inline" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
            Yes, Delete My Account Forever
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useForm, useField, Field, ErrorMessage } from 'vee-validate'
import { accountDeletionSchema } from '#shared/schemas'
import { useAuthProfileActions } from '~/composables/useAuthProfileActions'

// Props
const props = defineProps({
  user: {
    type: Object,
    required: true
  }
})

// Composables
const { deleteAccount, isLoading, error } = useAuthProfileActions()

// Reactive state
const showDeleteModal = ref(false)

// VeeValidate form setup
const { handleSubmit, resetForm, errors } = useForm({
  validationSchema: accountDeletionSchema,
  initialValues: {
    emailConfirmation: ''
  }
})

const { value: emailConfirmation } = useField('emailConfirmation')

// Computed
const isEmailValid = computed(() => {
  return emailConfirmation.value === props.user.email
})

// Methods
const closeModal = () => {
  showDeleteModal.value = false
  resetForm()
}

const confirmDelete = handleSubmit(async (values) => {
  if (!isEmailValid.value) return

  try {
    await deleteAccount()
    // Account deletion successful - redirect to home page
    // The composable handles session clearing
    await navigateTo('/')
  } catch (err) {
    console.error('Account deletion failed:', err)
    // Error handling is done by the composable
  }
})
</script>
