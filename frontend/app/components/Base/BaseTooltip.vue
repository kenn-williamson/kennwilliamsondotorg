<template>
  <div class="relative inline-block">
    <!-- Trigger element (what user hovers over) -->
    <div
      ref="reference"
      v-bind="triggerAttrs"
      @mouseenter="show"
      @mouseleave="hide"
      @focus="showImmediate"
      @blur="hideImmediate"
      @click="handleTriggerClick"
    >
      <slot name="trigger">
        <!-- Default trigger slot -->
      </slot>
    </div>

    <!-- Tooltip portal (rendered at body level) -->
    <Teleport to="body">
      <Transition name="tooltip-fade">
        <div
          v-if="isOpen"
          ref="floating"
          v-bind="tooltipAttrs"
          :style="floatingStyles"
          :class="tooltipClasses"
          @mouseenter="handleTooltipMouseEnter"
          @mouseleave="handleTooltipMouseLeave"
        >
          <!-- Tooltip content -->
          <slot name="content">
            <div v-if="text">{{ text }}</div>
          </slot>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup>
import { useTooltip } from '~/composables/useTooltip'

const props = defineProps({
  text: {
    type: String,
    default: ''
  },
  placement: {
    type: String,
    default: 'top',
    validator: (value) => ['top', 'bottom', 'left', 'right'].includes(value)
  },
  showDelay: {
    type: Number,
    default: 200
  },
  hideDelay: {
    type: Number,
    default: 100
  },
  tooltipClasses: {
    type: String,
    default: ''
  },
  clickable: {
    type: Boolean,
    default: false
  }
})

// Use the tooltip composable
const {
  reference,
  floating,
  isOpen,
  isPinned,
  floatingStyles,
  arrowStyles,
  triggerAttrs,
  tooltipAttrs,
  show,
  hide,
  showImmediate,
  hideImmediate,
  togglePin,
} = useTooltip({
  placement: props.placement,
  showDelay: props.showDelay,
  hideDelay: props.hideDelay,
  clickable: props.clickable,
})

// Handle trigger click for clickable tooltips
function handleTriggerClick(event) {
  if (props.clickable) {
    event.stopPropagation()
    togglePin()
  }
}

// Keep tooltip open when hovering over it (for clickable tooltips with links)
function handleTooltipMouseEnter() {
  if (props.clickable && !isPinned.value) {
    showImmediate()
  }
}

function handleTooltipMouseLeave() {
  if (props.clickable && !isPinned.value) {
    hide()
  }
}
</script>

<style scoped>
/* Fade transition for tooltip */
.tooltip-fade-enter-active,
.tooltip-fade-leave-active {
  transition: opacity 0.15s ease;
}

.tooltip-fade-enter-from,
.tooltip-fade-leave-to {
  opacity: 0;
}
</style>
