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
│   ├── stores/            # Pinia stores
│   │   ├── incident-timers.ts # Timer management state
│   │   └── phrases.ts     # Phrases management state
│   ├── middleware/        # Route middleware
│   │   ├── auth.ts        # Route protection
│   │   └── admin.ts       # Admin route protection
│   ├── composables/       # Composition API logic
│   │   ├── useAuthFetch.ts # HTTP client with auth interceptors
│   │   ├── useAuthService.ts # Authentication operations
│   │   ├── useBackendFetch.ts # Direct backend client with automatic JWT management
│   │   ├── useJwtManager.ts # JWT token management with automatic refresh
│   │   ├── useIncidentTimerService.ts # Timer CRUD operations
│   │   ├── usePhraseService.ts # Phrases CRUD operations
│   │   ├── useAdminService.ts # Admin panel operations
│   │   └── useBaseService.ts # Base service utilities
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

**Layout**: AppHeader.vue (responsive header with auth states and mobile menu)

**Timer Components** (10 components):
- TimerStats.vue, TimerListItem.vue, TimerEditModal.vue, TimerResetModal.vue
- TimerDisplayTab.vue, TimerControlsTab.vue, PhraseSuggestionsTab.vue, PhraseFilterTab.vue, SuggestionHistoryTab.vue, TabNavigation.vue

**Profile Components** (2 components):
- AccountInformationForm.vue (display name and slug editing with validation)
- SecurityForm.vue (password change with current password verification)

**Admin Components** (6 components):
- AdminPanel.vue (main admin interface with tabbed navigation and URL state management)
- AdminTabNavigation.vue (tab navigation with URL synchronization)
- OverviewTab.vue (system statistics dashboard)
- UsersTab.vue (user management with search and actions)
- PhraseSuggestionApprovalTab.vue (phrase suggestion moderation)
- UserSearchBox.vue (user search functionality)

**Steampunk Design** (6 components):
- SteamClock.vue, FlippingDigit.vue, SlidingTimeGroup.vue, SteampunkBackground.vue, SteampunkBanner.vue, VintageNoteCard.vue

**Phrases**: RandomPhrase.vue (random phrase display component)

**Page Structure**: 5-tab interface in incidents.vue with TimerDisplayTab, TimerControlsTab, PhraseSuggestionsTab, PhraseFilterTab, SuggestionHistoryTab

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
- **Stores**: Pinia for reactive state (timers, phrases)
- **Forms**: All forms use VeeValidate + Yup validation

### Service Architecture

**Unified Service Pattern:**
All services use `useBaseService()` which provides:
- `backendFetch` - Direct backend calls with automatic JWT for protected routes
- `authFetch` - SSR proxy calls for auth operations
- `executeRequest()` - Wraps API calls with loading/error handling
- `executeRequestWithSuccess()` - Wraps with success messaging
- `isLoading`, `error`, `hasError` - Unified state management
- **Simplified Architecture**: Removed caching complexity (lastFetchTime, isStale, invalidateCache) for cleaner, more maintainable code

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

**Decision Tree for API Calls:**

```
Client-side API call needed?
├── Auth operation (login/register/refresh/logout)?
│   └── Use: executeRequest(() => authFetch(API_ROUTES.API.AUTH.*))
├── Backend operation (CRUD, data fetching)?
│   └── Use: executeRequest(() => backendFetch(API_ROUTES.PROTECTED.*))
└── Public operation (no auth needed)?
    └── Use: executeRequest(() => backendFetch(API_ROUTES.PUBLIC.*))

SSR data fetching in pages/components?
└── Use: useFetch('/api/*') for proper SSR support

Server API routes (server/api/*)?
└── Use: $fetch(config.apiBase + API_ROUTES.PUBLIC/PROTECTED.*)
```

**Environment Configuration:**
- `NUXT_API_BASE=http://backend:8080/backend` (SSR - internal Docker network)
- `NUXT_PUBLIC_API_BASE=https://localhost/backend` (CSR - browser access)
- `useBackendFetch()` automatically adds JWT tokens to protected routes only

**Service Implementation Examples:**

```typescript
// ✅ CORRECT: Service using unified pattern
export function useMyService() {
  const { executeRequest, executeRequestWithSuccess, backendFetch, authFetch } = useBaseService()
  
  return {
    // Protected backend call
    async getData() {
      return executeRequest(
        () => backendFetch(API_ROUTES.PROTECTED.SOMETHING.LIST),
        'getData'
      )
    },
    
    // Auth operation
    async login(credentials) {
      return executeRequestWithSuccess(
        () => authFetch(API_ROUTES.API.AUTH.LOGIN, { method: 'POST', body: credentials }),
        'Login successful',
        'login'
      )
    },
    
    // Public backend call
    async getPublicData() {
      return executeRequest(
        () => backendFetch(API_ROUTES.PUBLIC.SOMETHING.LIST),
        'getPublicData'
      )
    }
  }
}

// ✅ CORRECT: SSR data fetching in component
const { data: user } = await useFetch('/api/auth/me')

// ✅ CORRECT: Server API route
const response = await $fetch(`${config.apiBase}${API_ROUTES.PROTECTED.SOMETHING.LIST}`)

// ❌ WRONG: Raw $fetch in client-side service
const response = await $fetch('/api/something') // Use useBaseService instead

// ❌ WRONG: Manual JWT handling
const token = await jwtManager.getToken()
const response = await $fetch(url, { headers: { Authorization: `Bearer ${token}` } }) // Use backendFetch instead
```

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