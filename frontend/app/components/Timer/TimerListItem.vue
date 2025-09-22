<template>
  <div class="p-6 hover:bg-blue-50/50 transition-colors duration-200">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div>
        <p class="font-medium text-gray-900">
          Started: {{ formatDate(timer.reset_timestamp) }}
        </p>
        <p v-if="timer.notes" class="text-gray-600 text-sm mt-1">
          {{ timer.notes }}
        </p>
        <p class="text-xs text-gray-500 mt-1">
          Created {{ formatDate(timer.created_at) }}
        </p>
      </div>
      <div class="flex items-center gap-3">
        <span class="font-mono text-lg font-medium text-blue-600">
          {{ elapsedTimeDisplay }}
        </span>
        <div class="flex gap-2">
          <button
            @click="$emit('edit', timer)"
            class="p-2 text-gray-500 hover:text-blue-600 hover:bg-blue-100 rounded-md transition-colors duration-200"
            title="Edit timer"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
            </svg>
          </button>
          <button
            @click="handleDeleteTimer"
            class="p-2 text-gray-500 hover:text-red-600 hover:bg-red-100 rounded-md transition-colors duration-200"
            title="Delete timer"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { formatDisplayDate } from '~/utils/dateUtils'
import { useIncidentTimerStore } from '~/stores/incident-timers'
import { useIncidentTimerActions } from '~/composables/useIncidentTimerActions'

const props = defineProps({
  timer: {
    type: Object,
    required: true
  },
  isLatest: {
    type: Boolean,
    default: false
  },
  liveElapsedTime: {
    type: String,
    default: ''
  }
})

defineEmits(['edit'])

// Get timer store for formatting non-latest timers
const timerStore = useIncidentTimerStore()
const { deleteTimer } = useIncidentTimerActions()

// Compute elapsed time display
const elapsedTimeDisplay = computed(() => {
  if (props.isLatest) {
    return props.liveElapsedTime
  } else {
    return timerStore.formatElapsedTime(props.timer)
  }
})

// Format date for display using utility
const formatDate = (dateString) => {
  return formatDisplayDate(dateString)
}

// Delete timer using action composable
const handleDeleteTimer = async () => {
  if (confirm('Are you sure you want to delete this timer?')) {
    await deleteTimer(props.timer.id)
  }
}
</script>