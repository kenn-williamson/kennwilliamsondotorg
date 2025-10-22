<template>
  <div class="admin-panel">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-nautical-900 mb-2">Admin Panel</h1>
      <p class="text-nautical-600">Manage users, moderate content, and view system statistics</p>
    </div>

    <!-- Tab Navigation -->
    <AdminTabNavigation />

    <!-- Tab Content -->
    <div class="tab-content">
      <!-- Overview Tab -->
      <div v-if="activeTab === 'overview'" class="tab-panel">
        <OverviewTab />
      </div>

      <!-- Users Tab -->
      <div v-else-if="activeTab === 'users'" class="tab-panel">
        <UsersTab />
      </div>

      <!-- Suggestions Tab -->
      <div v-else-if="activeTab === 'suggestions'" class="tab-panel">
        <PhraseSuggestionApprovalTab />
      </div>

      <!-- Access Requests Tab -->
      <div v-else-if="activeTab === 'access-requests'" class="tab-panel">
        <AccessRequestsTab />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ADMIN_TABS, type AdminTabId } from '~/constants/tabs'

// Get tab state from composable (reactive via route.query)
// URL updates are handled automatically by the composable
const { activeTab } = useTabs<AdminTabId>(
  ADMIN_TABS.ids as readonly AdminTabId[],
  ADMIN_TABS.default
)
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
