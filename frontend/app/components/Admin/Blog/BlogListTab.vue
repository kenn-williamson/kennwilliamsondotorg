<template>
  <div class="space-y-6">
    <!-- Header with Create Button -->
    <div class="flex justify-between items-center">
      <h2 class="text-2xl font-bold text-nautical-900">Blog Posts</h2>
      <button
        @click="$emit('create-new')"
        class="px-4 py-2 bg-sky-600 text-white rounded-md hover:bg-sky-700 transition-colors font-medium flex items-center gap-2"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        Create New Post
      </button>
    </div>

    <!-- Filters -->
    <div class="flex gap-4 items-center">
      <select
        v-model="statusFilter"
        class="px-4 py-2 border border-nautical-300 rounded-md focus:ring-2 focus:ring-sky-500 focus:border-transparent"
      >
        <option value="all">All Status</option>
        <option value="published">Published</option>
        <option value="draft">Draft</option>
      </select>
    </div>

    <!-- Loading State -->
    <div v-if="blogStore.isLoading" class="text-center py-12">
      <svg class="animate-spin h-8 w-8 text-sky-600 mx-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
      <p class="text-nautical-600 mt-4">Loading posts...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="blogStore.hasError" class="bg-red-50 border border-red-300 rounded-lg p-6">
      <p class="text-red-700">{{ blogStore.error }}</p>
    </div>

    <!-- Empty State -->
    <div v-else-if="!posts.length" class="text-center py-12">
      <svg class="w-16 h-16 text-nautical-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
      </svg>
      <h3 class="text-xl font-bold text-nautical-700 mb-2">No posts yet</h3>
      <p class="text-nautical-600 mb-4">Get started by creating your first blog post!</p>
      <button
        @click="$emit('create-new')"
        class="px-6 py-3 bg-sky-600 text-white rounded-md hover:bg-sky-700 transition-colors font-medium"
      >
        Create First Post
      </button>
    </div>

    <!-- Posts Table -->
    <div v-else class="bg-white rounded-lg shadow overflow-hidden">
      <table class="min-w-full divide-y divide-nautical-200">
        <thead class="bg-nautical-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-nautical-700 uppercase tracking-wider">
              Title
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-nautical-700 uppercase tracking-wider">
              Status
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-nautical-700 uppercase tracking-wider">
              Tags
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-nautical-700 uppercase tracking-wider">
              Published
            </th>
            <th class="px-6 py-3 text-right text-xs font-medium text-nautical-700 uppercase tracking-wider">
              Actions
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-nautical-200">
          <tr v-for="post in posts" :key="post.id" class="hover:bg-nautical-50">
            <td class="px-6 py-4">
              <div class="text-sm font-medium text-nautical-900">{{ post.title }}</div>
              <div class="text-sm text-nautical-500 truncate max-w-md">{{ post.excerpt || post.content.substring(0, 100) + '...' }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span
                :class="[
                  'px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full',
                  post.status === 'published' ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800'
                ]"
              >
                {{ post.status }}
              </span>
            </td>
            <td class="px-6 py-4">
              <div class="flex flex-wrap gap-1">
                <span
                  v-for="tag in post.tags.slice(0, 3)"
                  :key="tag"
                  class="px-2 py-1 text-xs bg-sky-100 text-sky-700 rounded-full"
                >
                  {{ tag }}
                </span>
                <span v-if="post.tags.length > 3" class="px-2 py-1 text-xs text-nautical-500">
                  +{{ post.tags.length - 3 }}
                </span>
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-nautical-500">
              {{ post.published_at ? formatDate(post.published_at) : 'Not published' }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <button
                @click="$emit('edit-post', post)"
                class="text-sky-600 hover:text-sky-900 mr-4"
              >
                Edit
              </button>
              <button
                @click="handleDelete(post)"
                class="text-red-600 hover:text-red-900"
                :disabled="deletingId === post.id"
              >
                {{ deletingId === post.id ? 'Deleting...' : 'Delete' }}
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Pagination -->
    <BlogPagination
      v-if="blogStore.totalPages > 1"
      :current="blogStore.currentPage"
      :total-pages="blogStore.totalPages"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useBlogStore } from '~/stores/blog'
import type { BlogPost } from '#shared/types'

// Emits
const emit = defineEmits<{
  'create-new': []
  'edit-post': [post: BlogPost]
}>()

// State
const statusFilter = ref<'all' | 'draft' | 'published'>('all')
const deletingId = ref<string | null>(null)

// Store
const blogStore = useBlogStore()

// Computed
const posts = computed(() => blogStore.posts)

// Load posts on mount
const loadPosts = async () => {
  await blogStore.loadPosts({
    page: 1,
    limit: 50,
    status: statusFilter.value === 'all' ? undefined : statusFilter.value
  })
}

// Watch filter changes
watch(statusFilter, loadPosts)

// Load initial data
loadPosts()

// Methods
const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

const handleDelete = async (post: BlogPost) => {
  if (!confirm(`Are you sure you want to delete "${post.title}"? This action cannot be undone.`)) {
    return
  }

  deletingId.value = post.id

  try {
    await blogStore.deletePost(post.id)
    // Reload posts after deletion
    await loadPosts()
  } catch (error) {
    console.error('Failed to delete post:', error)
  } finally {
    deletingId.value = null
  }
}
</script>
