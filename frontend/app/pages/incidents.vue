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

      <!-- Unauthenticated View: Public Timer List -->
      <div v-if="!user">
        <!-- CTA Section -->
        <div class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 p-6 mb-8">
          <div class="max-w-3xl mx-auto text-center">
            <h2 class="text-2xl font-semibold text-gray-900 mb-2">
              Want to track your own timer?
            </h2>
            <p class="text-gray-600 mb-6">
              Sign in or create an account to start tracking your incident-free periods.
            </p>
            <div class="flex flex-col sm:flex-row gap-3 justify-center">
              <NuxtLink
                to="/login"
                class="px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors duration-200 font-medium"
              >
                Sign In
              </NuxtLink>
              <NuxtLink
                to="/register"
                class="px-6 py-3 border border-blue-600 text-blue-600 rounded-md hover:bg-blue-50 transition-colors duration-200 font-medium"
              >
                Create Account
              </NuxtLink>
            </div>
          </div>
        </div>

        <!-- Public Timers List -->
        <div class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 p-6">
          <h2 class="text-2xl font-semibold text-gray-900 mb-6">Public Timers</h2>
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
        <div class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 mb-6">
          <TabNavigation />
        </div>

        <!-- Tab Content -->
        <div class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200">
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

// Tab state using composable (SSR-compatible, reactive to route.query)
const { activeTab, setActiveTab } = useTabs<IncidentTabId>(
  INCIDENT_TABS.ids as readonly IncidentTabId[],
  INCIDENT_TABS.default
)

// Clear public timer when navigating away (client-side only)
onMounted(() => {
  incidentTimerStore.clearPublicTimerOnNavigation()
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