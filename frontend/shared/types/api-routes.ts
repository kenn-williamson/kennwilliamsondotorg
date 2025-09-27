/**
 * API Routes Configuration Types
 * Centralized type definitions for route configuration
 */

export interface SmartFetchOptions {
  query?: Record<string, string | number | boolean | undefined>
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH' | 'HEAD' | 'OPTIONS'
  body?: any
  headers?: Record<string, string>
  [key: string]: any
}

export type RoutingType = 'passthrough' | 'direct'
