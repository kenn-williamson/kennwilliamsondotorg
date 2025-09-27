<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 via-gray-50 to-gray-100">
    <div class="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <AdminPanel />
    </div>
  </div>
</template>

<script setup>
import { useAdminStore } from '~/stores/admin'

// Apply admin middleware
definePageMeta({
  middleware: 'admin'
})

// Page meta
useHead({
  title: 'Admin Panel',
  meta: [
    { name: 'description', content: 'Admin panel for managing users, moderating content, and viewing system statistics.' }
  ]
})

// Store for hydration
const adminStore = useAdminStore()

// SSR: Get route query parameters
const route = useRoute()
const tabParam = route.query.tab

// SSR: Set active tab from query parameter
if (tabParam && typeof tabParam === 'string' && ['overview', 'users', 'suggestions'].includes(tabParam)) {
  adminStore.setActiveTab(tabParam)
}

// SSR: Fetch admin stats using store
const stats = await adminStore.fetchStatsSSR()

onMounted(() => {
  console.log('Admin page mounted with stats:', stats)
})
</script>

<style scoped>
/* Additional admin-specific styles can be added here */
</style>
