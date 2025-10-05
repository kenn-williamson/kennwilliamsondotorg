declare module '#auth-utils' {
  interface User {
    id: string
    email: string
    display_name: string
    slug: string
    roles: string[]
    created_at: string
    email_verified?: boolean
    real_name?: string
    google_user_id?: string
  }

  interface UserSession {
    user: User
    loggedInAt: Date
  }

  interface SecureSessionData {
    jwtToken?: string
    refreshToken?: string
  }
}

export {}
