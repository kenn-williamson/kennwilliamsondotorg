<template>
  <div class="min-h-screen nautical-background">
    <!-- Steampunk Background -->
    <SteampunkBackground />

    <div class="relative z-10 max-w-4xl mx-auto px-4 py-8">
      <!-- Content Card -->
      <article class="bg-gradient-to-br from-nautical-50 via-primary-50 to-sky-50 border-2 border-primary-700 rounded-lg shadow-xl overflow-hidden">
        <!-- Decorative Header Border -->
        <div class="h-2 bg-gradient-to-r from-primary-600 via-indigo-600 to-primary-700"></div>

        <div class="p-6 sm:p-8 lg:p-12">
          <!-- Header -->
          <header class="mb-8">
            <div class="mb-4 flex items-center justify-between">
              <h1 class="text-4xl sm:text-5xl font-bold text-primary-900">Blog</h1>
              <div class="relative feed-menu">
                <button
                  type="button"
                  class="feed-link"
                  :aria-expanded="showFeedMenu"
                  aria-haspopup="menu"
                  aria-label="Subscribe via RSS, Atom, or JSON Feed"
                  title="Subscribe (RSS, Atom, JSON)"
                  @click="showFeedMenu = !showFeedMenu"
                >
                  <svg class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                    <path d="M6.18 15.64a2.18 2.18 0 0 1 2.18 2.18C8.36 19 7.38 20 6.18 20C5 20 4 19 4 17.82a2.18 2.18 0 0 1 2.18-2.18M4 4.44A15.56 15.56 0 0 1 19.56 20h-2.83A12.73 12.73 0 0 0 4 7.27V4.44m0 5.66a9.9 9.9 0 0 1 9.9 9.9h-2.83A7.07 7.07 0 0 0 4 12.93V10.1z"/>
                  </svg>
                  <svg class="w-3 h-3 ml-0.5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                    <path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 11.06l3.71-3.83a.75.75 0 011.08 1.04l-4.25 4.39a.75.75 0 01-1.08 0L5.21 8.27a.75.75 0 01.02-1.06z" clip-rule="evenodd" />
                  </svg>
                  <span class="sr-only">Subscribe</span>
                </button>
                <div
                  v-if="showFeedMenu"
                  role="menu"
                  class="absolute right-0 mt-2 w-64 bg-white rounded-md shadow-lg border border-nautical-200 py-1 z-50"
                >
                  <a
                    href="/feed/rss"
                    role="menuitem"
                    class="feed-menu-item"
                    @click="showFeedMenu = false"
                  >
                    <span class="feed-menu-title">RSS</span>
                    <span class="feed-menu-desc">Widest reader support</span>
                  </a>
                  <a
                    href="/feed/atom"
                    role="menuitem"
                    class="feed-menu-item"
                    @click="showFeedMenu = false"
                  >
                    <span class="feed-menu-title">Atom</span>
                    <span class="feed-menu-desc">Richer metadata</span>
                  </a>
                  <a
                    href="/feed/json"
                    role="menuitem"
                    class="feed-menu-item"
                    @click="showFeedMenu = false"
                  >
                    <span class="feed-menu-title">JSON Feed</span>
                    <span class="feed-menu-desc">Modern, easy to parse</span>
                  </a>
                </div>
              </div>
            </div>

            <p class="text-lg text-nautical-700">
              Thoughts on Christian Voluntarism, technology, and personal growth
            </p>
          </header>

          <!-- Search Bar -->
          <BlogSearchBar />

          <!-- Loading State -->
          <div v-if="pending" class="space-y-6">
            <div v-for="i in 3" :key="i" class="bg-white rounded-lg shadow-md p-6 animate-pulse border border-nautical-200">
              <div class="h-48 bg-nautical-200 rounded-md mb-4"></div>
              <div class="h-6 bg-nautical-200 rounded w-3/4 mb-2"></div>
              <div class="h-4 bg-nautical-200 rounded w-full mb-2"></div>
              <div class="h-4 bg-nautical-200 rounded w-5/6"></div>
            </div>
          </div>

          <!-- Error State -->
          <div v-else-if="error" class="bg-red-50 border-2 border-red-300 rounded-lg p-6 text-center">
            <svg class="w-12 h-12 text-red-500 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <h2 class="text-xl font-bold text-red-900 mb-2">Failed to load blog posts</h2>
            <p class="text-red-700">{{ error.message }}</p>
          </div>

          <!-- No Results -->
          <div v-else-if="!postsData?.posts || postsData.posts.length === 0" class="text-center py-12">
            <svg class="w-16 h-16 text-nautical-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            <h2 class="text-2xl font-bold text-nautical-800 mb-2">No posts yet</h2>
            <p class="text-nautical-600">Check back soon for new content!</p>
          </div>

          <!-- Posts Grid -->
          <div v-else class="space-y-6">
            <BlogPostCard
              v-for="post in postsData.posts"
              :key="post.id"
              :post="post"
            />

            <!-- Pagination -->
            <BlogPagination
              v-if="postsData.total_pages > 1"
              :current="postsData.page"
              :total-pages="postsData.total_pages"
            />
          </div>
        </div>
      </article>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useBlogStore } from '~/stores/blog'
