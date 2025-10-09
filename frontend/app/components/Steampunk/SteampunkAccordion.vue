<template>
  <BaseAccordion
    :title="title"
    :content="content"
    :initially-open="initiallyOpen"
    :header-classes="headerClasses"
    :panel-classes="panelClasses"
    @toggle="$emit('toggle', $event)"
    @open="$emit('open')"
    @close="$emit('close')"
  >
    <template #header="{ isOpen }">
      <slot name="header" :is-open="isOpen">
        <div class="steampunk-accordion-header">
          <!-- Decorative gear icon -->
          <span class="accordion-icon" :class="{ 'rotate-90': isOpen }">
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
            </svg>
          </span>

          <!-- Title -->
          <span class="accordion-title">{{ title }}</span>

          <!-- Expand/collapse indicator -->
          <span class="accordion-arrow" :class="{ 'rotate-180': isOpen }">
            ▼
          </span>
        </div>
      </slot>
    </template>

    <template #content>
      <slot>
        {{ content }}
      </slot>
    </template>
  </BaseAccordion>
</template>

<script setup>
import BaseAccordion from '~/components/Base/BaseAccordion.vue'
import { computed } from 'vue'

const props = defineProps({
  title: {
    type: String,
    required: true
  },
  content: {
    type: String,
    default: ''
  },
  initiallyOpen: {
    type: Boolean,
    default: false
  },
  variant: {
    type: String,
    default: 'brass', // 'brass', 'mahogany', 'copper'
    validator: (value) => ['brass', 'mahogany', 'copper'].includes(value)
  }
})

defineEmits(['toggle', 'open', 'close'])

// Steampunk-themed header classes
const headerClasses = computed(() => {
  const base = 'steampunk-accordion-header-base cursor-pointer select-none transition-all duration-200'

  const variants = {
    brass: 'bg-gradient-to-r from-amber-100 to-yellow-100 hover:from-amber-200 hover:to-yellow-200 border-2 border-amber-600 text-amber-900',
    mahogany: 'bg-gradient-to-r from-red-900 to-amber-900 hover:from-red-800 hover:to-amber-800 border-2 border-amber-700 text-amber-100',
    copper: 'bg-gradient-to-r from-orange-200 to-red-200 hover:from-orange-300 hover:to-red-300 border-2 border-orange-600 text-red-900'
  }

  return `${base} ${variants[props.variant]}`
})

// Steampunk-themed panel classes
const panelClasses = computed(() => {
  const base = 'steampunk-accordion-panel'

  const variants = {
    brass: 'bg-gradient-to-br from-amber-50 to-yellow-50 border-2 border-t-0 border-amber-600 text-gray-800',
    mahogany: 'bg-gradient-to-br from-amber-50 to-orange-50 border-2 border-t-0 border-amber-700 text-gray-900',
    copper: 'bg-gradient-to-br from-orange-50 to-red-50 border-2 border-t-0 border-orange-600 text-gray-900'
  }

  return `${base} ${variants[props.variant]}`
})
</script>

<style scoped>
/* Header styling */
.steampunk-accordion-header-base {
  @apply px-4 py-3 rounded-t-lg;
}

.steampunk-accordion-header {
  @apply flex items-center gap-3;
}

.accordion-icon {
  @apply transition-transform duration-300;
  @apply flex-shrink-0;
}

.accordion-title {
  @apply flex-grow font-semibold;
}

.accordion-arrow {
  @apply transition-transform duration-300;
  @apply flex-shrink-0 text-sm;
}

/* Panel styling */
.steampunk-accordion-panel {
  @apply px-4 py-3 rounded-b-lg;
}

/* Add subtle texture to panel */
:deep(.steampunk-accordion-panel) {
  background-image:
    repeating-linear-gradient(
      45deg,
      transparent,
      transparent 10px,
      rgba(0, 0, 0, 0.02) 10px,
      rgba(0, 0, 0, 0.02) 20px
    );
}

/* Hover effect */
.steampunk-accordion-header-base:hover {
  @apply shadow-md;
}

/* Focus styles for accessibility */
.steampunk-accordion-header-base:focus {
  @apply outline-none ring-2 ring-amber-500 ring-offset-2;
}
</style>
