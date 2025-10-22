<template>
  <div class="tab-navigation">
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
import { INCIDENT_TABS, type IncidentTabId } from '~/constants/tabs'

// Get tab state from composable (reactive via route.query)
const { activeTab, setActiveTab } = useTabs<IncidentTabId>(
  INCIDENT_TABS.ids as readonly IncidentTabId[],
  INCIDENT_TABS.default
)

// Use tab definitions from constants
const tabs = INCIDENT_TABS.tabs
</script>

<style scoped>
.tab-navigation {
  @apply mb-0;
}

.tab-list {
  @apply flex flex-wrap gap-1 p-2;
  background: linear-gradient(145deg, #1e3a8a 0%, #1e40af 50%, #1e3a8a 100%);
  border-radius: 8px 8px 0 0;
}

.tab-button {
  @apply flex items-center gap-2 px-4 py-3 text-sm font-medium transition-all duration-200;
  background: linear-gradient(145deg, rgba(139, 69, 19, 0.3) 0%, rgba(160, 82, 45, 0.3) 50%, rgba(139, 69, 19, 0.3) 100%);
  border: 2px solid rgba(192, 192, 192, 0.3);
  border-radius: 6px;
  color: #D4AF37;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
}

.tab-button:hover {
  background: linear-gradient(145deg, rgba(139, 69, 19, 0.5) 0%, rgba(160, 82, 45, 0.5) 50%, rgba(139, 69, 19, 0.5) 100%);
  border-color: rgba(218, 165, 32, 0.5);
  color: #FFD700;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
}

.tab-button.active {
  background: linear-gradient(145deg, #B8860B 0%, #DAA520 50%, #B8860B 100%);
  border: 2px solid #C0C0C0;
  color: #1a0900;
  font-weight: bold;
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.2),
    0 4px 8px rgba(0, 0, 0, 0.3);
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

  .tab-label {
    @apply inline;
  }
}
</style>
