/**
 * Access Request type definitions
 */

// Access request with user details (for admin panel)
export interface AccessRequestWithUser {
  id: string
  user_id: string
  user_email: string
  user_display_name: string
  message: string
  requested_role: string
  status: 'pending' | 'approved' | 'rejected'
  admin_id: string | null
  admin_reason: string | null
  created_at: string
  updated_at: string
}

// Access requests list response
export interface AccessRequestsResponse {
  requests: AccessRequestWithUser[]
  total: number
}

// Single access request response
export interface AccessRequest {
  id: string
  user_id: string
  message: string
  requested_role: string
  status: 'pending' | 'approved' | 'rejected'
  admin_id: string | null
  admin_reason: string | null
  created_at: string
  updated_at: string
}
