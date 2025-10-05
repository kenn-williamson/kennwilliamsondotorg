// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  future: {
    compatibilityVersion: 4
  },
  debug: true,
  devtools: { 
    enabled: true,
  },
  // TypeScript configuration
  typescript: {
    strict: true,
    typeCheck: true
  },

  
  // Components auto-import with nested directory support
  components: [
    {
      path: '~/components',
      pathPrefix: false
    }
  ],
  
  // Modules
  modules: ['@nuxtjs/tailwindcss', '@pinia/nuxt', '@vueuse/nuxt', 'nuxt-auth-utils', '@nuxt/test-utils/module'],

  // Server configuration
  serverDir: 'server',

  // Runtime config
  runtimeConfig: {
    // Server-side environment variables
    jwtSecret: process.env.JWT_SECRET,
    session: {
      maxAge: 60 * 60 * 24 * 7 // 1 week
    },
    // Server-side API base URL (for internal Docker network)
    apiBase: process.env.NUXT_API_BASE || 'http://backend:8080/backend',
    
    public: {
      // Client-side environment variables
      apiBase: process.env.NUXT_PUBLIC_API_BASE || 'https://localhost/backend',
      appName: 'KennWilliamson.org'
    }
  },

  // SSR Configuration
  ssr: true,
  nitro: {
    preset: 'node-server',
    minify: false,
    experimental: {
      wasm: false
    },
    // Ensure API routes are properly handled
    routeRules: {
      '/api/**': { cors: true }
    },
    // Standard logging level
    logLevel: 2 // 0=silent, 1=error, 2=warn, 3=info, 4=verbose
  },

  // Build optimization for Docker
  build: {
    transpile: []
  },

  // Development server configuration for containers
  devServer: {
    host: '0.0.0.0',  // Required for container access
    port: 3000
  },


  // Vite configuration for hot reload in containers
  vite: {
    server: {
      hmr: {
        protocol: 'wss',    // WebSocket Secure for HTTPS
        clientPort: 443,    // Connect to nginx HTTPS port
        path: '/_nuxt/hmr'  // Dedicated HMR path
      }
    }
  },

  // App configuration
  app: {
    head: {
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      link: [
        { rel: 'icon', type: 'image/png', sizes: '32x32', href: '/favicon-small.png' },
        { rel: 'icon', type: 'image/png', sizes: '192x192', href: '/favicon-large.png' },
        { rel: 'apple-touch-icon', sizes: '192x192', href: '/favicon-large.png' }
      ]
    }
  }
})