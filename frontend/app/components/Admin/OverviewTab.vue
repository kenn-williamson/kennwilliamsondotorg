<template>
  <div class="overview-tab">
    <!-- Loading State -->
    <div v-if="isLoading" class="flex justify-center items-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
      <p class="text-red-800 text-sm">{{ error }}</p>
      <button 
        @click="refreshStats"
        class="mt-2 text-sm text-red-600 hover:text-red-700 underline"
      >
        Try again
      </button>
    </div>

    <!-- Stats Display -->
    <div v-else-if="adminStore.stats" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <!-- Total Users -->
      <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-blue-100 rounded-md flex items-center justify-center">
              <span class="text-blue-600 text-lg">👥</span>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500">Total Users</p>
            <p class="text-2xl font-semibold text-gray-900">{{ adminStore.stats.total_users }}</p>
          </div>
        </div>
      </div>

      <!-- Active Users -->
      <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-green-100 rounded-md flex items-center justify-center">
              <span class="text-green-600 text-lg">✅</span>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500">Active Users</p>
            <p class="text-2xl font-semibold text-gray-900">{{ adminStore.stats.active_users }}</p>
          </div>
        </div>
      </div>

      <!-- Pending Suggestions -->
      <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-yellow-100 rounded-md flex items-center justify-center">
              <span class="text-yellow-600 text-lg">⏳</span>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500">Pending Suggestions</p>
            <p class="text-2xl font-semibold text-gray-900">{{ adminStore.stats.pending_suggestions }}</p>
          </div>
        </div>
      </div>

      <!-- Total Phrases -->
      <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-purple-100 rounded-md flex items-center justify-center">
              <span class="text-purple-600 text-lg">💬</span>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500">Total Phrases</p>
            <p class="text-2xl font-semibold text-gray-900">{{ adminStore.stats.total_phrases }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="text-center py-12">
      <div class="text-gray-400 text-6xl mb-4">📊</div>
      <h3 class="text-lg font-medium text-gray-900 mb-2">No Statistics Available</h3>
      <p class="text-gray-500 mb-4">Unable to load system statistics.</p>
      <button 
        @click="refreshStats"
        class="px-4 py-2 bg-gray-900 text-white rounded-md hover:bg-gray-800 transition-colors"
      >
        Refresh
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAdminStore } from '~/stores/admin'
import { useAdminActions } from '~/composables/useAdminActions'

const adminStore = useAdminStore()
const { fetchStats, isLoading, error } = useAdminActions()

// Load stats on mount
onMounted(async () => {
  await fetchStats()
})

// Refresh stats function
const refreshStats = async () => {
  await fetchStats()
}
</script>

<style scoped>
.overview-tab {
  @apply space-y-6;
}
</style>
