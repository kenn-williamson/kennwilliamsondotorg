<template>
  <div class="timer-item p-6 transition-colors duration-200">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div>
        <p class="timer-timestamp font-medium">
          Started: {{ formatDate(timer.reset_timestamp) }}
        </p>
        <p v-if="timer.notes" class="timer-notes text-sm mt-1">
          {{ timer.notes }}
        </p>
        <p class="timer-created text-xs mt-1">
          Created {{ formatDate(timer.created_at) }}
        </p>
      </div>
      <div class="flex items-center gap-3">
        <span class="timer-elapsed font-mono text-lg font-medium">
          {{ elapsedTimeDisplay }}
        </span>
        <div class="flex gap-2">
          <button
            @click="handleEditTimer"
            class="action-button edit-button p-2 rounded-md transition-colors duration-200"
            title="Edit timer"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
            </svg>
          </button>
          <button
            @click="handleDeleteTimer"
            class="action-button delete-button p-2 rounded-md transition-colors duration-200"
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

  <!-- Edit Timer Modal -->
  <TimerEditModal
    v-if="showEditModal"
    :show="true"
    :timer="timer"
    @close="closeEditModal"
  />
</template>

<script setup>
import { ref } from 'vue'
import { formatDisplayDate } from '~/utils/dateUtils'
import { useIncidentTimerStore } from '~/stores/incident-timers'
import TimerEditModal from '~/components/Timer/TimerEditModal.vue'

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

// Get timer store for formatting non-latest timers
const incidentTimerStore = useIncidentTimerStore()

// Local state for edit modal
const showEditModal = ref(false)

// Compute elapsed time display
const elapsedTimeDisplay = computed(() => {
  if (props.isLatest) {
    return props.liveElapsedTime
  } else {
    return incidentTimerStore.formatElapsedTime(props.timer)
  }
})

// Format date for display using utility
const formatDate = (dateString) => {
  return formatDisplayDate(dateString)
}

// Edit timer - show modal
const handleEditTimer = () => {
  showEditModal.value = true
}

// Close edit modal and refresh timers
const closeEditModal = async () => {
  showEditModal.value = false
  await incidentTimerStore.loadUserTimers() // Refresh the list after edit
}

// Delete timer using store
const handleDeleteTimer = async () => {
  if (confirm('Are you sure you want to delete this timer?')) {
    await incidentTimerStore.deleteTimer(props.timer.id)
  }
}
</script>

<style scoped>
.timer-item {
  border-radius: 8px;
  border: 1px solid rgba(139, 69, 19, 0.2);
}

.timer-item:hover {
  background: rgba(255, 255, 255, 0.4);
}

.timer-timestamp {
  color: #3c2414;
}

.timer-notes {
  color: #5d3820;
}

.timer-created {
  color: #8B6914;
}

.timer-elapsed {
  color: #B8860B;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.action-button {
  color: #8B6914;
}

.edit-button:hover {
  color: #B8860B;
  background: rgba(184, 134, 11, 0.1);
}

.delete-button:hover {
  color: #dc2626;
  background: rgba(220, 38, 38, 0.1);
}
</style>