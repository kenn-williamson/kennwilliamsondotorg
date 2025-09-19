/**
 * Phrase-related type definitions
 * Consolidated from app/types/phrases.ts for better organization
 */

export interface Phrase {
  id: string
  phrase_text: string
  active: boolean
  created_by: string
  created_at: string
  updated_at: string
}

export interface PhraseWithExclusion extends Phrase {
  is_excluded: boolean
}

export interface UserExcludedPhrase {
  id: string
  user_id: string
  phrase_id: string
  excluded_at: string
}

export interface PhraseSuggestion {
  id: string
  user_id: string
  phrase_text: string
  status: 'pending' | 'approved' | 'rejected'
  admin_id?: string
  admin_reason?: string
  created_at: string
  updated_at: string
}

// Response types
export interface PhraseExclusionResponse {
  message: string
}

export interface PhraseSuggestionResponse {
  suggestion: PhraseSuggestion
}

export interface PhraseSuggestionsResponse {
  suggestions: PhraseSuggestion[]
  total: number
}

export interface ExcludedPhrasesResponse {
  excluded_phrases: UserExcludedPhrase[]
}

export interface AllPhrasesResponse {
  phrases: Phrase[]
  total: number
}

// Request types
export interface PhraseSuggestionRequest {
  phrase_text: string
}

export interface PhraseExclusionRequest {
  phrase_id: string
}

// Admin types
export interface AdminPhraseCreateRequest {
  phrase_text: string
}

export interface AdminPhraseUpdateRequest {
  phrase_text: string
  active: boolean
}

export interface AdminSuggestionApproveRequest {
  admin_reason?: string
}

export interface AdminSuggestionRejectRequest {
  admin_reason?: string
}
