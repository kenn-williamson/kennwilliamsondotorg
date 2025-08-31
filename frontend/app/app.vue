<template>
  <div id="app" class="min-h-screen bg-gradient-to-br from-slate-50 to-sky-50">
    <NuxtRouteAnnouncer />
    <AppHeader />
    <main class="min-h-screen">
      <NuxtPage />
    </main>
  </div>
</template>

<script setup>
// Global app setup
useHead({
  titleTemplate: '%s - KennWilliamson.org',
  meta: [
    { name: 'description', content: 'Personal website of Kenn Williamson - A man of the margins exploring truth, beauty, and love through technology and tradition.' },
    { name: 'viewport', content: 'width=device-width, initial-scale=1' },
  ],
})
</script>

<style>
/* Global styles will be handled by TailwindCSS */
</style>