/**
 * Timer-related type definitions
 */

export interface IncidentTimer {
  id: string
  reset_timestamp: string
  notes?: string
  created_at: string
  updated_at: string
}

export interface PublicIncidentTimer extends IncidentTimer {
  user_display_name: string
}

export interface StreakStats {
  longest_streak_seconds: number
  average_streak_seconds: number
  total_completed_streaks: number
}

export interface PublicTimerResponse extends IncidentTimer {
  user_display_name: string
  streak_stats?: StreakStats
}

// Request types
export interface CreateTimerRequest {
  reset_timestamp?: string
  notes?: string
}

export interface UpdateTimerRequest {
  reset_timestamp?: string
  notes?: string
}

// Response types
export interface TimersResponse {
  timers: IncidentTimer[]
}

export interface TimerResponse {
  timer: IncidentTimer
}

// Form types for UI
export interface TimerFormData {
  notes: string
  reset_timestamp: string
}
