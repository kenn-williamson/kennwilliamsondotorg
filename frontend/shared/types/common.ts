/**
 * Common type definitions shared across the application
 */

// Generic fetcher type for service functions
import type { SmartFetchOptions } from './api-routes'

export type Fetcher = <T = any>(route: string, options?: SmartFetchOptions) => Promise<T>

// API response wrapper
export interface ApiResponse<T = any> {
  data?: T
  error?: string
  message?: string
}

// Pagination types
export interface PaginationParams {
  limit?: number
  offset?: number
}

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  limit: number
  offset: number
}

// Form state types
export interface FormState {
  isSubmitting: boolean
  isDirty: boolean
  isValid: boolean
  errors: Record<string, string>
}

// Loading states
export interface LoadingState {
  isLoading: boolean
  error?: string
}

// Generic CRUD operations
export interface CrudState<T> extends LoadingState {
  items: T[]
  selectedItem?: T
  total: number
}

// Health check types
export interface HealthCheckResponse {
  status: string
}

export interface DatabaseHealthResponse {
  status: string
}

// Error types
export interface ValidationError {
  field: string
  message: string
}

export interface ApiError {
  error: string
  details?: ValidationError[]
}
