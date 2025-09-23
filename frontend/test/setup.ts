import { vi } from 'vitest'
import '@testing-library/jest-dom'
import { ref, computed, readonly } from 'vue'
import { defineStore, createPinia, setActivePinia } from 'pinia'

// Set up Pinia for testing
const pinia = createPinia()
setActivePinia(pinia)

// Mock Nuxt auto-imports with proper implementations
global.defineStore = defineStore
global.ref = ref
global.computed = computed
global.readonly = readonly
global.defineNuxtRouteMiddleware = vi.fn()
global.navigateTo = vi.fn()
global.useRuntimeConfig = vi.fn(() => ({
  public: {
    apiBase: 'https://localhost/backend'
  }
}))
global.useJwtManager = vi.fn(() => ({
  getToken: vi.fn().mockResolvedValue('mock-jwt-token'),
  refreshToken: vi.fn().mockResolvedValue('mock-refresh-token'),
  clearTokens: vi.fn()
}))
// Mock useUserSession with proper structure
global.useUserSession = vi.fn(() => ({
  ready: { value: true },
  loggedIn: { value: false },
  user: { value: null },
  session: { value: null },
  clear: vi.fn().mockResolvedValue(undefined),
  fetch: vi.fn().mockResolvedValue(undefined),
  openInPopup: vi.fn()
}))
global.createError = vi.fn()
global.defineEventHandler = vi.fn()
global.$fetch = vi.fn()

// Mock action composables for new architecture
global.useIncidentTimerActions = vi.fn(() => ({
  loadUserTimers: vi.fn(),
  loadPublicTimer: vi.fn(),
  createTimer: vi.fn(),
  updateTimer: vi.fn(),
  deleteTimer: vi.fn(),
  quickReset: vi.fn(),
  fetchTimers: vi.fn(),
  fetchPublicTimer: vi.fn(),
  startLiveTimerUpdates: vi.fn(),
  stopLiveTimerUpdates: vi.fn(),
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
}))

global.useAuthActions = vi.fn(() => ({
  login: vi.fn(),
  register: vi.fn(),
  previewSlug: vi.fn(),
  revokeAllSessions: vi.fn(),
  logout: vi.fn(),
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
}))

global.usePhrasesActions = vi.fn(() => ({
  loadUserPhrases: vi.fn(),
  loadAdminPhrases: vi.fn(),
  loadUserSuggestions: vi.fn(),
  loadAdminSuggestions: vi.fn(),
  excludePhrase: vi.fn(),
  removePhraseExclusion: vi.fn(),
  submitPhraseSuggestion: vi.fn(),
  approveSuggestion: vi.fn(),
  rejectSuggestion: vi.fn(),
  getRandomPhrase: vi.fn(),
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
}))

global.useAdminActions = vi.fn(() => ({
  loadStats: vi.fn(),
  loadUsers: vi.fn(),
  loadSuggestions: vi.fn(),
  loadPhrases: vi.fn(),
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

// Mock base composables
global.useBaseService = vi.fn(() => ({
  executeRequest: vi.fn(),
  executeRequestWithSuccess: vi.fn(),
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
}))

// Mock fetchers that return mock functions
global.useBackendFetch = vi.fn(() => vi.fn())
global.useAuthFetch = vi.fn(() => vi.fn())

// Mock services - these are curried functions that take a fetcher and return service methods
global.incidentTimerService = vi.fn(() => ({
  getUserTimers: vi.fn(),
  getPublicTimer: vi.fn(),
  createTimer: vi.fn(),
  updateTimer: vi.fn(),
  deleteTimer: vi.fn(),
  quickReset: vi.fn()
}))

global.phraseService = vi.fn(() => ({
  getRandomPhrase: vi.fn(),
  getUserPhrases: vi.fn(),
  excludePhrase: vi.fn(),
  removePhraseExclusion: vi.fn(),
  submitSuggestion: vi.fn(),
  getUserSuggestions: vi.fn(),
  getAdminPhrases: vi.fn(),
  getAdminSuggestions: vi.fn(),
  approveSuggestion: vi.fn(),
  rejectSuggestion: vi.fn()
}))

global.authService = vi.fn(() => ({
  login: vi.fn(),
  register: vi.fn(),
  previewSlug: vi.fn(),
  revokeAllSessions: vi.fn(),
  logout: vi.fn()
}))

global.authProfileService = vi.fn(() => ({
  updateProfile: vi.fn(),
  changePassword: vi.fn(),
  previewSlug: vi.fn()
}))

global.adminService = vi.fn(() => ({
  getStats: vi.fn(),
  getUsers: vi.fn(),
  getSuggestions: vi.fn(),
  getPhrases: vi.fn(),
  deactivateUser: vi.fn(),
  activateUser: vi.fn(),
  resetUserPassword: vi.fn(),
  promoteUser: vi.fn(),
  approveSuggestion: vi.fn(),
  rejectSuggestion: vi.fn()
}))

// Mock stores
global.useIncidentTimerStore = vi.fn()
global.usePhrasesStore = vi.fn()
global.useAdminStore = vi.fn()

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
