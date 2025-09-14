/**
 * JWT parsing utilities with proper TypeScript typing
 * 
 * Shared between client and server for consistent JWT token handling
 */

export interface JwtPayload {
  exp: number
  iat: number
  sub: string
  [key: string]: any
}

export interface JwtParseResult {
  payload: JwtPayload
  expiration: Date
  isValid: boolean
  error?: string
}

/**
 * Parse a JWT token and extract its payload and expiration
 * 
 * @param token - The JWT token string
 * @returns JwtParseResult with parsed data or error information
 */
export function parseJwtToken(token: string): JwtParseResult {
  try {
    // Validate token format
    const parts = token.split('.')
    if (parts.length !== 3) {
      return {
        payload: {} as JwtPayload,
        expiration: new Date(0),
        isValid: false,
        error: 'Invalid JWT token format - must have 3 parts'
      }
    }

    // Decode the payload (middle part)
    const payload = JSON.parse(atob(parts[1]!)) as JwtPayload

    // Validate required fields
    if (typeof payload.exp !== 'number') {
      return {
        payload: {} as JwtPayload,
        expiration: new Date(0),
        isValid: false,
        error: 'Invalid JWT payload - missing exp field'
      }
    }

    if (typeof payload.iat !== 'number') {
      return {
        payload: {} as JwtPayload,
        expiration: new Date(0),
        isValid: false,
        error: 'Invalid JWT payload - missing iat field'
      }
    }

    if (typeof payload.sub !== 'string') {
      return {
        payload: {} as JwtPayload,
        expiration: new Date(0),
        isValid: false,
        error: 'Invalid JWT payload - missing sub field'
      }
    }

    const expiration = new Date(payload.exp * 1000)

    return {
      payload,
      expiration,
      isValid: true
    }
  } catch (error) {
    return {
      payload: {} as JwtPayload,
      expiration: new Date(0),
      isValid: false,
      error: `Failed to parse JWT token: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

/**
 * Check if a JWT token is expired
 * 
 * @param token - The JWT token string
 * @returns true if the token is expired, false otherwise
 */
export function isJwtExpired(token: string): boolean {
  const result = parseJwtToken(token)
  if (!result.isValid) {
    return true
  }
  
  return new Date() >= result.expiration
}

/**
 * Check if a JWT token is expiring soon (within specified minutes)
 * 
 * @param token - The JWT token string
 * @param minutes - Number of minutes to check ahead (default: 5)
 * @returns true if the token is expiring within the specified time
 */
export function isJwtExpiringSoon(token: string, minutes: number = 5): boolean {
  const result = parseJwtToken(token)
  if (!result.isValid) {
    return true
  }
  
  const timeUntilExpiry = result.expiration.getTime() - Date.now()
  const minutesUntilExpiry = timeUntilExpiry / (1000 * 60)
  
  return minutesUntilExpiry <= minutes
}

/**
 * Get the expiration date of a JWT token
 * 
 * @param token - The JWT token string
 * @returns Date object of expiration or null if invalid
 */
export function getJwtExpiration(token: string): Date | null {
  const result = parseJwtToken(token)
  return result.isValid ? result.expiration : null
}
