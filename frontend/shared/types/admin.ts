/**
 * Admin-related type definitions
 * Consolidated from scattered locations for better organization
 */

import type { User } from './auth'
import type { PhraseSuggestion } from './phrases'

// Admin statistics
export interface AdminStats {
  total_users: number
  active_users: number
  pending_suggestions: number
  total_phrases: number
}

// User management response types
export interface UsersResponse {
  users: User[]
  total: number
}

// Phrase suggestion management response types
export interface SuggestionsResponse {
  suggestions: PhraseSuggestion[]
  total: number
}

// Password reset response
export interface ResetPasswordResponse {
  new_password: string
  message: string
}

// Generic admin action response
export interface AdminActionResponse {
  message: string
}
