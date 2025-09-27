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
  link: [
    { rel: 'icon', type: 'image/png', sizes: '32x32', href: '/favicon-small.png' },
    { rel: 'icon', type: 'image/png', sizes: '192x192', href: '/favicon-large.png' },
    { rel: 'apple-touch-icon', sizes: '192x192', href: '/favicon-large.png' }
  ]
})

// Set up global callOnce cache clearing on logout
useCallOnceWatcher()
</script>

<style>
/* Global styles will be handled by TailwindCSS */
</style>