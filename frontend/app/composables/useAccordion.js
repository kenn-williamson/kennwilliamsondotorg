/**
 * useAccordion - Composable for accordion/expandable section functionality
 *
 * Provides expand/collapse logic, ARIA attributes, and keyboard navigation
 * for accessible accordion components.
 */

import { ref, computed } from 'vue'

export function useAccordion(options = {}) {
  const {
    initiallyOpen = false,
    animationDuration = 300,
  } = options

  // State
  const isOpen = ref(initiallyOpen)

  // Generate unique ID for ARIA
  const accordionId = ref(`accordion-${Math.random().toString(36).substr(2, 9)}`)
  const headerId = computed(() => `${accordionId.value}-header`)
  const panelId = computed(() => `${accordionId.value}-panel`)

  // Toggle function
  function toggle() {
    isOpen.value = !isOpen.value
  }

  // Explicit open/close
  function open() {
    isOpen.value = true
  }

  function close() {
    isOpen.value = false
  }

  // Keyboard handler for accessibility
  function handleKeydown(event) {
    // Space or Enter to toggle
    if (event.key === ' ' || event.key === 'Enter') {
      event.preventDefault()
      toggle()
    }
    // Home/End for first/last item (if part of accordion group)
    // Can be extended for multi-item accordions
  }

  // ARIA attributes for the button/header
  const headerAttrs = computed(() => ({
    id: headerId.value,
    'aria-expanded': isOpen.value,
    'aria-controls': panelId.value,
    role: 'button',
    tabindex: 0,
  }))

  // ARIA attributes for the panel/content
  const panelAttrs = computed(() => ({
    id: panelId.value,
    role: 'region',
    'aria-labelledby': headerId.value,
  }))

  return {
    // State
    isOpen,
    accordionId,

    // Computed
    headerId,
    panelId,
    headerAttrs,
    panelAttrs,

    // Methods
    toggle,
    open,
    close,
    handleKeydown,

    // Config
    animationDuration,
  }
}
