import Redis from 'ioredis'

// Redis client for rate limiting
let redis: Redis | null = null

export function getRedisClient(): Redis {
  if (!redis) {
    const redisUrl = process.env.REDIS_URL || 'redis://redis:6379'
    redis = new Redis(redisUrl, {
      maxRetriesPerRequest: 3,
      lazyConnect: true,
    })
  }
  return redis
}

export interface RateLimitConfig {
  requestsPerHour: number
  burstLimit: number
  burstWindow: number // seconds
}

export const RATE_LIMIT_CONFIGS: Record<string, RateLimitConfig> = {
  register: {
    requestsPerHour: 3,
    burstLimit: 2,
    burstWindow: 300, // 5 minutes
  },
  login: {
    requestsPerHour: 10,
    burstLimit: 3,
    burstWindow: 300, // 5 minutes
  },
  phrases: {
    requestsPerHour: 50,
    burstLimit: 10,
    burstWindow: 600, // 10 minutes
  },
  general: {
    requestsPerHour: 200,
    burstLimit: 30,
    burstWindow: 300, // 5 minutes
  },
}

export async function checkRateLimit(
  identifier: string,
  endpoint: string,
  config: RateLimitConfig
): Promise<boolean> {
  try {
    const client = getRedisClient()
    
    // Check hourly limit
    const hourlyKey = `rate_limit:ssr:hourly:${identifier}:${endpoint}`
    const hourlyCount = await client.get(hourlyKey)
    
    if (hourlyCount && parseInt(hourlyCount) >= config.requestsPerHour) {
      console.warn(`SSR Rate limit exceeded for ${identifier} on ${endpoint}: ${hourlyCount} requests/hour`)
      return true
    }

    // Check burst limit
    const burstKey = `rate_limit:ssr:burst:${identifier}:${endpoint}`
    const burstCount = await client.get(burstKey)
    
    if (burstCount && parseInt(burstCount) >= config.burstLimit) {
      console.warn(`SSR Burst limit exceeded for ${identifier} on ${endpoint}: ${burstCount} requests/${config.burstWindow}s`)
      return true
    }

    return false
  } catch (error) {
    console.error('SSR Rate limit check failed:', error)
    return false // Fail open for SSR
  }
}

export async function incrementRateLimit(
  identifier: string,
  endpoint: string,
  config: RateLimitConfig
): Promise<void> {
  try {
    const client = getRedisClient()
    
    // Increment hourly counter
    const hourlyKey = `rate_limit:ssr:hourly:${identifier}:${endpoint}`
    await client.incr(hourlyKey)
    await client.expire(hourlyKey, 3600) // 1 hour
    
    // Increment burst counter
    const burstKey = `rate_limit:ssr:burst:${identifier}:${endpoint}`
    await client.incr(burstKey)
    await client.expire(burstKey, config.burstWindow)
  } catch (error) {
    console.error('SSR Rate limit increment failed:', error)
  }
}

export function getClientIdentifier(event: any): string {
  // For now, use IP-based identification
  // TODO: Add session-based user identification
  const ip = event.node.req.socket.remoteAddress || 'unknown'
  const userAgent = event.node.req.headers['user-agent'] || 'unknown'
  return `ip:${ip}:${userAgent.slice(0, 50)}`
}

export function getEndpointType(path: string): string {
  if (path.includes('/auth/register')) {
    return 'register'
  } else if (path.includes('/auth/login')) {
    return 'login'
  } else if (path.includes('/phrase')) {
    return 'phrases'
  } else {
    return 'general'
  }
}

export async function rateLimitMiddleware(
  event: any,
  endpoint: string
): Promise<boolean> {
  const identifier = getClientIdentifier(event)
  const endpointType = getEndpointType(endpoint)
  
  const config = RATE_LIMIT_CONFIGS[endpointType]
  if (!config) {
    console.warn(`No rate limit config for endpoint: ${endpointType}`)
    return false
  }
  
  // Check rate limit
  const isLimited = await checkRateLimit(identifier, endpointType, config)
  
  if (isLimited) {
    return true // Rate limited
  }
  
  // Increment counters
  await incrementRateLimit(identifier, endpointType, config)
  
  return false // Not rate limited
}
