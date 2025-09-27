# Frontend Implementation

## Overview
Nuxt.js 4.0.3 frontend with Vue 3, TypeScript, and SSR support. Features authentication, incident timers, phrases system, and page-specific design aesthetics.

## Technology Stack
- **Framework**: Nuxt.js 4.0.3, Vue 3, TypeScript
- **State Management**: Pinia, nuxt-auth-utils
- **Styling**: TailwindCSS
- **Forms**: VeeValidate + Yup
- **Utilities**: VueUse

## Project Structure
```
frontend/
├── app/                    # Nuxt 4 app directory structure
│   ├── app.vue            # Main application entry point
│   ├── assets/            # Assets directory
│   │   ├── css/           # TailwindCSS configuration
│   │   │   └── main.css   # Main CSS file
│   │   └── images/        # Image assets
│   │       ├── construction-castle.jpg
│   │       ├── favicon-large.png.png
│   │       ├── favicon-small.png.png
│   │       ├── mahogany-wood.jpg
│   │       └── scroll.png
│   ├── components/        # Vue components (feature-based organization)
│   │   ├── Layout/        # Layout and navigation components
│   │   │   └── AppHeader.vue # Responsive header with auth states
│   │   ├── Timer/         # Timer feature components
│   │   │   ├── TimerStats.vue     # Current timer display with SteamClock
│   │   │   ├── TimerListItem.vue  # Individual timer row component
│   │   │   ├── TimerEditModal.vue # Edit timer modal with validation
│   │   │   ├── TimerResetModal.vue# Quick reset modal
│   │   │   ├── TimerDisplayTab.vue # Timer display tab component
│   │   │   ├── TimerControlsTab.vue # Timer controls tab component
│   │   │   ├── PhraseSuggestionsTab.vue # Phrase suggestions tab component
│   │   │   ├── PhraseFilterTab.vue # Phrase filtering tab component
│   │   │   ├── SuggestionHistoryTab.vue # Suggestion history tab component
│   │   │   └── TabNavigation.vue # Tab navigation component
│   │   ├── Phrases/       # Phrases feature components
│   │   │   └── RandomPhrase.vue  # Random phrase display component
│   │   ├── Profile/       # Profile management components
│   │   │   ├── AccountInformationForm.vue # Account info editing form
│   │   │   └── SecurityForm.vue # Password change form
│   │   ├── Admin/         # Admin panel components
│   │   │   ├── AdminPanel.vue           # Main admin panel with tabs
│   │   │   ├── AdminTabNavigation.vue   # Tab navigation component
│   │   │   ├── OverviewTab.vue          # System statistics display
│   │   │   ├── UsersTab.vue             # User management interface
│   │   │   ├── PhraseSuggestionApprovalTab.vue # Phrase moderation interface
│   │   │   └── UserSearchBox.vue        # User search component
│   │   └── Steampunk/     # Steampunk design system components
│   │       ├── SteamClock.vue     # Main steampunk timer display
│   │       ├── FlippingDigit.vue  # Animated flip-digit component
│   │       ├── SlidingTimeGroup.vue # Time unit group container
│   │       ├── SteampunkBackground.vue # Mahogany wood + gears background
│   │       ├── SteampunkBanner.vue # Gold enamel motto plaque
│   │       └── VintageNoteCard.vue # Vintage scroll note display
│   ├── pages/             # File-based routing
│   │   ├── index.vue      # Homepage with gothic construction theme
│   │   ├── about.vue      # About page (placeholder)
│   │   ├── login.vue      # Authentication login page
│   │   ├── register.vue   # User registration with dynamic URL preview
│   │   ├── profile.vue    # User profile management page
│   │   ├── incidents.vue  # Protected CRUD management
│   │   ├── admin/         # Admin panel pages
│   │   │   └── index.vue  # Admin panel main page
│   │   └── [user_slug]/
│   │       └── incident-timer.vue # Public timer display
│   ├── stores/            # Pinia stores with embedded actions for SSR hydration
│   │   ├── incident-timers.ts # Timer state management with actions
│   │   ├── phrases.ts     # Phrase state management with actions
│   │   ├── admin.ts       # Admin state management with actions
│   │   ├── incident-timers.spec.ts # Timer store tests
│   │   ├── phrases.spec.ts # Phrase store tests
│   │   └── admin.spec.ts  # Admin store tests
│   ├── middleware/        # Route middleware
│   │   ├── auth.ts        # Route protection
│   │   └── admin.ts       # Admin route protection
│   ├── composables/       # Composition API utilities
│   │   ├── useAuthActions.ts # Authentication operations (only remaining action composable)
│   │   ├── useAuthProfileActions.ts # Profile management operations
│   │   ├── useAuthFetch.ts # HTTP client with auth interceptors
│   │   ├── useBackendFetch.ts # Direct backend client with automatic JWT management
│   │   ├── useJwtManager.ts # JWT token management with automatic refresh
│   │   ├── useBaseService.ts # Base service utilities (loading states, error handling)
│   │   ├── useSmartFetch.ts # Smart routing fetcher with SSR/client detection
│   │   ├── useSessionWatcher.ts # Session state watcher for automatic cleanup
│   │   └── useCallOnceWatcher.ts # One-time watcher utility
│   ├── services/          # Pure services with curried dependency injection
│   │   ├── authService.ts # Authentication operations
│   │   ├── authProfileService.ts # Profile operations
│   │   ├── phraseService.ts # Phrase operations
│   │   ├── adminService.ts # Admin operations
│   │   └── incidentTimerService.ts # Timer operations
│   ├── utils/             # Utility functions
│   │   └── timer-manager.ts # Timer management utilities
│   ├── types/             # TypeScript definitions
│   │   └── phrases.ts     # Phrases type definitions
│   ├── constants/         # Application constants
│   ├── layouts/           # Layout components
│   └── plugins/           # Nuxt plugins
├── shared/                # Shared utilities and types
│   ├── types/             # Shared type definitions
│   │   ├── admin.ts       # Admin type definitions
│   │   ├── api-routes.ts  # API route type definitions
│   │   ├── auth.d.ts      # Authentication type definitions
│   │   ├── auth.ts        # Authentication types
│   │   ├── common.ts      # Common type definitions
│   │   ├── index.ts       # Type exports
│   │   ├── phrases.ts     # Phrases type definitions
│   │   ├── timers.ts      # Timer type definitions
│   │   └── ui.ts          # UI type definitions
│   ├── utils/             # Shared utility functions
│   │   ├── jwt.ts         # JWT token utilities
│   │   └── validation.ts  # Validation utilities
│   ├── config/            # Configuration files
│   │   └── api-routes.ts  # Centralized API route definitions
│   └── schemas/           # Validation schemas
│       ├── auth.ts        # Authentication validation schemas
│       ├── index.ts       # Schema exports
│       ├── phrases.ts     # Phrase validation schemas
│       └── timers.ts      # Timer validation schemas
├── server/                # Server API routes
│   ├── api/               # API endpoint handlers
│   │   ├── auth/          # Authentication endpoints
│   │   │   ├── debug-refresh-token.get.ts # Debug refresh token endpoint
│   │   │   ├── jwt.get.ts # Get current JWT token
│   │   │   ├── login.post.ts # Login endpoint
│   │   │   ├── logout.post.ts # Logout endpoint
│   │   │   ├── me.get.ts  # Get current user
│   │   │   ├── register.post.ts # Registration endpoint
│   │   │   └── session-check.get.ts # Session validation
│   │   ├── phrases/       # Phrases endpoints
│   │   │   └── random.get.ts # Get random phrase
│   │   ├── [user_slug]/   # Dynamic user routes
│   │   │   ├── incident-timer.get.ts # Public timer display
│   │   │   └── phrase.get.ts # Public phrase display
│   │   ├── incident-timers.get.ts # Get user timers
│   │   ├── health.get.ts  # Health check
│   │   ├── simple.ts      # Simple test endpoint
│   │   ├── test/          # Test endpoints
│   │   │   ├── clear-jwt.post.ts # Clear JWT test endpoint
│   │   │   └── session-state.get.ts # Session state test endpoint
│   │   └── test.get.ts    # Test endpoint
│   └── utils/             # Server utilities
│       ├── client-ip.ts   # Client IP utilities
│       └── jwt-handler.ts # JWT handling utilities
├── nuxt.config.ts         # Nuxt configuration
├── package.json           # Dependencies
├── Dockerfile             # Production container
├── Dockerfile.dev         # Development container
├── tsconfig.json          # TypeScript config
├── vitest.config.ts       # Vitest configuration
└── README.md              # Frontend documentation
```

