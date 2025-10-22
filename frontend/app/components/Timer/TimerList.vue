<template>
  <div class="timer-list-component">
    <!-- Timer History -->
    <div class="history-section">
      <h3 class="section-title">Timer History</h3>
      
      <div v-if="incidentTimerStore.isLoading" class="loading-state">
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
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useIncidentTimerStore } from '~/stores/incident-timers'
import TimerListItem from '~/components/Timer/TimerListItem.vue'

const incidentTimerStore = useIncidentTimerStore()

// Get timers sorted by reset_timestamp descending
const timers = computed(() => {
  if (!incidentTimerStore.timers) return []
  
  return [...incidentTimerStore.timers].sort((a, b) => 
    new Date(b.reset_timestamp).getTime() - new Date(a.reset_timestamp).getTime()
  )
})
</script>

<style scoped>
.timer-list-component {
  @apply w-full;
}

.history-section {
  @apply border-t pt-6;
  border-color: rgba(139, 69, 19, 0.2);
}

.section-title {
  @apply text-xl font-semibold mb-4;
  color: #5d3820;
  font-family: Georgia, serif;
  text-shadow: 0 1px 1px rgba(255, 255, 255, 0.5);
}

.loading-state,
.empty-state {
  @apply text-center py-8;
  color: #8B6914;
}

.timer-list {
  @apply space-y-4;
}
</style>
