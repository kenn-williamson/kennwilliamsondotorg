<template>
  <div class="base-accordion">
    <!-- Header/Trigger -->
    <div
      v-bind="headerAttrs"
      :class="headerClasses"
      @click="toggle"
      @keydown="handleKeydown"
    >
      <slot name="header" :is-open="isOpen">
        <!-- Default header slot -->
        <div class="flex items-center justify-between">
          <span>{{ title }}</span>
          <span :class="{ 'rotate-180': isOpen }" class="transition-transform">
            ▼
          </span>
        </div>
      </slot>
    </div>

    <!-- Content Panel -->
    <Transition
      name="accordion-slide"
      @enter="onEnter"
      @after-enter="onAfterEnter"
      @leave="onLeave"
    >
      <div
        v-if="isOpen"
        v-bind="panelAttrs"
        :class="panelClasses"
      >
        <slot name="content">
          <!-- Default content slot -->
          <div>{{ content }}</div>
        </slot>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { useAccordion } from '~/composables/useAccordion'

const props = defineProps({
  title: {
    type: String,
    default: ''
  },
  content: {
    type: String,
    default: ''
  },
  initiallyOpen: {
    type: Boolean,
    default: false
  },
  headerClasses: {
    type: String,
    default: ''
  },
  panelClasses: {
    type: String,
    default: ''
  }
})

const emit = defineEmits(['toggle', 'open', 'close'])

// Use the accordion composable
const {
  isOpen,
  headerAttrs,
  panelAttrs,
  toggle: toggleAccordion,
  open: openAccordion,
  close: closeAccordion,
  handleKeydown,
  animationDuration,
} = useAccordion({
  initiallyOpen: props.initiallyOpen
})

// Emit events when state changes
function toggle() {
  toggleAccordion()
  emit('toggle', isOpen.value)
  if (isOpen.value) {
    emit('open')
  } else {
    emit('close')
  }
}

// Animation hooks for smooth height transitions
function onEnter(el) {
  el.style.height = '0'
  el.style.overflow = 'hidden'
}

function onAfterEnter(el) {
  el.style.height = 'auto'
  el.style.overflow = ''
}

function onLeave(el) {
  el.style.height = `${el.scrollHeight}px`
  el.style.overflow = 'hidden'
  // Force reflow
  el.offsetHeight
  el.style.height = '0'
}

// Expose methods to parent
defineExpose({
  toggle,
  open: openAccordion,
  close: closeAccordion,
  isOpen,
})
</script>

<style scoped>
/* Slide down/up animation */
.accordion-slide-enter-active,
.accordion-slide-leave-active {
  transition: height 0.3s ease;
}

.accordion-slide-enter-from,
.accordion-slide-leave-to {
  height: 0;
  overflow: hidden;
}
</style>
