import { ref, computed, getCurrentInstance, onUnmounted } from 'vue'

export function useBaseService() {
  const backendFetch = useBackendFetch()
  
  // Service state
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const lastFetchTime = ref<Date | null>(null)

  // Computed properties
  const hasError = computed(() => !!error.value)
  const isStale = computed(() => {
    if (!lastFetchTime.value) return true
    return Date.now() - lastFetchTime.value.getTime() > 5 * 60 * 1000 // 5 minutes
  })

  // State management
  const setLoading = (loading: boolean): void => {
    isLoading.value = loading
  }

  const setError = (err: string | null): void => {
    error.value = err
  }

  const setLastFetchTime = (): void => {
    lastFetchTime.value = new Date()
  }

  const clearError = (): void => {
    error.value = null
  }

  const invalidateCache = (): void => {
    lastFetchTime.value = null
  }

  // Simple error handler (can be enhanced later)
  const handleError = (err: any, context?: string): void => {
    const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
    console.error(`[BaseService] Error${context ? ` in ${context}` : ''}:`, errorMessage)
    // TODO: Add toast notifications or other error handling here
  }

  // Simple success handler (can be enhanced later)
  const handleSuccess = (message: string): void => {
    console.log(`[BaseService] Success: ${message}`)
    // TODO: Add toast notifications or other success handling here
  }

  // Wrapper for API calls with error handling
  const executeRequest = async <T>(
    requestFn: () => Promise<T>,
    context?: string
  ): Promise<T> => {
    setLoading(true)
    setError(null)
    
    try {
      const result = await requestFn()
      setLastFetchTime()
      return result
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An unexpected error occurred'
      setError(errorMessage)
      handleError(err, context)
      throw err
    } finally {
      setLoading(false)
    }
  }

  // Wrapper for API calls with success handling
  const executeRequestWithSuccess = async <T>(
    requestFn: () => Promise<T>,
    successMessage: string,
    context?: string
  ): Promise<T> => {
    const result = await executeRequest(requestFn, context)
    handleSuccess(successMessage)
    return result
  }

  // Cleanup on unmount (only if called from a component)
  const instance = getCurrentInstance()
  if (instance) {
    onUnmounted(() => {
      clearError()
    })
  }

  return {
    // State
    isLoading: computed(() => isLoading.value),
    error: computed(() => error.value),
    lastFetchTime: computed(() => lastFetchTime.value),
    hasError,
    isStale,
    
    // State management
    setLoading,
    setError,
    setLastFetchTime,
    clearError,
    invalidateCache,
    
    // Request execution
    executeRequest,
    executeRequestWithSuccess,
    
    // API client access
    backendFetch
  }
}
