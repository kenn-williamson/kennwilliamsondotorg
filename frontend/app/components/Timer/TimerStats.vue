<template>
  <div v-if="timer" class="mb-8 bg-white/90 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 p-8">
    <div class="text-center">
      <h2 class="text-lg font-medium text-gray-700 mb-4">Current Incident-Free Time</h2>
      
      <!-- Steam Clock Component -->
      <div class="flex justify-center mb-6">
        <SteamClock :time-breakdown="breakdown" />
      </div>
      
      <p class="text-gray-600 mb-2">
        Started {{ formatDate(timer.reset_timestamp) }}
      </p>
      <p v-if="timer.notes" class="text-gray-500 mt-2 italic">
        "{{ timer.notes }}"
      </p>
    </div>
  </div>
  
  <!-- No Timer State -->
  <div v-else-if="!loading" class="mb-8 bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 p-8 text-center">
    <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
    </svg>
    <h2 class="text-xl font-semibold text-gray-900 mb-2">No Active Timer</h2>
    <p class="text-gray-600 mb-4">Start tracking your incident-free time by creating your first timer.</p>
    <button
      @click="handleCreateTimer"
      :disabled="isLoading"
      class="px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 transition-colors duration-200 font-medium"
    >
      {{ isLoading ? 'Creating...' : 'Create First Timer' }}
    </button>
  </div>
</template>

<script setup>
import { formatDisplayDate } from '~/utils/dateUtils'
import { useIncidentTimerActions } from '~/composables/useIncidentTimerActions'

const props = defineProps({
  timer: {
    type: Object,
    default: null
  },
  breakdown: {
    type: Object,
    required: true,
    default: () => ({
      years: 0,
      months: 0,
      weeks: 0,
      days: 0,
      hours: 0,
      minutes: 0,
      seconds: 0
    })
  },
  loading: {
    type: Boolean,
    default: false
  }
})

// Action composable
const { createTimer, isLoading } = useIncidentTimerActions()

// Handle create timer
const handleCreateTimer = async () => {
  try {
    const timerData = {
      reset_timestamp: new Date().toISOString(),
      notes: undefined
    }
    await createTimer(timerData)
  } catch (error) {
    console.error('Error creating timer:', error)
  }
}

// Format date for display using utility
const formatDate = (dateString) => {
  return formatDisplayDate(dateString)
}
</script>

