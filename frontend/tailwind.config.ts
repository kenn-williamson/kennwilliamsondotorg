import type { Config } from 'tailwindcss'

export default {
  content: [
    './app/components/**/*.{js,vue,ts}',
    './app/layouts/**/*.vue',
    './app/pages/**/*.vue',
    './app/plugins/**/*.{js,ts}',
    './app/assets/**/*.css',
    './app.vue',
    './app/error.vue'
  ],
  theme: {
    extend: {
      colors: {
        // PRIMARY BRAND COLORS - Blue spectrum for trust & professionalism
        primary: {
          50: '#eff6ff',   // blue-50 - lightest backgrounds
          100: '#dbeafe',  // blue-100 - light backgrounds
          200: '#bfdbfe',  // blue-200 - borders, dividers
          300: '#93c5fd',  // blue-300 - hover states
          400: '#60a5fa',  // blue-400 - active elements
          500: '#3b82f6',  // blue-500 - buttons, main actions
          600: '#2563eb',  // blue-600 - links, emphasis
          700: '#1d4ed8',  // blue-700 - headings, strong text
          800: '#1e40af',  // blue-800 - dark headings
          900: '#1e3a8a',  // blue-900 - darkest text
        },
        // ACCENT COLORS - Cyan for nautical steampunk feel
        accent: {
          50: '#ecfeff',    // cyan-50 - subtle highlights
          100: '#cffafe',   // cyan-100 - light accents
          200: '#a5f3fc',   // cyan-200 - borders
          300: '#67e8f9',   // cyan-300 - interactive highlights
          400: '#22d3ee',   // cyan-400 - hover states
          500: '#06b6d4',   // cyan-500 - primary accents
          600: '#0891b2',   // cyan-600 - active accents
          700: '#0e7490',   // cyan-700 - dark accents
        },
        // NAUTICAL STEAMPUNK - Slate/Steel tones for industrial aesthetic
        nautical: {
          50: '#f8fafc',     // slate-50 - lightest surfaces
          100: '#f1f5f9',    // slate-100 - light surfaces
          200: '#e2e8f0',    // slate-200 - borders
          300: '#cbd5e1',    // slate-300 - silver highlights
          400: '#94a3b8',    // slate-400 - muted text
          500: '#64748b',    // slate-500 - steel/medium text
          600: '#475569',    // slate-600 - borders, icons
          700: '#334155',    // slate-700 - dark backgrounds
          800: '#1e293b',    // slate-800 - darker backgrounds
          900: '#0f172a',    // slate-900 - darkest backgrounds
        },
        // GOLD - Premium accents (use sparingly)
        gold: {
          50: '#fefce8',      // yellow-50 - subtle gold tint
          100: '#fef9c3',     // yellow-100 - light gold
          200: '#fef08a',     // yellow-200 - medium gold
          300: '#fde047',     // yellow-300 - bright gold
          400: '#fbbf24',     // amber-400 - gold accent
          500: '#f59e0b',     // amber-500 - strong gold
          600: '#ca8a04',     // yellow-600 - dark gold
          700: '#a16207',     // yellow-700 - darker gold
          800: '#854d0e',     // yellow-800 - deep gold
          900: '#713f12',     // yellow-900 - darkest gold
        },
      },
    },
  },
  plugins: [],
} satisfies Config
