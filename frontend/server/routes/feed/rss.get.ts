import { defineEventHandler, setHeader } from 'h3'
import { useRuntimeConfig } from '#imports'

/**
 * GET /feed/rss
 *
 * Proxy RSS feed from backend with proper caching headers.
 */
export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()

  try {
    const response = await fetch(`${config.apiBase}/public/feed/rss`)

    if (!response.ok) {
      throw new Error(`Backend returned ${response.status}`)
    }

    const xml = await response.text()

    setHeader(event, 'Content-Type', 'application/rss+xml; charset=utf-8')
    setHeader(event, 'Cache-Control', 'public, max-age=3600')

    return xml
  } catch (error) {
    console.error('Failed to fetch RSS feed:', error)
    setHeader(event, 'Content-Type', 'text/plain')
    event.node.res.statusCode = 500
    return 'Failed to fetch RSS feed'
  }
})
