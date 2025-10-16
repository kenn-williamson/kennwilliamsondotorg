<template>
  <div class="min-h-screen bg-parchment-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-2xl mx-auto">
      <div class="bg-white rounded-lg shadow-lg p-8 border-2 border-amber-200">
        <div class="text-center mb-8">
          <h1 class="text-3xl font-bold text-amber-900 mb-2">
            Request Access to Personal Content
          </h1>
          <p class="text-gray-600">
            The page you're trying to access contains personal and family information that
            requires approval.
          </p>
        </div>

        <div v-if="!submitted" class="space-y-6">
          <div class="bg-amber-50 border border-amber-200 rounded-md p-4">
            <h2 class="text-lg font-semibold text-amber-900 mb-2">
              What you'll get access to:
            </h2>
            <ul class="list-disc list-inside space-y-1 text-gray-700">
              <li>Origins - Family background and early life</li>
              <li>The Wilderness - Personal struggles and transformation</li>
              <li>Finding Faith - Spiritual journey</li>
              <li>Theology & Practice - Beliefs and theological framework</li>
              <li>Life Now - Current life, kids, and daily reality</li>
              <li>Philosophy & Vision - Values, goals, and legacy</li>
            </ul>
          </div>

          <form @submit.prevent="handleSubmit" class="space-y-6">
            <div>
              <label for="message" class="block text-sm font-medium text-gray-700 mb-2">
                How do you know Kenn? <span class="text-red-600">*</span>
              </label>
              <textarea
                id="message"
                v-model="formData.message"
                rows="6"
                required
                class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-amber-500 focus:border-transparent"
                placeholder="Please share how you know Kenn (e.g., friend, coworker, church, etc.) and why you'd like access to this content..."
              ></textarea>
            </div>

            <div class="flex items-start">
              <input
                id="understand"
                v-model="formData.understand"
                type="checkbox"
                required
                class="mt-1 h-4 w-4 text-amber-600 focus:ring-amber-500 border-gray-300 rounded"
              />
              <label for="understand" class="ml-2 text-sm text-gray-700">
                I understand that access must be manually approved by Kenn and may take some
                time. <span class="text-red-600">*</span>
              </label>
            </div>

            <div class="flex gap-4">
              <button
                type="submit"
                :disabled="isLoading"
                class="flex-1 bg-amber-700 hover:bg-amber-800 disabled:bg-gray-400 text-white font-semibold py-3 px-6 rounded-md transition duration-200"
              >
                {{ isLoading ? 'Submitting...' : 'Submit Request' }}
              </button>
              <NuxtLink
                to="/about"
                class="flex-1 bg-gray-200 hover:bg-gray-300 text-gray-700 font-semibold py-3 px-6 rounded-md text-center transition duration-200"
              >
                Cancel
              </NuxtLink>
            </div>

            <p v-if="validationError" class="text-red-600 text-sm text-center">
              {{ validationError }}
            </p>
            <p v-if="hasError && error" class="text-red-600 text-sm text-center">
              {{ error }}
            </p>
          </form>
        </div>

        <div v-else class="text-center space-y-4">
          <div class="text-green-600 text-5xl mb-4">✓</div>
          <h2 class="text-2xl font-bold text-amber-900">Request Submitted!</h2>
          <p class="text-gray-700">
            Your request has been sent to Kenn for review. You'll receive an email notification
            once your request has been processed.
          </p>
          <p class="text-gray-600 text-sm">
            This usually takes 1-2 business days, but may be longer depending on availability.
          </p>
          <div class="pt-6">
            <NuxtLink
              to="/about"
              class="inline-block bg-amber-700 hover:bg-amber-800 text-white font-semibold py-3 px-8 rounded-md transition duration-200"
            >
              Return to About Page
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
const formData = ref({
  message: '',
  understand: false
})

const submitted = ref(false)
const validationError = ref('')

const { user } = useUserSession()
const { createAccessRequest, isLoading, error, hasError } = useAccessRequestActions()

async function handleSubmit() {
  // Clear previous validation errors
  validationError.value = ''

  // Client-side validation
  if (!formData.value.message.trim()) {
    validationError.value = 'Please tell us how you know Kenn'
    return
  }

  if (!formData.value.understand) {
    validationError.value = 'Please confirm that you understand the approval process'
    return
  }

  // Call the composable action
  const result = await createAccessRequest(formData.value.message)

  // If successful, show success state
  if (result) {
    submitted.value = true
  }
}

useHead({
  title: 'Request Access - Kenn Williamson',
  meta: [
    {
      name: 'description',
      content: 'Request access to personal content including family stories, faith journey, and current life details.'
    }
  ]
})

definePageMeta({
  middleware: 'auth' // Requires login but not trusted-contact role
})
</script>
