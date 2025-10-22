<template>
  <div class="overview-tab">
    <!-- Loading State -->
    <div v-if="adminStore.isLoading" class="flex justify-center items-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-nautical-900"></div>
    </div>

    <!-- Error State -->
    <div v-else-if="adminStore.error" class="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
      <p class="text-red-800 text-sm">{{ adminStore.error }}</p>
      <button 
        @click="refreshStats"
        class="mt-2 text-sm text-red-600 hover:text-red-700 underline"
      >
        Try again
      </button>
    </div>

    <!-- Stats Display -->
    <div v-else-if="displayStats" class="space-y-6">
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <!-- Total Users -->
      <div class="bg-white rounded-lg shadow-sm border border-nautical-200 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-primary-100 rounded-md flex items-center justify-center">
              <span class="text-primary-600 text-lg">👥</span>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-nautical-500">Total Users</p>
            <p class="text-2xl font-semibold text-nautical-900">{{ displayStats?.total_users || 0 }}</p>
          </div>
        </div>
      </div>

      <!-- Active Users -->
      <div class="bg-white rounded-lg shadow-sm border border-nautical-200 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-green-100 rounded-md flex items-center justify-center">
              <span class="text-green-600 text-lg">✅</span>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-nautical-500">Active Users</p>
            <p class="text-2xl font-semibold text-nautical-900">{{ displayStats?.active_users || 0 }}</p>
          </div>
        </div>
      </div>

      <!-- Pending Suggestions -->
      <div class="bg-white rounded-lg shadow-sm border border-nautical-200 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-gold-100 rounded-md flex items-center justify-center">
              <span class="text-gold-600 text-lg">⏳</span>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-nautical-500">Pending Suggestions</p>
            <p class="text-2xl font-semibold text-nautical-900">{{ displayStats?.pending_suggestions || 0 }}</p>
          </div>
        </div>
      </div>

      <!-- Total Phrases -->
      <div class="bg-white rounded-lg shadow-sm border border-nautical-200 p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-purple-100 rounded-md flex items-center justify-center">
              <span class="text-purple-600 text-lg">💬</span>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-nautical-500">Total Phrases</p>
            <p class="text-2xl font-semibold text-nautical-900">{{ displayStats?.total_phrases || 0 }}</p>
          </div>
        </div>
      </div>
      </div>

      <!-- Access Requests Section -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- Pending Access Requests -->
        <div class="bg-white rounded-lg shadow-sm border border-nautical-200 p-6">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <div class="w-8 h-8 bg-orange-100 rounded-md flex items-center justify-center">
                <span class="text-orange-600 text-lg">🔑</span>
              </div>
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium text-nautical-500">Pending Access Requests</p>
              <p class="text-2xl font-semibold text-nautical-900">{{ displayStats?.pending_access_requests || 0 }}</p>
            </div>
          </div>
        </div>

        <!-- Total Access Requests -->
        <div class="bg-white rounded-lg shadow-sm border border-nautical-200 p-6">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <div class="w-8 h-8 bg-indigo-100 rounded-md flex items-center justify-center">
                <span class="text-indigo-600 text-lg">🔐</span>
              </div>
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium text-nautical-500">Total Access Requests</p>
              <p class="text-2xl font-semibold text-nautical-900">{{ displayStats?.total_access_requests || 0 }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="text-center py-12">
      <div class="text-nautical-400 text-6xl mb-4">📊</div>
      <h3 class="text-lg font-medium text-nautical-900 mb-2">No Statistics Available</h3>
      <p class="text-nautical-500 mb-4">Unable to load system statistics.</p>
      <button 
        @click="refreshStats"
        class="px-4 py-2 bg-nautical-900 text-white rounded-md hover:bg-nautical-800 transition-colors"
      >
        Refresh
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAdminStore } from '~/stores/admin'

// Type for admin stats
interface AdminStats {
  total_users: number
  active_users: number
  pending_suggestions: number
  total_phrases: number
  pending_access_requests: number
  total_access_requests: number
}

const adminStore = useAdminStore()

// Reactive data source: Use store data directly
const displayStats = computed((): AdminStats | null => {
  return adminStore.stats
})

// Load stats directly in setup. This runs ON THE SERVER.
// Nuxt will wait for this to complete before sending the page.
if (!adminStore.stats) {
  console.log('🔄 Loading admin stats for OverviewTab...')
  await adminStore.fetchStats()
}

// Refresh stats function
const refreshStats = async () => {
  await adminStore.fetchStats()
}
</script>

<style scoped>
.overview-tab {
  @apply space-y-6;
}
</style>
