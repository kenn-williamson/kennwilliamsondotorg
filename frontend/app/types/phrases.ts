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


export interface PhraseExclusionResponse {
  message: string
}

export interface PhraseSuggestionResponse {
  suggestion: PhraseSuggestion
}

export interface PhraseSuggestionsResponse {
  suggestions: PhraseSuggestion[]
}

export interface ExcludedPhrasesResponse {
  excluded_phrases: UserExcludedPhrase[]
}

export interface AllPhrasesResponse {
  phrases: Phrase[]
}
