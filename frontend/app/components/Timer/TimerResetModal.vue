<template>
  <div v-if="show" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
    <div class="bg-white rounded-lg shadow-xl max-w-md w-full p-6">
      <h3 class="text-lg font-semibold text-nautical-900 mb-4">Reset Incident Timer</h3>

      <!-- First-time timer warning -->
      <div v-if="isFirstTimer" class="bg-primary-50 border border-primary-200 rounded-md p-4 mb-4">
        <div class="flex">
          <div class="flex-shrink-0">
            <svg class="h-5 w-5 text-primary-400" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
            </svg>
          </div>
          <div class="ml-3">
            <p class="text-sm text-primary-700">
              Your timer will be public by default. You can opt out in Account Settings.
            </p>
          </div>
        </div>
      </div>

      <form @submit.prevent="handleSubmit">
        <div class="mb-4">
          <label for="resetNotes" class="block text-sm font-medium text-nautical-700 mb-2">
            Notes (optional)
          </label>
          <textarea
            id="resetNotes"
            v-model="form.notes"
            class="w-full px-3 py-2 border border-nautical-300 rounded-md focus:ring-blue-500 focus:border-primary-500"
            rows="3"
            placeholder="Add any notes about this reset..."
          ></textarea>
        </div>
        
        <div class="flex gap-3 justify-end">
          <button
            type="button"
            @click="handleClose"
            class="px-4 py-2 text-nautical-700 border border-nautical-300 rounded-md hover:bg-nautical-50 transition-colors duration-200"
          >
            Cancel
          </button>
          <button
            type="submit"
            :disabled="incidentTimerStore.isLoading"
            class="px-4 py-2 bg-primary-600 text-white rounded-md hover:bg-primary-700 disabled:opacity-50 transition-colors duration-200"
          >
            {{ incidentTimerStore.isLoading ? 'Resetting...' : 'Reset Timer' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup>
import { useIncidentTimerStore } from '~/stores/incident-timers'

const props = defineProps({
  show: {
    type: Boolean,
    required: true
  }
})

const emit = defineEmits(['close'])

const form = reactive({
  notes: ''
})

// Store
const incidentTimerStore = useIncidentTimerStore()

// Check if this is the user's first timer
const isFirstTimer = computed(() => {
  return incidentTimerStore.timers.length === 0
})

const handleSubmit = async () => {
  try {
    const timerData = {
      reset_timestamp: new Date().toISOString(),
      notes: form.notes || undefined
    }
    await incidentTimerStore.createTimer(timerData)
    emit('close') // Close modal after successful creation
  } catch (error) {
    console.error('Error creating timer:', error)
  }
}

const handleClose = () => {
  form.notes = ''
  emit('close')
}

// Clear form when modal closes
watch(() => props.show, (isShowing) => {
  if (!isShowing) {
    form.notes = ''
  }
})
</script>