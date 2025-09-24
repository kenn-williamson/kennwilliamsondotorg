<template>
  <div class="timer-display-tab">
    <div class="tab-content">
      <!-- Current Timer Display -->
      <div class="timer-section">
        <h3 class="section-title">Current Timer</h3>
        <div class="timer-display">
          <SteamClock 
            v-if="latestTimer" 
            :time-breakdown="activeTimerBreakdown"
            class="mb-4"
          />
          <div v-else class="no-timer">
            <p class="text-gray-500">No active timer</p>
            <p class="text-sm text-gray-400">Go to Timer Controls to create a new timer</p>
          </div>
        </div>
      </div>

      <!-- Share Timer Section -->
      <div v-if="latestTimer" class="share-section">
        <h3 class="section-title">Share Timer</h3>
        <div class="share-controls">
          <button
            @click="shareTimer"
            class="share-button"
            :disabled="isSharing"
          >
            <span v-if="isSharing">Sharing...</span>
            <span v-else>Open Public Timer</span>
          </button>
          <p class="share-description">
            Opens your public timer page in a new tab
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useIncidentTimerStore } from '~/stores/incident-timers'
import { useAuthStore } from '~/stores/auth'
import SteamClock from '~/components/Steampunk/SteamClock.vue'

const incidentTimerStore = useIncidentTimerStore()
const { activeTimerBreakdown, latestTimer } = storeToRefs(incidentTimerStore)
const isSharing = ref(false)

// ✅ CORRECT: Use callOnce to prevent double execution during SSR/hydration
await callOnce('user-timers', () => incidentTimerStore.loadUserTimers())

// Start timers after hydration (client-side only)
onMounted(() => {
  console.log('🔄 Starting timers after hydration in TimerDisplayTab...')
  incidentTimerStore.startLiveTimerUpdates()
})

// latestTimer is now provided by the store via storeToRefs

const shareTimer = async () => {
  if (!latestTimer.value) return
  
  isSharing.value = true
  
  try {
    // Get user slug from auth store or API
    const userSlug = await getUserSlug()
    if (userSlug) {
      const publicUrl = `${window.location.origin}/${userSlug}/incident-timer`
      window.open(publicUrl, '_blank')
    } else {
      console.error('Could not get user slug for sharing')
    }
  } catch (error) {
    console.error('Error sharing timer:', error)
  } finally {
    isSharing.value = false
  }
}

const getUserSlug = async (): Promise<string | null> => {
  try {
    // Use auth store for consistent state management
    const authStore = useAuthStore()
    return await authStore.getUserSlug()
  } catch (error) {
    console.error('Error getting user slug:', error)
    return null
  }
}
</script>

<style scoped>
.timer-display-tab {
  @apply p-6;
}

.tab-content {
  @apply max-w-4xl mx-auto;
}

.timer-section {
  @apply mb-8;
}

.section-title {
  @apply text-xl font-semibold text-gray-900 mb-4;
}

.timer-display {
  @apply flex justify-center;
}

.no-timer {
  @apply text-center py-8;
}

.share-section {
  @apply border-t pt-6;
}

.share-controls {
  @apply text-center;
}

.share-button {
  @apply bg-blue-600 text-white px-6 py-3 rounded-lg font-medium
         hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed
         transition-colors;
}

.share-description {
  @apply text-sm text-gray-500 mt-2;
}
</style>
