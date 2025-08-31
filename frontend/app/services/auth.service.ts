import { BaseService } from './base.service'

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

export class AuthService extends BaseService {
  constructor(apiBase: string) {
    super(apiBase)
  }

  async login(credentials: LoginRequest): Promise<AuthResponse> {
    return this.makeRequest<AuthResponse>('/auth/login', {
      method: 'POST',
      body: credentials,
    })
  }

  async register(userData: RegisterRequest): Promise<AuthResponse> {
    return this.makeRequest<AuthResponse>('/auth/register', {
      method: 'POST',
      body: userData,
    })
  }

  async logout(): Promise<void> {
    // Optional: implement backend logout endpoint when available
    // return this.makeRequest<void>('/auth/logout', {
    //   method: 'POST',
    //   headers: this.getAuthHeaders(),
    // })
  }

  async me(): Promise<User> {
    return this.makeRequest<User>('/auth/me', {
      method: 'GET',
    })
  }

  async previewSlug(displayName: string): Promise<SlugPreviewResponse> {
    return this.makeRequest<SlugPreviewResponse>('/auth/preview-slug', {
      method: 'POST',
      body: { display_name: displayName },
    })
  }
}

// Export factory function instead of singleton
export function createAuthService(apiBase: string): AuthService {
  return new AuthService(apiBase)
}