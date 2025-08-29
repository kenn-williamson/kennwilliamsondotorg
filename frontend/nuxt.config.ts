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
  css: ['~/assets/css/main.css'],
  
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
    preset: 'node-server'
  },

  // Build optimization for Docker
  build: {
    transpile: []
  }
})
