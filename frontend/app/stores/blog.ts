/**
 * Blog Store - Centralized state management for blog posts
 */

import type {
  BlogPost,
  BlogPostList,
  BlogPostFilters,
  TagCount,
  CreateBlogPostRequest,
  UpdateBlogPostRequest
} from '#shared/types'
import { blogService } from '~/services/blogService'
import { useSmartFetch } from '~/composables/useSmartFetch'

export const useBlogStore = defineStore('blog', () => {
  // State
  const posts = ref<BlogPost[]>([])
  const currentPost = ref<BlogPost | null>(null)
  const tags = ref<TagCount[]>([])
  const totalPosts = ref(0)
  const currentPage = ref(1)
  const totalPages = ref(1)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Service instance - uses useSmartFetch for automatic routing
  const smartFetch = useSmartFetch()
  const blogServiceInstance = blogService(smartFetch)

  // Private action handler
  const _handleAction = async <T>(
    action: () => Promise<T>,
    context?: string
  ): Promise<T | undefined> => {
    isLoading.value = true
    error.value = null

    try {
      const result = await action()
      return result
    } catch (err: any) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      error.value = errorMessage
      console.error(`[BlogStore] Error${context ? ` in ${context}` : ''}:`, errorMessage)
      return undefined
    } finally {
      isLoading.value = false
    }
  }

  // Actions
  const loadPosts = async (filters: BlogPostFilters = {}) => {
    const data = await _handleAction(() => blogServiceInstance.getPosts(filters), 'loadPosts')
    if (data) {
      posts.value = data.posts
      totalPosts.value = data.total
      currentPage.value = data.page
      totalPages.value = data.total_pages
    }
    return data
  }

  const loadPostBySlug = async (slug: string) => {
    const data = await _handleAction(() => blogServiceInstance.getPostBySlug(slug), 'loadPostBySlug')
    if (data) {
      currentPost.value = data
    }
    return data
  }

  const searchPosts = async (query: string, page: number = 1, limit: number = 10) => {
    const data = await _handleAction(
      () => blogServiceInstance.searchPosts(query, page, limit),
      'searchPosts'
    )
    if (data) {
      posts.value = data.posts
      totalPosts.value = data.total
      currentPage.value = data.page
      totalPages.value = data.total_pages
    }
    return data
  }

  const loadTags = async (status?: string) => {
    const data = await _handleAction(() => blogServiceInstance.getTags(status), 'loadTags')
    if (data) {
      tags.value = data.tags
    }
    return data
  }

  const createPost = async (postData: CreateBlogPostRequest) => {
    const data = await _handleAction(() => blogServiceInstance.createPost(postData), 'createPost')
    if (data) {
      posts.value.unshift(data)
      console.log('[BlogStore] Post created successfully')
    }
    return data
  }

  const updatePost = async (id: string, postData: UpdateBlogPostRequest) => {
    const data = await _handleAction(() => blogServiceInstance.updatePost(id, postData), 'updatePost')
    if (data) {
      const index = posts.value.findIndex(p => p.id === id)
      if (index !== -1) {
        posts.value[index] = data
      }
      if (currentPost.value?.id === id) {
        currentPost.value = data
      }
      console.log('[BlogStore] Post updated successfully')
    }
    return data
  }

  const deletePost = async (id: string) => {
    await _handleAction(() => blogServiceInstance.deletePost(id), 'deletePost')
    posts.value = posts.value.filter(p => p.id !== id)
    if (currentPost.value?.id === id) {
      currentPost.value = null
    }
    console.log('[BlogStore] Post deleted successfully')
  }

  const uploadImage = async (imageFile: File) => {
    return await _handleAction(() => blogServiceInstance.uploadImage(imageFile), 'uploadImage')
  }

  // Computed
  const hasError = computed(() => !!error.value)

  return {
    // State
    posts,
    currentPost,
    tags,
    totalPosts,
    currentPage,
    totalPages,
    isLoading,
    error,
    hasError,

    // Actions
    loadPosts,
    loadPostBySlug,
    searchPosts,
    loadTags,
    createPost,
    updatePost,
    deletePost,
    uploadImage
  }
})
