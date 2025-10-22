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
    default: 'steel', // 'steel', 'naval-brass', 'bronze', 'gold'
    validator: (value) => ['steel', 'naval-brass', 'bronze', 'gold'].includes(value)
  }
})

defineEmits(['toggle', 'open', 'close'])

// Metallic plaque header classes
const headerClasses = computed(() => {
  const base = 'steampunk-accordion-header-base metallic-plaque cursor-pointer select-none transition-all duration-200'

  const variants = {
    steel: 'border-nautical-700 text-nautical-50',
    'naval-brass': 'border-accent-700 text-accent-50',
    bronze: 'border-primary-800 text-primary-50',
    gold: 'border-gold-700 text-amber-950'
  }

  return `${base} ${variants[props.variant]}`
})

// Metallic plaque panel classes
const panelClasses = computed(() => {
  const base = 'steampunk-accordion-panel px-6 py-5 rounded-b-lg'

  const variants = {
    steel: 'bg-gradient-to-br from-nautical-50 via-slate-50 to-nautical-100 border-2 border-t-0 border-nautical-700 text-nautical-900',
    'naval-brass': 'bg-gradient-to-br from-accent-50 via-cyan-50 to-accent-100 border-2 border-t-0 border-accent-700 text-accent-900',
    bronze: 'bg-gradient-to-br from-primary-50 via-blue-50 to-primary-100 border-2 border-t-0 border-primary-800 text-primary-900',
    gold: 'bg-gradient-to-br from-amber-50 via-yellow-50 to-amber-100 border-2 border-t-0 border-gold-700 text-amber-950'
  }

  return `${base} ${variants[props.variant]}`
})
</script>

<style scoped>
/* Header styling */
.steampunk-accordion-header-base {
  @apply px-4 py-3 rounded-t-lg border-2;
}

/* Metallic plaque effect with inset shadows */
.metallic-plaque {
  box-shadow:
    inset 0 4px 8px rgba(255, 255, 255, 0.2),
    inset 0 -4px 8px rgba(0, 0, 0, 0.4),
    0 8px 32px rgba(0, 0, 0, 0.3);
}

/* Variant-specific metallic backgrounds */
.metallic-plaque.border-nautical-700 {
  background: linear-gradient(145deg, #475569 0%, #64748b 50%, #475569 100%);
}

.metallic-plaque.border-accent-700 {
  background: linear-gradient(145deg, #0891b2 0%, #06b6d4 50%, #0891b2 100%);
}

.metallic-plaque.border-primary-800 {
  background: linear-gradient(145deg, #1d4ed8 0%, #2563eb 50%, #1d4ed8 100%);
}

.metallic-plaque.border-gold-700 {
  background: linear-gradient(145deg, #ca8a04 0%, #eab308 50%, #ca8a04 100%);
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

/* Hover effect - enhance the top highlight */
.metallic-plaque:hover {
  box-shadow:
    inset 0 4px 8px rgba(255, 255, 255, 0.3),
    inset 0 -4px 8px rgba(0, 0, 0, 0.4),
    0 10px 40px rgba(0, 0, 0, 0.4);
}

/* Focus styles for accessibility */
.steampunk-accordion-header-base:focus {
  @apply outline-none ring-2 ring-accent-400 ring-offset-2;
}
</style>
