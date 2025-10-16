/**
 * Authentication-related type definitions
 * Consolidated from scattered locations for better organization
 */

// Nested profile data structure (matches backend UserResponse)
export interface ProfileData {
  real_name?: string
  bio?: string
  avatar_url?: string
  location?: string
  website?: string
}

// External account data structure (OAuth providers)
export interface ExternalAccount {
  provider: string
  linked_at: string
}

// User preferences structure
export interface PreferencesData {
  timer_is_public: boolean
  timer_show_in_list: boolean
}

// User types (from shared/types/auth.d.ts)
export interface AuthenticatedUser {
  id: string
  email: string
  display_name: string
  slug: string
  roles: string[]
  created_at: string
  email_verified: boolean
  has_credentials: boolean
  profile?: ProfileData
  external_accounts: ExternalAccount[]
  preferences?: PreferencesData
}

export interface User {
  id: string
  email: string
  display_name: string
  slug: string
  roles: string[]
  created_at: string
  active: boolean
  email_verified: boolean
  has_credentials: boolean
  profile?: ProfileData
  external_accounts: ExternalAccount[]
  preferences?: PreferencesData
}

export interface UserSession {
  user: AuthenticatedUser
  loggedInAt: Date
}

export interface SecureSessionData {
  jwtToken?: string
  refreshToken?: string
}

// JWT types (from shared/utils/jwt.ts)
export interface JwtPayload {
  exp: number
  iat: number
  sub: string
  [key: string]: any
}

export interface JwtParseResult {
  payload: JwtPayload
  expiration: Date
  isValid: boolean
  error?: string
}

// Authentication request/response types
export interface LoginRequest {
  email: string
  password: string
}

export interface RegisterRequest {
  email: string
  password: string
  display_name: string
}

export interface AuthResponse {
  token: string
  refresh_token: string
  user: AuthenticatedUser
}

export interface SlugPreviewRequest {
  display_name: string
}

export interface SlugPreviewResponse {
  slug: string
  available: boolean
  final_slug: string
}

export interface SlugValidationRequest {
  slug: string
}

export interface SlugValidationResponse {
  slug: string
  valid: boolean
  available: boolean
}

// Profile management types
export interface ProfileUpdateRequest {
  display_name: string
  slug: string
}

export interface PasswordChangeRequest {
  current_password: string
  new_password: string
}

export interface SetPasswordRequest {
  new_password: string
}

export interface PasswordChangeResponse {
  message: string
}

// Token management types
export interface TokenRevokeRequest {
  refresh_token: string
}

export interface TokenRevokeResponse {
  message: string
}

export interface TokenRevokeAllResponse {
  message: string
}

// JWT token management types
export interface JwtToken {
  token: string
  expiresAt: string
}

// Google OAuth types
export interface GoogleOAuthUrlResponse {
  url: string
}

export interface GoogleOAuthCallbackRequest {
  code: string
}

// Email verification types
export interface SendVerificationEmailResponse {
  message: string
}

export interface VerifyEmailRequest {
  token: string
}

export interface VerifyEmailResponse {
  message: string
}
