export default defineNuxtPlugin(async () => {
  // This plugin runs after Pinia is initialized
  // Wait for the next tick to ensure everything is ready
  await nextTick()
  
  // Initialize auth state
  const authStore = useAuthStore()
  await authStore.checkAuth()
})
