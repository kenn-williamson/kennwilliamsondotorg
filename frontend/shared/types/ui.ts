/**
 * UI component type definitions
 * Consolidated from scattered component locations for better organization
 */

// Tab navigation types
export interface Tab {
  id: string
  label: string
  icon: string
}

// Component prop types
export interface RandomPhraseProps {
  userSlug?: string // Optional user slug - if not provided, uses authenticated endpoint
  refreshInterval?: number // Optional refresh interval in milliseconds
}
