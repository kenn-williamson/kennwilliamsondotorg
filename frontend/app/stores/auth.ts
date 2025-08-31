import { defineStore } from 'pinia'

interface User {
  id: string
  email: string
  display_name: string
  slug: string
  roles: string[]
  created_at: string
}

interface AuthState {
  user: User | null
  token: string | null
  isAuthenticated: boolean
  loading: boolean
}

interface LoginRequest {
  email: string
  password: string
}

interface RegisterRequest {
  email: string
  password: string
  display_name: string
}


export const useAuthStore = defineStore('auth', {
  state: (): AuthState => ({
    user: null,
    token: null,
    isAuthenticated: false,
    loading: false,
  }),

  getters: {
    isAdmin: (state): boolean => {
      return state.user?.roles?.includes('admin') ?? false
    },
    userInitial: (state): string => {
      return state.user?.email?.charAt(0).toUpperCase() ?? 'U'
    },
  },

  actions: {
    async login(credentials: LoginRequest): Promise<{ success: boolean; error?: string }> {
      try {
        this.loading = true
        
        const { authService } = useServices()
        const data = await authService.login(credentials)

        // Store authentication data
        this.token = data.token
        this.user = data.user
        this.isAuthenticated = true

        // Store token in cookie for SSR
        const tokenCookie = useCookie<string | null>('auth-token', {
          default: () => null,
          httpOnly: false,
          secure: true,
          sameSite: 'strict',
          maxAge: 60 * 60 * 24, // 24 hours
        })
        tokenCookie.value = data.token

        return { success: true }
      } catch (error: any) {
        console.error('Login error:', error)
        return {
          success: false,
          error: error.message || 'Login failed. Please check your credentials.',
        }
      } finally {
        this.loading = false
      }
    },

    async register(userData: RegisterRequest, authService: any): Promise<{ success: boolean; error?: string }> {
      try {
        this.loading = true
        
        const data = await authService.register(userData)

        // Store authentication data
        this.token = data.token
        this.user = data.user
        this.isAuthenticated = true

        // Store token in cookie for SSR
        const tokenCookie = useCookie<string | null>('auth-token', {
          default: () => null,
          httpOnly: false,
          secure: true,
          sameSite: 'strict',
          maxAge: 60 * 60 * 24, // 24 hours
        })
        tokenCookie.value = data.token

        return { success: true }
      } catch (error: any) {
        console.error('Registration error:', error)
        return {
          success: false,
          error: error.message || 'Registration failed. Please try again.',
        }
      } finally {
        this.loading = false
      }
    },

    async logout(): Promise<void> {
      try {
        // Clear local state
        this.user = null
        this.token = null
        this.isAuthenticated = false

        // Clear cookie
        const tokenCookie = useCookie<string | null>('auth-token')
        tokenCookie.value = null

        // Optional: Call backend logout endpoint if implemented
        // await $fetch(`${config.public.apiBase}/auth/logout`, { method: 'POST' })
      } catch (error) {
        console.error('Logout error:', error)
        // Even if logout fails on backend, clear local state
        this.user = null
        this.token = null
        this.isAuthenticated = false
      }
    },

    async checkAuth(): Promise<void> {
      try {
        const tokenCookie = useCookie<string | null>('auth-token')
        
        if (!tokenCookie.value) {
          this.clearAuth()
          return
        }

        // Set token from cookie
        this.token = tokenCookie.value

        // Verify token and get user data from backend
        const { authService } = useServices()
        const userData = await authService.me()
        
        // Store user data and mark as authenticated
        this.user = userData
        this.isAuthenticated = true
      } catch (error) {
        console.error('Auth check error:', error)
        // Token is invalid or expired, clear auth state
        this.clearAuth()
      }
    },

    clearAuth(): void {
      this.user = null
      this.token = null
      this.isAuthenticated = false
      
      const tokenCookie = useCookie('auth-token')
      tokenCookie.value = null
    },

    // Get authorization header for API requests
    getAuthHeaders(): Record<string, string> {
      if (!this.token) return {}
      
      return {
        Authorization: `Bearer ${this.token}`
      }
    },
  },
})

// Note: Auth state initialization should be handled in components or plugins
// where Pinia is properly available