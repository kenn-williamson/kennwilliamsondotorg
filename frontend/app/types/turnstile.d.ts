// Cloudflare Turnstile SDK Type Definitions

interface TurnstileOptions {
  sitekey: string
  theme?: 'light' | 'dark' | 'auto'
  size?: 'normal' | 'compact' | 'flexible'
  appearance?: 'always' | 'execute' | 'interaction-only'
  tabindex?: number
  callback?: (token: string) => void
  'error-callback'?: () => void
  'expired-callback'?: () => void
  'timeout-callback'?: () => void
  'before-interactive-callback'?: () => void
  'after-interactive-callback'?: () => void
  'unsupported-callback'?: () => void
}

interface Turnstile {
  render: (container: string | HTMLElement, options: TurnstileOptions) => string
  reset: (widgetId: string) => void
  remove: (widgetId: string) => void
  getResponse: (widgetId: string) => string | null
}

declare global {
  interface Window {
    turnstile?: Turnstile
  }
}

export {}
