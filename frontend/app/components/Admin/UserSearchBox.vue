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
import { ref, watch } from 'vue'
import { useAdminStore } from '~/stores/admin'

const adminStore = useAdminStore()

// Local search query
const searchQuery = ref('')

// Handle search input with debouncing (handled by store)
const onSearchInput = () => {
  adminStore.searchUsers(searchQuery.value)
}

const clearSearch = () => {
  searchQuery.value = ''
  adminStore.searchUsers('')
}

// Sync local search query with store
watch(() => adminStore.searchQuery, (newQuery) => {
  searchQuery.value = newQuery
}, { immediate: true })
</script>

<style scoped>
.user-search-box {
  @apply mb-6;
}
</style>
