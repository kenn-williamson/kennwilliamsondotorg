import { getRequestIP, getHeader } from 'h3'

export interface ClientInfo {
  ip: string
  userAgent: string
  protocol: string
}

/**
 * Extract complete client information from Nuxt event for proper IP forwarding
 * Returns IP, User-Agent, and protocol for backend API calls
 */
export function getClientInfo(event: any): ClientInfo {
  return {
    ip: getRequestIP(event) || 'Unknown',
    userAgent: getHeader(event, 'user-agent') || 'Unknown',
    protocol: getHeader(event, 'x-forwarded-proto') || 
              (event.node.req.connection?.encrypted ? 'https' : 'http')
  }
}
