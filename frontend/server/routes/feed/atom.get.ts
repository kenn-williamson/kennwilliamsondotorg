import { defineEventHandler, setHeader } from 'h3'
import { useRuntimeConfig } from '#imports'

/**
 * GET /feed/atom
 *
 * Proxy Atom feed from backend with proper caching headers.
 */
export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()

  try {
    const response = await fetch(`${config.apiBase}/public/feed/atom`)

    if (!response.ok) {
      throw new Error(`Backend returned ${response.status}`)
    }

    const xml = await response.text()

    setHeader(event, 'Content-Type', 'application/atom+xml; charset=utf-8')
    setHeader(event, 'Cache-Control', 'public, max-age=3600')

    return xml
  } catch (error) {
    console.error('Failed to fetch Atom feed:', error)
    setHeader(event, 'Content-Type', 'text/plain')
    event.node.res.statusCode = 500
    return 'Failed to fetch Atom feed'
  }
})