import type { BlogPostList } from '#shared/types'

const route = useRoute()
const blogStore = useBlogStore()

const showFeedMenu = ref(false)

onMounted(() => {
  const handleClickOutside = (event: MouseEvent) => {
    const target = event.target as HTMLElement | null
    if (!target?.closest('.feed-menu')) {
      showFeedMenu.value = false
    }
  }
  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === 'Escape') showFeedMenu.value = false
  }
  document.addEventListener('click', handleClickOutside)
  document.addEventListener('keydown', handleKeydown)
  onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside)
    document.removeEventListener('keydown', handleKeydown)
  })
})

// Reactive query params
const page = computed(() => parseInt(route.query.page as string) || 1)
const tag = computed(() => route.query.tag as string)
const searchQuery = computed(() => route.query.q as string)

// SSR data fetching
await useAsyncData(
  `blog-posts-${page.value}-${tag.value || 'all'}-${searchQuery.value || ''}`,
  async () => {
    // Handle search vs filter
    if (searchQuery.value) {
      await blogStore.searchPosts(searchQuery.value, page.value)
    } else {
      await blogStore.loadPosts({
        page: page.value,
        limit: 10,
        tag: tag.value,
        status: 'published'
      })
    }
  },
  {
    watch: [page, tag, searchQuery],
    server: true,
    lazy: false
  }
)

// Use store's reactive state
const postsData = computed(() => ({
  posts: blogStore.posts,
  total: blogStore.totalPosts,
  page: blogStore.currentPage,
  total_pages: blogStore.totalPages
}))
const pending = computed(() => blogStore.isLoading)
const error = computed(() => blogStore.error ? { message: blogStore.error } : null)

// SEO Meta Tags
useHead({
  title: 'Blog',
  meta: [
    {
      name: 'description',
      content: 'Thoughts on Christian Voluntarism, technology, and personal growth from Kenn Williamson'
    }
  ]
})

// Social Share Meta Tags
const { useSocialShare } = await import('~/composables/useSocialShare')
useSocialShare({
  title: 'Blog - Kenn Williamson',
  description: 'Thoughts on Christian Voluntarism, technology, and personal growth',
  imageKey: 'blog'
})
</script>

<style scoped>
.nautical-background {
  background: linear-gradient(
    135deg,
    #0f172a 0%,      /* Nautical-900 - Deep navy */
    #1e293b 25%,     /* Nautical-800 - Slate */
    #334155 50%,     /* Nautical-700 - Steel */
    #1e293b 75%,     /* Nautical-800 - Back to slate */
    #0f172a 100%     /* Nautical-900 - Back to deep navy */
  );
}

.feed-link {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.5rem;
  border-radius: 0.375rem;
  color: #64748b; /* slate-500 */
  transition: all 0.2s ease;
}

.feed-link:hover {
  color: #f97316; /* orange-500 - RSS orange */
  background: rgba(249, 115, 22, 0.1);
}

.feed-menu-item {
  display: flex;
  flex-direction: column;
  padding: 0.5rem 1rem;
  text-decoration: none;
  color: #334155; /* nautical-700 */
  transition: background 0.15s ease;
}

.feed-menu-item:hover {
  background: #f1f5f9; /* nautical-100 */
}

.feed-menu-title {
  font-weight: 600;
  font-size: 0.875rem;
  color: #0f172a; /* nautical-900 */
}

.feed-menu-desc {
  font-size: 0.75rem;
  color: #64748b; /* slate-500 */
}
</style>
