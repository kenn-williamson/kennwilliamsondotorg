<template>
  <BaseTooltip
    :text="text"
    :placement="placement"
    :show-delay="showDelay"
    :hide-delay="hideDelay"
    :clickable="clickable"
    :tooltip-classes="tooltipClasses"
  >
    <template #trigger>
      <!-- ALWAYS wrap slot content with styled span and icon -->
      <span class="steampunk-tooltip-trigger">
        <slot>{{ triggerText }}</slot><sup v-if="showIcon" class="tooltip-icon">ℹ</sup>
      </span>
    </template>

    <template #content>
      <slot name="content">
        {{ text }}
      </slot>
    </template>
  </BaseTooltip>
</template>

<script setup>
import { computed } from 'vue'
import BaseTooltip from '~/components/Base/BaseTooltip.vue'

const props = defineProps({
  text: {
    type: String,
    default: ''
  },
  triggerText: {
    type: String,
    default: ''
  },
  placement: {
    type: String,
    default: 'top'
  },
  showDelay: {
    type: Number,
    default: 200
  },
  hideDelay: {
    type: Number,
    default: 100
  },
  clickable: {
    type: Boolean,
    default: false
  },
  showIcon: {
    type: Boolean,
    default: true
  },
  variant: {
    type: String,
    default: 'steel', // 'steel', 'naval-brass', 'bronze', 'gold'
    validator: (value) => ['steel', 'naval-brass', 'bronze', 'gold'].includes(value)
  }
})

// Metallic plaque tooltip classes
const tooltipClasses = computed(() => {
  const base = 'steampunk-tooltip metallic-tooltip z-50 px-3 py-2 text-sm rounded shadow-lg max-w-xs leading-relaxed'

  const variants = {
    steel: 'border-nautical-700 text-white',
    'naval-brass': 'border-accent-700 text-white',
    bronze: 'border-primary-800 text-white',
    gold: 'border-gold-700 text-amber-950'
  }

  return `${base} ${variants[props.variant]}`
})
</script>

<style scoped>
/* Steampunk tooltip trigger styling */
.steampunk-tooltip-trigger {
  text-decoration: underline;
  text-decoration-color: #2563eb; /* primary-600 */
  text-decoration-thickness: 2px;
  text-decoration-style: solid;
  text-underline-offset: 3px;
  cursor: help;
  transition: all 0.2s ease;
  font-weight: 500;
  background: linear-gradient(to right, #dbeafe, transparent);
  padding: 0 0.25rem;
  margin: 0 -0.25rem;
  border-radius: 0.25rem;
  color: #1e40af; /* primary-800 for better visibility */
  display: inline;
}

.steampunk-tooltip-trigger:hover {
  color: #1d4ed8; /* primary-700 */
  text-decoration-color: #1d4ed8;
  background: linear-gradient(to right, #bfdbfe, #dbeafe);
  box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
}

/* Info icon styling */
.tooltip-icon {
  display: inline-block;
  font-size: 0.75em;
  margin-left: 0.15em;
  color: #2563eb; /* primary-600 */
  font-weight: 600;
  vertical-align: super;
  line-height: 0;
}

.steampunk-tooltip-trigger:hover .tooltip-icon {
  color: #1d4ed8; /* primary-700 */
}
</style>

<style>
/* UNSCOPED: Styles for teleported tooltip elements */

/* Metallic tooltip styling */
.steampunk-tooltip {
  border-width: 2px;
}

/* Metallic plaque effect for tooltips */
.metallic-tooltip {
  box-shadow:
    inset 0 2px 4px rgba(255, 255, 255, 0.2),
    inset 0 -2px 4px rgba(0, 0, 0, 0.4),
    0 4px 16px rgba(0, 0, 0, 0.3);
}

/* Variant-specific metallic backgrounds for tooltips */
.metallic-tooltip.border-nautical-700 {
  background:
    repeating-linear-gradient(
      45deg,
      rgba(255, 255, 255, 0.05),
      rgba(255, 255, 255, 0.05) 10px,
      rgba(0, 0, 0, 0.05) 10px,
      rgba(0, 0, 0, 0.05) 20px
    ),
    linear-gradient(145deg, #475569 0%, #64748b 50%, #475569 100%);
}

.metallic-tooltip.border-accent-700 {
  background:
    repeating-linear-gradient(
      45deg,
      rgba(255, 255, 255, 0.05),
      rgba(255, 255, 255, 0.05) 10px,
      rgba(0, 0, 0, 0.05) 10px,
      rgba(0, 0, 0, 0.05) 20px
    ),
    linear-gradient(145deg, #0891b2 0%, #06b6d4 50%, #0891b2 100%);
}

.metallic-tooltip.border-primary-800 {
  background:
    repeating-linear-gradient(
      45deg,
      rgba(255, 255, 255, 0.05),
      rgba(255, 255, 255, 0.05) 10px,
      rgba(0, 0, 0, 0.05) 10px,
      rgba(0, 0, 0, 0.05) 20px
    ),
    linear-gradient(145deg, #1d4ed8 0%, #2563eb 50%, #1d4ed8 100%);
}

.metallic-tooltip.border-gold-700 {
  background:
    repeating-linear-gradient(
      45deg,
      rgba(255, 255, 255, 0.05),
      rgba(255, 255, 255, 0.05) 10px,
      rgba(0, 0, 0, 0.05) 10px,
      rgba(0, 0, 0, 0.05) 20px
    ),
    linear-gradient(145deg, #eab308 0%, #fbbf24 50%, #eab308 100%);
}

/* Ensure tooltip content has proper spacing and doesn't clip */
.metallic-tooltip p:first-child {
  margin-top: 0;
}

.metallic-tooltip p:last-child {
  margin-bottom: 0;
}

/* Style links within metallic tooltips for readability on dark backgrounds */
.metallic-tooltip a {
  @apply text-accent-300 hover:text-accent-200 underline transition-colors;
}
</style>
