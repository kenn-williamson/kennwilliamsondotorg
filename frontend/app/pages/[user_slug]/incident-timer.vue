<template>
  <div class="min-h-screen flex items-center justify-center px-4 sm:px-6 lg:px-8 relative mahogany-background">
    <!-- Steampunk Background -->
    <SteampunkBackground />
    
    <div class="max-w-4xl w-full text-center relative z-10">
      
      <!-- Loading State -->
      <div v-if="incidentTimerStore.isLoading" class="steampunk-loading-card">
        <div class="loading-gears">
          <div class="loading-gear gear-1"></div>
          <div class="loading-gear gear-2"></div>
        </div>
        <p class="loading-text">Initializing Steam Timer...</p>
      </div>

      <!-- Error State -->
      <div v-else-if="incidentTimerStore.error" class="steampunk-error-card">
        <div class="error-icon">
          <svg viewBox="0 0 24 24" class="error-svg">
            <path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z" 
                  fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <h1 class="error-title">Timer Not Found</h1>
        <p class="error-message">The requested timer could not be found.</p>
        <NuxtLink to="/" class="error-button">
          <svg class="button-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
          </svg>
          Return Home
        </NuxtLink>
      </div>

      <!-- Timer Display -->
      <div v-else-if="incidentTimerStore.publicTimer" class="space-y-8">
        <!-- Steampunk Banner -->
        <SteampunkBanner :user-slug="userSlug" />
        
        <!-- User Info -->
        <div class="user-info">
          <p class="user-tracking">
            Live tracking for <span class="user-name">{{ incidentTimerStore.publicTimer.user_display_name }}</span>
          </p>
        </div>

        <!-- Main Steam Clock -->
        <SteamClock :time-breakdown="timeBreakdown" />
        


        <!-- Vintage Note Card -->
        <VintageNoteCard 
          v-if="incidentTimerStore.publicTimer.notes"
          :notes="incidentTimerStore.publicTimer.notes"
          :reset-timestamp="incidentTimerStore.publicTimer.reset_timestamp"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { storeToRefs } from 'pinia'
import { useIncidentTimerStore } from '~/stores/incident-timers'

const route = useRoute()
const incidentTimerStore = useIncidentTimerStore()

// Get user slug from route params
const userSlug = String(route.params.user_slug);

// Computed meta for dynamic updates
const pageTitle = computed(() => {
  const displayName = incidentTimerStore.publicTimer?.user_display_name || userSlug
  return `${displayName}'s Incident Timer`
})

const pageDescription = computed(() => {
  const displayName = incidentTimerStore.publicTimer?.user_display_name || userSlug
  return `Live incident-free timer for ${displayName}. Real-time tracking of incident-free periods.`
})

// Page meta with computed values
useHead(() => ({
  title: pageTitle.value,
  meta: [
    { name: 'description', content: pageDescription.value },
    { property: 'og:title', content: pageTitle.value },
    { property: 'og:description', content: 'Real-time incident tracking timer' },
    { property: 'og:type', content: 'website' },
  ]
}))

// Reactive time breakdown from store - automatically updates every second
const { activeTimerBreakdown } = storeToRefs(incidentTimerStore)
const timeBreakdown = computed(() => activeTimerBreakdown.value)

// Utility formatting functions (kept for potential future use)

// Load public timer data directly in setup. This runs ON THE SERVER.
// Nuxt will wait for this to complete before sending the page.
console.log('🔄 Loading public timer for user:', userSlug)
await incidentTimerStore.loadPublicTimer(userSlug)

// Start timers after hydration (client-side only)
onMounted(() => {
  console.log('🔄 Starting timers after hydration in public timer page...')
  incidentTimerStore.startLiveTimerUpdates()
})

// Cleanup on unmount
onUnmounted(() => {
  // Stop live timer when component unmounts
  incidentTimerStore.stopLiveTimerUpdates()
})
</script>

<style scoped>
/* Mahogany Background */
.mahogany-background {
  background-color: #371200; /* Fallback solid color */
  background-image: url('~/assets/images/mahogany-wood.jpg');
  background-repeat: repeat;
  background-size: 400px 400px; /* Adjust size as needed */
  background-attachment: fixed;
}

