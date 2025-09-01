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

        <!-- Current Timer Display with Steampunk Design -->
        <TimerStats 
          :timer="timerStore.latestTimer"
          :breakdown="activeTimerBreakdown"
          :loading="timerStore.loading"
          @create-timer="showResetModal = true"
        />

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
            <TimerListItem
              v-for="timer in timerStore.timers"
              :key="timer.id"
              :timer="timer"
              :is-latest="timer === timerStore.latestTimer"
              :live-elapsed-time="formatElapsedTime()"
              @edit="editTimer"
              @delete="confirmDelete"
            />
          </div>
        </div>
      </div>

      <!-- Timer Reset Modal -->
      <TimerResetModal
        :show="showResetModal"
        :loading="timerStore.loading"
        @close="showResetModal = false"
        @reset="handleReset"
      />

      <!-- Timer Edit Modal -->
      <TimerEditModal
        :show="showEditModal"
        :timer="editingTimer"
        :loading="timerStore.loading"
        @close="closeEditModal"
        @updated="handleEditTimer"
      />
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
const showEditModal = ref(false)
const editingTimer = ref(null)

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

// Handle timer reset
const handleReset = async (notes) => {
  const result = await timerStore.quickReset(notes)
  
  if (result.success) {
    showResetModal.value = false
  }
}

// Edit timer - open modal with current values
const editTimer = (timer) => {
  editingTimer.value = timer
  showEditModal.value = true
}

// Close edit modal
const closeEditModal = () => {
  showEditModal.value = false
  editingTimer.value = null
}

// Handle edit form submission
const handleEditTimer = async (timerId, updateData) => {
  try {
    const result = await timerStore.updateTimer(timerId, updateData)
    
    if (result.success) {
      showEditModal.value = false
      editingTimer.value = null
    }
  } catch (error) {
    console.error('Failed to update timer:', error)
  }
}

// Confirm delete timer
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