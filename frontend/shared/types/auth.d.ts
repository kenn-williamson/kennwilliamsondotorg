declare module '#auth-utils' {
  interface User {
    id: string
    email: string
    display_name: string
    slug: string
    roles: string[]
    created_at: string
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
