import { defineEventHandler, createError } from 'h3'

export default defineEventHandler(async (event) => {
  try {
    // Get the current user session
    const session = await getUserSession(event)

    if (!session?.user) {
      throw createError({
        statusCode: 401,
        statusMessage: 'Authentication required'
      })
    }

    const refreshToken = session.secure?.refreshToken
    const jwtToken = session.secure?.jwtToken

    return {
      user: {
        id: session.user.id,
        email: session.user.email,
        display_name: session.user.display_name,
        slug: session.user.slug
      },
      tokens: {
        jwt: jwtToken ? `${jwtToken.substring(0, 20)}...` : null,
        refresh: refreshToken ? `${refreshToken.substring(0, 20)}...` : null,
        refresh_full: refreshToken, // Full token for debugging
        refresh_hash: refreshToken ? await hashToken(refreshToken) : null
      },
      session_exists: !!session,
      secure_exists: !!session?.secure
    }
  } catch (error: any) {
    console.error('❌ [Debug Refresh Token] Error:', error.message)
    throw createError({
      statusCode: 500,
      statusMessage: 'Failed to get refresh token info'
    })
  }
})

// Helper function to hash token (same as backend)
async function hashToken(token: string): Promise<string> {
  const encoder = new TextEncoder()
  const data = encoder.encode(token)
  const hashBuffer = await crypto.subtle.digest('SHA-256', data)
  const hashArray = Array.from(new Uint8Array(hashBuffer))
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('')
}
