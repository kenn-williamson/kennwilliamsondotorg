<template>
  <NuxtLink
    :to="`/blog/${post.slug}`"
    class="block bg-white border border-nautical-200 rounded-lg shadow-md hover:shadow-lg hover:border-primary-400 transition-all duration-200 overflow-hidden"
  >
    <!-- Featured Image -->
    <img
      v-if="post.featured_image_url"
      :src="post.featured_image_url"
      :alt="post.featured_image_alt || post.title"
      class="w-full max-h-96 object-contain bg-nautical-50"
    />

    <!-- Content -->
    <div class="p-6">
      <h2 class="text-2xl font-bold text-primary-900 mb-2 hover:text-primary-700 transition-colors">
        {{ post.title }}
      </h2>

      <p v-if="post.excerpt" class="text-nautical-800 mb-4 line-clamp-3">
        {{ post.excerpt }}
      </p>

      <!-- Meta Info -->
      <div class="flex items-center justify-between text-sm text-nautical-600">
        <time v-if="post.published_at" :datetime="post.published_at">
          {{ formatDate(post.published_at) }}
        </time>

        <!-- Tags -->
        <div v-if="post.tags && post.tags.length > 0" class="flex gap-2 flex-wrap">
          <span
            v-for="tag in post.tags.slice(0, 3)"
            :key="tag"
            class="text-primary-600 font-medium"
          >
            #{{ tag }}
          </span>
        </div>
      </div>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import type { BlogPost } from '#shared/types'

defineProps<{
  post: BlogPost
}>()

const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  })
}
</script>

<style scoped>
.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
