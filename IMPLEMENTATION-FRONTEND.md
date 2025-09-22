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
│   │   └── images/        # Image assets (construction-castle.jpg)
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
│   ├── stores/            # Pure Pinia stores (no service calls)
│   │   ├── incident-timers.ts # Pure timer state management
│   │   ├── phrases.ts     # Pure phrase state management
│   │   └── admin.ts       # Pure admin state management
│   ├── middleware/        # Route middleware
│   │   ├── auth.ts        # Route protection
│   │   └── admin.ts       # Admin route protection
│   ├── composables/       # Composition API logic
│   │   ├── useAuthFetch.ts # HTTP client with auth interceptors
│   │   ├── useBackendFetch.ts # Direct backend client with automatic JWT management
│   │   ├── useJwtManager.ts # JWT token management with automatic refresh
│   │   ├── useBaseService.ts # Base service utilities
│   │   ├── useAuthActions.ts # Authentication operations orchestration
│   │   ├── useAuthProfileActions.ts # Profile management operations orchestration
│   │   ├── usePhrasesActions.ts # Phrase operations orchestration
│   │   ├── useAdminActions.ts # Admin operations orchestration
│   │   └── useIncidentTimerActions.ts # Timer operations orchestration
│   ├── services/          # Pure services (no Vue context)
│   │   ├── authService.ts # Pure authentication operations
│   │   ├── authProfileService.ts # Pure profile operations
│   │   ├── phraseService.ts # Pure phrase operations
│   │   ├── adminService.ts # Pure admin operations
│   │   └── incidentTimerService.ts # Pure timer operations
│   ├── types/             # TypeScript definitions
│   │   └── phrases.ts     # Phrases type definitions
│   └── utils/             # Utility functions
│       └── dateUtils.ts   # Date formatting utilities
├── shared/                # Shared utilities and types
│   ├── types/             # Shared type definitions
│   │   └── auth.d.ts      # Authentication type definitions
│   └── utils/             # Shared utility functions
│       └── jwt.ts         # JWT token utilities
├── server/                # Server API routes
│   └── api/               # API endpoint handlers
│       ├── auth/          # Authentication endpoints
│       │   └── jwt.get.ts # Get current JWT token
│       ├── phrases/       # Phrases endpoints
│       │   └── random.get.ts # Get random phrase
│       ├── [user_slug]/   # Dynamic user routes
│       │   ├── incident-timer.get.ts # Public timer display
│       │   └── phrase.get.ts # Public phrase display
│       ├── incident-timers.get.ts # Get user timers
│       ├── health.get.ts  # Health check
│       └── test.get.ts    # Test endpoint
├── nuxt.config.ts         # Nuxt configuration
├── package.json           # Dependencies
├── Dockerfile             # Production container
└── tsconfig.json          # TypeScript config
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

**Refactored Architecture**: All 25 components have been migrated to the new action composable + pure store pattern, eliminating event emission antipatterns and improving maintainability.

**Layout**: AppHeader.vue (responsive header with auth states and mobile menu) ✅ **Refactored**

**Timer Components** (10 components) ✅ **All Refactored**:
- TimerStats.vue, TimerListItem.vue, TimerEditModal.vue, TimerResetModal.vue
- TimerDisplayTab.vue, TimerControlsTab.vue, PhraseSuggestionsTab.vue, PhraseFilterTab.vue, SuggestionHistoryTab.vue, TabNavigation.vue
- **Pattern**: All use `useIncidentTimerActions()` + `useIncidentTimerStore()`

**Profile Components** (2 components) ✅ **All Refactored**:
- AccountInformationForm.vue (display name and slug editing with validation)
- SecurityForm.vue (password change with current password verification)
- **Pattern**: All use `useAuthProfileActions()` (no store needed)

**Admin Components** (6 components) ✅ **All Refactored**:
- AdminPanel.vue (main admin interface with tabbed navigation and URL state management)
- AdminTabNavigation.vue (tab navigation with URL synchronization)
- OverviewTab.vue (system statistics dashboard)
- UsersTab.vue (user management with search and actions)
- PhraseSuggestionApprovalTab.vue (phrase suggestion moderation)
- UserSearchBox.vue (user search functionality)
- **Pattern**: All use `useAdminActions()` + `useAdminStore()`

**Auth Components** (3 components) ✅ **All Refactored**:
- login.vue, register.vue, AppHeader.vue
- **Pattern**: All use `useAuthActions()` + `useAuthStore()`

**Steampunk Design** (6 components):
- SteamClock.vue, FlippingDigit.vue, SlidingTimeGroup.vue, SteampunkBackground.vue, SteampunkBanner.vue, VintageNoteCard.vue

**Phrases**: RandomPhrase.vue (random phrase display component) ✅ **Refactored**

**Page Structure**: 5-tab interface in incidents.vue with TimerDisplayTab, TimerControlsTab, PhraseSuggestionsTab, PhraseFilterTab, SuggestionHistoryTab ✅ **All Refactored**

