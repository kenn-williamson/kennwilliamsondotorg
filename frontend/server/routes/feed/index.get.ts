import { defineEventHandler, sendRedirect } from 'h3'

/**
 * GET /feed
 *
 * Redirect to RSS feed (default feed format).
 */
export default defineEventHandler(async (event) => {
  return sendRedirect(event, '/feed/rss', 307)
})
