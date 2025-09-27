/**
 * API Routes Configuration - Centralized endpoint definitions
 * 
 * This file contains all API endpoint constants organized by access level:
 * - PUBLIC: No authentication required
 * - PROTECTED: JWT authentication required  
 * - API: SSR passthrough routes (server-side only)
 * 
 * Architecture mirrors backend route structure for clarity and maintainability.
 */

export const API_ROUTES = {
  /**
   * Public endpoints - No authentication required
   * Used by useSmartFetch() with /public/ prefix
   */
  PUBLIC: {
    AUTH: {
      LOGIN: '/public/auth/login',
      REGISTER: '/public/auth/register',
      REFRESH: '/public/auth/refresh',
      PREVIEW_SLUG: '/public/auth/preview-slug',
    },
    HEALTH: {
      BASIC: '/public/health',
      DATABASE: '/public/health/db',
    },
    TIMERS: {
      BY_USER_SLUG: (userSlug: string) => `/public/${userSlug}/incident-timer`,
    },
    PHRASES: {
      BY_USER_SLUG: (userSlug: string) => `/public/${userSlug}/phrase`,
    },
  },

  /**
   * Protected endpoints - JWT authentication required
   * Used by useSmartFetch() with /protected/ prefix
   */
  PROTECTED: {
    AUTH: {
      ME: '/protected/auth/me',
      REVOKE: '/protected/auth/revoke',
      REVOKE_ALL: '/protected/auth/revoke-all',
      PROFILE: '/protected/auth/profile',
      CHANGE_PASSWORD: '/protected/auth/change-password',
      VALIDATE_SLUG: '/protected/auth/validate-slug',
    },
    TIMERS: {
      LIST: '/protected/incident-timers',
      CREATE: '/protected/incident-timers',
      UPDATE: (id: string) => `/protected/incident-timers/${id}`,
      DELETE: (id: string) => `/protected/incident-timers/${id}`,
    },
    PHRASES: {
      RANDOM: '/protected/phrases/random',
      LIST: '/protected/phrases',
      USER: '/protected/phrases/user',
      EXCLUDED: '/protected/phrases/excluded',
      EXCLUDE: (id: string) => `/protected/phrases/exclude/${id}`,
      SUGGESTIONS: '/protected/phrases/suggestions',
    },
    ADMIN: {
      STATS: '/protected/admin/stats',
      USERS: '/protected/admin/users',
      USER_DEACTIVATE: (id: string) => `/protected/admin/users/${id}/deactivate`,
      USER_ACTIVATE: (id: string) => `/protected/admin/users/${id}/activate`,
      USER_RESET_PASSWORD: (id: string) => `/protected/admin/users/${id}/reset-password`,
      USER_PROMOTE: (id: string) => `/protected/admin/users/${id}/promote`,
      PHRASES: {
        LIST: '/protected/admin/phrases',
        CREATE: '/protected/admin/phrases',
        UPDATE: (id: string) => `/protected/admin/phrases/${id}`,
        DELETE: (id: string) => `/protected/admin/phrases/${id}`,
      },
      SUGGESTIONS: {
        LIST: '/protected/admin/suggestions',
        APPROVE: (id: string) => `/protected/admin/suggestions/${id}/approve`,
        REJECT: (id: string) => `/protected/admin/suggestions/${id}/reject`,
      },
    },
  },


  /**
   * SSR Passthrough routes - Server-side only
   * Used by server API routes for SSR data fetching
   */
  API: {
    AUTH: {
      LOGIN: '/api/auth/login',
      REGISTER: '/api/auth/register',
      LOGOUT: '/api/auth/logout',
      ME: '/api/auth/me',
      PROFILE: '/api/auth/profile',
    },
    TIMERS: {
      LIST: '/api/incident-timers',
      BY_USER_SLUG: (userSlug: string) => `/api/${userSlug}/incident-timer`,
    },
    PHRASES: {
      RANDOM: '/api/phrases/random',
      BY_USER_SLUG: (userSlug: string) => `/api/${userSlug}/phrase`,
    },
    HEALTH: {
      BASIC: '/api/health',
    },
  },
} as const

/**
 * Type definitions for API route categories
 */
export type PublicRoutes = typeof API_ROUTES.PUBLIC
export type ProtectedRoutes = typeof API_ROUTES.PROTECTED
export type ApiRoutes = typeof API_ROUTES.API

/**
 * Check if a route requires authentication
 * Simple string check - most performant and clear approach
 */
export const requiresAuth = (route: string): boolean => {
  return route.startsWith('/protected/')
}