## Core Features

### Authentication
- **Implementation**: JWT-based with refresh tokens
- **Session Management**: nuxt-auth-utils for secure session handling
- **Route Protection**: Middleware-based authentication
- **Details**: See [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md#frontend-security)

### Incident Timer System
- **Interface**: 5-tab dashboard (display, controls, suggestions, filtering, history)
- **CRUD Operations**: Full timer management with ownership validation
- **Public Sharing**: Shareable URLs at `/{user_slug}/incident-timer`
- **Design**: Steampunk aesthetic with flip-clock animation

### Phrases System
- **Display**: Random motivational phrases
- **User Features**: Submit suggestions, filter phrases, track status
- **Admin Features**: Approve/reject suggestions, manage phrases
- **State Management**: Pinia stores for data management

### Admin Panel System
- **User Management**: Search users, deactivate accounts, reset passwords, promote to admin
- **Phrase Moderation**: Review and approve/reject user suggestions
- **System Statistics**: Overview of users, phrases, and pending suggestions
- **Access Control**: Admin-only routes with role validation via admin middleware
- **Tab Navigation**: URL-synchronized tab navigation with state persistence
- **Admin Interface**: Clean, minimal design following authentication page styling

### Design System
Page-specific aesthetics following [UX-LAYOUT.md](UX-LAYOUT.md):
- **Homepage**: Sacred/Gothic construction theme
- **Authentication**: Minimal with sacred elements
- **Incidents**: Technology/geometric patterns
- **Public Timer**: Full steampunk aesthetic
- **About**: Frontier/Nature with Japanese influences

## Component Architecture

**Refactored Architecture**: All components have been migrated to the new store-based actions pattern, eliminating event emission antipatterns and improving maintainability. The data layer has been completely refactored for SSR hydration with stores containing embedded actions and curried services.

**Layout**: AppHeader.vue (responsive header with auth states and mobile menu) ✅ **Refactored**

**Timer Components** (10 components) ✅ **All Refactored**:
- TimerStats.vue, TimerListItem.vue, TimerEditModal.vue, TimerResetModal.vue
- TimerDisplayTab.vue, TimerControlsTab.vue, PhraseSuggestionsTab.vue, PhraseFilterTab.vue, SuggestionHistoryTab.vue, TabNavigation.vue
- **Pattern**: All use `useIncidentTimerStore()` with embedded actions for SSR hydration

**Profile Components** (2 components) ✅ **All Refactored**:
- AccountInformationForm.vue (display name and slug editing with validation)
- SecurityForm.vue (password change with current password verification)
- **Pattern**: All use `useAuthProfileActions()` composable (only remaining action composable)

**Admin Components** (6 components) ✅ **All Refactored**:
- AdminPanel.vue (main admin interface with tabbed navigation and URL state management)
- AdminTabNavigation.vue (tab navigation with URL synchronization)
- OverviewTab.vue (system statistics dashboard)
- UsersTab.vue (user management with search and actions)
- PhraseSuggestionApprovalTab.vue (phrase suggestion moderation)
- UserSearchBox.vue (user search functionality)
- **Pattern**: All use `useAdminStore()` with embedded actions for SSR hydration

**Auth Components** (3 components) ✅ **All Refactored**:
- login.vue, register.vue, AppHeader.vue
- **Pattern**: All use `useAuthActions()` composable + `useAuthStore()` for session management

**Steampunk Design** (6 components):
- SteamClock.vue, FlippingDigit.vue, SlidingTimeGroup.vue, SteampunkBackground.vue, SteampunkBanner.vue, VintageNoteCard.vue

**Phrases**: RandomPhrase.vue (random phrase display component) ✅ **Refactored**

**Page Structure**: 5-tab interface in incidents.vue with TimerDisplayTab, TimerControlsTab, PhraseSuggestionsTab, PhraseFilterTab, SuggestionHistoryTab ✅ **All Refactored**

**Migration Status**: All components (100% complete) - All components now use store-based actions pattern for SSR hydration

## Steampunk Design System
- **Flip Clock**: FlippingDigit.vue (split-flap animation), SlidingTimeGroup.vue (time units), SteamClock.vue (main assembly)
- **Visual Elements**: SteampunkBackground.vue (mahogany wood + gears), SteampunkBanner.vue (gold plaque with phrases), VintageNoteCard.vue (scroll notes)
- **Animations**: Split-flap mechanics, gear synchronization, slide transitions, engraved text effects

## Architecture Patterns

### Hybrid API Architecture
- **SSR Routes**: `/api/*` for server-side data fetching
- **Direct API**: `/backend/*` for client-side mutations
- **Authentication**: See [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md#session-security)
- **Details**: See [ARCHITECTURE.md](ARCHITECTURE.md#data-flow-architecture)

### State Management

**Refactored Architecture**: Stores with embedded actions for SSR hydration and rendering. Actions are embedded within stores to enable proper server-side rendering and client-side hydration.

**Stores with Embedded Actions (3 Complete):**
- `stores/phrases.ts` - Phrase state management with embedded actions for SSR hydration
- `stores/admin.ts` - Admin state management with embedded actions for SSR hydration  
- `stores/incident-timers.ts` - Timer state management with embedded actions for SSR hydration

**Action Composables (2 Remaining):**
- `useAuthActions.ts` - Authentication operations orchestration (session management)
- `useAuthProfileActions.ts` - Profile management operations orchestration

**Architecture Benefits:**
- **SSR Hydration**: Stores with embedded actions enable proper server-side rendering
- **State Persistence**: Actions within stores maintain state across SSR/client boundary
- **Easy Testing**: Each store can be tested in isolation with comprehensive mocking
- **Better Patterns**: Direct store action calls instead of event emissions
- **Maintainable**: Clean separation of concerns with actions co-located with state
- **Testable**: Store actions enable comprehensive unit testing with embedded service calls

**Forms**: All forms use VeeValidate + Yup validation

### Service Architecture

**Refactored Architecture Pattern:**
The frontend has been completely refactored to use stores with embedded actions and curried services for maximum testability and SSR hydration:

```
Components <-> Stores (with embedded actions) <-> Curried Services <-> Smart Fetch
```

**Stores with Embedded Actions (3 Complete):**
- `stores/phrases.ts` - Phrase state management with embedded actions
- `stores/admin.ts` - Admin state management with embedded actions
- `stores/incident-timers.ts` - Timer state management with embedded actions

**Action Composables (2 Remaining):**
- `useAuthActions.ts` - Authentication operations orchestration (session management)
- `useAuthProfileActions.ts` - Profile management operations orchestration

**Curried Services with Dependency Injection (5 Complete):**
- `services/authService.ts` - Authentication operations
- `services/authProfileService.ts` - Profile operations
- `services/phraseService.ts` - Phrase operations
- `services/adminService.ts` - Admin operations
- `services/incidentTimerService.ts` - Timer operations

**Smart Fetch System:**
- `useSmartFetch.ts` - Smart routing fetcher with SSR/client detection
- `useSessionWatcher.ts` - Session state watcher for automatic cleanup
- `useJwtManager.ts` - JWT token management with automatic refresh

**Timer Management Utilities:**
- `utils/timer-manager.ts` - Timer utilities (live updates, calculations, formatting)

**Architecture Benefits:**
- **SSR Hydration**: Stores with embedded actions enable proper server-side rendering
- **State Persistence**: Actions within stores maintain state across SSR/client boundary
- **Easy Testing**: Each layer can be tested in isolation with comprehensive mocking
- **Reusable Services**: Curried services can be used outside Vue context
- **Better Component Patterns**: Direct store action calls instead of event emissions
- **Maintainable**: Clean separation of concerns with actions co-located with state
- **Testable**: Curried services enable easy mocking and comprehensive test coverage

**Centralized API Routes:**
- **Configuration**: `frontend/shared/config/api-routes.ts` - Single source of truth for all endpoints
- **Categorization**: PUBLIC (no auth), PROTECTED (JWT required), API (SSR passthrough)
- **Type Safety**: TypeScript support with route parameter functions
- **Maintainability**: Centralized route definitions eliminate duplication

**Smart Fetch System:**
- **Server-Side**: Uses `useRequestFetch()` for SSR-safe requests with cookie forwarding
- **Client-Side**: Uses `$fetch` for standard client-side requests
- **JWT Management**: Automatic JWT token addition for protected requests only
- **URL Routing**: SSR uses internal Docker network, Client uses nginx proxy
- **Smart Routing**: Automatically chooses passthrough vs direct based on route config

**API Route Usage:**
- `API_ROUTES.PUBLIC.*` - Public endpoints (no auth needed)
- `API_ROUTES.PROTECTED.*` - Protected endpoints (JWT required)
- `API_ROUTES.API.*` - SSR proxy routes (session-based)

**Store with Embedded Actions Pattern:**

```typescript
// ✅ CORRECT: Store with embedded actions for SSR hydration
export const useMyStore = defineStore('my-store', () => {
  const data = ref<MyData[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  
  // Service instance with curried dependency injection
  const smartFetch = useSmartFetch()
  const myService = myService(smartFetch)
  
  // Embedded actions for SSR hydration
  const loadData = async () => {
    isLoading.value = true
    error.value = null
    
    try {
      const result = await myService.getData()
      data.value = result
    } catch (err) {
      error.value = 'Failed to load data'
    } finally {
      isLoading.value = false
    }
  }
  
  const updateSomething = async (id: string, updates: any) => {
    isLoading.value = true
    error.value = null
    
    try {
      const result = await myService.updateSomething(id, updates)
      const index = data.value.findIndex(item => item.id === id)
      if (index !== -1) {
        data.value[index] = { ...data.value[index], ...result }
      }
    } catch (err) {
      error.value = 'Failed to update data'
    } finally {
      isLoading.value = false
    }
  }
  
  return {
    data,
    isLoading,
    error,
    loadData,
    updateSomething
  }
})

// ✅ CORRECT: Component usage
const { data, loadData, isLoading } = useMyStore()
await loadData() // Store action calls curried service
```

**Service Currying Pattern:**

```typescript
// ✅ CORRECT: Curried service for testability
export const myService = (fetcher: FetcherFunction) => ({
  getData: () => fetcher('/protected/data'),
  updateSomething: (id: string, updates: any) => 
    fetcher(`/protected/data/${id}`, { method: 'PUT', body: updates })
})

// ✅ CORRECT: Easy testing with mock fetcher
const mockFetcher = vi.fn()
const service = myService(mockFetcher)
await service.getData()
expect(mockFetcher).toHaveBeenCalledWith('/protected/data')
```

**Environment Configuration:**
- `NUXT_API_BASE=http://backend:8080/backend` (SSR - internal Docker network)
- `NUXT_PUBLIC_API_BASE=https://localhost/backend` (CSR - browser access)
- `useSmartFetch()` automatically handles routing and JWT tokens

### Form Validation Standards
- **Required**: All forms must use VeeValidate + Yup for validation
- **Consistency**: Standardized validation patterns across authentication, timers, and phrases
- **User Experience**: Real-time validation with clear error messaging

## Development Environment

### Running the Frontend
```bash
./scripts/dev-start.sh frontend    # Start with hot reload
./scripts/dev-logs.sh frontend     # View logs
```

### Development Features
- **HMR**: Instant Vue/TypeScript updates
- **State Preservation**: Component hot swapping
- **Style Updates**: Real-time TailwindCSS changes

## API Integration

### Server Routes (SSR)
Proxy routes in `server/api/` for server-side rendering:
- **Authentication**: JWT token management
- **Data Fetching**: Timer and phrase data
- **Health Checks**: Service monitoring

### Backend Integration
- **Endpoints**: See [IMPLEMENTATION-DATA-CONTRACTS.md](IMPLEMENTATION-DATA-CONTRACTS.md)
- **Authentication**: See [IMPLEMENTATION-SECURITY.md](IMPLEMENTATION-SECURITY.md#frontend-security)
- **Architecture**: See [ARCHITECTURE.md](ARCHITECTURE.md#data-flow-architecture)