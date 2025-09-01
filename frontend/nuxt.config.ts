// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  
  // TypeScript configuration
  typescript: {
    strict: true,
    typeCheck: true
  },

  // CSS framework
  // css: ['~/assets/css/main.css'], // Removed - TailwindCSS handles this
  
  // Components auto-import with nested directory support
  components: [
    {
      path: '~/components',
      pathPrefix: false
    }
  ],
  
  // Modules
  modules: [
    '@nuxtjs/tailwindcss',
    '@pinia/nuxt',
    '@vueuse/nuxt'
  ],

  // Runtime config
  runtimeConfig: {
    // Server-side environment variables
    jwtSecret: process.env.JWT_SECRET,
    
    public: {
      // Client-side environment variables
      apiBase: process.env.NUXT_PUBLIC_API_BASE || 'http://localhost:8080/api',
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
    }
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
