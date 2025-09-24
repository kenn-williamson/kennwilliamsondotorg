<template>
  <div class="tab-navigation">
    <div class="tab-list" role="tablist">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        :class="['tab-button', { active: incidentTimerStore.activeTab === tab.id }]"
        :aria-selected="incidentTimerStore.activeTab === tab.id"
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
import { useIncidentTimerStore } from '~/stores/incident-timers'

const incidentTimerStore = useIncidentTimerStore()

const tabs: Tab[] = [
  {
    id: 'timer-display',
    label: 'Timer Display',
    icon: '⏰'
  },
  {
    id: 'timer-controls',
    label: 'Timer Controls',
    icon: '⚙️'
  },
  {
    id: 'phrase-suggestions',
    label: 'Suggest Phrases',
    icon: '✍️'
  },
  {
    id: 'phrase-filter',
    label: 'Filter Phrases',
    icon: '🔧'
  },
  {
    id: 'suggestion-history',
    label: 'My Suggestions',
    icon: '📋'
  }
]

const setActiveTab = (tabId: string) => {
  incidentTimerStore.setActiveTab(tabId)
}
</script>

<style scoped>
.tab-navigation {
  @apply mb-6;
}

.tab-list {
  @apply flex flex-wrap gap-2 border-b border-gray-300;
}

.tab-button {
  @apply flex items-center gap-2 px-4 py-3 text-sm font-medium text-gray-600 
         hover:text-gray-900 hover:bg-gray-50 transition-colors
         border-b-2 border-transparent;
}

.tab-button.active {
  @apply text-blue-600 border-blue-600 bg-blue-50;
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
