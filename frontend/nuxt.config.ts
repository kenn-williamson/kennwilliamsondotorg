// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  future: {
    compatibilityVersion: 4
  },
  devtools: {
    enabled: true,
  },
  // TypeScript configuration
  typescript: {
    strict: true,
    typeCheck: true,
    tsConfig: {
      exclude: ['**/*.spec.ts', '**/*.test.ts', '**/test/**', '**/tests/**']
    }
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
      appName: 'KennWilliamson.org',
      // Cloudflare Turnstile site key (public, safe to expose)
      turnstileSiteKey: process.env.NUXT_PUBLIC_TURNSTILE_SITE_KEY || '1x00000000000000000000AA'
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
    },
  },

  // App configuration
  app: {
    head: {
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' },
        { rel: 'icon', type: 'image/png', sizes: '16x16', href: '/favicon-16x16.png' },
        { rel: 'icon', type: 'image/png', sizes: '32x32', href: '/favicon-32x32.png' },
        { rel: 'icon', type: 'image/png', sizes: '192x192', href: '/favicon-192x192.png' },
        { rel: 'icon', type: 'image/png', sizes: '512x512', href: '/favicon-512x512.png' },
        { rel: 'apple-touch-icon', sizes: '180x180', href: '/apple-touch-icon.png' }
      ],
      script: [
        {
          src: 'https://challenges.cloudflare.com/turnstile/v0/api.js',
          async: true,
          defer: true
        }
      ]
    }
  }
})