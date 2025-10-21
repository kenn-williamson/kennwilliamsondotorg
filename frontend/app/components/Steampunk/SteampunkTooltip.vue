<template>
  <BaseTooltip
    :text="text"
    :placement="placement"
    :show-delay="showDelay"
    :hide-delay="hideDelay"
    :show-arrow="showArrow"
    :clickable="clickable"
    :tooltip-classes="tooltipClasses"
    :arrow-classes="arrowClasses"
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
  showArrow: {
    type: Boolean,
    default: true
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
    default: 'brass', // 'brass', 'parchment', 'copper'
    validator: (value) => ['brass', 'parchment', 'copper'].includes(value)
  }
})

// Steampunk-themed tooltip classes
const tooltipClasses = computed(() => {
  const base = 'steampunk-tooltip z-50 px-3 py-2 text-sm rounded shadow-lg max-w-xs'

  const variants = {
    brass: 'bg-gradient-to-br from-amber-100 to-yellow-100 text-amber-900 border-2 border-amber-600',
    parchment: 'bg-gradient-to-br from-amber-50 to-orange-50 text-gray-800 border-2 border-amber-400',
    copper: 'bg-gradient-to-br from-orange-100 to-red-100 text-red-900 border-2 border-orange-600'
  }

  return `${base} ${variants[props.variant]}`
})

// Arrow classes to match tooltip variant
const arrowClasses = computed(() => {
  const base = 'steampunk-arrow w-2 h-2 rotate-45'

  const variants = {
    brass: 'bg-amber-100 border-amber-600',
    parchment: 'bg-amber-50 border-amber-400',
    copper: 'bg-orange-100 border-orange-600'
  }

  return `${base} ${variants[props.variant]}`
})
</script>

<style scoped>
/* Steampunk tooltip trigger styling */
.steampunk-tooltip-trigger {
  text-decoration: underline;
  text-decoration-color: #d97706; /* amber-600 */
  text-decoration-thickness: 2px;
  text-decoration-style: solid;
  text-underline-offset: 3px;
  cursor: help;
  transition: all 0.2s ease;
  font-weight: 500;
  background: linear-gradient(to right, #fffbeb, transparent);
  padding: 0 0.25rem;
  margin: 0 -0.25rem;
  border-radius: 0.25rem;
  color: #92400e; /* amber-800 for better visibility */
  display: inline;
}

.steampunk-tooltip-trigger:hover {
  color: #b45309; /* amber-700 */
  text-decoration-color: #b45309;
  background: linear-gradient(to right, #fef3c7, #fffbeb);
  box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
}

/* Info icon styling */
.tooltip-icon {
  display: inline-block;
  font-size: 0.75em;
  margin-left: 0.15em;
  color: #d97706; /* amber-600 */
  font-weight: 600;
  vertical-align: super;
  line-height: 0;
}

.steampunk-tooltip-trigger:hover .tooltip-icon {
  color: #b45309; /* amber-700 */
}

/* Tooltip content styling */
:deep(.steampunk-tooltip) {
  /* Add subtle texture */
  background-image:
    repeating-linear-gradient(
      45deg,
      transparent,
      transparent 10px,
      rgba(0, 0, 0, 0.02) 10px,
      rgba(0, 0, 0, 0.02) 20px
    );
}

/* Arrow positioning and styling */
:deep(.steampunk-arrow) {
  position: absolute;
  border-left: 1px solid currentColor;
  border-top: 1px solid currentColor;
}
</style>
