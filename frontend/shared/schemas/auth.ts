/**
 * Authentication validation schemas using VeeValidate + Yup
 */

import * as yup from 'yup'

// Login form validation
export const loginSchema = yup.object({
  email: yup
    .string()
    .required('Email is required')
    .email('Please enter a valid email address'),
  password: yup
    .string()
    .required('Password is required')
    .min(8, 'Password must be at least 8 characters'),
})

// Registration form validation
export const registerSchema = yup.object({
  email: yup
    .string()
    .required('Email is required')
    .email('Please enter a valid email address'),
  displayName: yup
    .string()
    .required('Display name is required')
    .min(2, 'Display name must be at least 2 characters')
    .max(50, 'Display name must be no more than 50 characters')
    .trim(),
  password: yup
    .string()
    .required('Password is required')
    .min(8, 'Password must be at least 8 characters')
    .matches(/(?=.*[a-z])/, 'Password must contain at least one lowercase letter')
    .matches(/(?=.*[A-Z])/, 'Password must contain at least one uppercase letter')
    .matches(/(?=.*\d)/, 'Password must contain at least one number'),
  confirmPassword: yup
    .string()
    .required('Please confirm your password')
    .oneOf([yup.ref('password')], 'Passwords must match'),
})

// Profile update validation
export const profileUpdateSchema = yup.object({
  displayName: yup
    .string()
    .required('Display name is required')
    .min(2, 'Display name must be at least 2 characters')
    .max(50, 'Display name must be no more than 50 characters')
    .trim(),
  slug: yup
    .string()
    .required('URL slug is required')
    .min(2, 'URL slug must be at least 2 characters')
    .max(50, 'URL slug must be no more than 50 characters')
    .matches(/^[a-z0-9-]+$/, 'URL slug can only contain lowercase letters, numbers, and hyphens')
    .test('no-consecutive-hyphens', 'URL slug cannot have consecutive hyphens', (value) => {
      if (!value) return true
      return !value.includes('--')
    })
    .test('no-leading-trailing-hyphens', 'URL slug cannot start or end with hyphens', (value) => {
      if (!value) return true
      return !value.startsWith('-') && !value.endsWith('-')
    })
    .test('not-empty-after-cleanup', 'URL slug cannot be empty', (value) => {
      if (!value) return true
      return value.trim().length > 0
    }),
})

// Password change validation
export const passwordChangeSchema = yup.object({
  currentPassword: yup
    .string()
    .required('Current password is required'),
  newPassword: yup
    .string()
    .required('New password is required')
    .min(8, 'Password must be at least 8 characters')
    .matches(/(?=.*[a-z])/, 'Password must contain at least one lowercase letter')
    .matches(/(?=.*[A-Z])/, 'Password must contain at least one uppercase letter')
    .matches(/(?=.*\d)/, 'Password must contain at least one number'),
  confirmPassword: yup
    .string()
    .required('Please confirm your new password')
    .oneOf([yup.ref('newPassword')], 'Passwords must match'),
})

// Slug preview validation (for registration)
export const slugPreviewSchema = yup.object({
  displayName: yup
    .string()
    .required('Display name is required')
    .min(2, 'Display name must be at least 2 characters')
    .max(50, 'Display name must be no more than 50 characters')
    .trim(),
})

// Utility function to generate slug from display name
export function generateSlug(displayName: string): string {
  return displayName
    .toLowerCase()
    .trim()
    .replace(/\s+/g, '-') // Replace spaces with hyphens
    .replace(/[^a-z0-9-]/g, '') // Remove invalid characters
    .replace(/-+/g, '-') // Replace multiple hyphens with single hyphen
    .replace(/^-|-$/g, '') // Remove leading/trailing hyphens
}
