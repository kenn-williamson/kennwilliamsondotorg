<template>
  <div class="public-timers-tab p-6">
    <div class="max-w-6xl mx-auto">
      <h2 class="text-2xl font-semibold text-gray-900 mb-6">
        Public Timers
      </h2>
      <p class="text-gray-600 mb-6">
        See who else is tracking their incident-free time publicly.
      </p>

      <!-- Reuse the public timer list display component -->
      <PublicTimerListDisplay />
    </div>
  </div>
</template>

<script setup>
// This tab simply wraps the PublicTimerListDisplay component
// No additional logic needed - the display component handles everything
</script>

<style scoped>
.public-timers-tab {
  min-height: 400px;
}
</style>
