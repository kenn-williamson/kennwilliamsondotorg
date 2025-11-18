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
          <!-- Back to Blog -->
          <NuxtLink
            to="/blog"
            class="inline-flex items-center gap-2 text-primary-700 hover:text-primary-900 mb-8 transition-colors font-medium"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
            </svg>
            <span>Back to Blog</span>
          </NuxtLink>

          <!-- Header with Title -->
          <header class="mb-6">
            <h1 class="text-4xl sm:text-5xl font-bold text-primary-900 mb-4 pb-3 border-b-2 border-primary-300">
              {{ post.title }}
            </h1>

            <div class="flex flex-col sm:flex-row sm:items-center gap-4 text-nautical-700">
              <!-- Date Info -->
              <div class="flex items-center gap-2">
                <time v-if="post.published_at" :datetime="post.published_at" class="font-medium">
                  {{ formatDate(post.published_at) }}
                </time>
                <time
                  v-if="wasEdited"
                  :datetime="post.updated_at"
                  class="text-sm italic text-nautical-600"
                >
                  • Updated {{ formatDate(post.updated_at) }}
                </time>
              </div>

              <!-- Tags -->
              <div v-if="post.tags && post.tags.length > 0" class="flex gap-2 flex-wrap">
                <NuxtLink
                  v-for="tag in post.tags"
                  :key="tag"
                  :to="`/blog?tag=${tag}`"
                  class="px-3 py-1 bg-primary-100 text-primary-700 border border-primary-300 rounded-full text-sm font-medium hover:bg-primary-200 transition-colors"
                >
                  #{{ tag }}
                </NuxtLink>
              </div>
            </div>
          </header>

          <!-- Featured Image -->
          <img
            v-if="post.featured_image_url"
            :src="post.featured_image_url"
            :alt="post.featured_image_alt || post.title"
            class="w-full rounded-lg shadow-lg mb-8 max-h-96 object-cover"
          />

          <!-- Content -->
          <div class="prose prose-lg max-w-none mb-8">
            <BlogPostContent :markdown="post.content" />
          </div>

          <!-- Footer: Share Buttons -->
          <footer class="border-t-2 border-primary-300 pt-6 mt-8">
            <BlogShareButtons
              :title="post.title"
              :url="postUrl"
              :post-id="post.id"
              :slug="post.slug"
            />
          </footer>
        </div>
      </article>
    </div>
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useBlogStore } from '~/stores/blog'

const route = useRoute()
const slug = route.params.slug as string
const blogStore = useBlogStore()

// SSR data fetching - populate store
await useAsyncData(
  `blog-post-${slug}`,
  () => blogStore.loadPostBySlug(slug),
  {
    server: true,
    lazy: false
  }
)

// Use storeToRefs to properly unwrap the reactive ref
const { currentPost } = storeToRefs(blogStore)

// Handle 404 - check if post loaded
if (!currentPost.value) {
  throw createError({
    statusCode: 404,
    message: 'Blog post not found',
    fatal: true
  })
}

// At this point, TypeScript knows currentPost.value is not null
// Create a non-null ref for the template
const post = computed(() => currentPost.value!)

// Check if post was edited after publication
const wasEdited = computed(() => {
  const p = currentPost.value
  if (!p || !p.published_at || !p.updated_at) return false

  const publishTime = new Date(p.published_at).getTime()
  const updateTime = new Date(p.updated_at).getTime()

  // Consider "edited" if updated more than 1 hour after publish
  return (updateTime - publishTime) > (60 * 60 * 1000)
})

// Full post URL for sharing
const postUrl = computed(() => {
  return `https://kennwilliamson.org/blog/${currentPost.value?.slug || ''}`
})

// Format date helper
const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  })
}

// SEO Meta Tags
useHead({
  title: post.value.title
})

// Social Share Meta Tags
const { useSocialShare } = await import('~/composables/useSocialShare')
useSocialShare({
  title: post.value.title,
  description: post.value.meta_description || post.value.excerpt || '',
  customImage: post.value.featured_image_url || undefined
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
</style>
