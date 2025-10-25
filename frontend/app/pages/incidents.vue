<template>
  <div class="min-h-screen mahogany-background px-4 sm:px-6 lg:px-8 py-8 relative">
    <!-- Steampunk Background -->
    <SteampunkBackground />

    <div class="max-w-6xl mx-auto relative z-10">
      <!-- Header -->
      <div class="mb-8">
        <h1 class="text-3xl sm:text-4xl font-bold mb-2" style="color: #FFD700; text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5), 0 0 8px rgba(255, 215, 0, 0.3);">
          Incident
          <span class="text-transparent bg-clip-text bg-gradient-to-r" style="background-image: linear-gradient(to right, #FFD700, #FFA500);">
            Management
          </span>
        </h1>
        <p class="text-lg" style="color: #D4AF37; text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);">
          Track and manage your incident-free time periods with precision.
        </p>
      </div>

      <!-- Unauthenticated View: Public Timer List -->
      <div v-if="!user">
        <!-- CTA Section -->
        <div class="steampunk-card rounded-lg shadow-lg border-4 border-brass p-6 mb-8">
          <div class="max-w-3xl mx-auto text-center">
            <h2 class="text-2xl font-semibold mb-2" style="color: #FFD700; text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);">
              Want to track your own timer?
            </h2>
            <p class="mb-6" style="color: #D4AF37; text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);">
              Sign in or create an account to start tracking your incident-free periods.
            </p>
            <div class="flex flex-col sm:flex-row gap-3 justify-center">
              <NuxtLink
                to="/login?redirect=/incidents"
                class="brass-button px-6 py-3 rounded-md transition-all duration-200 font-medium"
              >
                Sign In
              </NuxtLink>
              <NuxtLink
                to="/register?redirect=/incidents"
                class="brass-button-outline px-6 py-3 rounded-md transition-all duration-200 font-medium"
              >
                Create Account
              </NuxtLink>
            </div>
          </div>
        </div>

        <!-- Public Timers List -->
        <div class="steampunk-card rounded-lg shadow-lg border-4 border-brass p-6">
          <h2 class="text-2xl font-semibold mb-6" style="color: #FFD700; text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);">Public Timers</h2>
          <PublicTimerListDisplay />
        </div>
      </div>

      <!-- Main Content (Authenticated Users) -->
      <div v-else>
        <!-- Steampunk Banner with Random Phrase (Above Tabs) -->
        <div class="mb-8">
          <SteampunkBanner />
        </div>

        <!-- Tab Navigation -->
        <div class="steampunk-card rounded-lg shadow-lg border-4 border-brass mb-6">
          <TabNavigation />
        </div>

        <!-- Tab Content -->
        <div class="steampunk-card rounded-lg shadow-lg border-4 border-brass">
          <TimerDisplayTab
            v-if="activeTab === 'timer-display'"
          />

          <TimerControlsTab
            v-else-if="activeTab === 'timer-controls'"
          />

          <PhraseSuggestionsTab
            v-else-if="activeTab === 'phrase-suggestions'"
          />

          <PhraseFilterTab
            v-else-if="activeTab === 'phrase-filter'"
          />

          <SuggestionHistoryTab
            v-else-if="activeTab === 'suggestion-history'"
          />

          <PublicTimersTab
            v-else-if="activeTab === 'public-timers'"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { INCIDENT_TABS, type IncidentTabId } from '~/constants/tabs'

// Page meta - No middleware, accessible to both authenticated and unauthenticated users
useHead({
  title: 'Incidents',
  meta: [
    { name: 'description', content: 'View public incident timers and track your own incident-free periods with precision timing and historical data.' }
  ]
})

// Social media sharing
useSocialShare({
  title: 'Incident Timers - KennWilliamson.org',
  imageKey: 'timer'
})

// Stores
const { user } = useUserSession()
const incidentTimerStore = useIncidentTimerStore()

// Load public timer list for all users (authenticated and unauthenticated)
// This is the default view for unauthenticated users and a tab for authenticated users
await useAsyncData(
  'public-timers-list',
  () => incidentTimerStore.loadPublicTimerList(),
  {
    server: true,
    lazy: false
  }
)

// User timers will be loaded by the individual tabs that need them (TimerDisplayTab, TimerControlsTab)

// Tab state using composable (SSR-compatible, reactive to route.query)
const { activeTab, setActiveTab } = useTabs<IncidentTabId>(
  INCIDENT_TABS.ids as readonly IncidentTabId[],
  INCIDENT_TABS.default
)

// Client-side lifecycle hooks
onMounted(() => {
  // Clear public timer when navigating away
  incidentTimerStore.clearPublicTimerOnNavigation()

  // Start live timer updates on client-side
  if (user.value) {
    incidentTimerStore.startLiveTimerUpdates()
  }
})

onUnmounted(() => {
  if (user.value) {
    incidentTimerStore.stopLiveTimerUpdates()
  }
})
</script>

<style scoped>
/* Mahogany Background - Airship Interior */
.mahogany-background {
  background-color: #371200; /* Fallback solid color */
  background-image: url('~/assets/images/mahogany-wood.jpg');
  background-repeat: repeat;
  background-size: 400px 400px;
  background-attachment: fixed;
}

/* Steampunk Card - Brass and Leather */
.steampunk-card {
  background: linear-gradient(145deg, #8B4513 0%, #A0522D 50%, #8B4513 100%);
  box-shadow:
    inset 0 4px 8px rgba(255, 255, 255, 0.2),
    inset 0 -4px 8px rgba(0, 0, 0, 0.4),
    0 8px 32px rgba(0, 0, 0, 0.5);
}

/* Brass Border */
.border-brass {
  border-color: #C0C0C0;
}

/* Brass Buttons */
.brass-button {
  background: linear-gradient(145deg, #B8860B 0%, #DAA520 50%, #B8860B 100%);
  border: 2px solid #C0C0C0;
  color: #1a0900;
  font-weight: bold;
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.2),
    0 4px 8px rgba(0, 0, 0, 0.3);
}

.brass-button:hover {
  transform: translateY(-2px);
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.3),
    0 6px 12px rgba(0, 0, 0, 0.4);
}

.brass-button-outline {
  background: linear-gradient(145deg, rgba(139, 69, 19, 0.3) 0%, rgba(160, 82, 45, 0.3) 50%, rgba(139, 69, 19, 0.3) 100%);
  border: 2px solid #DAA520;
  color: #FFD700;
  font-weight: bold;
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.1),
    0 4px 8px rgba(0, 0, 0, 0.3);
}

.brass-button-outline:hover {
  transform: translateY(-2px);
  background: linear-gradient(145deg, rgba(139, 69, 19, 0.5) 0%, rgba(160, 82, 45, 0.5) 50%, rgba(139, 69, 19, 0.5) 100%);
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.2),
    0 6px 12px rgba(0, 0, 0, 0.4);
}
</style>