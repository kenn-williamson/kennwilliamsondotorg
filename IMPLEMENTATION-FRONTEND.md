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
│   ├── components/        # Vue components
│   │   └── AppHeader.vue  # Responsive header with auth states
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
- **JWT Token Management**: httpOnly cookies with automatic refresh capability
- **Route Protection**: Middleware-based authentication for protected pages
- **Authentication Store**: Complete Pinia store with login/register/logout operations
- **Auth Plugin**: Client-side initialization after Pinia is ready

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
- **Real-time Display**: Live timer updates every second
- **Public Access**: Shareable URLs for public timer viewing
- **Timer Management**: History, notes, reset functionality
- **State Management**: Complete Pinia store for timer operations

### Design System
Page-specific aesthetic themes per [UX-LAYOUT.md](UX-LAYOUT.md):
- **Homepage**: Sacred/Gothic with construction motifs
- **Authentication**: Clean, minimal with subtle sacred elements
- **Incidents**: Technology theme with geometric patterns
- **About**: Frontier/Nature with Japanese traditional influences
- **Color Palette**: Sky blue primary with gold/silver accents
- **Typography**: Ornate headers, clean body text
- **Responsive Breakpoints**: Content-first approach (320px, 480px, 768px, 1024px, 1440px)

## Architecture Implementation

### HTTP Client Architecture
Modern composable-based HTTP client following Vue 3 conventions:
- **`useAuthFetch()`**: Composable with request/response interceptors
- **Automatic Authentication**: Headers injected automatically from Pinia store
- **Error Handling**: 401 error handling with automatic logout and redirect
- **Service Composables**: `useAuthService()` and `useIncidentTimerService()`
- **Token Management**: Interceptor architecture prepared for token refresh

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