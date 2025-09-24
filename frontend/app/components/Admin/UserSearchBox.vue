<template>
  <div class="user-search-box">
    <div class="relative">
      <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
        <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
      </div>
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search users by name or email..."
        class="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-gray-900 focus:border-gray-900 sm:text-sm"
        @input="onSearchInput"
      />
      <div v-if="searchQuery" class="absolute inset-y-0 right-0 pr-3 flex items-center">
        <button
          @click="clearSearch"
          class="text-gray-400 hover:text-gray-600"
        >
          <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAdminStore } from '~/stores/admin'

const adminStore = useAdminStore()

// Local search query
const searchQuery = ref('')

// Debounced search function
let searchTimeout: NodeJS.Timeout | null = null

const onSearchInput = () => {
  // Clear existing timeout
  if (searchTimeout) {
    clearTimeout(searchTimeout)
  }

  // Set new timeout for debounced search
  searchTimeout = setTimeout(async () => {
    adminStore.setSearchQuery(searchQuery.value)
    await adminStore.fetchUsers(searchQuery.value)
  }, 300) // 300ms debounce
}

const clearSearch = () => {
  searchQuery.value = ''
  adminStore.setSearchQuery('')
  adminStore.fetchUsers('')
}

// Watch for external search query changes
watch(() => adminStore.searchQuery, (newQuery) => {
  if (newQuery !== searchQuery.value) {
    searchQuery.value = newQuery
  }
})

// Cleanup timeout on unmount
onUnmounted(() => {
  if (searchTimeout) {
    clearTimeout(searchTimeout)
  }
})
</script>

<style scoped>
.user-search-box {
  @apply mb-6;
}
</style>
