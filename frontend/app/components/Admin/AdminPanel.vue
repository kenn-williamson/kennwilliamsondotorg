<template>
  <div class="admin-panel">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-gray-900 mb-2">Admin Panel</h1>
      <p class="text-gray-600">Manage users, moderate content, and view system statistics</p>
    </div>

    <!-- Tab Navigation -->
    <AdminTabNavigation />

    <!-- Tab Content -->
    <div class="tab-content">
      <!-- Overview Tab -->
      <div v-if="adminStore.activeTab === 'overview'" class="tab-panel">
        <OverviewTab />
      </div>

      <!-- Users Tab -->
      <div v-else-if="adminStore.activeTab === 'users'" class="tab-panel">
        <UsersTab />
      </div>

      <!-- Suggestions Tab -->
      <div v-else-if="adminStore.activeTab === 'suggestions'" class="tab-panel">
        <PhraseSuggestionApprovalTab />
      </div>

      <!-- Access Requests Tab -->
      <div v-else-if="adminStore.activeTab === 'access-requests'" class="tab-panel">
        <AccessRequestsTab />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAdminStore } from '~/stores/admin'

const adminStore = useAdminStore()

// Watch for tab changes and update URL
watch(() => adminStore.activeTab, (newTab) => {
  const url = new URL(window.location.href)
  url.searchParams.set('tab', newTab)
  window.history.pushState({}, '', url.toString())
})
</script>

<style scoped>
.admin-panel {
  @apply max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8;
}

.tab-content {
  @apply mt-6;
}

.tab-panel {
  @apply min-h-96;
}
</style>
