<template>
  <div class="timer-controls-tab">
    <div class="tab-content">
      <!-- Quick Actions -->
      <div class="quick-actions-section">
        <h3 class="section-title">Quick Actions</h3>
        <div class="action-buttons">
          <button
            @click="resetTimer"
            class="action-button primary"
            :disabled="isResetting"
          >
            <span v-if="isResetting">Resetting...</span>
            <span v-else>Reset Timer</span>
          </button>
        </div>
      </div>

      <!-- Timer History -->
      <div class="history-section">
        <h3 class="section-title">Timer History</h3>
        
        <div v-if="isLoading" class="loading-state">
          <p>Loading timer history...</p>
        </div>
        
        <div v-else-if="timers.length === 0" class="empty-state">
          <p class="text-gray-500">No timers found</p>
          <p class="text-sm text-gray-400">Create your first timer using the reset button above</p>
        </div>
        
        <div v-else class="timer-list">
          <TimerListItem
            v-for="timer in timers"
            :key="timer.id"
            :timer="timer"
            @edit="editTimer"
          />
        </div>
      </div>
    </div>

    <!-- Edit Timer Modal -->
    <TimerEditModal
      v-if="editingTimer"
      :show="true"
      :timer="editingTimer"
      @close="closeEditModal"
    />

    <!-- Reset Timer Modal -->
    <TimerResetModal
      :show="showResetModal"
      @close="closeResetModal"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useIncidentTimerStore } from '~/stores/incident-timers'
import { useIncidentTimerActions } from '~/composables/useIncidentTimerActions'
import TimerListItem from '~/components/Timer/TimerListItem.vue'
import TimerEditModal from '~/components/Timer/TimerEditModal.vue'
import TimerResetModal from '~/components/Timer/TimerResetModal.vue'

const incidentTimerStore = useIncidentTimerStore()
const { fetchTimers, createTimer, updateTimer, isLoading, error } = useIncidentTimerActions()

const isResetting = ref(false)
const editingTimer = ref(null)
const showResetModal = ref(false)

// Get timers sorted by reset_timestamp descending
const timers = computed(() => {
  if (!incidentTimerStore.timers) return []
  
  return [...incidentTimerStore.timers].sort((a, b) => 
    new Date(b.reset_timestamp).getTime() - new Date(a.reset_timestamp).getTime()
  )
})

onMounted(async () => {
  await loadTimers()
})

const loadTimers = async () => {
  await fetchTimers()
}

const resetTimer = () => {
  showResetModal.value = true
}

const editTimer = (timer: any) => {
  editingTimer.value = timer
}


const closeEditModal = async () => {
  editingTimer.value = null
  await loadTimers() // Refresh the list after edit
}

const closeResetModal = async () => {
  showResetModal.value = false
  await loadTimers() // Refresh the list after reset
}
</script>

<style scoped>
.timer-controls-tab {
  @apply p-6;
}

.tab-content {
  @apply max-w-4xl mx-auto;
}

.quick-actions-section {
  @apply mb-8;
}

.section-title {
  @apply text-xl font-semibold text-gray-900 mb-4;
}

.action-buttons {
  @apply flex gap-4;
}

.action-button {
  @apply px-6 py-3 rounded-lg font-medium transition-colors;
}

.action-button.primary {
  @apply bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed;
}

.history-section {
  @apply border-t pt-6;
}

.loading-state,
.empty-state {
  @apply text-center py-8;
}

.timer-list {
  @apply space-y-4;
}
</style>
