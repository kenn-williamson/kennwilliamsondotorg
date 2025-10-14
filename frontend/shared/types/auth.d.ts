declare module '#auth-utils' {
  interface ProfileData {
    real_name?: string
    bio?: string
    avatar_url?: string
    location?: string
    website?: string
  }

  interface ExternalAccount {
    provider: string
    linked_at: string
  }

  interface PreferencesData {
    timer_is_public: boolean
    timer_show_in_list: boolean
  }

  interface User {
    id: string
    email: string
    display_name: string
    slug: string
    roles: string[]
    created_at: string
    email_verified: boolean
    profile?: ProfileData
    external_accounts: ExternalAccount[]
    preferences?: PreferencesData
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
