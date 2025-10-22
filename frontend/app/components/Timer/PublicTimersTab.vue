<template>
  <div class="public-timers-tab p-6">
    <div class="max-w-6xl mx-auto">
      <h2 class="section-title text-2xl font-semibold mb-6">
        Public Timers
      </h2>
      <p class="section-description mb-6">
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
  background: linear-gradient(145deg, #f5f0e8 0%, #faf7f0 50%, #f5f0e8 100%);
  border-radius: 0 0 8px 8px;
  box-shadow: inset 0 2px 4px rgba(139, 69, 19, 0.1);
}

.section-title {
  color: #5d3820;
  font-family: Georgia, serif;
  text-shadow: 0 1px 1px rgba(255, 255, 255, 0.5);
}

.section-description {
  color: #8B6914;
}
</style>
