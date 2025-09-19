/**
 * Shared validation utilities
 * Common validation functions used across the application
 */

/**
 * Generate a URL-friendly slug from a display name
 * Converts spaces to hyphens and removes invalid characters
 */
export function generateSlug(displayName: string): string {
  return displayName
    .toLowerCase()
    .trim()
    .replace(/\s+/g, '-') // Replace spaces with hyphens
    .replace(/[^a-z0-9-]/g, '') // Remove invalid characters (keep only a-z, 0-9, -)
    .replace(/-+/g, '-') // Replace multiple hyphens with single hyphen
    .replace(/^-|-$/g, '') // Remove leading/trailing hyphens
}

/**
 * Validate if a slug is properly formatted
 * Checks for valid characters and proper formatting
 */
export function isValidSlug(slug: string): boolean {
  if (!slug || slug.length < 2) return false
  
  // Check for valid characters only
  if (!/^[a-z0-9-]+$/.test(slug)) return false
  
  // Check for consecutive hyphens
  if (slug.includes('--')) return false
  
  // Check for leading/trailing hyphens
  if (slug.startsWith('-') || slug.endsWith('-')) return false
  
  return true
}

/**
 * Validate email format
 */
export function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(email)
}

/**
 * Validate password strength
 * Checks for minimum length and character requirements
 */
export function isStrongPassword(password: string): boolean {
  if (password.length < 8) return false
  
  const hasLowercase = /[a-z]/.test(password)
  const hasUppercase = /[A-Z]/.test(password)
  const hasNumber = /\d/.test(password)
  
  return hasLowercase && hasUppercase && hasNumber
}

/**
 * Format validation error message
 */
export function formatValidationError(field: string, message: string): string {
  return `${field}: ${message}`
}

/**
 * Check if a string is empty or only whitespace
 */
export function isEmpty(value: string | null | undefined): boolean {
  return !value || value.trim().length === 0
}

/**
 * Sanitize string input (trim and normalize whitespace)
 */
export function sanitizeString(value: string): string {
  return value.trim().replace(/\s+/g, ' ')
}
