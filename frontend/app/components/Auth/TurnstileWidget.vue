<template>
  <div ref="widgetContainer" class="turnstile-widget"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const widgetContainer = ref<HTMLElement | null>(null)
const widgetId = ref<string | null>(null)

// Expose method to get the CAPTCHA token
const getToken = (): string | null => {
  if (!window.turnstile || !widgetId.value) {
    console.warn('Turnstile not loaded or widget not rendered')
    return null
  }

  try {
    return window.turnstile.getResponse(widgetId.value)
  } catch (error) {
    console.error('Failed to get Turnstile token:', error)
    return null
  }
}

// Expose method to reset the widget
const reset = (): void => {
  if (window.turnstile && widgetId.value) {
    window.turnstile.reset(widgetId.value)
  }
}

// Get the site key from runtime config
const config = useRuntimeConfig()
const siteKey = config.public.turnstileSiteKey

onMounted(() => {
  // Wait for Turnstile SDK to load
  const initTurnstile = () => {
    if (!window.turnstile) {
      console.error('Turnstile SDK not loaded')
      return
    }

    if (!widgetContainer.value) {
      console.error('Widget container not found')
      return
    }

    try {
      // Render the Turnstile widget (non-interactive mode)
      widgetId.value = window.turnstile.render(widgetContainer.value, {
        sitekey: siteKey,
        theme: 'light',
        size: 'compact', // compact (150×140px), normal (300×65px), or flexible (100% width)
        appearance: 'interaction-only', // Only show widget if user interaction is required
        callback: (_token: string) => {
          console.debug('Turnstile token generated')
        },
        'error-callback': () => {
          console.error('Turnstile verification failed')
        },
        'expired-callback': () => {
          console.warn('Turnstile token expired')
        }
      }) as string
    } catch (error) {
      console.error('Failed to render Turnstile widget:', error)
    }
  }

  // Check if Turnstile is already loaded
  if (window.turnstile) {
    initTurnstile()
  } else {
    // Wait for Turnstile to load
    const checkInterval = setInterval(() => {
      if (window.turnstile) {
        clearInterval(checkInterval)
        initTurnstile()
      }
    }, 100)

    // Cleanup after 10 seconds
    setTimeout(() => {
      clearInterval(checkInterval)
      if (!window.turnstile) {
        console.error('Turnstile SDK failed to load within 10 seconds')
      }
    }, 10000)
  }
})

onUnmounted(() => {
  // Clean up the widget
  if (window.turnstile && widgetId.value) {
    try {
      window.turnstile.remove(widgetId.value)
    } catch (error) {
      console.error('Failed to remove Turnstile widget:', error)
    }
  }
})

// Expose methods to parent component
defineExpose({
  getToken,
  reset
})
</script>

<style scoped>
.turnstile-widget {
  display: flex;
  justify-content: center;
  margin: 1rem 0;
}
</style>
