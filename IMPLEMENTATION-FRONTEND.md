# Frontend Implementation

## Overview
Full-featured Nuxt.js 4.0.3 frontend with authentication, incident timer features, responsive design, and proper UX/layout architecture. Built with Nuxt 4 directory structure and fully integrated with the Rust backend.

## Technology Stack
- **Framework**: Nuxt.js 4.0.3 (latest stable)
- **Node.js**: 20+ (even-numbered version)
- **TypeScript**: Full support with strict mode
- **State Management**: Pinia (built-in)
- **Styling**: TailwindCSS 6.14.0
- **Form Validation**: VeeValidate 4.15.1 + Yup 1.7.0
- **Utilities**: VueUse 13.8.0
- **HTTP Client**: Custom composables with interceptors (Vue 3 best practices)

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
│   │   │   └── TimerResetModal.vue# Quick reset modal
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
│   │   ├── auth.ts        # Authentication state
│   │   └── incident-timers.ts # Timer management state
│   ├── middleware/        # Route middleware
│   │   └── auth.ts        # Route protection
│   ├── plugins/           # Nuxt plugins
│   │   └── auth.client.ts # Client-side auth initialization
│   ├── composables/       # Composition API logic
│   │   ├── useAuthFetch.ts # HTTP client with auth interceptors
│   │   ├── useAuthService.ts # Authentication operations
│   │   ├── useBackendFetch.ts # Direct backend client with automatic JWT management
│   │   ├── useJwtManager.ts # JWT token management with automatic refresh
│   │   └── useIncidentTimerService.ts # Timer CRUD operations
│   ├── layouts/           # Application layouts
│   └── types/             # TypeScript definitions
├── nuxt.config.ts         # Nuxt configuration
├── package.json           # Dependencies
├── Dockerfile             # Production container
└── tsconfig.json          # TypeScript config
```

## Current Features

### Authentication System
- **Registration Page** (`/register`): Email, display name, password with VeeValidate validation
- **Login Page** (`/login`): Email/password authentication with error handling
- **JWT Token Management**: Rolling refresh tokens with simplified pre-request checking system
- **Route Protection**: Middleware-based authentication for protected pages
- **Authentication Store**: Complete Pinia store with login/register/logout operations
- **Auth Plugin**: Client-side initialization after Pinia is ready
- **Token Refresh**: Automatic refresh when token expires within 1-minute threshold

### User Interface & Navigation
- **Responsive Header**: Sticky navigation with mobile hamburger menu
- **Authentication States**: Different UI for authenticated vs unauthenticated users
- **Avatar Dropdown**: User initial display with account menu
- **Navigation Links**: About, Incidents with active state indicators
- **Mobile-First Design**: Fully responsive across all breakpoints

### Page Implementation
- **Homepage** (`/`): Gothic construction theme with optimized image
- **About Page** (`/about`): Placeholder with frontier/traditional aesthetic
- **Login/Register**: Complete forms with VeeValidate + Yup validation
- **Incidents Management** (`/incidents`): Protected CRUD interface
- **Public Timer Display** (`/{user_slug}/incident-timer`): Real-time timer display

### Incident Timer Features
- **CRUD Operations**: Create, read, update, delete incident timers
- **Real-time Display**: Live timer updates every second with steampunk flip-clock animation
- **Public Access**: Shareable URLs for public timer viewing with full steampunk aesthetic
- **Timer Management**: History, notes, reset functionality
- **State Management**: Complete Pinia store for timer operations
- **Steampunk Design**: Gold engraved flip cards, mahogany wood background, animated gears, vintage scroll notes

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

### Feature-Based Organization
Components are organized by domain/feature following Vue 3/Nuxt best practices:

**Layout Components** (`components/Layout/`)
- **AppHeader.vue**: Responsive header with authentication states, mobile hamburger menu, and user avatar dropdown

**Timer Feature Components** (`components/Timer/`)
- **TimerStats.vue**: Current timer display integrating the SteamClock component for beautiful time visualization
- **TimerListItem.vue**: Individual timer row with edit/delete actions and live time display
- **TimerEditModal.vue**: Modal for editing timers with VeeValidate + Yup validation and datetime conversion
- **TimerResetModal.vue**: Quick reset modal with optional notes input

**Steampunk Design System** (`components/Steampunk/`)
- **SteamClock.vue**: Main steampunk timer display with animated gears and flip-digit mechanics
- **FlippingDigit.vue**: Individual animated split-flap digits with gold engraved styling
- **SlidingTimeGroup.vue**: Time unit containers with mechanical slide animations
- **SteampunkBackground.vue**: Mahogany wood texture with rotating gear overlays
- **SteampunkBanner.vue**: Gold enamel plaque with etched motto
- **VintageNoteCard.vue**: Scroll-style note display with aging effects

### Refactored Page Architecture
The incidents page has been refactored from a monolithic 425+ line component into orchestrated, focused components:
- **Main incidents.vue** (~180 lines): Orchestrates components and manages state
- **Component Communication**: Clean props/emits interfaces between components
- **Single Responsibility**: Each component has one focused purpose
- **Improved Maintainability**: Easier to test, debug, and extend individual features

## Steampunk Component Architecture

### Flip Clock Components
- **FlippingDigit.vue**: Individual split-flap digit with authentic mechanical animation
  - Gold plate background with engraved dark text effects
  - CSS-based flip animation using `data-value` attributes and pseudo-elements
  - Perfect timing replicating real flip/flap clock mechanics
- **SlidingTimeGroup.vue**: Container for time units with mechanical slide animations
  - Handles Y/M/W/D/H/M/S labels with proper padding to 2-digit display
  - Smooth slide-in/out animations when time units appear/disappear
- **SteamClock.vue**: Main clock assembly with decorative elements
  - Silver frame with rivets and animated clockwork gears
  - Blue enamel face with proper time unit organization

### Steampunk Visual Elements
- **SteampunkBackground.vue**: Mahogany wood texture with rotating gear overlays
  - Real mahogany wood image (`~/assets/images/mahogany-wood.jpg`) as tiled background
  - Animated gears with proper teeth and mechanical timing
- **SteampunkBanner.vue**: Gold enamel plaque with etched motto
  - "Vigilance Maintained - Until the Next Challenge Arises"
  - Wood-mounted plaque with brass screws and engraved text effects
- **VintageNoteCard.vue**: Scroll-style note display
  - Vintage scroll background image with handwritten-style text
  - Proper aging effects and realistic paper texture

### Animation System
- **Split-flap Mechanics**: Authentic two-phase flip animation (top flips down, bottom slides up)
- **Gear Synchronization**: Gears tick with seconds, spin with minutes
- **Slide Transitions**: Mechanical slide-in/out for time units with 3D perspective
- **Engraving Effects**: Multi-layer text shadows creating realistic etched metal appearance

## Architecture Implementation

### HTTP Client Architecture - Hybrid API Pattern
Modern composable-based HTTP client with dual API call patterns:

**SSR Proxy Pattern** (`$fetch('/api/...')`):
- **Initial Data Loading**: Uses Nuxt server proxy for SSR optimization
- **Public Endpoints**: Non-authenticated calls routed through server
- **Examples**: `getUserTimers()`, `getPublicTimer()`

**Direct Backend Pattern** (`backendFetch('/...')`):
- **Client Mutations**: Direct calls to backend with JWT in Authorization header
- **Real-time Operations**: POST/PUT/DELETE operations bypass proxy for performance
- **Examples**: `createTimer()`, `updateTimer()`, `deleteTimer()`

**Authentication Architecture - Simplified Refresh System**:
- **JWT (Access Token)**: Stored in client memory via `jwtManager.getToken()` with 1-hour expiration
- **Refresh Token**: Secure httpOnly cookie managed by Nuxt server with 1-week expiration (rolling)
- **Pre-Request Check**: JWT manager checks token expiration before each API call (1-minute threshold)
- **Automatic Refresh**: Seamless token renewal via `/api/auth/refresh` endpoint when needed
- **Direct Backend Auth**: `useBackendFetch()` automatically adds `Authorization: Bearer` header

### State Management
- **Pinia Integration**: Modern Vue 3 state management
- **Authentication Store**: User state, token management, login/logout operations
- **Timer Store**: CRUD operations, real-time updates, timer history
- **Context Handling**: Proper initialization sequence to avoid Pinia context errors

### Form Handling
- **VeeValidate Integration**: Modern form validation with `useForm` and `handleSubmit`
- **Yup Schema Validation**: Type-safe validation schemas
- **Real-time Validation**: Instant feedback during form input
- **Dynamic URL Preview**: Real-time slug generation preview during registration

## Development Environment

### Running the Frontend
The frontend is typically run through development scripts:
```bash
# Start with hot reload (recommended)
./scripts/dev-start.sh frontend

