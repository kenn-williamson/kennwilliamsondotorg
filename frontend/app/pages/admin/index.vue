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

// Fetch admin stats using store
// Note: Tab state is now managed by useTabs composable in child components
const stats = await adminStore.fetchStats()

onMounted(() => {
  console.log('Admin page mounted with stats:', stats)
})
</script>

<style scoped>
/* Additional admin-specific styles can be added here */
</style>
