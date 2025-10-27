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
            <p class="text-nautical-500">No active timer</p>
            <p class="text-sm text-nautical-400">Go to Timer Controls to create a new timer</p>
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
import SteamClock from '~/components/Steampunk/SteamClock.vue'

const incidentTimerStore = useIncidentTimerStore()
const { activeTimerBreakdown, latestTimer } = storeToRefs(incidentTimerStore)
const isSharing = ref(false)

// Load user timers (useAsyncData caches and handles navigation correctly)
await useAsyncData('user-timers', () => incidentTimerStore.loadUserTimers())

// Start timers after hydration (client-side only)
onMounted(async () => {
  // Use nextTick to ensure all reactive state updates are processed before starting timer
  await nextTick()
  console.log('🔄 Starting timers after hydration in TimerDisplayTab...')
  incidentTimerStore.startLiveTimerUpdates()
})

// Stop timers when tab is switched away or component unmounts
onUnmounted(() => {
  console.log('⏹️ Stopping timers in TimerDisplayTab unmount')
  incidentTimerStore.stopLiveTimerUpdates()
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
    // Get user slug directly from session
    const { user } = useUserSession()
    return user.value?.slug || null
  } catch (error) {
    console.error('Error getting user slug:', error)
    return null
  }
}
</script>

<style scoped>
.timer-display-tab {
  @apply p-6;
  background: linear-gradient(145deg, #f5f0e8 0%, #faf7f0 50%, #f5f0e8 100%);
  border-radius: 0 0 8px 8px;
  box-shadow: inset 0 2px 4px rgba(139, 69, 19, 0.1);
}

.tab-content {
  @apply max-w-4xl mx-auto;
}

.timer-section {
  @apply mb-8;
}

.section-title {
  @apply text-xl font-semibold mb-4;
  color: #5d3820;
  font-family: Georgia, serif;
  text-shadow: 0 1px 1px rgba(255, 255, 255, 0.5);
}

.timer-display {
  @apply flex justify-center;
}

.no-timer {
  @apply text-center py-8;
  color: #8B6914;
}

.share-section {
  @apply border-t pt-6;
  border-color: rgba(139, 69, 19, 0.2);
}

.share-controls {
  @apply text-center;
}

.share-button {
  @apply px-6 py-3 rounded-lg font-medium transition-all;
  background: linear-gradient(145deg, #B8860B 0%, #DAA520 50%, #B8860B 100%);
  border: 2px solid #8B6914;
  color: #1a0900;
  font-weight: bold;
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.2),
    0 4px 8px rgba(0, 0, 0, 0.2);
}

.share-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.3),
    0 6px 12px rgba(0, 0, 0, 0.3);
}

.share-button:disabled {
  @apply opacity-50 cursor-not-allowed;
}

.share-description {
  @apply text-sm mt-2;
  color: #8B6914;
}
</style>
