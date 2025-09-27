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

// Mock composables for new architecture
export const mockUseBackendFetch = () => vi.fn()

export const mockUseAuthFetch = () => vi.fn()

export const mockUseUserSession = () => ({
  loggedIn: { value: true },
  user: { value: createMockUser() },
  clear: vi.fn(),
  fetch: vi.fn(),
})

// Mock stores for new architecture
export const mockUseIncidentTimerStore = () => ({
  timers: { value: [] },
  currentTimer: { value: null },
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
  fetchTimers: vi.fn(),
  createTimer: vi.fn(),
  updateTimer: vi.fn(),
  deleteTimer: vi.fn(),
  getPublicTimer: vi.fn(),
})

export const mockUseAuthStore = () => ({
  user: { value: null },
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
  isAuthenticated: { value: false },
  login: vi.fn(),
  register: vi.fn(),
  logout: vi.fn(),
  refreshToken: vi.fn(),
  getCurrentUser: vi.fn(),
  updateProfile: vi.fn(),
  changePassword: vi.fn(),
  previewSlug: vi.fn(),
})

export const mockUsePhrasesStore = () => ({
  phrases: { value: [] },
  userPhrases: { value: [] },
  suggestions: { value: [] },
  currentPhrase: { value: null },
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
  activePhrases: { value: [] },
  pendingSuggestions: { value: [] },
  fetchRandomPhrase: vi.fn(),
  fetchUserPhrases: vi.fn(),
  excludePhrase: vi.fn(),
  removePhraseExclusion: vi.fn(),
  submitSuggestion: vi.fn(),
  fetchSuggestions: vi.fn(),
})

export const mockUseAdminStore = () => ({
  users: { value: [] },
  suggestions: { value: [] },
  stats: { value: null },
  searchQuery: { value: '' },
  selectedUser: { value: null },
  newPassword: { value: null },
  activeTab: { value: 'overview' },
  isLoading: { value: false },
  error: { value: null },
  hasError: { value: false },
  filteredUsers: { value: [] },
  pendingSuggestions: { value: [] },
  fetchStats: vi.fn(),
  fetchUsers: vi.fn(),
  fetchSuggestions: vi.fn(),
  deactivateUser: vi.fn(),
  activateUser: vi.fn(),
  resetUserPassword: vi.fn(),
  promoteUser: vi.fn(),
  approveSuggestion: vi.fn(),
  rejectSuggestion: vi.fn(),
  setSearchQuery: vi.fn(),
  setSelectedUser: vi.fn(),
  clearNewPassword: vi.fn(),
  clearState: vi.fn(),
})

// Mock pure services
export const mockIncidentTimerService = () => ({
  getUserTimers: vi.fn(),
  getPublicTimer: vi.fn(),
  createTimer: vi.fn(),
  updateTimer: vi.fn(),
  deleteTimer: vi.fn(),
  quickReset: vi.fn(),
})

export const mockAuthService = () => ({
  login: vi.fn(),
  register: vi.fn(),
  previewSlug: vi.fn(),
  revokeAllSessions: vi.fn(),
  logout: vi.fn(),
})

export const mockPhraseService = () => ({
  fetchUserPhrases: vi.fn(),
  fetchAllPhrases: vi.fn(),
  excludePhrase: vi.fn(),
  removePhraseExclusion: vi.fn(),
  submitPhraseSuggestion: vi.fn(),
  fetchPhraseSuggestions: vi.fn(),
  fetchRandomPhraseAuth: vi.fn(),
  fetchRandomPhraseClient: vi.fn(),
  approveSuggestion: vi.fn(),
  rejectSuggestion: vi.fn(),
})

export const mockAdminService = () => ({
  getStats: vi.fn(),
  getUsers: vi.fn(),
  getSuggestions: vi.fn(),
  getPhrases: vi.fn(),
  deactivateUser: vi.fn(),
  activateUser: vi.fn(),
  resetUserPassword: vi.fn(),
  promoteUser: vi.fn(),
  approveSuggestion: vi.fn(),
  rejectSuggestion: vi.fn(),
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
