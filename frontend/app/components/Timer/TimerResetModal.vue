<template>
  <div v-if="show" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
    <div class="bg-white rounded-lg shadow-xl max-w-md w-full p-6">
      <h3 class="text-lg font-semibold text-gray-900 mb-4">Reset Incident Timer</h3>
      
      <form @submit.prevent="handleSubmit">
        <div class="mb-4">
          <label for="resetNotes" class="block text-sm font-medium text-gray-700 mb-2">
            Notes (optional)
          </label>
          <textarea
            id="resetNotes"
            v-model="form.notes"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            rows="3"
            placeholder="Add any notes about this reset..."
          ></textarea>
        </div>
        
        <div class="flex gap-3 justify-end">
          <button
            type="button"
            @click="handleClose"
            class="px-4 py-2 text-gray-700 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors duration-200"
          >
            Cancel
          </button>
          <button
            type="submit"
            :disabled="incidentTimerStore.isLoading"
            class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 transition-colors duration-200"
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