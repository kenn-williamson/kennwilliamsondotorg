/**
 * Blog post validation schemas using VeeValidate + Yup
 */

import * as yup from 'yup'

// URL-safe slug pattern (lowercase letters, numbers, hyphens only)
const SLUG_PATTERN = /^[a-z0-9]+(?:-[a-z0-9]+)*$/

// Helper function to generate slug from title
export const generateSlugFromTitle = (title: string): string => {
  return title
    .toLowerCase()
    .trim()
    .replace(/[^a-z0-9\s-]/g, '') // Remove special characters
    .replace(/\s+/g, '-') // Replace spaces with hyphens
    .replace(/-+/g, '-') // Replace multiple hyphens with single
    .replace(/^-+|-+$/g, '') // Remove leading/trailing hyphens
}

// Blog post creation/update schema
export const blogPostSchema = yup.object({
  title: yup
    .string()
    .required('Title is required')
    .min(1, 'Title must be at least 1 character')
    .max(200, 'Title must be no more than 200 characters')
    .trim(),

  slug: yup
    .string()
    .matches(SLUG_PATTERN, 'Slug must contain only lowercase letters, numbers, and hyphens')
    .max(200, 'Slug must be no more than 200 characters')
    .optional(),

  excerpt: yup
    .string()
    .max(500, 'Excerpt must be no more than 500 characters')
    .trim()
    .optional()
    .nullable(),

  content: yup
    .string()
    .required('Content is required')
    .min(50, 'Content must be at least 50 characters'),

  featured_image_url: yup
    .string()
    .url('Please enter a valid URL')
    .optional()
    .nullable(),

  featured_image_alt: yup
    .string()
    .max(200, 'Alt text must be no more than 200 characters')
    .when('featured_image_url', {
      is: (value: string | null | undefined) => !!value,
      then: (schema) => schema.required('Alt text is required when an image is provided'),
      otherwise: (schema) => schema.optional().nullable()
    }),

  tags: yup
    .array()
    .of(yup.string().required())
    .max(10, 'You can add up to 10 tags')
    .default([]),

  status: yup
    .string()
    .oneOf(['draft', 'published'], 'Status must be either draft or published')
    .required('Status is required'),

  meta_description: yup
    .string()
    .max(160, 'Meta description must be no more than 160 characters (for SEO)')
    .trim()
    .optional()
    .nullable()
})

// Type inference from schema
export type BlogPostFormData = yup.InferType<typeof blogPostSchema>
