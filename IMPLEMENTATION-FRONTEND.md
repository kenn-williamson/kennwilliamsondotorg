# Frontend Implementation

## Overview
Nuxt.js 4.0.3 frontend with authentication, incident timer features, and steampunk design system. Built with Nuxt 4 directory structure and integrated with Rust backend.

## Technology Stack
- **Framework**: Nuxt.js 4.0.3 + Vue 3 + TypeScript
- **State Management**: Pinia + nuxt-auth-utils
- **Styling**: TailwindCSS 6.14.0
- **Form Validation**: VeeValidate 4.15.1 + Yup 1.7.0
- **Utilities**: VueUse 13.8.0

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
│   │   ├── incidents.vue  # Protected CRUD management
│   │   └── [user_slug]/
│   │       └── incident-timer.vue # Public timer display
│   ├── stores/            # Pinia stores
│   │   ├── incident-timers.ts # Timer management state
│   │   └── phrases.ts     # Phrases management state
│   ├── middleware/        # Route middleware
│   │   └── auth.ts        # Route protection
│   ├── composables/       # Composition API logic
│   │   ├── useAuthFetch.ts # HTTP client with auth interceptors
│   │   ├── useAuthService.ts # Authentication operations
│   │   ├── useBackendFetch.ts # Direct backend client with automatic JWT management
│   │   ├── useJwtManager.ts # JWT token management with automatic refresh
│   │   ├── useIncidentTimerService.ts # Timer CRUD operations
│   │   ├── usePhraseService.ts # Phrases CRUD operations
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

## Current Features

### Authentication & Navigation
- **Registration/Login**: Email/password with VeeValidate validation and JWT token management
- **Route Protection**: Middleware-based authentication for protected pages
- **Responsive Header**: Sticky navigation with mobile hamburger menu and avatar dropdown
- **Pages**: Homepage (gothic theme), About, Login/Register, Incidents management, Public timer display

### Incident Timer System
- **5-Tab Interface**: Timer display, controls, phrase suggestions, filtering, and history
- **CRUD Operations**: Create, read, update, delete incident timers with real-time updates
- **Public Sharing**: Shareable URLs with full steampunk aesthetic
- **Steampunk Design**: Gold engraved flip cards, mahogany wood background, animated gears

### Phrases System
- **Dynamic Display**: Random phrase selection from database with steampunk integration
- **User Workflow**: Submit suggestions, filter phrases, track approval status
- **State Management**: Pinia stores for timers and phrases management

### Design System
Page-specific aesthetic themes per [UX-LAYOUT.md](UX-LAYOUT.md):
- **Homepage**: Sacred/Gothic with construction motifs
- **Authentication**: Clean, minimal with subtle sacred elements
- **Incidents**: Technology theme with geometric patterns
- **Public Timer**: Complete steampunk aesthetic with airship cabin theme
- **About**: Frontier/Nature with Japanese traditional influences
- **Color Palette**: Sky blue primary with gold/silver accents, mahogany wood for steampunk
- **Typography**: Ornate headers, clean body text, engraved gold text for steampunk
- **Responsive Breakpoints**: Content-first approach (320px, 480px, 768px, 1024px, 1440px)

## Component Architecture

**Layout**: AppHeader.vue (responsive header with auth states and mobile menu)

**Timer Components** (10 components):
- TimerStats.vue, TimerListItem.vue, TimerEditModal.vue, TimerResetModal.vue
- TimerDisplayTab.vue, TimerControlsTab.vue, PhraseSuggestionsTab.vue, PhraseFilterTab.vue, SuggestionHistoryTab.vue, TabNavigation.vue

**Steampunk Design** (6 components):
- SteamClock.vue, FlippingDigit.vue, SlidingTimeGroup.vue, SteampunkBackground.vue, SteampunkBanner.vue, VintageNoteCard.vue

**Phrases**: RandomPhrase.vue (random phrase display component)

**Page Structure**: 5-tab interface in incidents.vue with TimerDisplayTab, TimerControlsTab, PhraseSuggestionsTab, PhraseFilterTab, SuggestionHistoryTab

## Steampunk Design System
- **Flip Clock**: FlippingDigit.vue (split-flap animation), SlidingTimeGroup.vue (time units), SteamClock.vue (main assembly)
- **Visual Elements**: SteampunkBackground.vue (mahogany wood + gears), SteampunkBanner.vue (gold plaque with phrases), VintageNoteCard.vue (scroll notes)
- **Animations**: Split-flap mechanics, gear synchronization, slide transitions, engraved text effects

## Architecture Implementation

### HTTP Client - Hybrid API Pattern
- **SSR Proxy**: Server API routes (`/api/*`) with session-based auth for initial data loading
- **Direct Backend**: Client calls (`/backend/*`) with JWT headers for mutations and real-time operations
- **Authentication**: JWT tokens in memory with automatic refresh, refresh tokens in httpOnly cookies

### State Management
- **Pinia Stores**: incident-timers.ts, phrases.ts for state management
- **Form Validation**: VeeValidate + Yup for real-time validation and error handling

## Development Environment

### Running the Frontend
```bash
./scripts/dev-start.sh frontend    # Start with hot reload
./scripts/dev-logs.sh frontend     # View logs
```

### Hot Module Replacement
- Vue/TypeScript changes update instantly
- Component hot swapping with state preservation
- TailwindCSS updates apply immediately

## Integration with Backend

### Server API Routes
**Authentication**: `GET /api/auth/jwt` (get current JWT token)
**Timers**: `GET /api/incident-timers`, `GET /api/[user_slug]/incident-timer`
**Phrases**: `GET /api/phrases/random`, `GET /api/[user_slug]/phrase`
**Health**: `GET /api/health`, `GET /api/test`

### Client API Integration
- **Hybrid Pattern**: SSR uses `/api/*` routes, CSR uses direct `/backend/*` calls
- **Authentication**: JWT tokens handled transparently via `useBackendFetch`
- **Route Structure**: Protected (`/incidents`), Public (homepage, about, login, register), Dynamic (`/{user_slug}/incident-timer`)


---

*This document describes the current frontend implementation. For future enhancements and planned features, see [ROADMAP.md](ROADMAP.md). For design guidelines, see [UX-LAYOUT.md](UX-LAYOUT.md).*