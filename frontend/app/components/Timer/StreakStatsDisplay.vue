<template>
  <div v-if="totalCompletedStreaks > 0" class="streak-stats">
    <div class="stats-grid">
      <div class="stat-card">
        <span class="stat-label">Longest Streak</span>
        <span class="stat-value">{{ formattedLongest }}</span>
      </div>
      <div class="stat-card">
        <span class="stat-label">Average Streak</span>
        <span class="stat-value">{{ formattedAverage }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useIncidentTimerStore } from '~/stores/incident-timers'

const props = defineProps({
  longestStreakSeconds: {
    type: Number,
    required: true
  },
  averageStreakSeconds: {
    type: Number,
    required: true
  },
  totalCompletedStreaks: {
    type: Number,
    required: true
  }
})

const incidentTimerStore = useIncidentTimerStore()

const formattedLongest = computed(() =>
  incidentTimerStore.formatDurationSeconds(props.longestStreakSeconds)
)

const formattedAverage = computed(() =>
  incidentTimerStore.formatDurationSeconds(props.averageStreakSeconds)
)
</script>

<style scoped>
.streak-stats {
  width: 100%;
}

.stats-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.stat-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px 16px;
  border-radius: 8px;
  border: 1px solid rgba(139, 69, 19, 0.2);
  background: rgba(255, 255, 255, 0.3);
}

.stat-label {
  font-family: Georgia, serif;
  font-size: 14px;
  font-weight: 600;
  color: #5d3820;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 8px;
}

.stat-value {
  font-family: Georgia, serif;
  font-size: 16px;
  color: #B8860B;
  text-align: center;
  line-height: 1.4;
}

@media (max-width: 480px) {
  .stats-grid {
    grid-template-columns: 1fr;
  }

  .stat-card {
    padding: 16px 12px;
  }

  .stat-value {
    font-size: 14px;
  }
}
</style>