**Migration Status**: 25/25 components (100% complete) - All components now use action composables + pure stores pattern

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

**Refactored Architecture**: Pure stores with action composables for clean separation of concerns.

**Pure Stores (3 Complete):**
- `stores/phrases.ts` - Pure phrase state management (no service calls)
- `stores/admin.ts` - Pure admin state management (no service calls)  
- `stores/incident-timers.ts` - Pure timer state management (no service calls)

**Action Composables (5 Complete):**
- `useAuthActions.ts` - Authentication operations orchestration
- `useAuthProfileActions.ts` - Profile management operations orchestration
- `usePhrasesActions.ts` - Phrase operations orchestration
- `useAdminActions.ts` - Admin operations orchestration
- `useIncidentTimerActions.ts` - Timer operations orchestration

**Architecture Benefits:**
- **Clear Separation**: Stores only manage state, actions orchestrate services
- **Easy Testing**: Each layer can be tested in isolation
- **Better Patterns**: Direct action calls instead of event emissions
- **Maintainable**: Clean separation of concerns across all layers

**Forms**: All forms use VeeValidate + Yup validation

### Service Architecture

**Refactored Architecture Pattern:**
The frontend has been completely refactored to use a clean separation of concerns with action composables orchestrating pure services and pure stores:

```
Components <-> Action Composables <-> Pure Services + Pure Stores
```

**Action Composables (5 Complete):**
- `useAuthActions.ts` - Authentication operations orchestration
- `useAuthProfileActions.ts` - Profile management operations orchestration  
- `usePhrasesActions.ts` - Phrase operations orchestration
- `useAdminActions.ts` - Admin operations orchestration
- `useIncidentTimerActions.ts` - Timer operations orchestration

**Pure Services (5 Complete):**
- `services/authService.ts` - Pure authentication operations (no Vue context)
- `services/authProfileService.ts` - Pure profile operations (no Vue context)
- `services/phraseService.ts` - Pure phrase operations (no Vue context)
- `services/adminService.ts` - Pure admin operations (no Vue context)
- `services/incidentTimerService.ts` - Pure timer operations (no Vue context)

**Pure Stores (3 Complete):**
- `stores/phrases.ts` - Pure phrase state management (no service calls)
- `stores/admin.ts` - Pure admin state management (no service calls)
- `stores/incident-timers.ts` - Pure timer state management (no service calls)

**Architecture Benefits:**
- **Clear Separation**: Action composables orchestrate, services handle API calls, stores manage state
- **Easy Testing**: Each layer can be tested in isolation with proper mocking
- **Reusable Services**: Pure services can be used outside Vue context
- **Better Component Patterns**: Direct action calls instead of event emissions
- **Maintainable**: Clean separation of concerns across all layers

**Centralized API Routes:**
- **Configuration**: `frontend/shared/config/api-routes.ts` - Single source of truth for all endpoints
- **Categorization**: PUBLIC (no auth), PROTECTED (JWT required), API (SSR passthrough)
- **Type Safety**: TypeScript support with route parameter functions
- **Maintainability**: Centralized route definitions eliminate duplication

**JWT Management:**
- **Server-Side**: `frontend/server/utils/jwt-handler.ts` - Centralized JWT handling with automatic refresh
- **Client-Side**: `useBackendFetch()` automatically adds JWT tokens to protected routes
- **Refresh Logic**: Server-side refresh with session delegation and refresh locks
- **Error Handling**: Automatic token refresh on expiration with fallback to login

**API Route Usage:**
- `API_ROUTES.PUBLIC.*` - Public endpoints (no auth needed)
- `API_ROUTES.PROTECTED.*` - Protected endpoints (JWT required)
- `API_ROUTES.API.*` - SSR proxy routes (session-based)

**Action Composable Pattern:**

```typescript
// ✅ CORRECT: Action composable pattern
export const useMyActions = () => {
  const { executeRequest, executeRequestWithSuccess, isLoading, error, hasError } = useBaseService()
  const backendFetch = useBackendFetch()
  
  // Create service instance
  const myService = myService(backendFetch)
  
  // Destructure store methods
  const { setData, updateData } = useMyStore()
  
  const loadData = async () => {
    const data = await executeRequest(() => myService.getData(), 'loadData')
    setData(data)
    return data
  }
  
  const updateSomething = async (id: string, updates: any) => {
    const result = await executeRequestWithSuccess(
      () => myService.updateSomething(id, updates),
      'Updated successfully',
      'updateSomething'
    )
    updateData(id, result)
    return result
  }
  
  return {
    loadData,
    updateSomething,
    isLoading,
    error,
    hasError
  }
}

// ✅ CORRECT: Component usage
const { loadData, updateSomething, isLoading } = useMyActions()
await loadData() // Action calls service + updates store
```

**Environment Configuration:**
- `NUXT_API_BASE=http://backend:8080/backend` (SSR - internal Docker network)
- `NUXT_PUBLIC_API_BASE=https://localhost/backend` (CSR - browser access)
- `useBackendFetch()` automatically adds JWT tokens to protected routes only

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