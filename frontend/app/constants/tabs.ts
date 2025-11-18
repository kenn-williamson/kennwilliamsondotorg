/**
 * Tab Definitions - Single source of truth for all tab configurations
 *
 * Each tab configuration includes:
 * - Tab definitions with IDs, labels, and icons
 * - Array of valid tab IDs for validation
 * - Default tab to show when no query param is present
 */

import type { Tab } from '#shared/types'

/**
 * Incident Timer Page Tabs
 */
export const INCIDENT_TABS = {
  tabs: [
    {
      id: 'timer-display',
      label: 'Timer Display',
      icon: '⏰'
    },
    {
      id: 'timer-controls',
      label: 'Timer Controls',
      icon: '⚙️'
    },
    {
      id: 'phrase-suggestions',
      label: 'Suggest Phrases',
      icon: '✍️'
    },
    {
      id: 'phrase-filter',
      label: 'Filter Phrases',
      icon: '🔧'
    },
    {
      id: 'suggestion-history',
      label: 'My Suggestions',
      icon: '📋'
    },
    {
      id: 'public-timers',
      label: 'Public Timers',
      icon: '🌐'
    }
  ] as const satisfies readonly Tab[],

  get ids() {
    return this.tabs.map(t => t.id) as readonly string[]
  },

  default: 'timer-display' as const
} as const

/**
 * Admin Panel Page Tabs
 */
export const ADMIN_TABS = {
  tabs: [
    {
      id: 'overview',
      label: 'Overview',
      icon: '📊'
    },
    {
      id: 'users',
      label: 'Users',
      icon: '👥'
    },
    {
      id: 'suggestions',
      label: 'Phrase Suggestions',
      icon: '✍️'
    },
    {
      id: 'access-requests',
      label: 'Access Requests',
      icon: '🔑'
    }
  ] as const satisfies readonly Tab[],

  get ids() {
    return this.tabs.map(t => t.id) as readonly string[]
  },

  default: 'overview' as const
} as const

/**
 * Blog Admin Page Tabs
 */
export const BLOG_ADMIN_TABS = {
  tabs: [
    {
      id: 'list',
      label: 'All Posts',
      icon: '📋'
    },
    {
      id: 'editor',
      label: 'Create/Edit',
      icon: '✍️'
    }
  ] as const satisfies readonly Tab[],

  get ids() {
    return this.tabs.map(t => t.id) as readonly string[]
  },

  default: 'list' as const
} as const

// Type exports for type-safe usage
export type IncidentTabId = typeof INCIDENT_TABS.tabs[number]['id']
export type AdminTabId = typeof ADMIN_TABS.tabs[number]['id']
export type BlogAdminTabId = typeof BLOG_ADMIN_TABS.tabs[number]['id']
