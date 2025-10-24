# Frontend Implementation

## Overview
Nuxt.js 4.0.3 frontend with Vue 3, TypeScript, and SSR support.

## Technology Stack Decisions

### Core Technologies
- **Framework**: Nuxt.js 4.0.3 + Vue 3 + TypeScript
  - Why: SSR for SEO, file-based routing, TypeScript for type safety
- **State Management**: Pinia + nuxt-auth-utils
  - Why: Composition API compatible, SSR support, separate session management
- **Styling**: TailwindCSS
  - Why: Utility-first, responsive by default, fast iteration
- **Forms**: VeeValidate + Yup
  - Why: Type-safe validation, async rules, consistent error handling
- **Testing**: Vitest + Vue Test Utils
  - Why: Fast, Vite-native, component testing support

## Architecture Decisions

### Hybrid API Architecture
Dual-path API strategy for SSR and client-side needs:

**SSR Proxy Pattern** (`/api/*`)
- Server-side data fetching for initial page loads
- Session-based authentication (cookies)
- Nuxt server acts as proxy to backend

**Direct Backend Pattern** (`/backend/*`)
- Client-side API calls for mutations
- JWT authentication in request headers
- Direct routing through nginx to backend

**Why hybrid:**
- SSR needs cookies (secure, httpOnly)
- Client mutations need JWT (stateless, flexible)
- Each optimized for its use case

**Trade-offs:**
- Two auth mechanisms to maintain
- Worth it: SSR performance + client flexibility

### Store Architecture (SSR Hydration Pattern)

**Decision**: Actions embedded within stores, not separate composables

**Pattern:**
```
Components → Stores (with actions) → Services → API
```

**Why:**
- SSR Hydration: Actions in stores survive server→client handoff
- State Persistence: Reactive state + actions together
- No event emissions: Direct action calls

**Alternative rejected:**
- Action composables: Don't hydrate properly in SSR
- Caused state mismatches between server/client renders

**Trade-offs:**
- Larger store files (state + actions)
- Worth it: SSR actually works correctly

**Exceptions:**
- Auth actions: Session management requires composable for lifecycle hooks
- Profile actions: Composable for form-specific orchestration

### Service Layer Pattern

**Decision**: Curried services with dependency injection

**Pattern:**
```typescript
// Service takes fetch function as parameter
export const myService = (fetcher) => ({
  getData: () => fetcher('/endpoint'),
  updateData: (data) => fetcher('/endpoint', { method: 'PUT', body: data })
})

// Usage in store
const smartFetch = useSmartFetch()
const service = myService(smartFetch)
```

**Why:**
- Testability: Easy to mock fetcher function
- Reusability: Services work outside Vue context
- Flexibility: Swap fetcher implementation

**Trade-offs:**
- Extra function call wrapper
- Worth it: Tests don't need real API

### Smart Fetch System

**Decision**: Automatic SSR/client detection with environment-aware routing

**Behavior:**
- Server-side: Uses internal Docker network
- Client-side: Uses nginx proxy
- Automatic JWT token management
- Cookie forwarding for SSR

**Why:**
- Services don't know about environment
- SSR can't access localhost from container
- Client can't access internal Docker network

**Implementation:**
- Detects `import.meta.server` vs `import.meta.client`
- Routes to appropriate API base URL
- Handles JWT injection automatically

### Centralized API Routes

**Decision**: Single source of truth for all endpoint definitions

**Location**: `shared/config/api-routes.ts`

**Categories:**
- PUBLIC: No auth required
- PROTECTED: JWT required
- API: SSR proxy routes

**Why:**
- No magic strings in code
- Type safety for routes
- Single place to update endpoints
- Self-documenting API surface

### Component Organization

**Decision**: Feature-based, not type-based

**Structure:**
- `components/Timer/` - All timer components
- `components/Admin/` - All admin components
- `components/Profile/` - All profile components

**Why:**
- Easier to find related components
- Clearer feature boundaries
- Co-locate feature-specific logic

**Alternative rejected:**
- Type-based (`modals/`, `forms/`, `cards/`)
- Gets unwieldy as features grow

### Form Validation Standards

**Decision**: VeeValidate + Yup required for all forms

**Why:**
- Consistent validation experience
- Type-safe with TypeScript
- Async validation support
- Composable pattern works with Composition API

**Enforcement:**
- No raw form submissions
- Yup schemas define validation rules
- VeeValidate handles UI feedback

## Design System Decisions

### Page-Specific Aesthetics
Different pages have different themes (see UX-LAYOUT.md):

**Why:**
- Each section has unique mood/purpose
- Gothic for construction, steampunk for timers
- Visual variety maintains interest

**Trade-offs:**
- More CSS, more components
- Worth it: Memorable user experience

### Steampunk Components
Custom flip-clock, gears, mahogany backgrounds:

**Decision**: Build custom, don't use library

**Why:**
- Unique aesthetic matches brand
- Full control over animations
- Learning opportunity

**Trade-offs:**
- More implementation time
- Worth it: Distinctive design

## Route Protection

**Middleware Pattern:**
- `auth.ts`: Redirects unauthenticated users to login
- `admin.ts`: Checks for admin role

**Why middleware:**
- Declarative protection in route definitions
- Runs before page load
- Consistent behavior across routes

## State Management Philosophy

**Pinia for application state:**
- Timers, phrases, admin data
- Persists across navigation

**nuxt-auth-utils for session:**
- User authentication state
- Secure session management
- Automatic refresh handling

**Why separate:**
- Session is special: Security-critical
- Application state is different: Feature data
- Each optimized for its purpose

## Development Features

### Hot Module Replacement
Development scripts provide instant updates:
- Vue components update without reload
- TailwindCSS updates instantly
- TypeScript errors show immediately

**Configuration:**
- WebSocket through nginx SSL
- Development container mounts code
- cargo-watch + Nuxt HMR

### Testing Strategy
See [IMPLEMENTATION-TESTING.md](IMPLEMENTATION-TESTING.md) for testing philosophy and paradigm-based approach.

## Key Patterns

### No Event Emission for Data Mutations
**Decision**: Components call store actions directly

**Why:**
- Events are for DOM interactions
- Data mutations should be direct
- Easier to trace data flow

**Alternative rejected:**
- Event-driven architecture
- Hard to debug, unclear flow

### Session Watcher
**Pattern**: Automatic cleanup on session changes

**Why:**
- Logout should clear all user data
- Prevents stale data after re-login
- Centralized cleanup logic

### JWT Manager
**Pattern**: Automatic token refresh before expiration

**Why:**
- Seamless user experience
- No sudden logouts
- Handles refresh token rotation

**Implementation:**
- Checks expiration on API calls
- Refreshes proactively
- Falls back to login on refresh failure
