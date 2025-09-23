import { defineConfig } from 'vitest/config'
import { resolve } from 'path'

export default defineConfig({
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./test/setup.ts'],
    coverage: {
      provider: 'istanbul',
      reporter: ['text', 'html', 'json'],
      include: [
        'app/composables/**/*.ts',
        'app/services/**/*.ts',
        'app/stores/**/*.ts',
        'app/utils/**/*.ts',
        'shared/**/*.ts',
      ],
      exclude: [
        'node_modules/',
        'test/',
        '**/*.d.ts',
        '**/*.config.*',
        '**/coverage/**',
        '**/*.vue',
        '**/pages/**',
        '**/components/**',
        '**/layouts/**',
        '**/plugins/**',
        '**/middleware/**',
        '**/server/**',
      ],
    },
  },
  resolve: {
    alias: {
      '~': resolve(__dirname, './app'),
      '#shared': resolve(__dirname, './shared'),
      '@': resolve(__dirname, './app'),
    },
  },
  define: {
    __VUE_OPTIONS_API__: true,
    __VUE_PROD_DEVTOOLS__: false,
  },
})
