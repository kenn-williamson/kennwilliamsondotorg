<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-900 via-blue-900 to-indigo-900 flex items-center justify-center px-4 sm:px-6 lg:px-8">
    <div class="max-w-2xl w-full text-center">
      
      <!-- Loading State -->
      <div v-if="timerStore.loading" class="bg-white/10 backdrop-blur-sm rounded-2xl shadow-2xl border border-white/20 p-12">
        <div class="animate-spin rounded-full h-16 w-16 border-b-4 border-white mx-auto mb-6"></div>
        <p class="text-white/80 text-lg">Loading timer...</p>
      </div>

      <!-- Error State -->
      <div v-else-if="timerStore.error" class="bg-red-900/20 backdrop-blur-sm rounded-2xl shadow-2xl border border-red-500/30 p-12">
        <svg class="w-20 h-20 text-red-400 mx-auto mb-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z"/>
        </svg>
        <h1 class="text-2xl font-bold text-white mb-4">Timer Not Found</h1>
        <p class="text-red-200 mb-8">{{ timerStore.error }}</p>
        <NuxtLink 
          to="/" 
          class="inline-flex items-center px-6 py-3 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors duration-200"
        >
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
          </svg>
          Go Home
        </NuxtLink>
      </div>

      <!-- Timer Display -->
      <div v-else-if="timerStore.publicTimer" class="space-y-8">
        <!-- Header -->
        <div class="mb-12">
          <h1 class="text-3xl sm:text-4xl font-bold text-white mb-4">
            Incident-Free Time
          </h1>
          <p class="text-white/70 text-lg">
            Live tracking for {{ userSlug }}
          </p>
        </div>

        <!-- Main Timer Display -->
        <div class="bg-white/10 backdrop-blur-sm rounded-3xl shadow-2xl border border-white/20 p-12 mb-8">
          <div class="mb-8">
            <div 
              class="text-6xl sm:text-8xl lg:text-9xl font-mono font-bold text-white tracking-tight"
              id="public-live-timer"
            >
              {{ formatElapsedTime(timerStore.publicTimer) }}
            </div>
          </div>
          
          <div class="space-y-4">
            <p class="text-white/80 text-xl">
              Started {{ formatDate(timerStore.publicTimer.reset_timestamp) }}
            </p>
            
            <div v-if="timerStore.publicTimer.notes" class="bg-white/5 rounded-lg p-4 border border-white/10">
              <p class="text-white/70 italic text-lg">
                "{{ timerStore.publicTimer.notes }}"
              </p>
            </div>

            <div class="pt-4">
              <p class="text-white/50 text-sm">
                Last updated: {{ formatTime(new Date()) }}
              </p>
            </div>
          </div>
        </div>

        <!-- Stats/Info -->
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          <!-- Days -->
          <div class="bg-white/5 backdrop-blur-sm rounded-xl border border-white/10 p-6">
            <div class="text-3xl font-bold text-blue-300 mb-1">
              {{ getDays(timerStore.publicTimer) }}
            </div>
            <p class="text-white/70">Days</p>
          </div>

          <!-- Hours -->
          <div class="bg-white/5 backdrop-blur-sm rounded-xl border border-white/10 p-6">
            <div class="text-3xl font-bold text-indigo-300 mb-1">
              {{ getHours(timerStore.publicTimer) }}
            </div>
            <p class="text-white/70">Hours Today</p>
          </div>

          <!-- Total Seconds -->
          <div class="bg-white/5 backdrop-blur-sm rounded-xl border border-white/10 p-6 sm:col-span-2 lg:col-span-1">
            <div class="text-3xl font-bold text-purple-300 mb-1">
              {{ getTotalSeconds(timerStore.publicTimer).toLocaleString() }}
            </div>
            <p class="text-white/70">Total Seconds</p>
          </div>
        </div>

        <!-- Geometric decorative elements -->
        <div class="flex justify-center space-x-8 opacity-30 mt-12">
          <!-- Tech-inspired geometric shapes -->
          <div class="w-12 h-12 border-2 border-blue-400 rotate-45 bg-blue-400/10"></div>
          <div class="w-12 h-12 border-2 border-indigo-400 bg-indigo-400/10"></div>
          <div class="w-12 h-12 border-2 border-purple-400 rotate-45 bg-purple-400/10"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
const route = useRoute()
const timerStore = useIncidentTimerStore()

// Get user slug from route params
const userSlug = String(route.params.user_slug);

// Page meta
useHead({
  title: `${userSlug}'s Incident Timer`,
  meta: [
    { name: 'description', content: `Live incident-free timer for ${userSlug}. Real-time tracking of incident-free periods.` },
    { property: 'og:title', content: `${userSlug}'s Incident Timer` },
    { property: 'og:description', content: 'Real-time incident tracking timer' },
    { property: 'og:type', content: 'website' },
  ]
})

// Timer formatting functions
const formatElapsedTime = (timer) => {
  if (!timer?.reset_timestamp) return '00:00:00'
  return timerStore.formatElapsedTime(timer)
}

const formatDate = (dateString) => {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

const formatTime = (date) => {
  return date.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  })
}

// Calculate days, hours, total seconds
const getDays = (timer) => {
  const seconds = timerStore.getElapsedTime(timer)
  return Math.floor(seconds / (24 * 60 * 60))
}

const getHours = (timer) => {
  const seconds = timerStore.getElapsedTime(timer)
  return Math.floor((seconds % (24 * 60 * 60)) / (60 * 60))
}

const getTotalSeconds = (timer) => {
  return timerStore.getElapsedTime(timer)
}

// Load public timer data
onMounted(async () => {
  await timerStore.fetchPublicTimer(userSlug)
  
  // Set up live updates every second
  const updateInterval = setInterval(() => {
    if (timerStore.publicTimer) {
      const timerElement = document.getElementById('public-live-timer')
      if (timerElement) {
        timerElement.textContent = formatElapsedTime(timerStore.publicTimer)
      }
    }
  }, 1000)

  onUnmounted(() => {
    clearInterval(updateInterval)
  })
})

// Refresh timer data every 5 minutes to catch updates
const refreshInterval = setInterval(() => {
  if (!timerStore.loading) {
    timerStore.fetchPublicTimer(userSlug)
  }
}, 5 * 60 * 1000) // 5 minutes

onUnmounted(() => {
  if (refreshInterval) {
    clearInterval(refreshInterval)
  }
})
</script>

<style scoped>
/* Glowing effect for the main timer */
#public-live-timer {
  text-shadow: 0 0 20px rgba(255, 255, 255, 0.5);
  animation: gentle-glow 2s ease-in-out infinite alternate;
}

@keyframes gentle-glow {
  from {
    text-shadow: 0 0 20px rgba(255, 255, 255, 0.3);
  }
  to {
    text-shadow: 0 0 30px rgba(255, 255, 255, 0.6);
  }
}

/* Subtle animation for stat cards */
.bg-white\/5:hover {
  background: rgba(255, 255, 255, 0.1);
  transform: translateY(-2px);
  transition: all 0.3s ease;
}

/* Tech-inspired background pattern */
body {
  background-attachment: fixed;
}

/* Geometric decorative elements animation */
.rotate-45 {
  animation: slow-rotate 20s linear infinite;
}

@keyframes slow-rotate {
  from {
    transform: rotate(45deg);
  }
  to {
    transform: rotate(405deg);
  }
}
</style>