export default defineEventHandler(async (event) => {
  console.log('🏥 Health check endpoint called')
  return {
    status: 'ok',
    timestamp: new Date().toISOString(),
    message: 'Server is running'
  }
})
