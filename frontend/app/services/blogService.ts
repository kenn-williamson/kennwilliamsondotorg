/**
 * Blog Service - Uses useRequestFetch for SSR-safe requests
 */

import { API_ROUTES } from '#shared/config/api-routes'
import type {
  BlogPost,
  BlogPostList,
  BlogPostFilters,
  TagCount,
  CreateBlogPostRequest,
  UpdateBlogPostRequest,
  Fetcher
} from '#shared/types'

export const blogService = (fetcher: Fetcher) => ({
  /**
   * Get paginated list of blog posts (public)
   */
  getPosts: async (filters: BlogPostFilters = {}): Promise<BlogPostList> => {
    const params = new URLSearchParams()
    if (filters.page) params.append('page', filters.page.toString())
    if (filters.limit) params.append('limit', filters.limit.toString())
    if (filters.tag) params.append('tag', filters.tag)
    if (filters.status) params.append('status', filters.status)

    const query = params.toString()
    const url = query ? `${API_ROUTES.PUBLIC.BLOG.POSTS}?${query}` : API_ROUTES.PUBLIC.BLOG.POSTS

    return fetcher<BlogPostList>(url)
  },

  /**
   * Get single blog post by slug (public)
   */
  getPostBySlug: async (slug: string): Promise<BlogPost> => {
    return fetcher<BlogPost>(API_ROUTES.PUBLIC.BLOG.POST_BY_SLUG(slug))
  },

  /**
   * Search blog posts (public)
   */
  searchPosts: async (query: string, page: number = 1, limit: number = 10): Promise<BlogPostList> => {
    const params = new URLSearchParams({
      q: query,
      page: page.toString(),
      limit: limit.toString()
    })

    return fetcher<BlogPostList>(`${API_ROUTES.PUBLIC.BLOG.SEARCH}?${params.toString()}`)
  },

  /**
   * Get all tags with counts (public)
   */
  getTags: async (status?: string): Promise<{ tags: TagCount[] }> => {
    const url = status
      ? `${API_ROUTES.PUBLIC.BLOG.TAGS}?status=${status}`
      : API_ROUTES.PUBLIC.BLOG.TAGS

    return fetcher<{ tags: TagCount[] }>(url)
  },

  /**
   * Create new blog post (admin only)
   */
  createPost: async (postData: CreateBlogPostRequest): Promise<BlogPost> => {
    return fetcher<BlogPost>(API_ROUTES.PROTECTED.ADMIN.BLOG.POSTS, {
      method: 'POST',
      body: postData
    })
  },

  /**
   * Update existing blog post (admin only)
   */
  updatePost: async (id: string, postData: UpdateBlogPostRequest): Promise<BlogPost> => {
    return fetcher<BlogPost>(API_ROUTES.PROTECTED.ADMIN.BLOG.POST_BY_ID(id), {
      method: 'PUT',
      body: postData
    })
  },

  /**
   * Delete blog post (admin only)
   */
  deletePost: async (id: string): Promise<void> => {
    return fetcher<void>(API_ROUTES.PROTECTED.ADMIN.BLOG.POST_BY_ID(id), {
      method: 'DELETE'
    })
  },

  /**
   * Upload featured image (admin only)
   */
  uploadImage: async (imageFile: File): Promise<{ url: string; original_url: string }> => {
    const formData = new FormData()
    formData.append('image', imageFile)

    return fetcher<{ url: string; original_url: string }>(
      API_ROUTES.PROTECTED.ADMIN.BLOG.UPLOAD_IMAGE,
      {
        method: 'POST',
        body: formData
      }
    )
  }
})
