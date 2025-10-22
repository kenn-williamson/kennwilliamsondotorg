/**
 * useTooltip - Composable for tooltip functionality
 *
 * Provides positioning, show/hide logic, and ARIA attributes
 * for accessible tooltips.
 */

import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useFloating, offset, flip, shift } from '@floating-ui/vue'

export function useTooltip(options = {}) {
  const {
    placement = 'top',
    offsetDistance = 8,
    showDelay = 200,
    hideDelay = 100,
    clickable = false, // Whether tooltip can be clicked to stay open
  } = options

  // Refs for positioning
  const reference = ref(null)
  const floating = ref(null)
  const isOpen = ref(false)
  const isPinned = ref(false) // Pinned via click

  // Timeout refs for delayed show/hide
  let showTimeout = null
  let hideTimeout = null

  // Generate unique ID for ARIA
  const tooltipId = ref(`tooltip-${Math.random().toString(36).substr(2, 9)}`)

  // Floating UI positioning
  const { floatingStyles } = useFloating(reference, floating, {
    placement,
    middleware: [
      offset(offsetDistance),
      flip(),
      shift({ padding: 10 }) // Prevent viewport clipping
    ]
  })

  // Show with delay
  function show() {
    if (isPinned.value) return // Don't auto-hide if pinned
    clearTimeout(hideTimeout)
    showTimeout = setTimeout(() => {
      isOpen.value = true
    }, showDelay)
  }

  // Hide with delay
  function hide() {
    if (isPinned.value) return // Don't auto-hide if pinned
    clearTimeout(showTimeout)
    hideTimeout = setTimeout(() => {
      isOpen.value = false
    }, hideDelay)
  }

  // Immediate show/hide (for programmatic control)
  function showImmediate() {
    clearTimeout(hideTimeout)
    clearTimeout(showTimeout)
    isOpen.value = true
  }

  function hideImmediate() {
    clearTimeout(showTimeout)
    clearTimeout(hideTimeout)
    isOpen.value = false
    isPinned.value = false
  }

  // Toggle pinned state (for click-to-stay-open)
  function togglePin() {
    if (!clickable) return
    isPinned.value = !isPinned.value
    if (isPinned.value) {
      showImmediate()
    } else {
      hideImmediate()
    }
  }

  // Handle clicking outside to close pinned tooltip
  function handleClickOutside(event) {
    if (!isPinned.value || !clickable) return

    const refEl = reference.value
    const floatEl = floating.value

    if (refEl && floatEl) {
      const clickedReference = refEl.contains(event.target)
      const clickedFloating = floatEl.contains(event.target)

      if (!clickedReference && !clickedFloating) {
        hideImmediate()
      }
    }
  }

  // Setup click-outside listener
  onMounted(() => {
    if (clickable) {
      document.addEventListener('click', handleClickOutside)
    }
  })

  onUnmounted(() => {
    if (clickable) {
      document.removeEventListener('click', handleClickOutside)
    }
  })

  // ARIA attributes for the trigger element
  const triggerAttrs = computed(() => ({
    'aria-describedby': isOpen.value ? tooltipId.value : undefined,
  }))

  // ARIA attributes for the tooltip element
  const tooltipAttrs = computed(() => ({
    role: 'tooltip',
    id: tooltipId.value,
  }))

  // Cleanup on unmount
  onUnmounted(() => {
    clearTimeout(showTimeout)
    clearTimeout(hideTimeout)
  })

  return {
    // Refs
    reference,
    floating,
    isOpen,
    isPinned,

    // Computed
    floatingStyles,
    triggerAttrs,
    tooltipAttrs,

    // Methods
    show,
    hide,
    showImmediate,
    hideImmediate,
    togglePin,
  }
}
