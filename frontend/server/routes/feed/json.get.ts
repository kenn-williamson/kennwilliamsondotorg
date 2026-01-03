import { defineEventHandler, setHeader } from 'h3'
import { useRuntimeConfig } from '#imports'

/**
 * GET /feed/json
 *
 * Proxy JSON Feed from backend with proper caching headers.
 */
export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()

  try {
    const response = await fetch(`${config.apiBase}/public/feed/json`)

    if (!response.ok) {
      throw new Error(`Backend returned ${response.status}`)
    }

    const json = await response.text()

    setHeader(event, 'Content-Type', 'application/feed+json; charset=utf-8')
    setHeader(event, 'Cache-Control', 'public, max-age=3600')

    return json
  } catch (error) {
    console.error('Failed to fetch JSON feed:', error)
    setHeader(event, 'Content-Type', 'text/plain')
    event.node.res.statusCode = 500
    return 'Failed to fetch JSON feed'
  }
})
