import { defineEventHandler, getRequestURL } from 'h3'

console.log('🔧 Loading test.get.ts API endpoint...')

export default defineEventHandler(async (event: any) => {
  const url = getRequestURL(event)
  console.log('🎯 Test API endpoint called:', event.method, url.toString())
  return {
    message: 'Hello World!',
    timestamp: new Date().toISOString(),
    method: event.method,
    url: url.toString()
  }
})
