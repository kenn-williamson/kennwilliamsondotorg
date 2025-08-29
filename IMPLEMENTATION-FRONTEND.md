# Frontend Implementation Plan - Nuxt.js 3

## Overview
Create fresh Nuxt.js 3 application to replace current Vue.js + Vite app. Clean slate approach for optimal SSR, routing, and full-stack integration.

## Technology Stack
- **Framework**: Nuxt.js 3.x (latest stable)
- **Node.js**: 20+ (even-numbered version recommended)
- **TypeScript**: Full support with strict mode
- **State Management**: Pinia (built-in)
- **Styling**: TailwindCSS + CSS Modules
- **Testing**: Vitest + @nuxt/test-utils

## Project Structure
```
frontend/
├── assets/           # Uncompiled assets
├── components/       # Vue components
├── composables/      # Composition API logic
├── layouts/          # Application layouts
├── middleware/       # Route middleware
├── pages/            # File-based routing
│   ├── login.vue     # Authentication page
│   ├── incident-manager.vue # Protected CRUD management page
│   └── incident-timer/
│       └── [user_slug].vue # Public timer display page
├── plugins/          # Vue plugins
├── public/           # Static assets
├── server/api/       # API routes (proxy to Rust backend)
├── stores/           # Pinia stores
│   ├── auth.ts       # Authentication state
│   └── incident-timers.ts # Timer management state
├── types/            # TypeScript definitions
├── utils/            # Utility functions
├── nuxt.config.ts    # Nuxt configuration
└── package.json      # Dependencies
```

## Key Features
- **Server-Side Rendering**: Full SSR with hydration
- **Authentication**: JWT-based auth with httpOnly cookies and route protection
- **API Integration**: Composables for Rust backend communication
- **Forms**: VeeValidate with Yup validation
- **Responsive Design**: Mobile-first with TailwindCSS
- **SEO**: Meta tags, Open Graph, structured data
- **Incident Timer Features**:
  - Public timer display page accessible via user slug
  - Protected CRUD management interface for authenticated users
  - Real-time timer calculation similar to legacy Vue app

## Docker Configuration
- Multi-stage build (Node 20 Alpine)
- Production optimized output
- Non-root user (security)
- Health check endpoint

## Environment Variables
```env
NUXT_PUBLIC_API_BASE=http://localhost:8080/api  # Development
JWT_SECRET=your-secret-key
```

## Setup Commands
```bash
# Create fresh Nuxt app
npx nuxt@latest init frontend
cd frontend && npm install

# Add dependencies
npm install @nuxtjs/tailwindcss @pinia/nuxt @sidebase/nuxt-auth
npm install @vueuse/nuxt vee-validate @vee-validate/yup yup
npm install -D @nuxt/test-utils vitest happy-dom playwright
```

## Asset Migration
- Copy `vue-project/public/smokey1.png` → `frontend/public/`
- Extract useful CSS patterns for TailwindCSS conversion
- Keep `vue-project/` as reference during development
- Recreate HeaderComponent, CounterBanner with SSR compatibility

## Implementation Benefits
- **Clean architecture**: No legacy Vue/Vite config conflicts
- **Modern conventions**: Proper Nuxt.js structure from start
- **Better Docker**: Single-purpose container optimization
- **Faster development**: No migration overhead, build features immediately