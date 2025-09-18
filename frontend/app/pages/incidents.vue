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
      <div v-if="!user" class="text-center py-16">
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
        <!-- Steampunk Banner with Random Phrase (Above Tabs) -->
        <div class="mb-8">
          <SteampunkBanner />
        </div>

        <!-- Tab Navigation -->
        <div class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-blue-200 mb-6">
          <TabNavigation 
            :active-tab="activeTab"
            @update:active-tab="setActiveTab"
          />
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
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
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
const { user } = useUserSession()

// Tab state management
const activeTab = ref('timer-display')

// Initialize tab from URL query parameter
onMounted(() => {
  const urlParams = new URLSearchParams(window.location.search)
  const tabParam = urlParams.get('tab')
  if (tabParam && ['timer-display', 'timer-controls', 'phrase-suggestions', 'phrase-filter', 'suggestion-history'].includes(tabParam)) {
    activeTab.value = tabParam
  }
})

// Handle tab changes
const setActiveTab = (tabId) => {
  activeTab.value = tabId
}
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