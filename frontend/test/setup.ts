import { vi } from 'vitest'
import '@testing-library/jest-dom'
import { ref, computed, readonly } from 'vue'
import { defineStore } from 'pinia'

// Mock Nuxt auto-imports with proper implementations
global.defineStore = defineStore
global.ref = ref
global.computed = computed
global.readonly = readonly
global.defineNuxtRouteMiddleware = vi.fn()
global.navigateTo = vi.fn()
global.useUserSession = vi.fn()
global.createError = vi.fn()
global.defineEventHandler = vi.fn()
global.$fetch = vi.fn()

// Mock composables
global.useIncidentTimerService = vi.fn(() => ({
  getUserTimers: vi.fn(),
  getPublicTimer: vi.fn(),
  createTimer: vi.fn(),
  updateTimer: vi.fn(),
  deleteTimer: vi.fn(),
  quickReset: vi.fn(),
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
}))

global.usePhraseService = vi.fn(() => ({
  fetchAllPhrases: vi.fn(),
  fetchUserPhrases: vi.fn(),
  fetchExcludedPhrases: vi.fn(),
  excludePhrase: vi.fn(),
  removePhraseExclusion: vi.fn(),
  submitPhraseSuggestion: vi.fn(),
  fetchPhraseSuggestions: vi.fn(),
  fetchRandomPhrase: vi.fn(),
  fetchRandomPhraseClient: vi.fn(),
  fetchRandomPhraseAuth: vi.fn(),
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
}))

global.useAdminService = vi.fn(() => ({
  getStats: vi.fn(),
  getUsers: vi.fn(),
  getSuggestions: vi.fn(),
  deactivateUser: vi.fn(),
  activateUser: vi.fn(),
  resetUserPassword: vi.fn(),
  promoteUser: vi.fn(),
  approveSuggestion: vi.fn(),
  rejectSuggestion: vi.fn(),
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
}))

// Mock console methods to reduce noise in tests
global.console = {
  ...console,
  log: vi.fn(),
  warn: vi.fn(),
  error: vi.fn(),
}

// Mock window and document for browser APIs
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
})

// Mock IntersectionObserver
global.IntersectionObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}))

// Mock ResizeObserver
global.ResizeObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}))
