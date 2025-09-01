<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-50 px-4 sm:px-6 lg:px-8 py-8">
    <div class="max-w-6xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <h1 class="text-3xl sm:text-4xl font-bold text-gray-900 mb-2">
          Incident 
          <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-indigo-600">
            Management
          </span>
        </h1>
        <p class="text-gray-600 text-lg">
          Track and manage your incident-free time periods with precision.
        </p>
      </div>

      <!-- Authentication Check -->
      <div v-if="!authStore.isAuthenticated" class="text-center py-16">
        <div class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 p-8 max-w-md mx-auto">
          <svg class="w-16 h-16 text-blue-500 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
          </svg>
          <h2 class="text-xl font-semibold text-gray-900 mb-2">Authentication Required</h2>
          <p class="text-gray-600 mb-6">Please sign in to access your incident timers.</p>
          <div class="flex flex-col sm:flex-row gap-3 justify-center">
            <NuxtLink 
              to="/login" 
              class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors duration-200"
            >
              Sign In
            </NuxtLink>
            <NuxtLink 
              to="/register" 
              class="px-4 py-2 border border-blue-600 text-blue-600 rounded-md hover:bg-blue-50 transition-colors duration-200"
            >
              Create Account
            </NuxtLink>
          </div>
        </div>
      </div>

      <!-- Main Content (Authenticated Users) -->
      <div v-else>
        <!-- Quick Actions -->
        <div class="mb-8 bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 p-6">
          <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
            <div>
              <h2 class="text-xl font-semibold text-gray-900 mb-1">Quick Reset</h2>
              <p class="text-gray-600 text-sm">Start a new incident-free period from now.</p>
            </div>
            <div class="flex flex-col sm:flex-row gap-3">
              <button
                @click="showResetModal = true"
                :disabled="timerStore.loading"
                class="px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 transition-colors duration-200 font-medium"
              >
                <svg class="w-4 h-4 inline mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"/>
                </svg>
                Reset Timer
              </button>
              
              <NuxtLink 
                :to="`/${authStore.user?.slug || 'user'}/incident-timer`"
                target="_blank"
                class="px-6 py-3 border border-blue-600 text-blue-600 rounded-md hover:bg-blue-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors duration-200 font-medium text-center"
              >
                <svg class="w-4 h-4 inline mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
                </svg>
                View Public Timer
              </NuxtLink>
            </div>
          </div>
        </div>

        <!-- Current Timer Display -->
        <div v-if="timerStore.latestTimer" class="mb-8 bg-white/90 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 p-8">
          <div class="text-center">
            <h2 class="text-lg font-medium text-gray-700 mb-2">Current Incident-Free Time</h2>
            <div class="text-6xl sm:text-7xl font-mono font-bold text-blue-600 mb-4" id="live-timer">
              {{ formatElapsedTime() }}
            </div>
            <p class="text-gray-600">
              Started {{ formatDate(timerStore.latestTimer.reset_timestamp) }}
            </p>
            <p v-if="timerStore.latestTimer.notes" class="text-gray-500 mt-2 italic">
              "{{ timerStore.latestTimer.notes }}"
            </p>
          </div>
        </div>

        <!-- No Timer State -->
        <div v-else-if="!timerStore.loading" class="mb-8 bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 p-8 text-center">
          <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
          </svg>
          <h2 class="text-xl font-semibold text-gray-900 mb-2">No Active Timer</h2>
          <p class="text-gray-600 mb-4">Start tracking your incident-free time by creating your first timer.</p>
          <button
            @click="showResetModal = true"
            class="px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors duration-200 font-medium"
          >
            Create First Timer
          </button>
        </div>

        <!-- Timer History -->
        <div class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200">
          <div class="px-6 py-4 border-b border-blue-100">
            <h2 class="text-xl font-semibold text-gray-900">Timer History</h2>
          </div>
          
          <div v-if="timerStore.loading" class="p-8 text-center">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
            <p class="text-gray-500 mt-2">Loading timers...</p>
          </div>
          
          <div v-else-if="timerStore.timers.length === 0" class="p-8 text-center text-gray-500">
            No timer history yet. Create your first timer to get started!
          </div>
          
          <div v-else class="divide-y divide-blue-100">
            <div 
              v-for="timer in timerStore.timers" 
              :key="timer.id"
              class="p-6 hover:bg-blue-50/50 transition-colors duration-200"
            >
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
                    {{ timer === timerStore.latestTimer ? formatElapsedTime() : timerStore.formatElapsedTime(timer) }}
                  </span>
                  <div class="flex gap-2">
                    <button
                      @click="editTimer(timer)"
                      class="p-2 text-gray-500 hover:text-blue-600 hover:bg-blue-100 rounded-md transition-colors duration-200"
                      title="Edit timer"
                    >
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
                      </svg>
                    </button>
                    <button
                      @click="confirmDelete(timer)"
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
          </div>
        </div>
      </div>

      <!-- Reset Timer Modal -->
      <div v-if="showResetModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-md w-full p-6">
          <h3 class="text-lg font-semibold text-gray-900 mb-4">Reset Incident Timer</h3>
          
          <form @submit.prevent="handleReset">
            <div class="mb-4">
              <label for="resetNotes" class="block text-sm font-medium text-gray-700 mb-2">
                Notes (optional)
              </label>
              <textarea
                id="resetNotes"
                v-model="resetForm.notes"
                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                rows="3"
                placeholder="Add any notes about this reset..."
              ></textarea>
            </div>
            
            <div class="flex gap-3 justify-end">
              <button
                type="button"
                @click="showResetModal = false"
                class="px-4 py-2 text-gray-700 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors duration-200"
              >
                Cancel
              </button>
              <button
                type="submit"
                :disabled="timerStore.loading"
                class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 transition-colors duration-200"
              >
                {{ timerStore.loading ? 'Resetting...' : 'Reset Timer' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { storeToRefs } from 'pinia'

// Page meta and middleware
definePageMeta({
  middleware: 'auth'
})

useHead({
  title: 'Incidents',
  meta: [
    { name: 'description', content: 'Manage your incident timers and track incident-free periods with precision timing and historical data.' }
  ]
})

// Stores
const authStore = useAuthStore()
const timerStore = useIncidentTimerStore()
const { activeTimerBreakdown } = storeToRefs(timerStore)

// Reactive state
const showResetModal = ref(false)
const resetForm = reactive({
  notes: ''
})

// Format elapsed time using reactive store breakdown
const formatElapsedTime = () => {
  const breakdown = activeTimerBreakdown.value
  if (!breakdown || (breakdown.years === 0 && breakdown.months === 0 && breakdown.weeks === 0 && breakdown.days === 0 && breakdown.hours === 0 && breakdown.minutes === 0 && breakdown.seconds === 0)) {
    return 'No incident started'
  }
  
  const parts = []
  
  if (breakdown.years > 0) parts.push(`${breakdown.years} year${breakdown.years !== 1 ? 's' : ''}`)
  if (breakdown.months > 0) parts.push(`${breakdown.months} month${breakdown.months !== 1 ? 's' : ''}`)
  if (breakdown.weeks > 0) parts.push(`${breakdown.weeks} week${breakdown.weeks !== 1 ? 's' : ''}`)
  if (breakdown.days > 0) parts.push(`${breakdown.days} day${breakdown.days !== 1 ? 's' : ''}`)
  if (breakdown.hours > 0) parts.push(`${breakdown.hours} hour${breakdown.hours !== 1 ? 's' : ''}`)
  if (breakdown.minutes > 0) parts.push(`${breakdown.minutes} minute${breakdown.minutes !== 1 ? 's' : ''}`)
  if (breakdown.seconds > 0 || parts.length === 0) parts.push(`${breakdown.seconds} second${breakdown.seconds !== 1 ? 's' : ''}`)
  
  return parts.join(', ')
}

// Format date for display
const formatDate = (dateString) => {
  return new Date(dateString).toLocaleString()
}

// Handle timer reset
const handleReset = async () => {
  const result = await timerStore.quickReset(resetForm.notes || undefined)
  
  if (result.success) {
    showResetModal.value = false
    resetForm.notes = ''
  }
}

// Edit timer (placeholder for future implementation)
const editTimer = (timer) => {
  console.log('Edit timer:', timer.id)
  // TODO: Implement edit functionality
}

// Confirm delete timer (placeholder for future implementation)
const confirmDelete = (timer) => {
  if (confirm('Are you sure you want to delete this timer?')) {
    timerStore.deleteTimer(timer.id)
  }
}

// Load timers on mount
onMounted(async () => {
  if (authStore.isAuthenticated) {
    await timerStore.fetchUserTimers()
  }
})

// Cleanup on unmount
onUnmounted(() => {
  timerStore.stopLiveTimerUpdates()
})

// Watch for authentication changes
watch(() => authStore.isAuthenticated, (isAuth) => {
  if (isAuth) {
    timerStore.fetchUserTimers()
  } else {
    timerStore.clearState()
  }
})
</script>

<style scoped>
/* Geometric tech pattern background */
.bg-tech-pattern {
  background-image: 
    linear-gradient(45deg, rgba(59, 130, 246, 0.03) 25%, transparent 25%),
    linear-gradient(-45deg, rgba(59, 130, 246, 0.03) 25%, transparent 25%),
    linear-gradient(45deg, transparent 75%, rgba(59, 130, 246, 0.03) 75%),
    linear-gradient(-45deg, transparent 75%, rgba(59, 130, 246, 0.03) 75%);
  background-size: 20px 20px;
  background-position: 0 0, 0 10px, 10px -10px, -10px 0px;
}

/* Subtle metallic accent for timer display */
.timer-display {
  background: linear-gradient(135deg, rgba(219, 234, 254, 0.8), rgba(147, 197, 253, 0.8));
  border: 1px solid rgba(59, 130, 246, 0.2);
}
</style>