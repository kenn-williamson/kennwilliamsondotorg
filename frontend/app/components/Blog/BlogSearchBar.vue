<template>
  <div class="w-full max-w-2xl mx-auto mb-8">
    <form @submit.prevent="handleSearch" class="relative">
      <input
        v-model="searchQuery"
        type="search"
        placeholder="Search blog posts..."
        class="w-full px-4 py-3 pl-12 pr-4 text-nautical-900 bg-white border border-nautical-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent shadow-sm"
        aria-label="Search blog posts"
      />
      <svg
        class="absolute left-4 top-1/2 transform -translate-y-1/2 w-5 h-5 text-nautical-500 pointer-events-none"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
    </form>

    <!-- Search suggestions or recent searches could go here -->
  </div>
</template>

<script setup lang="ts">
const searchQuery = ref('')
const router = useRouter()

const handleSearch = () => {
  const query = searchQuery.value.trim()

  if (query) {
    // Navigate to search results with query parameter
    router.push({
      path: '/blog',
      query: { q: query }
    })
  }
}

// Optional: Auto-search with debounce
// import { useDebounceFn } from '@vueuse/core'
// const debouncedSearch = useDebounceFn(handleSearch, 500)
// watch(searchQuery, () => {
//   if (searchQuery.value.length >= 3) {
//     debouncedSearch()
//   }
// })
</script>
