<template>
  <div class="public-timer-list-display">
    <!-- Search Bar -->
    <div class="mb-6">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search by display name..."
        @input="handleSearch"
        class="w-full px-4 py-2 border border-nautical-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />
    </div>

    <!-- Loading State -->
    <div v-if="incidentTimerStore.publicTimersLoading" class="text-center py-8">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600 mx-auto"></div>
      <span class="mt-3 text-nautical-600">Loading public timers...</span>
    </div>

    <!-- Empty State -->
    <div v-else-if="incidentTimerStore.publicTimersList.length === 0" class="text-center py-16">
      <p class="text-nautical-600">{{ searchQuery ? 'No timers found matching your search.' : 'No public timers available yet.' }}</p>
      <p class="text-sm text-nautical-500 mt-2">{{ searchQuery ? 'Try a different search term.' : 'Be the first to share your timer publicly!' }}</p>
    </div>

    <!-- Timer Grid -->
    <div v-else class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      <NuxtLink
        v-for="timer in incidentTimerStore.publicTimersList"
        :key="timer.id"
        :to="`/${timer.slug}/incident-timer`"
        class="bg-white/80 backdrop-blur-sm rounded-lg shadow-lg border border-primary-200 p-6 hover:shadow-xl hover:border-primary-300 transition-all duration-200"
      >
        <h3 class="text-lg font-semibold text-nautical-900 mb-2">
          {{ timer.display_name }}
        </h3>
        <p class="text-sm text-nautical-600 mb-3">
          <span class="font-medium">Last reset:</span> {{ formatDate(timer.reset_timestamp) }}
        </p>
        <p
          v-if="timer.notes"
          class="text-sm text-nautical-500 line-clamp-2"
        >
          {{ timer.notes }}
        </p>
        <div class="mt-4 flex items-center text-xs text-primary-600">
          <span>View timer</span>
          <svg class="ml-1 w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </div>
      </NuxtLink>
    </div>

    <!-- Pagination Controls -->
    <div
      v-if="incidentTimerStore.publicTimersList.length > 0"
      class="mt-8 flex items-center justify-center gap-4"
    >
      <button
        @click="previousPage"
        :disabled="incidentTimerStore.publicTimersPage <= 1 || incidentTimerStore.publicTimersLoading"
        class="px-4 py-2 text-sm font-medium text-nautical-700 bg-white border border-nautical-300 rounded-md hover:bg-nautical-50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
      >
        Previous
      </button>
      <span class="text-sm text-nautical-600">
        Page {{ incidentTimerStore.publicTimersPage }}
      </span>
      <button
        @click="nextPage"
        :disabled="incidentTimerStore.publicTimersList.length < incidentTimerStore.publicTimersPageSize || incidentTimerStore.publicTimersLoading"
        class="px-4 py-2 text-sm font-medium text-nautical-700 bg-white border border-nautical-300 rounded-md hover:bg-nautical-50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
      >
        Next
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useIncidentTimerStore } from '~/stores/incident-timers'
import { useDebounce } from '~/composables/useDebounce'

// Store
const incidentTimerStore = useIncidentTimerStore()

// Search state
const searchQuery = ref('')

// Format date helper
const formatDate = (timestamp: string) => {
  return new Date(timestamp).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// Debounced search handler
const { debounced: debouncedSearch } = useDebounce(async (query: string) => {
  // Reset to page 1 when searching
  await incidentTimerStore.loadPublicTimerList(1, incidentTimerStore.publicTimersPageSize, query || undefined)
}, 300)

// Search handler
const handleSearch = () => {
  debouncedSearch(searchQuery.value.trim())
}

// Pagination handlers
const nextPage = async () => {
  const nextPageNum = incidentTimerStore.publicTimersPage + 1
  await incidentTimerStore.loadPublicTimerList(nextPageNum, incidentTimerStore.publicTimersPageSize, searchQuery.value.trim() || undefined)
}

const previousPage = async () => {
  const prevPageNum = Math.max(1, incidentTimerStore.publicTimersPage - 1)
  await incidentTimerStore.loadPublicTimerList(prevPageNum, incidentTimerStore.publicTimersPageSize, searchQuery.value.trim() || undefined)
}

// Load initial data when component mounts
onMounted(async () => {
  await incidentTimerStore.loadPublicTimerList()
})
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