/* Steampunk Loading State */
.steampunk-loading-card {
  background: 
    linear-gradient(145deg, #8B4513 0%, #A0522D 50%, #8B4513 100%);
  border: 4px solid #C0C0C0;
  border-radius: 16px;
  padding: 48px;
  box-shadow: 
    inset 0 4px 8px rgba(255, 255, 255, 0.2),
    inset 0 -4px 8px rgba(0, 0, 0, 0.4),
    0 8px 32px rgba(0, 0, 0, 0.5);
  
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
}

.loading-gears {
  display: flex;
  gap: 16px;
  align-items: center;
}

.loading-gear {
  width: 40px;
  height: 40px;
  border: 4px solid #FFD700;
  border-radius: 50%;
  border-right-color: transparent;
  border-top-color: transparent;
}

.loading-gear.gear-1 {
  animation: spin-clockwise 2s linear infinite;
}

.loading-gear.gear-2 {
  animation: spin-counter-clockwise 1.5s linear infinite;
}

@keyframes spin-clockwise {
  to {
    transform: rotate(360deg);
  }
}

@keyframes spin-counter-clockwise {
  to {
    transform: rotate(-360deg);
  }
}

.loading-text {
  font-family: 'Georgia', serif;
  font-size: 20px;
  font-weight: bold;
  color: #FFD700;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
}

/* Steampunk Error State */
.steampunk-error-card {
  background: 
    linear-gradient(145deg, #5d3420 0%, #4a2c18 50%, #3c2414 100%);
  border: 4px solid #8B4513;
  border-radius: 16px;
  padding: 48px;
  box-shadow: 
    inset 0 4px 8px rgba(255, 255, 255, 0.1),
    inset 0 -4px 8px rgba(0, 0, 0, 0.4),
    0 8px 32px rgba(0, 0, 0, 0.5);
  
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
}

.error-icon {
  width: 80px;
  height: 80px;
  background: 
    radial-gradient(circle at 30% 30%, #CC0000, #AA0000 60%, #880000);
  border: 3px solid #660000;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 
    0 4px 8px rgba(0, 0, 0, 0.4),
    inset 0 2px 4px rgba(255, 255, 255, 0.2);
}

.error-svg {
  width: 48px;
  height: 48px;
  color: rgba(255, 255, 255, 0.9);
}

.error-title {
  font-family: 'Georgia', serif;
  font-size: 28px;
  font-weight: bold;
  color: #FFD700;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
  margin: 0;
}

.error-message {
  font-family: 'Georgia', serif;
  font-size: 18px;
  color: #D4AF37;
  text-align: center;
  opacity: 0.9;
  margin: 0;
}

.error-button {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background: 
    linear-gradient(145deg, #1e40af 0%, #2563eb 50%, #1e40af 100%);
  border: 2px solid #C0C0C0;
  border-radius: 8px;
  color: #FFD700;
  font-family: 'Georgia', serif;
  font-weight: bold;
  text-decoration: none;
  box-shadow: 
    inset 0 2px 4px rgba(255, 255, 255, 0.2),
    0 4px 8px rgba(0, 0, 0, 0.3);
  
  transition: all 0.3s ease;
}

.error-button:hover {
  transform: translateY(-2px);
  box-shadow: 
    inset 0 2px 4px rgba(255, 255, 255, 0.3),
    0 6px 12px rgba(0, 0, 0, 0.4);
}

.button-icon {
  width: 16px;
  height: 16px;
}

/* User Info Styling */
.user-info {
  margin-bottom: 32px;
}

.user-tracking {
  font-family: 'Georgia', serif;
  font-size: 20px;
  color: #D4AF37;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
  margin: 0;
}

.user-name {
  color: #FFD700;
  font-weight: bold;
  text-shadow: 0 0 8px rgba(255, 215, 0, 0.3);
}



/* Responsive Design */
@media (max-width: 768px) {
  .steampunk-loading-card,
  .steampunk-error-card {
    padding: 32px;
  }
  
  .loading-gear {
    width: 32px;
    height: 32px;
    border-width: 3px;
  }
  
  .loading-text {
    font-size: 18px;
  }
  
  .error-icon {
    width: 64px;
    height: 64px;
  }
  
  .error-svg {
    width: 36px;
    height: 36px;
  }
  
  .error-title {
    font-size: 24px;
  }
  
  .error-message {
    font-size: 16px;
  }
  
  .user-tracking {
    font-size: 18px;
  }
  

}

@media (max-width: 480px) {
  .steampunk-loading-card,
  .steampunk-error-card {
    padding: 24px;
  }
  
  .loading-gear {
    width: 28px;
    height: 28px;
  }
  
  .loading-text {
    font-size: 16px;
  }
  
  .error-title {
    font-size: 20px;
  }
  
  .error-message {
    font-size: 14px;
  }
  
  .user-tracking {
    font-size: 16px;
  }
  

}
</style>