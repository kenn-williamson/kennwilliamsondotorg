/**
 * useAuthService - Authentication operations using composable pattern
 * 
 * Replaces AuthService class with composable function.
 * Uses useAuthFetch for automatic authentication handling.
 */

interface User {
  id: string
  email: string
  display_name: string
  slug: string
  roles: string[]
  created_at: string
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

interface SlugPreviewRequest {
  display_name: string
}

interface SlugPreviewResponse {
  slug: string
  available: boolean
  final_slug: string
}

interface AuthResponse {
  token: string
  user: User
}

export function useAuthService() {
  const config = useRuntimeConfig()
  const authFetch = useAuthFetch()

  // Helper to create full API URL
  const apiUrl = (endpoint: string) => `${config.public.apiBase}${endpoint}`

  return {
    async login(credentials: LoginRequest): Promise<AuthResponse> {
      return authFetch<AuthResponse>(apiUrl('/auth/login'), {
        method: 'POST',
        body: credentials,
      })
    },

    async register(userData: RegisterRequest): Promise<AuthResponse> {
      return authFetch<AuthResponse>(apiUrl('/auth/register'), {
        method: 'POST',
        body: userData,
      })
    },

    async me(): Promise<User> {
      return authFetch<User>(apiUrl('/auth/me'), {
        method: 'GET',
      })
    },

    async previewSlug(displayName: string): Promise<SlugPreviewResponse> {
      return authFetch<SlugPreviewResponse>(apiUrl('/auth/preview-slug'), {
        method: 'POST',
        body: { display_name: displayName },
      })
    },

    // Future: logout endpoint when implemented
    async logout(): Promise<void> {
      // Optional: implement backend logout endpoint when available
      // return authFetch(apiUrl('/auth/logout'), {
      //   method: 'POST',
      // })
    },
  }
}