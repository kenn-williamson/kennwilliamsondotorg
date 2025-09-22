import { vi } from 'vitest'

// Mock data factories based on IMPLEMENTATION-DATA-CONTRACTS.md
export const createMockTimer = (overrides = {}) => ({
  id: '01234567-89ab-cdef-0123-456789abcdef',
  reset_timestamp: '2024-01-01T12:00:00Z',
  notes: 'System maintenance incident',
  created_at: '2024-01-01T11:00:00Z',
  updated_at: '2024-01-01T12:00:00Z',
  ...overrides,
})

export const createMockUser = (overrides = {}) => ({
  id: '01234567-89ab-cdef-0123-456789abcdef',
  email: 'user@example.com',
  display_name: 'John Doe',
  slug: 'john-doe',
  roles: ['user'],
  created_at: '2024-01-01T12:00:00Z',
  ...overrides,
})

export const createMockPhrase = (overrides = {}) => ({
  id: '01234567-89ab-cdef-0123-456789abcdef',
  phrase_text: 'Vigilance Maintained - Until the Next Challenge Arises',
  active: true,
  created_by: '01234567-89ab-cdef-0123-456789abcdef',
  created_at: '2024-01-01T12:00:00Z',
  updated_at: '2024-01-01T12:00:00Z',
  is_excluded: false,
  ...overrides,
})

export const createMockPhraseSuggestion = (overrides = {}) => ({
  id: '01234567-89ab-cdef-0123-456789abcdef',
  user_id: '01234567-89ab-cdef-0123-456789abcdef',
  phrase_text: 'A new motivational phrase suggestion',
  status: 'pending',
  admin_id: null,
  admin_reason: null,
  created_at: '2024-01-01T12:00:00Z',
  updated_at: '2024-01-01T12:00:00Z',
  ...overrides,
})

// Additional mock data for admin store
export const createMockAdminStats = (overrides = {}) => ({
  total_users: 25,
  active_users: 23,
  pending_suggestions: 3,
  total_phrases: 15,
  ...overrides,
})

export const createMockPhraseWithExclusion = (overrides = {}) => ({
  id: '01234567-89ab-cdef-0123-456789abcdef',
  phrase_text: 'Vigilance Maintained - Until the Next Challenge Arises',
  active: true,
  created_by: '01234567-89ab-cdef-0123-456789abcdef',
  created_at: '2024-01-01T12:00:00Z',
  updated_at: '2024-01-01T12:00:00Z',
  is_excluded: false,
  ...overrides,
})

export const createMockUserWithRoles = (overrides = {}) => ({
  id: '01234567-89ab-cdef-0123-456789abcdef',
  email: 'user@example.com',
  display_name: 'John Doe',
  slug: 'john-doe',
  active: true,
  roles: ['user'],
  created_at: '2024-01-01T12:00:00Z',
  ...overrides,
})

export const createMockAuthResponse = (overrides = {}) => ({
  token: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...',
  refresh_token: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...',
  user: createMockUser(),
  ...overrides,
})

// Mock composables
export const mockUseBaseService = () => ({
  executeRequest: vi.fn(),
  executeRequestWithSuccess: vi.fn(),
  backendFetch: vi.fn(),
  authFetch: vi.fn(),
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
})

export const mockUseUserSession = () => ({
  loggedIn: { value: true },
  user: { value: createMockUser() },
  clear: vi.fn(),
  fetch: vi.fn(),
})

export const mockUseIncidentTimerService = () => ({
  getUserTimers: vi.fn(),
  getPublicTimer: vi.fn(),
  createTimer: vi.fn(),
  updateTimer: vi.fn(),
  deleteTimer: vi.fn(),
  quickReset: vi.fn(),
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
})

export const mockUsePhraseService = () => ({
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
})

export const mockUseAdminService = () => ({
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
})

// Mock API responses
export const mockApiResponse = (data: any, success = true) => ({
  success,
  data,
  message: success ? 'Success' : 'Error',
})

// Mock error responses
export const mockApiError = (message = 'API Error', statusCode = 500) => {
  const error = new Error(message)
  ;(error as any).statusCode = statusCode
  return error
}

// Mock Nuxt utilities
export const mockNavigateTo = vi.fn()
export const mockCreateError = vi.fn()

// Setup common mocks
export const setupCommonMocks = () => {
  vi.mock('~/composables/useBaseService', () => ({
    useBaseService: mockUseBaseService,
  }))
  
  vi.mock('nuxt-auth-utils', () => ({
    useUserSession: mockUseUserSession,
  }))
  
  vi.mock('~/composables/useIncidentTimerService', () => ({
    useIncidentTimerService: mockUseIncidentTimerService,
  }))
  
  // Mock global Nuxt functions
  global.navigateTo = mockNavigateTo
  global.createError = mockCreateError
}
