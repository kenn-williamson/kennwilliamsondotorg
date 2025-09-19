<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 via-gray-50 to-gray-100">
    <div class="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <AdminPanel />
    </div>
  </div>
</template>

<script setup>
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
</script>

<style scoped>
/* Additional admin-specific styles can be added here */
</style>
