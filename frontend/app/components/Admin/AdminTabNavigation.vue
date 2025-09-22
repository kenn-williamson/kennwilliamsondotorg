<template>
  <div class="admin-tab-navigation">
    <div class="tab-list" role="tablist">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        :class="['tab-button', { active: activeTab === tab.id }]"
        :aria-selected="activeTab === tab.id"
        :aria-controls="`tab-panel-${tab.id}`"
        role="tab"
        @click="setActiveTab(tab.id)"
      >
        <span class="tab-icon" v-html="tab.icon"></span>
        <span class="tab-label">{{ tab.label }}</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Tab } from '#shared/types'

const props = defineProps<{
  activeTab: string
}>()

const emit = defineEmits<{
  'update:activeTab': [tabId: string]
}>()

const tabs: Tab[] = [
  {
    id: 'overview',
    label: 'Overview',
    icon: '📊'
  },
  {
    id: 'users',
    label: 'Users',
    icon: '👥'
  },
  {
    id: 'suggestions',
    label: 'Phrase Suggestions',
    icon: '✍️'
  }
]

const setActiveTab = (tabId: string) => {
  emit('update:activeTab', tabId)
  // Update URL without page reload
  const url = new URL(window.location.href)
  url.searchParams.set('tab', tabId)
  window.history.pushState({}, '', url.toString())
}
</script>

<style scoped>
.admin-tab-navigation {
  @apply mb-8;
}

.tab-list {
  @apply flex flex-wrap gap-1 border-b border-gray-200;
}

.tab-button {
  @apply flex items-center gap-2 px-6 py-4 text-sm font-medium text-gray-600 
         hover:text-gray-900 hover:bg-gray-50 transition-colors
         border-b-2 border-transparent;
}

.tab-button.active {
  @apply text-gray-900 border-gray-900 bg-gray-50;
}

.tab-icon {
  @apply text-lg;
}

.tab-label {
  @apply hidden sm:inline;
}

/* Mobile responsive */
@media (max-width: 640px) {
  .tab-list {
    @apply flex-col gap-1;
  }
  
  .tab-button {
    @apply justify-center w-full;
  }
}
</style>