# View frontend logs
./scripts/dev-logs.sh frontend

# Direct npm commands (if needed)
cd frontend
npm run dev
npm run build
```

### Hot Module Replacement
- **Vue/TypeScript Changes**: Update instantly without page refresh
- **Component Updates**: Real-time component hot swapping
- **Style Changes**: TailwindCSS updates apply immediately
- **State Preservation**: Component state maintained during updates

### Environment Configuration
Located in `frontend/.env` and Nuxt configuration:
- API base URL configuration
- Development vs production settings
- TailwindCSS and build optimization settings
- **Component auto-import**: Configured for nested directories with `pathPrefix: false`

## Integration with Backend

### API Integration
- **Automatic Authentication**: JWT tokens handled transparently
- **Error Handling**: Comprehensive error state management
- **Type Safety**: TypeScript interfaces aligned with backend contracts
- **Real-time Updates**: Timer displays update automatically

### Route Structure
- **Protected Routes**: `/incidents` requires authentication
- **Public Routes**: Homepage, about, login, register, public timer display
- **Dynamic Routes**: `/{user_slug}/incident-timer` for public timer access

For detailed API contracts, see [IMPLEMENTATION-DATA-CONTRACTS.md](IMPLEMENTATION-DATA-CONTRACTS.md).

## Docker Configuration

### Production Build
Multi-stage Dockerfile optimized for production:
- **Node.js 20**: Even-numbered LTS version for stability
- **Build Optimization**: Efficient production build process
- **Security**: Non-root user execution
- **Health Checks**: Container health monitoring

### Development Integration
Designed for seamless integration with Docker Compose development environment and hot reload functionality.

## Asset Management

### Image Assets
- **Optimized Images**: Build-time optimization for web delivery
- **Responsive Images**: Multiple sizes for different screen densities
- **Nuxt 4 Asset Handling**: Proper `~/assets/` path resolution

### CSS Architecture
- **TailwindCSS**: Utility-first CSS framework
- **Custom Design System**: Project-specific color palette and typography
- **Responsive Design**: Mobile-first approach with content-driven breakpoints

---

*This document describes the current frontend implementation. For future enhancements and planned features, see [ROADMAP.md](ROADMAP.md). For design guidelines, see [UX-LAYOUT.md](UX-LAYOUT.md).*