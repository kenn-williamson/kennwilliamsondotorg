/**
 * Blog Types - Frontend API types for blog feature
 */

export interface BlogPost {
  id: string
  slug: string
  title: string
  excerpt: string | null
  content: string
  featured_image_url: string | null
  featured_image_alt: string | null
  status: 'draft' | 'published'
  tags: string[]
  published_at: string | null
  created_at: string
  updated_at: string
  meta_description: string | null
}

export interface BlogPostList {
  posts: BlogPost[]
  total: number
  page: number
  total_pages: number
}

export interface BlogPostFilters {
  page?: number
  limit?: number
  tag?: string
  status?: string
}

export interface TagCount {
  tag: string
  count: number
}

export interface CreateBlogPostRequest {
  title: string
  slug?: string
  excerpt?: string
  content: string
  featured_image_url?: string
  featured_image_alt?: string
  tags: string[]
  status: 'draft' | 'published'
  meta_description?: string
}

export interface UpdateBlogPostRequest {
  title?: string
  slug?: string
  excerpt?: string
  content?: string
  featured_image_url?: string
  featured_image_alt?: string
  tags?: string[]
  status?: 'draft' | 'published'
  meta_description?: string
}
