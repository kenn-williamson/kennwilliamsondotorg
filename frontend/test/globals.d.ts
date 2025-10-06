/**
 * TypeScript declarations for test globals
 * These are defined in test/setup.ts
 */

import type { Mock } from 'vitest'
import type { Ref, ComputedRef } from 'vue'
import type { $Fetch } from 'ofetch'

declare global {
  // Nuxt auto-imports
  var useUserSession: Mock
  var useRuntimeConfig: Mock
  var useRequestFetch: Mock
  var useJwtManager: Mock
  var navigateTo: Mock
  var createError: Mock
  var defineEventHandler: Mock
  var defineNuxtRouteMiddleware: Mock
  var watch: Mock

  // Global fetch
  var $fetch: Mock & $Fetch

  // Store mocks
  var useIncidentTimerStore: Mock
  var usePhrasesStore: Mock
  var useAdminStore: Mock

  // Composable mocks
  var useSmartFetch: Mock
  var useSessionWatcher: Mock

  // Service mocks
  var incidentTimerService: Mock
  var phraseService: Mock
  var authService: Mock
  var authProfileService: Mock
  var adminService: Mock

  // Vue mocks
  var ref: typeof import('vue').ref
  var computed: typeof import('vue').computed
  var readonly: typeof import('vue').readonly
  var defineStore: typeof import('pinia').defineStore
}

export {}
