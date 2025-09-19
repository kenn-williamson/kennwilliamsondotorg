<template>
  <form @submit.prevent="onSubmit" class="space-y-6">
    <!-- Display Name Field -->
    <div>
      <label for="display_name" class="block text-sm font-medium text-gray-700 mb-2">
        Display Name
      </label>
      <Field
        name="display_name"
        type="text"
        v-model="form.display_name"
        :class="[
          'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
          errors.display_name ? 'border-red-300 bg-red-50' : 'border-gray-300'
        ]"
        placeholder="John Doe"
        @input="onDisplayNameChange"
      />
      <ErrorMessage name="display_name" class="text-red-600 text-sm mt-1" />
    </div>

    <!-- URL Slug Field -->
    <div>
      <label for="slug" class="block text-sm font-medium text-gray-700 mb-2">
        URL Slug
        <span class="text-xs text-gray-500 ml-1">(your public URL)</span>
      </label>
      <Field
        name="slug"
        type="text"
        v-model="form.slug"
        :class="[
          'w-full px-4 py-3 border rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent transition-colors duration-200',
          errors.slug ? 'border-red-300 bg-red-50' : 'border-gray-300'
        ]"
        placeholder="my-custom-slug"
        @input="onSlugChange"
        @keypress="onSlugKeypress"
      />
      <ErrorMessage name="slug" class="text-red-600 text-sm mt-1" />
      <p class="text-xs text-gray-500 mt-1">Spaces will automatically be converted to hyphens.</p>
      
      <!-- Slug Preview -->
      <div v-if="slugPreview" class="mt-2 p-3 bg-sky-50 border border-sky-200 rounded-md">
        <p class="text-xs text-gray-600 mb-2">Your public incident timer will be available at:</p>
        <p class="text-sm font-mono text-sky-700">
          {{ baseUrl }}/{{ slugPreview.slug }}
        </p>
        <div v-if="slugPreview.available === false" class="mt-2 text-xs text-amber-600">
          ⚠️ This username is taken. We suggest: {{ slugPreview.final_slug }}
        </div>
        <div v-else-if="slugPreview.available === true" class="mt-2 text-xs text-green-600">
          ✅ This username is available
        </div>
      </div>
    </div>

    <!-- Email Field (Read-only) -->
    <div>
      <label for="email" class="block text-sm font-medium text-gray-700 mb-2">
        Email Address
        <span class="text-xs text-gray-500 ml-1">(cannot be changed)</span>
      </label>
      <input
        type="email"
        :value="user?.email || ''"
        disabled
        class="w-full px-4 py-3 border border-gray-300 rounded-md bg-gray-50 text-gray-500 cursor-not-allowed"
      />
      <p class="text-xs text-gray-500 mt-1">Contact support if you need to change your email address.</p>
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
          Updating...
        </span>
        <span v-else>Update Profile</span>
      </button>
    </div>
  </form>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useForm, Field, ErrorMessage } from 'vee-validate'
import { profileUpdateSchema, generateSlug } from '#shared/schemas/auth'

// Props
const props = defineProps({
  user: {
    type: Object,
    required: true
  }
})

// Watch for user data changes
const user = computed(() => props.user)

// Emits
const emit = defineEmits(['profile-updated'])

// Composables
const { updateProfile, previewSlug, isLoading, error, hasError } = useAuthProfileService()

// Form setup
const { handleSubmit, errors, isSubmitting, setFieldValue, values } = useForm({
  validationSchema: profileUpdateSchema,
  initialValues: {
    display_name: user.value?.display_name || '',
    slug: user.value?.slug || ''
  }
})

// Reactive form data
const form = ref({
  display_name: user.value?.display_name || '',
  slug: user.value?.slug || ''
})

// Slug preview state
const slugPreview = ref(null)
const isCheckingSlug = ref(false)

// Base URL for preview
const baseUrl = computed(() => {
  if (process.client) {
    return window.location.origin
  }
  return 'https://kennwilliamson.org'
})

// Form validation
const isFormValid = computed(() => {
  return form.value.display_name && form.value.slug && !errors.value.display_name && !errors.value.slug
})

// Debounced slug uniqueness check
let slugCheckTimeout = null
const checkSlugUniqueness = async (slug) => {
  if (slugCheckTimeout) {
    clearTimeout(slugCheckTimeout)
  }
  
  slugCheckTimeout = setTimeout(async () => {
    if (slug && slug !== props.user.slug) {
      isCheckingSlug.value = true
      try {
        const response = await previewSlug(slug)
        slugPreview.value = response
      } catch (error) {
        console.error('Error checking slug uniqueness:', error)
        slugPreview.value = null
      } finally {
        isCheckingSlug.value = false
      }
    } else {
      slugPreview.value = null
    }
  }, 500)
}

// Event handlers
const onDisplayNameChange = (event) => {
  form.value.display_name = event.target.value
  setFieldValue('display_name', event.target.value)
}

const onSlugKeypress = (event) => {
  // Convert spaces to hyphens as user types
  if (event.key === ' ') {
    event.preventDefault()
    const currentValue = event.target.value
    const newValue = currentValue + '-'
    form.value.slug = newValue
    setFieldValue('slug', newValue)
    event.target.value = newValue
  }
}

const onSlugChange = (event) => {
  let value = event.target.value
  
  // Auto-convert spaces to hyphens
  if (value.includes(' ')) {
    value = value.replace(/\s+/g, '-')
    form.value.slug = value
    setFieldValue('slug', value)
    event.target.value = value
  } else {
    form.value.slug = value
    setFieldValue('slug', value)
  }
  
  // Check slug uniqueness
  checkSlugUniqueness(value)
}

// Form submission
const onSubmit = handleSubmit(async (values) => {
  try {
    const response = await updateProfile({
      display_name: values.display_name,
      slug: values.slug
    })
    
    // Emit success event with updated user data
    emit('profile-updated', response)
    
    // Clear slug preview
    slugPreview.value = null
    
  } catch (error) {
    console.error('Profile update error:', error)
    // Error handling is managed by the service
  }
})

// Watch for user data changes
watch(user, (newUser) => {
  if (newUser) {
    form.value.display_name = newUser.display_name
    form.value.slug = newUser.slug
    setFieldValue('display_name', newUser.display_name)
    setFieldValue('slug', newUser.slug)
  }
}, { immediate: true })
</script>
