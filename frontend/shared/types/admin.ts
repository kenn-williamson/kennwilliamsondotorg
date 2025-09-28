/**
 * Admin-related type definitions
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

// User management response types - now returns array directly
export type UsersResponse = User[]

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
