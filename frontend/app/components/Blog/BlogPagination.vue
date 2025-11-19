<template>
  <nav v-if="totalPages > 1" class="flex justify-center items-center gap-2 mt-8" aria-label="Pagination">
    <!-- Previous Button -->
    <NuxtLink
      v-if="current > 1"
      :to="buildPageUrl(current - 1)"
      class="px-4 py-2 rounded-lg bg-white text-nautical-900 hover:bg-primary-50 border border-nautical-300 transition-colors"
      aria-label="Previous page"
    >
      <span class="sr-only">Previous</span>
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
    </NuxtLink>
    <span v-else class="px-4 py-2 rounded-lg bg-nautical-100 text-nautical-400 border border-nautical-200 cursor-not-allowed">
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
    </span>

    <!-- Page Numbers -->
    <template v-for="page in displayedPages" :key="page">
      <span v-if="page === '...'" class="px-4 py-2 text-nautical-600">...</span>
      <NuxtLink
        v-else
        :to="buildPageUrl(Number(page))"
        :class="[
          'px-4 py-2 rounded-lg border transition-colors',
          Number(page) === current
            ? 'bg-primary-600 text-white border-primary-600 font-semibold'
            : 'bg-white text-nautical-900 hover:bg-primary-50 border-nautical-300'
        ]"
        :aria-label="`Page ${page}`"
        :aria-current="Number(page) === current ? 'page' : undefined"
      >
        {{ page }}
      </NuxtLink>
    </template>

    <!-- Next Button -->
    <NuxtLink
      v-if="current < totalPages"
      :to="buildPageUrl(current + 1)"
      class="px-4 py-2 rounded-lg bg-white text-nautical-900 hover:bg-primary-50 border border-nautical-300 transition-colors"
      aria-label="Next page"
    >
      <span class="sr-only">Next</span>
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </NuxtLink>
    <span v-else class="px-4 py-2 rounded-lg bg-nautical-100 text-nautical-400 border border-nautical-200 cursor-not-allowed">
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </span>
  </nav>
</template>

<script setup lang="ts">
const props = defineProps<{
  current: number
  totalPages: number
}>()

const route = useRoute()

/**
 * Build page URL preserving existing query params (like tag filter)
 */
const buildPageUrl = (page: number): string => {
  const query = { ...route.query, page: page.toString() }
  return { path: route.path, query } as any
}

/**
 * Calculate which page numbers to display
 * Shows: 1 ... 4 5 [6] 7 8 ... 20
 */
const displayedPages = computed(() => {
  const pages: (number | string)[] = []
  const { current, totalPages } = props

  // Always show first page
  pages.push(1)

  // Calculate range around current page
  const rangeStart = Math.max(2, current - 2)
  const rangeEnd = Math.min(totalPages - 1, current + 2)

  // Add ellipsis after first page if needed
  if (rangeStart > 2) {
    pages.push('...')
  }

  // Add pages around current
  for (let i = rangeStart; i <= rangeEnd; i++) {
    pages.push(i)
  }

  // Add ellipsis before last page if needed
  if (rangeEnd < totalPages - 1) {
    pages.push('...')
  }

  // Always show last page if there is more than one page
  if (totalPages > 1) {
    pages.push(totalPages)
  }

  return pages
})
</script>
