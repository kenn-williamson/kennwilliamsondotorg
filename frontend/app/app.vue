<template>
  <div id="app" class="min-h-screen bg-gradient-to-br from-slate-50 to-sky-50 flex flex-col">
    <NuxtRouteAnnouncer />
    <AppHeader />
    <main class="flex-grow">
      <NuxtPage />
    </main>
    <AppFooter />
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
    { rel: 'apple-touch-icon', sizes: '192x192', href: '/favicon-large.png' },
    // Feed autodiscovery links
    { rel: 'alternate', type: 'application/rss+xml', title: 'KennWilliamson.org RSS Feed', href: '/feed/rss' },
    { rel: 'alternate', type: 'application/atom+xml', title: 'KennWilliamson.org Atom Feed', href: '/feed/atom' },
    { rel: 'alternate', type: 'application/feed+json', title: 'KennWilliamson.org JSON Feed', href: '/feed/json' }
  ]
})

// Set up global callOnce cache clearing on logout
useCallOnceWatcher()
</script>

<style>
/* Global styles will be handled by TailwindCSS */
</style>